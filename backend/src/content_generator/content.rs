use serde::{Deserialize, Serialize};

// Stores question data about a video short
#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub options: Vec<String>,
    pub correct_option_index: usize,
}

// Holds short video information generated from large content
#[derive(Debug, Serialize, Deserialize)]
pub struct Clips {
    pub transcript: String,
    pub subject: String,
    pub summary: String,
    pub video_path: String,
    pub images_paths: Vec<String>,
    pub question: Question,
}

// Holds all content information in one place for easy access
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub video_path: String,
    pub audio_path: String,
    pub raw_transcript: String,
    pub summary: String,
    pub clips: Vec<Clips>,
}
