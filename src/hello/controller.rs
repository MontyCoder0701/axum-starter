use axum::{Router, routing::get};
use deadpool_diesel::mysql::Pool;

use super::service::get_greeting;

const PATH: &str = "/";

pub fn routes() -> Router<Pool> {
    Router::new().route(PATH, get(get_greeting()))
}
