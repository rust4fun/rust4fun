use rust_study_db_connector as db;
use rust_study_shared as shared;

use crate::error::Error;
use crate::model::AuthUser;
use crate::State;
use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
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
        (status = 200, description = "create article record", body = Article),
        (status = 401, description = "unauhtorization")
    ),
    tag = "articles",
)]
pub async fn create(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Json(body): Json<RequestBody>,
) -> Result<impl IntoResponse, Error> {
    // TODO: この処理をモジュールにまとめる
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
        auth_user.id.clone(),
    );
    repo.create(id.clone(), input).await?;
    let article = repo.find_by_id(id).await?;

    Ok(Json(Article::from(article)))
}

#[utoipa::path(
    get,
    path = "/articles/{id}",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "get article record", body = Article),
        (status = 401, description = "unauhtorization"),
        (status = 404, description = "not found")
    ),
    params(
        ("id" = Uuid, Path, description = "article id"),
    ),
    tag = "articles",
)]
pub async fn get_item(
    _auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let repo = db::ArticleRepository::new(state.db());
    let article = repo.find_by_id(id.into()).await?;

    Ok(Json(Article::from(article)))
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
pub async fn list(
    auth_user: Extension<AuthUser>,
    state: Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let repo = db::ArticleRepository::new(state.db());
    let article = repo.list_by_user(auth_user.id.clone()).await?;

    Ok(Json(
        article.into_iter().map(Article::from).collect::<Vec<_>>(),
    ))
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