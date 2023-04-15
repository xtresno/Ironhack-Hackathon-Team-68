use super::convert;
use crate::prelude::*;

use async_openai::{
    types::{AudioInput, AudioResponseFormat, CreateTranscriptionRequestArgs},
    Audio, Client,
};
use serde_json::Value;
use std::path::{Path, PathBuf};

async fn transcribe(client: &Client, wav_file: &PathBuf) -> Result<String> {
    let request = CreateTranscriptionRequestArgs::default()
        .file(AudioInput::new(Path::new(wav_file)))
        .model("whisper-1")
        .response_format(AudioResponseFormat::VerboseJson)
        .language("en")
        .prompt("Transcribe the video, and also give the timestamps of each sentence.")
        .build()
        .or(Err(Error::Transcribe(wav_file.to_owned())))?;

    let transcribe_result = Audio::new(client).transcribe(request).await?;

    tracing::debug!("Finished transcribing {wav_file:?}");

    Ok(transcribe_result.text)
}

pub async fn transcribe_video(
    client: &Client,
    video_path: &PathBuf,
    audio_path: &PathBuf,
) -> Result<String> {
    // Don't transcribe video if it s already completed
    // if Path::new(video_path).exists() {
    //     return Ok(PathBuf::from(video_path));
    // }

    // Decode video audio into WAV file raw audio
    convert::decode_video(video_path, audio_path).await?;

    // Generate transcription using openai
    transcribe(&client, audio_path).await
}
