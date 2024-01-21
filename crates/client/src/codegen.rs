#[allow(unused_imports)]
use progenitor::progenitor_client::{encode_path, RequestBuilderExt};
pub use progenitor::progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    ///Article
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "object",
      "required": [
        "id",
        "image_url",
        "url"
      ],
      "properties": {
        "description": {
          "type": [
            "string",
            "null"
          ]
        },
        "id": {
          "type": "string",
          "format": "uuid"
        },
        "image_url": {
          "type": "string"
        },
        "title": {
          "type": [
            "string",
            "null"
          ]
        },
        "url": {
          "type": "string"
        }
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Article {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        pub id: uuid::Uuid,
        pub image_url: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        pub url: String,
    }
    impl From<&Article> for Article {
        fn from(value: &Article) -> Self {
            value.clone()
        }
    }
    impl Article {
        pub fn builder() -> builder::Article {
            Default::default()
        }
    }
    ///RequestBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "object",
      "required": [
        "url"
      ],
      "properties": {
        "url": {
          "type": "string"
        }
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RequestBody {
        pub url: String,
    }
    impl From<&RequestBody> for RequestBody {
        fn from(value: &RequestBody) -> Self {
            value.clone()
        }
    }
    impl RequestBody {
        pub fn builder() -> builder::RequestBody {
            Default::default()
        }
    }
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct Article {
            description: Result<Option<String>, String>,
            id: Result<uuid::Uuid, String>,
            image_url: Result<String, String>,
            title: Result<Option<String>, String>,
            url: Result<String, String>,
        }
        impl Default for Article {
            fn default() -> Self {
                Self {
                    description: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    image_url: Err("no value supplied for image_url".to_string()),
                    title: Ok(Default::default()),
                    url: Err("no value supplied for url".to_string()),
                }
            }
        }
        impl Article {
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn image_url<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.image_url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for image_url: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
            pub fn url<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url: {}", e));
                self
            }
        }
        impl std::convert::TryFrom<Article> for super::Article {
            type Error = String;
            fn try_from(value: Article) -> Result<Self, String> {
                Ok(Self {
                    description: value.description?,
                    id: value.id?,
                    image_url: value.image_url?,
                    title: value.title?,
                    url: value.url?,
                })
            }
        }
        impl From<super::Article> for Article {
            fn from(value: super::Article) -> Self {
                Self {
                    description: Ok(value.description),
                    id: Ok(value.id),
                    image_url: Ok(value.image_url),
                    title: Ok(value.title),
                    url: Ok(value.url),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct RequestBody {
            url: Result<String, String>,
        }
        impl Default for RequestBody {
            fn default() -> Self {
                Self {
                    url: Err("no value supplied for url".to_string()),
                }
            }
        }
        impl RequestBody {
            pub fn url<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url: {}", e));
                self
            }
        }
        impl std::convert::TryFrom<RequestBody> for super::RequestBody {
            type Error = String;
            fn try_from(value: RequestBody) -> Result<Self, String> {
                Ok(Self { url: value.url? })
            }
        }
        impl From<super::RequestBody> for RequestBody {
            fn from(value: super::RequestBody) -> Self {
                Self { url: Ok(value.url) }
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
pub trait ClientArticlesExt {
    /**Sends a `GET` request to `/api/v1/articles`

    ```ignore
    let response = client.list()
        .send()
        .await;
    ```*/
    fn list(&self) -> builder::List;
    /**Sends a `POST` request to `/api/v1/articles`

    ```ignore
    let response = client.create()
        .body(body)
        .send()
        .await;
    ```*/
    fn create(&self) -> builder::Create;
    /**Sends a `GET` request to `/api/v1/articles/{id}`

    Arguments:
    - `id`: article id
    ```ignore
    let response = client.get_item()
        .id(id)
        .send()
        .await;
    ```*/
    fn get_item(&self) -> builder::GetItem;
}
impl ClientArticlesExt for Client {
    fn list(&self) -> builder::List {
        builder::List::new(self)
    }
    fn create(&self) -> builder::Create {
        builder::Create::new(self)
    }
    fn get_item(&self) -> builder::GetItem {
        builder::GetItem::new(self)
    }
}
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    /**Builder for [`ClientArticlesExt::list`]

    [`ClientArticlesExt::list`]: super::ClientArticlesExt::list*/
    #[derive(Debug, Clone)]
    pub struct List<'a> {
        client: &'a super::Client,
    }
    impl<'a> List<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }
        ///Sends a `GET` request to `/api/v1/articles`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::Article>>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/api/v1/articles", client.baseurl,);
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`ClientArticlesExt::create`]

    [`ClientArticlesExt::create`]: super::ClientArticlesExt::create*/
    #[derive(Debug, Clone)]
    pub struct Create<'a> {
        client: &'a super::Client,
        body: Result<types::builder::RequestBody, String>,
    }
    impl<'a> Create<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::RequestBody::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::RequestBody>,
            <V as std::convert::TryInto<types::RequestBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `RequestBody` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::RequestBody) -> types::builder::RequestBody,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/api/v1/articles`
        pub async fn send(self) -> Result<ResponseValue<types::Article>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::RequestBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/articles", client.baseurl,);
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
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`ClientArticlesExt::get_item`]

    [`ClientArticlesExt::get_item`]: super::ClientArticlesExt::get_item*/
    #[derive(Debug, Clone)]
    pub struct GetItem<'a> {
        client: &'a super::Client,
        id: Result<uuid::Uuid, String>,
    }
    impl<'a> GetItem<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/articles/{id}`
        pub async fn send(self) -> Result<ResponseValue<types::Article>, Error<()>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/articles/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}
pub mod prelude {
    pub use super::Client;
    pub use super::ClientArticlesExt;
}
