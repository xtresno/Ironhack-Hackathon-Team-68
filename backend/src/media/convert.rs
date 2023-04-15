use crate::prelude::*;

use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Command,
    process::Stdio,
};

pub async fn decode_video(video_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    // Execute ffmpeg, to decode mp4 and produce wav from audio
    let cmd = Command::new("ffmpeg")
        .args([
            // Specify input path
            "-i",
            video_path
                .to_str()
                .ok_or(Error::DecodeToWav(video_path.to_owned()))?,
            // Say yes to prompts (forcefully override file)
            "-y",
            // Specify output paths
            output_path
                .to_str()
                .ok_or(Error::DecodeToWav(video_path.to_owned()))?,
        ])
        .stdout(Stdio::piped())
        .spawn()
        .or(Err(Error::DecodeToWav(video_path.to_owned())))?;

    // Get ffmpeg output
    let output = cmd
        .wait_with_output()
        .or(Err(Error::DecodeToWav(video_path.to_owned())))?;

    if output.status.success() {
        tracing::debug!("Finished decoding {video_path:?}");
        Ok(())
    } else {
        Err(Error::DecodeToWav(video_path.to_owned()))
    }
}
