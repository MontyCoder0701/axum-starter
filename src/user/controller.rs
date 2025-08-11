use axum::{
    Router,
    routing::{MethodRouter, get},
};
use deadpool_diesel::mysql::Pool;

use super::service::UserService;

pub struct UserController {
    path: String,
}

impl UserController {
    pub fn new() -> Self {
        Self {
            path: "/users".to_string(),
        }
    }

    pub fn routes(&self) -> Router<Pool> {
        Router::new().route(&self.path, Self::get())
    }

    pub fn get() -> MethodRouter<Pool> {
        get(UserService::get)
    }
}
