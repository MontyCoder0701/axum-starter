use axum::{Router, routing::get};
use deadpool_diesel::mysql::Pool;

pub fn routes() -> Router<Pool> {
    Router::new().route("/", get(super::service::get_greeting()))
}
