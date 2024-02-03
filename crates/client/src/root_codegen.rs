#[allow(unused_imports)]
use progenitor::progenitor_client::{encode_path, RequestBuilderExt};
pub use progenitor::progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    ///AuthResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "object",
      "required": [
        "token"
      ],
      "properties": {
        "token": {
          "type": "string"
        }
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AuthResponse {
        pub token: String,
    }
    impl From<&AuthResponse> for AuthResponse {
        fn from(value: &AuthResponse) -> Self {
            value.clone()
        }
    }
    impl AuthResponse {
        pub fn builder() -> builder::AuthResponse {
            Default::default()
        }
    }
    ///LoginRequestBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "object",
      "required": [
        "email",
        "password"
      ],
      "properties": {
        "email": {
          "type": "string"
        },
        "password": {
          "type": "string"
        }
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LoginRequestBody {
        pub email: String,
        pub password: String,
    }
    impl From<&LoginRequestBody> for LoginRequestBody {
        fn from(value: &LoginRequestBody) -> Self {
            value.clone()
        }
    }
    impl LoginRequestBody {
        pub fn builder() -> builder::LoginRequestBody {
            Default::default()
        }
    }
    ///SignupRequestBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "object",
      "required": [
        "email",
        "password"
      ],
      "properties": {
        "email": {
          "type": "string"
        },
        "name": {
          "type": [
            "string",
            "null"
          ]
        },
        "password": {
          "type": "string"
        }
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SignupRequestBody {
        pub email: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        pub password: String,
    }
    impl From<&SignupRequestBody> for SignupRequestBody {
        fn from(value: &SignupRequestBody) -> Self {
            value.clone()
        }
    }
    impl SignupRequestBody {
        pub fn builder() -> builder::SignupRequestBody {
            Default::default()
        }
    }
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct AuthResponse {
            token: Result<String, String>,
        }
        impl Default for AuthResponse {
            fn default() -> Self {
                Self {
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }
        impl AuthResponse {
            pub fn token<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token: {}", e));
                self
            }
        }
        impl std::convert::TryFrom<AuthResponse> for super::AuthResponse {
            type Error = String;
            fn try_from(value: AuthResponse) -> Result<Self, String> {
                Ok(Self {
                    token: value.token?,
                })
            }
        }
        impl From<super::AuthResponse> for AuthResponse {
            fn from(value: super::AuthResponse) -> Self {
                Self {
                    token: Ok(value.token),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct LoginRequestBody {
            email: Result<String, String>,
            password: Result<String, String>,
        }
        impl Default for LoginRequestBody {
            fn default() -> Self {
                Self {
                    email: Err("no value supplied for email".to_string()),
                    password: Err("no value supplied for password".to_string()),
                }
            }
        }
        impl LoginRequestBody {
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
            pub fn password<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.password = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for password: {}", e));
                self
            }
        }
        impl std::convert::TryFrom<LoginRequestBody> for super::LoginRequestBody {
            type Error = String;
            fn try_from(value: LoginRequestBody) -> Result<Self, String> {
                Ok(Self {
                    email: value.email?,
                    password: value.password?,
                })
            }
        }
        impl From<super::LoginRequestBody> for LoginRequestBody {
            fn from(value: super::LoginRequestBody) -> Self {
                Self {
                    email: Ok(value.email),
                    password: Ok(value.password),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct SignupRequestBody {
            email: Result<String, String>,
            name: Result<Option<String>, String>,
            password: Result<String, String>,
        }
        impl Default for SignupRequestBody {
            fn default() -> Self {
                Self {
                    email: Err("no value supplied for email".to_string()),
                    name: Ok(Default::default()),
                    password: Err("no value supplied for password".to_string()),
                }
            }
        }
        impl SignupRequestBody {
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn password<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.password = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for password: {}", e));
                self
            }
        }
        impl std::convert::TryFrom<SignupRequestBody> for super::SignupRequestBody {
            type Error = String;
            fn try_from(value: SignupRequestBody) -> Result<Self, String> {
                Ok(Self {
                    email: value.email?,
                    name: value.name?,
                    password: value.password?,
                })
            }
        }
        impl From<super::SignupRequestBody> for SignupRequestBody {
            fn from(value: super::SignupRequestBody) -> Self {
                Self {
                    email: Ok(value.email),
                    name: Ok(value.name),
                    password: Ok(value.password),
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for rust-study-server

Rust study web application

Version: 0.1.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }
    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "0.1.0"
    }
}
pub trait ClientAuthExt {
    /**Sends a `POST` request to `/auth/login`

    ```ignore
    let response = client.login()
        .body(body)
        .send()
        .await;
    ```*/
    fn login(&self) -> builder::Login;
    /**Sends a `POST` request to `/auth/signup`

    ```ignore
    let response = client.signup()
        .body(body)
        .send()
        .await;
    ```*/
    fn signup(&self) -> builder::Signup;
}
impl ClientAuthExt for Client {
    fn login(&self) -> builder::Login {
        builder::Login::new(self)
    }
    fn signup(&self) -> builder::Signup {
        builder::Signup::new(self)
    }
}
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    /**Builder for [`ClientAuthExt::login`]

    [`ClientAuthExt::login`]: super::ClientAuthExt::login*/
    #[derive(Debug, Clone)]
    pub struct Login<'a> {
        client: &'a super::Client,
        body: Result<types::builder::LoginRequestBody, String>,
    }
    impl<'a> Login<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::LoginRequestBody::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::LoginRequestBody>,
            <V as std::convert::TryInto<types::LoginRequestBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `LoginRequestBody` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::LoginRequestBody,
            ) -> types::builder::LoginRequestBody,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/auth/login`
        pub async fn send(self) -> Result<ResponseValue<types::AuthResponse>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::LoginRequestBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/auth/login", client.baseurl,);
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`ClientAuthExt::signup`]

    [`ClientAuthExt::signup`]: super::ClientAuthExt::signup*/
    #[derive(Debug, Clone)]
    pub struct Signup<'a> {
        client: &'a super::Client,
        body: Result<types::builder::SignupRequestBody, String>,
    }
    impl<'a> Signup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::SignupRequestBody::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::SignupRequestBody>,
            <V as std::convert::TryInto<types::SignupRequestBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `SignupRequestBody` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::SignupRequestBody,
            ) -> types::builder::SignupRequestBody,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/auth/signup`
        pub async fn send(self) -> Result<ResponseValue<types::AuthResponse>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::SignupRequestBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/auth/signup", client.baseurl,);
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}
pub mod prelude {
    pub use super::Client;
    pub use super::ClientAuthExt;
}
