use rust_study_auth as auth;
use rust_study_db_connector as db;

use crate::model::AuthUser;
use crate::State;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
    Extension, RequestPartsExt,
};
use axum_extra::headers::authorization::{Authorization, Bearer};
use axum_extra::typed_header::TypedHeader;
use std::sync::Arc;

pub async fn authorization_middleware(
    state: Extension<Arc<State>>,
    request: Request,
    next: Next,
) -> Response {
    let (mut parts, body) = request.into_parts();
    let auth_user = AuthUser::from_request_parts(&mut parts, &state)
        .await
        .unwrap();
    let mut request = Request::from_parts(parts, body);
    request.extensions_mut().insert(auth_user.clone());

    next.run(request).await
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    Arc<State>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = Arc::<State>::from_ref(state);

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| tracing::error!("{e}"))
            .unwrap();

        // JWT の検証
        let jwt = auth::JWT::new(bearer.token().to_owned());
        let claims = jwt.validate("http::/localhost:8080/api/v1").unwrap();

        // ユーザーの存在確認
        let db = app_state.db();
        let repo = db::UserRepository::new(db);
        let user = repo.find_by_id(claims.sub().into()).await;

        Ok((user, claims).into())
    }
}
