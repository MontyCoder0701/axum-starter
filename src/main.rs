use std::env;

use axum::{
    Router,
    http::{
        HeaderValue, Method, StatusCode,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use deadpool_diesel::{
    Runtime,
    mysql::{Manager, Pool},
};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::{base::controller::BaseController, user::controller::UserController};

mod base;
mod common;
mod hello;
mod user;

#[tokio::main]
async fn main() {
    dotenv().ok();

    const PORT: u16 = 3200;
    const HOST: &str = "localhost";

    let db_url = env::var("DATABASE_URL").unwrap();
    let client_url = env::var("CLIENT_URL").unwrap();

    let manager = Manager::new(db_url, Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

    let cors = CorsLayer::new()
        .allow_origin(client_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = Router::new()
        .merge(hello::controller::routes())
        .merge(UserController::routes())
        .layer(cors)
        .with_state(pool)
        .fallback((StatusCode::NOT_FOUND, "404 Not Found"));

    let listener = TcpListener::bind(format!("{HOST}:{PORT}")).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
