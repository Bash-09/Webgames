use axum::Router;
use sqlx::PgPool;

pub fn router(_db: Option<PgPool>) -> Router {
    Router::new()
}
