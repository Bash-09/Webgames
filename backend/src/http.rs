use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::{
    api,
    config::Config,
    site::{self, UISource},
};

// pub async fn serve(config: Config, db: PgPool) -> Result<(), Box<dyn std::error::Error>> {
pub async fn serve(config: Config, db: Option<PgPool>) -> Result<(), Box<dyn std::error::Error>> {
    let ui_source = if config.no_ui {
        UISource::None
    } else if let Some(dynamic_path) = config.frontend_path {
        site::UISource::Dynamic(dynamic_path)
    } else {
        site::UISource::bundled()
    };

    let app = Router::new()
        .merge(site::router(ui_source))
        .merge(api::router(db));

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
