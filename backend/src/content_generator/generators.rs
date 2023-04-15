use super::content::*;
use crate::content_generator::text::text_query;
use crate::media::transcribe::transcribe_video;
use crate::prelude::*;

use async_openai::Client;
use futures::future::join_all;
use std::fs;
use std::path::{Path, PathBuf};

const OPENAI_KEY: &str = "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";

impl Content {
    // Creates new video content from input lecture video
    pub async fn new(video_path: PathBuf) -> Result<Self> {
        tracing::debug!("Producing new content for {video_path:?}");

        let file_name = video_path
            .clone()
            .file_name()
            .ok_or(Error::FileInfo(video_path.clone()))?
            .to_str()
            .ok_or(Error::FileInfo(video_path.clone()))?
            .to_owned();
        let output_path = PathBuf::from(&format!("./output/{file_name}/"));

        let full_video_path = output_path.clone().join("video.mp4");
        let full_data_path = output_path.clone().join("data.json");
        let full_audio_path = output_path.clone().join("audio.wav");

        // Copy video to the upload/processing directory
        fs::copy(&video_path, &full_video_path)?;

        // Connect to OpenAI API
        let client = Client::new().with_api_key(OPENAI_KEY);
        tracing::debug!("Connected to OpenAI API");

        // Generate a transcript from the video
        // Generate raw audio from video
        let transcription = transcribe_video(&client, &video_path, &full_audio_path).await?;
        let summary = text_query(&client, &format!("Make a summary of the sentence \"{}\" in under 20 words? only write the summary, no extra words.", transcription)).await?;

        // Get sentences to make clips by separating by punctuation
        let transcript_for_sentences = transcription.clone();
        let sentences: Vec<_> = transcript_for_sentences
            .split(['.', '?', '!'])
            .map(String::from)
            .collect();
        let sentences: Vec<(String, String, String)> = sentences.iter().map(|sentence|
                                                                                  (sentence.to_owned(),
                                                                                  format!("What is the topic of the sentence \"{sentence}\" in under 5 words? only write the topic, no extra words."),
                                                                                  format!("What is the summary of the sentence \"{sentence}\" in under 5 words? only write the summary, no extra words.")
            )).collect();
        let query_responses = sentences.into_iter().map(|(sentence, q1, q2)| {
            let client = client.clone();

            tokio::spawn(async move {
                (
                    sentence.to_owned(),
                    text_query(&client, &q1).await,
                    text_query(&client, &q2).await,
                )
            })
        });
        let query_responses = join_all(query_responses).await;

        let mut clips = Vec::new();

        for i in 0..query_responses.len() {
            if let Ok(query) = &query_responses[i] {
                if let (Ok(subject), Ok(summary)) = (&query.1, &query.2) {
                    clips.push(Clips {
                        transcript: query.0.to_owned(),
                        subject: subject.to_owned(),
                        summary: summary.to_owned(),
                        video_path: String::new(),
                        images_paths: Vec::new(),
                        question: Question {
                            question: String::new(),
                            options: Vec::new(),
                            correct_option_index: 0,
                        },
                    })
                }
            }
        }

        // Construct the content from the information we have
        let content = Self {
            video_path: full_video_path
                .to_str()
                .ok_or(Error::FileInfo(video_path.clone()))?
                .to_owned(),
            audio_path: full_audio_path
                .to_str()
                .ok_or(Error::FileInfo(video_path.clone()))?
                .to_owned(),
            raw_transcript: transcription.to_owned(),
            summary: String::new(),
            clips,
        };

        // Save to disk!!!
        fs::write(&full_data_path, serde_json::to_string(&content)?)?;
        tracing::debug!("Saved content data to {full_data_path:?}");

        // Keep content in memory, in case we need to quickly access it afterwards
        Ok(content)
    }
}
