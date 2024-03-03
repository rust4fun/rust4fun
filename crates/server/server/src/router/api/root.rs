pub mod me;

use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/", get(me::handler))
}
