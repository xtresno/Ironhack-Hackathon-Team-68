use std::path::PathBuf;

use async_openai::error::OpenAIError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to decode {0} into wav")]
    DecodeToWav(PathBuf),
    #[error("Failed to get raw samples from wav {0}")]
    ExtractSamples(PathBuf),
    #[error("Failed to transcribe {0}")]
    Transcribe(PathBuf),
    #[error("{0}")]
    OpenAI(#[from] OpenAIError),
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to get file information for {0}")]
    FileInfo(PathBuf),
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Failed to query OpenAi")]
    QueryOpenAi,
}
