use rust_study_shared::Article;

pub mod articles;
pub mod static_file;

/// opanapi 自動生成用のコード
use utoipa::OpenApi;

#[cfg(feature = "openapi")]
#[derive(OpenApi)]
#[openapi(
    paths(
        articles::get_item,
        articles::create,
    ),
    components(schemas(
        Article,
        articles::RequestBody
    )),
    tags((name = "api"))
)]
pub struct ApiDoc;
