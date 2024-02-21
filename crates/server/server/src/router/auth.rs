use rust_study_auth as auth;
use rust_study_db_connector as db;

use crate::error::Error;
use crate::State;
use axum::{response::IntoResponse, routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

pub fn router(state: Arc<State>) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .layer(Extension(state))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct AuthResponse {
    token: String,
}

// TODO: 暗号化（公開鍵とか使いたい）
// かつメモリ上は、暗語化されたもので扱いたい
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct LoginRequestBody {
    pub email: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/login",
    context_path = "/auth",
    request_body = LoginRequestBody,
    responses(
        (status = 200, description = "login", body = AuthResponse),
        (status = 404, description = "not found")
    ),
    tag = "auth",
)]
pub async fn login(
    state: Extension<Arc<State>>,
    Json(body): Json<LoginRequestBody>,
) -> Result<impl IntoResponse, Error> {
    let repo = db::UserRepository::new(state.db());

    let input = db::InputUserValidateEntity::new(body.email, body.password);
    let user = repo
        .validate_and_get(input)
        .await
        .map_err(|_| Error::NotFound("user".into()))?;

    let token = auth::JWT::create(
        "http::/localhost:8080/".to_string(),
        user.id.id(),
        "http::/localhost:8080/api/v1".to_string(),
        48,
    )?
    .access_token()
    .to_owned();

    Ok(Json(AuthResponse { token }))
}

// TODO: 暗号化（公開鍵とか使いたい）
// かつメモリ上は、暗語化されたもので扱いたい
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct SignupRequestBody {
    pub name: Option<String>,
    pub email: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/signup",
    context_path = "/auth",
    request_body = SignupRequestBody,
    responses(
        (status = 200, description = "get article records", body = AuthResponse),
        (status = 404, description = "not found")
    ),
    tag = "auth",
)]
pub async fn signup(
    state: Extension<Arc<State>>,
    Json(body): Json<SignupRequestBody>,
) -> Result<impl IntoResponse, Error> {
    let repo = db::UserRepository::new(state.db());

    let id = Uuid::new_v4();
    let input = db::InputUserEntity::new(body.name, body.email, body.password);
    repo.create(id.into(), input).await?;

    let token = auth::JWT::create(
        "http::/localhost:8080/api/v1".to_string(),
        id,
        "http::/localhost:8080/api/v1".to_string(),
        48,
    )?
    .access_token()
    .to_owned();

    Ok(Json(AuthResponse { token }))
}
