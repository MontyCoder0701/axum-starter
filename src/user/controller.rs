use axum::{Router, routing::get};
use deadpool_diesel::mysql::Pool;

use super::service::get_users;

const PATH: &str = "/users";

pub fn routes() -> Router<Pool> {
    Router::new().route(PATH, get(get_users))
}
