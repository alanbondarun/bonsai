use anyhow::{Context, Result};
use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use tokio::net::TcpListener;

use crate::handler::test_db;

mod error;
mod handler;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().map(|_| ()).unwrap_or_else(|_| {
        println!("Failed to load .env file. Skipping...");
    });

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let db_conn = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("Cannot connect to database")?;

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app(db_conn)).await?;

    Ok(())
}

fn app(db_conn: PgPool) -> Router {
    Router::new().route("/", get(test_db)).with_state(db_conn)
}
