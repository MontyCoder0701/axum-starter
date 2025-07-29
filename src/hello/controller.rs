use axum::{Router, routing::get};
use deadpool_diesel::mysql::Pool;

use super::service::get_greeting;

const PATH: &'static str = "/";

pub fn routes() -> Router<Pool> {
    return Router::new().route(PATH, get(get_greeting()));
}
