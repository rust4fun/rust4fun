use rust_study_client as client;

use super::error::Error;
pub use client::auth_types::{AuthResponse, LoginRequestBody, SignupRequestBody};
pub use client::{AuthClient, ClientAuthExt};

pub struct AuthRequester {
    client: AuthClient,
}

impl AuthRequester {
    pub fn new() -> Self {
        // TODO: 供給先をどうするか
        let base = std::env!("AUTH_URL");

        Self {
            client: AuthClient::new(base),
        }
    }

    fn client(&self) -> &AuthClient {
        &self.client
    }

    pub async fn login(&self, email: String, password: String) -> Result<AuthResponse, Error> {
        let body = LoginRequestBody::builder().email(email).password(password);
        let response = self.client().login().body(body).send().await?;

        if response.status() != 200 {
            return Err(Error::BadRequest("wrong indetify!".into()));
        }

        Ok(response.into_inner())
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
        let response = self.client().signup().body(body).send().await?;

        if response.status() != 200 {
            return Err(Error::BadRequest("wrong setting!".into()));
        }

        Ok(response.into_inner())
    }
}
