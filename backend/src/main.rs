mod content_generator;
mod media;
mod prelude;
mod routing;

use std::net::SocketAddr;

use async_openai::Client;
use axum::Server;
use media::transcribe::transcribe_video;
use routing::route::build_router;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = build_router();

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("Listening on {}", address);

    Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
