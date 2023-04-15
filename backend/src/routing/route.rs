use crate::content_generator::content::*;

use axum::{
    extract::Multipart,
    routing::{get, post},
    Router,
};

pub async fn is_alive() -> &'static str {
    "true"
}

pub async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };
        let data = field.bytes().await.unwrap();

        if data.len() > 256 {
            let video_path = format!("./output/processing/{filename}");
            std::fs::write(&video_path, data).unwrap();
            Content::new(video_path.into()).await.unwrap();
        } else {
            panic!("Upload failed!")
        }
    }
}

pub fn build_router() -> Router {
    Router::new()
        .route("/api/is_alive", get(is_alive))
        .route("/api/upload", post(upload))
}
