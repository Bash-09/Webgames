use axum::{Router, routing::get};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::config::Config;

pub async fn serve(config: Config, db: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(async || "Hello World"));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
