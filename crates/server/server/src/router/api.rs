pub mod articles;
pub mod root;

use crate::middleware::authorization_middleware;
use crate::State;
use axum::{middleware::from_fn_with_state, Extension, Router};
use std::sync::Arc;

pub fn api_roouter(state: Arc<State>) -> Router {
    Router::new()
        .nest_service("/me", root::router())
        .nest_service("/articles", articles::router())
        .layer(from_fn_with_state(state.clone(), authorization_middleware))
        .layer(Extension(state))
}
