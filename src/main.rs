use std::env;

use axum::{
    Router,
    http::{HeaderValue, Method, header::AUTHORIZATION, header::CONTENT_TYPE},
};
use deadpool_diesel::{
    Runtime,
    mysql::{Manager, Pool},
};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod common;
mod hello;
mod user;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let manager = Manager::new(db_url, Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = Router::new()
        .merge(hello::controller::routes())
        .merge(user::controller::routes())
        .layer(cors)
        .with_state(pool);

    let listener = TcpListener::bind("localhost:3200").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
