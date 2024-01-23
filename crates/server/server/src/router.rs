#[cfg(feature = "openapi")]
use rust_study_shared::Article;

pub mod articles;
pub mod hello;
pub mod static_file;

/// opanapi 自動生成用のコード
#[cfg(feature = "openapi")]
use utoipa::OpenApi;

#[cfg(feature = "openapi")]
#[derive(OpenApi)]
#[openapi(
    paths(
        articles::get_item,
        articles::create,
        articles::list,
    ),
    components(schemas(
        Article,
        articles::RequestBody
    )),
    tags((name = "api"))
)]
pub struct ApiDoc;
