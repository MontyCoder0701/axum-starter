use axum::Json;

pub fn get_greeting() -> Json<String> {
    let greeting = "Hello, World!".to_string();
    Json(greeting)
}
