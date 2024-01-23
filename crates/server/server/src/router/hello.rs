use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/", get(get_hello_world))
}

pub async fn get_hello_world() -> String{
    String::from("Hello World!")
}