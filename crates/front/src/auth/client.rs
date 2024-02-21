use rust_study_client as client;

use super::error::Error;
pub use client::auth_types::{AuthResponse, LoginRequestBody, SignupRequestBody};
pub use client::{AuthClient, AuthResponseValue, ClientAuthExt};

pub struct AuthRequester {
    client: AuthClient,
}

impl AuthRequester {
    pub fn new() -> Self {
        let base = std::env!("AUTH_URL");

        Self {
            client: AuthClient::new(base),
        }
    }

    pub async fn login(&self, email: String, password: String) -> Result<AuthResponse, Error> {
        let body = LoginRequestBody::builder().email(email).password(password);
        let response = self.client.login().body(body).send().await?;

        check_response(response)
    }

    pub async fn signup(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<AuthResponse, Error> {
        let body = SignupRequestBody::builder()
            .name(name)
            .email(email)
            .password(password);
        let response = self.client.signup().body(body).send().await?;

        check_response(response)
    }
}

fn check_response(response: AuthResponseValue<AuthResponse>) -> Result<AuthResponse, Error> {
    match response.status().as_u16() {
        200 => Ok(response.into_inner()),
        400 => Err(Error::BadRequest("wrong setting!".into())),
        404 => Err(Error::NotFound),
        _ => Err(Error::Unexpected),
    }
}
