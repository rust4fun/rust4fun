use rust_study_shared as shared;

pub mod router;

use axum::{response::IntoResponse, Json};
use shared::Article;

#[utoipa::path(
    get,
    path = "/hello",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "List all todos successfully", body = Article)
    ),
    tag = "hello",
)]
pub async fn comment() -> impl IntoResponse {
    let article = Article::new("rust の 勉強会".to_string());

    Json(article)
}
