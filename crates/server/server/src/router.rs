use rust_study_shared::{Article, User};

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

/// API
#[cfg(feature = "openapi")]
#[derive(OpenApi)]
#[openapi(
    paths(
        api::articles::get_item,
        api::articles::create,
        api::articles::list,
        api::root::me::handler,
    ),
    components(schemas(
        Article,
        User,
        api::articles::RequestBody
    )),
    modifiers(&Security),
    security(
        ("token" = [])
    ),
    tags((name = "api"))
)]
pub struct ApiDoc;

/// Auth
#[cfg(feature = "openapi")]
#[derive(OpenApi)]
#[openapi(
    paths(
        auth::login,
        auth::signup,
    ),
    components(
        schemas(
            auth::LoginRequestBody,
            auth::SignupRequestBody,
            auth::AuthResponse,
        ),
    ),
    tags((name = "auth"))
)]
pub struct AuthDoc;

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
