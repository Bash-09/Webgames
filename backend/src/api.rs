use axum::{
    Router,
    response::{IntoResponse, Response},
    routing::get,
};
use sqlx::PgPool;

pub fn router(_db: Option<PgPool>) -> Router {
    Router::new().route("/api/get_test", get(get_test))
}

async fn get_test() -> Response {
    "Get tested :sunglasses:".into_response()
}
