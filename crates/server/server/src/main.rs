use rust_study_server as server;

use axum::{extract::Request, response::Response, routing::get, Router};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing::Span;
use uuid::Uuid;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let api = Router::new()
        .route("/hello", get(server::comment))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_req: &Request<_>| {
                    let request_id = Uuid::new_v4();
                    tracing::span!(
                        Level::INFO,
                        "apis",
                        request_id = tracing::field::display(request_id)
                    )
                })
                .on_request(|req: &Request<_>, _span: &Span| {
                    tracing::info!("[Request Start]");
                    tracing::debug!("request: {req:?}");
                })
                .on_response(|res: &Response<_>, _latency: Duration, _span: &Span| {
                    tracing::info!("[Request End]");
                    tracing::debug!("response: {res:?}");
                }),
        );

    let router = Router::new()
        .merge(server::router::static_file::static_roouter())
        .nest("/api/v1", api);

    Ok(router.into())
}
