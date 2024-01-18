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
        "title"
      ],
      "properties": {
        "id": {
          "type": "string",
          "format": "uuid"
        },
        "title": {
          "type": "string"
        }
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Article {
        pub id: uuid::Uuid,
        pub title: String,
    }
    impl From<&Article> for Article {
        fn from(value: &Article) -> Self {
            value.clone()
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
    /**Sends a `GET` request to `/api/v1/hello`

    */
    pub async fn comment<'a>(&'a self) -> Result<ResponseValue<types::Article>, Error<()>> {
        let url = format!("{}/api/v1/hello", self.baseurl,);
        let request = self
            .client()
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
pub mod prelude {
    pub use super::Client;
}
