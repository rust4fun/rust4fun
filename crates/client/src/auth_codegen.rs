#[allow(unused_imports)]
use progenitor::progenitor_client::{encode_path, RequestBuilderExt};
pub use progenitor::progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    ///Auth
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
    pub struct Auth {
        pub token: String,
    }
    impl From<&Auth> for Auth {
        fn from(value: &Auth) -> Self {
            value.clone()
        }
    }
    impl Auth {
        pub fn builder() -> builder::Auth {
            Default::default()
        }
    }
    ///Login
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
    pub struct Login {
        pub email: String,
        pub password: String,
    }
    impl From<&Login> for Login {
        fn from(value: &Login) -> Self {
            value.clone()
        }
    }
    impl Login {
        pub fn builder() -> builder::Login {
            Default::default()
        }
    }
    ///Signup
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
    pub struct Signup {
        pub email: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        pub password: String,
    }
    impl From<&Signup> for Signup {
        fn from(value: &Signup) -> Self {
            value.clone()
        }
    }
    impl Signup {
        pub fn builder() -> builder::Signup {
            Default::default()
        }
    }
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct Auth {
            token: Result<String, String>,
        }
        impl Default for Auth {
            fn default() -> Self {
                Self {
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }
        impl Auth {
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
        impl std::convert::TryFrom<Auth> for super::Auth {
            type Error = String;
            fn try_from(value: Auth) -> Result<Self, String> {
                Ok(Self {
                    token: value.token?,
                })
            }
        }
        impl From<super::Auth> for Auth {
            fn from(value: super::Auth) -> Self {
                Self {
                    token: Ok(value.token),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct Login {
            email: Result<String, String>,
            password: Result<String, String>,
        }
        impl Default for Login {
            fn default() -> Self {
                Self {
                    email: Err("no value supplied for email".to_string()),
                    password: Err("no value supplied for password".to_string()),
                }
            }
        }
        impl Login {
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
        impl std::convert::TryFrom<Login> for super::Login {
            type Error = String;
            fn try_from(value: Login) -> Result<Self, String> {
                Ok(Self {
                    email: value.email?,
                    password: value.password?,
                })
            }
        }
        impl From<super::Login> for Login {
            fn from(value: super::Login) -> Self {
                Self {
                    email: Ok(value.email),
                    password: Ok(value.password),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct Signup {
            email: Result<String, String>,
            name: Result<Option<String>, String>,
            password: Result<String, String>,
        }
        impl Default for Signup {
            fn default() -> Self {
                Self {
                    email: Err("no value supplied for email".to_string()),
                    name: Ok(Default::default()),
                    password: Err("no value supplied for password".to_string()),
                }
            }
        }
        impl Signup {
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
        impl std::convert::TryFrom<Signup> for super::Signup {
            type Error = String;
            fn try_from(value: Signup) -> Result<Self, String> {
                Ok(Self {
                    email: value.email?,
                    name: value.name?,
                    password: value.password?,
                })
            }
        }
        impl From<super::Signup> for Signup {
            fn from(value: super::Signup) -> Self {
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
impl Client {
    /**Sends a `POST` request to `/login`

    Arguments:
    - `body`: post login
    ```ignore
    let response = client.login()
        .body(body)
        .send()
        .await;
    ```*/
    pub fn login(&self) -> builder::Login {
        builder::Login::new(self)
    }
    /**Sends a `POST` request to `/signup`

    Arguments:
    - `body`: post login
    ```ignore
    let response = client.signup()
        .body(body)
        .send()
        .await;
    ```*/
    pub fn signup(&self) -> builder::Signup {
        builder::Signup::new(self)
    }
}
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    /**Builder for [`Client::login`]

    [`Client::login`]: super::Client::login*/
    #[derive(Debug, Clone)]
    pub struct Login<'a> {
        client: &'a super::Client,
        body: Result<types::builder::Login, String>,
    }
    impl<'a> Login<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::Login::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::Login>,
            <V as std::convert::TryInto<types::Login>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `Login` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::Login) -> types::builder::Login,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/login`
        pub async fn send(self) -> Result<ResponseValue<types::Auth>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::Login>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/login", client.baseurl,);
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
    /**Builder for [`Client::signup`]

    [`Client::signup`]: super::Client::signup*/
    #[derive(Debug, Clone)]
    pub struct Signup<'a> {
        client: &'a super::Client,
        body: Result<types::builder::Signup, String>,
    }
    impl<'a> Signup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::Signup::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::Signup>,
            <V as std::convert::TryInto<types::Signup>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `Signup` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::Signup) -> types::builder::Signup,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/signup`
        pub async fn send(self) -> Result<ResponseValue<types::Auth>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::Signup>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/signup", client.baseurl,);
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
}
