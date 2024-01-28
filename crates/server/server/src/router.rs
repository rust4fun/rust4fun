#[cfg(feature = "openapi")]
use rust_study_shared::Article;

pub mod api;
pub mod auth;
pub mod static_file;

/// opanapi 自動生成用のコード
/// <https://docs.rs/utoipa/latest/utoipa/derive.OpenApi.html>
#[cfg(feature = "openapi")]
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

#[cfg(feature = "openapi")]
#[derive(OpenApi)]
#[openapi(
    paths(
        api::articles::get_item,
        api::articles::create,
        api::articles::list,
    ),
    components(schemas(
        Article,
        api::articles::RequestBody
    )),
    modifiers(&Security),
    security(
        ("token" = [])
    ),
    tags((name = "api"))
)]
pub struct ApiDoc;

#[cfg(feature = "openapi")]
struct Security;

#[cfg(feature = "openapi")]
impl Modify for Security {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
