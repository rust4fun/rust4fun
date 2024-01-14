use rust_study_shared::Article;

pub mod static_file;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::comment
    ),
    components(schemas(
        Article
    )),
    tags((name = "hello"))
)]
pub struct ApiDoc;
