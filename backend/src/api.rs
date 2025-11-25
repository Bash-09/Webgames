use axum::{
    Router,
    response::{IntoResponse, Response},
    routing::get,
};
use sqlx::PgPool;

pub fn router(_db: Option<PgPool>) -> Router {
    Router::new().route("/api/ping", get(get_ping))
}

async fn get_ping() -> Response {
    "Get ponged :sunglasses:".into_response()
}
