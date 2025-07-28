use std::env;

use axum::{Router, routing::get};
use deadpool_diesel::{
    Runtime,
    mysql::{Manager, Pool},
};
use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    let manager = Manager::new(db_url, Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(pool);

    let listener = TcpListener::bind("localhost:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
