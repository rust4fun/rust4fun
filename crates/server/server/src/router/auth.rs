use auth::JWT;
use rust_study_auth as auth;
use rust_study_db_connector as db;

use crate::State;
use axum::{routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ToSchema)]
pub struct AuthResponse {
    token: auth::JWT,
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
    path = "/auth/login",
    request_body = LoginRequestBody,
    responses(
        (status = 200, description = "post login request", body = AuthResponse)
    ),
    params(
        ("id" = Uuid, Path, description = "article id"),
    ),
    tag = "auth",
)]
pub async fn login(
    state: Extension<Arc<State>>,
    Json(body): Json<LoginRequestBody>,
) -> Json<AuthResponse> {
    let repo = db::UserRepository::new(state.db());

    let input = db::InputUserValidateEntity::new(body.email, body.password);
    let user = repo.validate_and_get(input).await;

    let token = auth::JWT::create(
        "http::/localhost:8080/api/v1".to_string(),
        user.id.id(),
        "http::/localhost:8080/api/v1".to_string(),
        48,
    )
    .unwrap();

    Json(AuthResponse { token })
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
    path = "/auth/signup",
    request_body = SignupRequestBody,
    responses(
        (status = 200, description = "create article record", body = AuthResponse)
    ),
    tag = "articles",
)]
pub async fn signup(
    state: Extension<Arc<State>>,
    Json(body): Json<SignupRequestBody>,
) -> Json<AuthResponse> {
    let repo = db::UserRepository::new(state.db());

    let id = Uuid::new_v4();
    let input = db::InputUserEntity::new(body.name, body.email, body.password);
    let row = repo.create(id.into(), input).await;

    // TODO: Error status で返す
    if row == 0 {
        return Json(AuthResponse {
            token: JWT::default(),
        });
    }

    let token = auth::JWT::create(
        "http::/localhost:8080/api/v1".to_string(),
        id,
        "http::/localhost:8080/api/v1".to_string(),
        48,
    )
    .map_err(|e| tracing::error!("{e:?}"))
    .unwrap();

    Json(AuthResponse { token })
}
