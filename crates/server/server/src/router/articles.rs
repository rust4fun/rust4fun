use rust_study_db_connector as db;
use rust_study_shared as shared;
use utoipa::ToSchema;

use crate::State;
use axum::{
    extract::Path,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use webpage::{Webpage, WebpageOptions};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Article(shared::Article);

pub fn router() -> Router {
    Router::new()
        .route("/", post(create).get(list))
        .route("/:id", get(get_item))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct RequestBody {
    pub url: String,
}

#[utoipa::path(
    post,
    path = "/articles",
    context_path = "/api/v1",
    request_body = RequestBody,
    responses(
        (status = 200, description = "create article record", body = Article)
    ),
    tag = "articles",
)]
pub async fn create(state: Extension<Arc<State>>, Json(body): Json<RequestBody>) -> Json<Article> {
    let info =
        Webpage::from_url(&body.url, WebpageOptions::default()).expect("Could not read from URL");

    let html = info.html;

    let image_url = html
        .opengraph
        .images
        .first()
        .map(|x| x.url.clone())
        .unwrap_or("https://www.rust-lang.org/static/images/rust-social-wide.jpg".to_string());

    let repo = db::ArticleRepository::new(state.db());

    let id = db::ArticleId::new_v4();
    let input = db::InputArticleEntity::new(
        body.url,
        html.title,
        html.description,
        image_url,
        Uuid::parse_str("ab48dc58-68b5-43d2-9751-1228f334e253")
            .unwrap()
            .into(),
    );

    repo.create(id.clone(), input).await;

    let article = repo.find_by_id(id).await;

    Json(article.into())
}

#[utoipa::path(
    get,
    path = "/articles/{id}",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "get article record", body = Article)
    ),
    params(
        ("id" = Uuid, Path, description = "article id"),
    ),
    tag = "articles",
)]
pub async fn get_item(state: Extension<Arc<State>>, Path(id): Path<Uuid>) -> Json<Article> {
    let repo = db::ArticleRepository::new(state.db());
    let article = repo.find_by_id(id.into()).await;

    Json(article.into())
}

#[utoipa::path(
    get,
    path = "/articles",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "get article records", body = Vec<Article>)
    ),
    tag = "articles",
)]
pub async fn list(state: Extension<Arc<State>>) -> Json<Vec<Article>> {
    let repo = db::ArticleRepository::new(state.db());
    let article = repo.list().await;

    Json(article.into_iter().map(|x| x.into()).collect())
}

impl From<db::ArticleEntity> for Article {
    fn from(value: db::ArticleEntity) -> Self {
        let article = shared::Article::new(
            value.id.id(),
            value.url,
            value.title,
            value.description,
            value.image_url,
        );

        Article(article)
    }
}
