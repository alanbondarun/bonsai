use anyhow::Result;
use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app()).await?;
    Ok(())
}

fn app() -> Router {
    Router::new().route("/", get(|| async { "Hello, world!" }))
}
