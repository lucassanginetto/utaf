mod handlers;
mod models;
mod scrapers;
mod views;

use std::{
    env::{self, VarError},
    error::Error,
};

use axum::{Router, routing};
use axum_embed::ServeEmbed;
use rust_embed::RustEmbed;
use tokio::net::TcpListener;

const DEFAULT_PORT: u16 = 8080;

#[derive(RustEmbed, Clone)]
#[folder = "static/"]
struct StaticAssets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    let port = env::var("PORT")
        .map_or_else(|var_error| {
            match var_error {
                VarError::NotPresent => tracing::info!(
                    "the PORT environment variable wasn't found. Using default port {}",
                    DEFAULT_PORT
                ),
                VarError::NotUnicode(_) => tracing::warn!(
                    "the PORT environment variable doesn't contain valid unicode data. Ignoring and using default port {}",
                    DEFAULT_PORT
                ),
            };
            DEFAULT_PORT
        }, |string| string.parse().unwrap_or_else(|_| {
            tracing::warn!("the PORT environment variable doesn't contain a valid port number. Ignoring and using default port {}", DEFAULT_PORT);
            DEFAULT_PORT
        }));

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    tracing::info!("listening on port {port}");

    axum::serve(
        listener,
        Router::new()
            .route("/", routing::get(handlers::home))
            .route("/robots.txt", routing::get(handlers::robots_txt))
            .route("/artist/lyric/{artist_id}", routing::get(handlers::artist))
            .route("/lyric/{song_id}/", routing::get(handlers::lyrics))
            .route(
                "/songWriter/{song_writer_id}/",
                routing::get(handlers::song_writer),
            )
            .route("/search", routing::get(handlers::search))
            .nest_service("/static", ServeEmbed::<StaticAssets>::new())
            .route("/{*key}", routing::get(handlers::not_found)),
    )
    .await?;

    Ok(())
}
