use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn static_roouter() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback_service(
            ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
        )
}
