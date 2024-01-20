use axum::Router;
use tower_http::services::ServeDir;

pub fn static_roouter() -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("dist"))
        .nest_service("/assets", ServeDir::new("assets"))
}
