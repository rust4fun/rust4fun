use rust_study_client as client;

use super::error::Error;
pub use client::auth_types::{Auth, Login, Signup};
pub use client::{AuthClient, AuthResponseValue};

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

    pub async fn login(&self, email: String, password: String) -> Result<Auth, Error> {
        let body = Login::builder().email(email).password(password);
        let response = self.client.login().body(body).send().await?;

        check_response(response)
    }

    pub async fn signup(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<Auth, Error> {
        let body = Signup::builder().name(name).email(email).password(password);
        let response = self.client.signup().body(body).send().await?;

        check_response(response)
    }
}

impl Default for AuthRequester {
    fn default() -> Self {
        Self::new()
    }
}

fn check_response(response: AuthResponseValue<Auth>) -> Result<Auth, Error> {
    match response.status().as_u16() {
        200 => Ok(response.into_inner()),
        400 => Err(Error::BadRequest("wrong setting!".into())),
        404 => Err(Error::NotFound),
        _ => Err(Error::Unexpected),
    }
}
