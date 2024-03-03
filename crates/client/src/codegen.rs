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
    ///GetMessagesResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "object",
      "properties": {
        "items": {
          "$ref": "#/components/schemas/ListPlanetMessages"
        },
        "limit": {
          "$ref": "#/components/schemas/PagenationLimit"
        },
        "offset": {
          "$ref": "#/components/schemas/PagenationOffset"
        },
        "total": {
          "type": "integer"
        }
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetMessagesResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub items: Option<ListPlanetMessages>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: Option<PagenationLimit>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: Option<PagenationOffset>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub total: Option<i64>,
    }
    impl From<&GetMessagesResponse> for GetMessagesResponse {
        fn from(value: &GetMessagesResponse) -> Self {
            value.clone()
        }
    }
    impl GetMessagesResponse {
        pub fn builder() -> builder::GetMessagesResponse {
            Default::default()
        }
    }
    ///ListPlanetMessages
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/PlanetMessage"
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ListPlanetMessages(pub Vec<PlanetMessage>);
    impl std::ops::Deref for ListPlanetMessages {
        type Target = Vec<PlanetMessage>;
        fn deref(&self) -> &Vec<PlanetMessage> {
            &self.0
        }
    }
    impl From<ListPlanetMessages> for Vec<PlanetMessage> {
        fn from(value: ListPlanetMessages) -> Self {
            value.0
        }
    }
    impl From<&ListPlanetMessages> for ListPlanetMessages {
        fn from(value: &ListPlanetMessages) -> Self {
            value.clone()
        }
    }
    impl From<Vec<PlanetMessage>> for ListPlanetMessages {
        fn from(value: Vec<PlanetMessage>) -> Self {
            Self(value)
        }
    }
    ///PagenationLimit
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "default": 50,
      "type": "integer",
      "maximum": 50.0,
      "minimum": 1.0
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PagenationLimit(pub i64);
    impl std::ops::Deref for PagenationLimit {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<PagenationLimit> for i64 {
        fn from(value: PagenationLimit) -> Self {
            value.0
        }
    }
    impl From<&PagenationLimit> for PagenationLimit {
        fn from(value: &PagenationLimit) -> Self {
            value.clone()
        }
    }
    impl From<i64> for PagenationLimit {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }
    impl std::str::FromStr for PagenationLimit {
        type Err = <i64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }
    impl std::convert::TryFrom<&str> for PagenationLimit {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for PagenationLimit {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for PagenationLimit {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl ToString for PagenationLimit {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }
    ///PagenationOffset
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "integer"
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PagenationOffset(pub i64);
    impl std::ops::Deref for PagenationOffset {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }
    impl From<PagenationOffset> for i64 {
        fn from(value: PagenationOffset) -> Self {
            value.0
        }
    }
    impl From<&PagenationOffset> for PagenationOffset {
        fn from(value: &PagenationOffset) -> Self {
            value.clone()
        }
    }
    impl From<i64> for PagenationOffset {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }
    impl std::str::FromStr for PagenationOffset {
        type Err = <i64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }
    impl std::convert::TryFrom<&str> for PagenationOffset {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for PagenationOffset {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for PagenationOffset {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl ToString for PagenationOffset {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }
    ///PlanetId
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "string",
      "format": "uuid"
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PlanetId(pub uuid::Uuid);
    impl std::ops::Deref for PlanetId {
        type Target = uuid::Uuid;
        fn deref(&self) -> &uuid::Uuid {
            &self.0
        }
    }
    impl From<PlanetId> for uuid::Uuid {
        fn from(value: PlanetId) -> Self {
            value.0
        }
    }
    impl From<&PlanetId> for PlanetId {
        fn from(value: &PlanetId) -> Self {
            value.clone()
        }
    }
    impl From<uuid::Uuid> for PlanetId {
        fn from(value: uuid::Uuid) -> Self {
            Self(value)
        }
    }
    impl std::str::FromStr for PlanetId {
        type Err = <uuid::Uuid as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }
    impl std::convert::TryFrom<&str> for PlanetId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for PlanetId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for PlanetId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl ToString for PlanetId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }
    ///PlanetMessage
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "object",
      "required": [
        "content",
        "created_at",
        "id",
        "planet_id",
        "updated_at",
        "user_id"
      ],
      "properties": {
        "content": {
          "type": "string"
        },
        "created_at": {
          "type": "string",
          "format": "date-time"
        },
        "id": {
          "$ref": "#/components/schemas/PlanetMessageId"
        },
        "planet_id": {
          "$ref": "#/components/schemas/PlanetId"
        },
        "updated_at": {
          "type": "string",
          "format": "date-time"
        },
        "user_id": {
          "$ref": "#/components/schemas/UserId"
        }
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PlanetMessage {
        pub content: String,
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        pub id: PlanetMessageId,
        pub planet_id: PlanetId,
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
        pub user_id: UserId,
    }
    impl From<&PlanetMessage> for PlanetMessage {
        fn from(value: &PlanetMessage) -> Self {
            value.clone()
        }
    }
    impl PlanetMessage {
        pub fn builder() -> builder::PlanetMessage {
            Default::default()
        }
    }
    ///PlanetMessageId
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "string",
      "format": "uuid"
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PlanetMessageId(pub uuid::Uuid);
    impl std::ops::Deref for PlanetMessageId {
        type Target = uuid::Uuid;
        fn deref(&self) -> &uuid::Uuid {
            &self.0
        }
    }
    impl From<PlanetMessageId> for uuid::Uuid {
        fn from(value: PlanetMessageId) -> Self {
            value.0
        }
    }
    impl From<&PlanetMessageId> for PlanetMessageId {
        fn from(value: &PlanetMessageId) -> Self {
            value.clone()
        }
    }
    impl From<uuid::Uuid> for PlanetMessageId {
        fn from(value: uuid::Uuid) -> Self {
            Self(value)
        }
    }
    impl std::str::FromStr for PlanetMessageId {
        type Err = <uuid::Uuid as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }
    impl std::convert::TryFrom<&str> for PlanetMessageId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for PlanetMessageId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for PlanetMessageId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl ToString for PlanetMessageId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }
    ///PostPlanet
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "object",
      "required": [
        "name",
        "sphere_id"
      ],
      "properties": {
        "description": {
          "type": "string"
        },
        "name": {
          "type": "string"
        },
        "sphere_id": {
          "$ref": "#/components/schemas/SphereId"
        }
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PostPlanet {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        pub name: String,
        pub sphere_id: SphereId,
    }
    impl From<&PostPlanet> for PostPlanet {
        fn from(value: &PostPlanet) -> Self {
            value.clone()
        }
    }
    impl PostPlanet {
        pub fn builder() -> builder::PostPlanet {
            Default::default()
        }
    }
    ///PostPlanetMessage
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "object",
      "properties": {
        "content": {
          "type": "string"
        }
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PostPlanetMessage {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub content: Option<String>,
    }
    impl From<&PostPlanetMessage> for PostPlanetMessage {
        fn from(value: &PostPlanetMessage) -> Self {
            value.clone()
        }
    }
    impl PostPlanetMessage {
        pub fn builder() -> builder::PostPlanetMessage {
            Default::default()
        }
    }
    ///SphereId
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "string",
      "format": "uuid"
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SphereId(pub uuid::Uuid);
    impl std::ops::Deref for SphereId {
        type Target = uuid::Uuid;
        fn deref(&self) -> &uuid::Uuid {
            &self.0
        }
    }
    impl From<SphereId> for uuid::Uuid {
        fn from(value: SphereId) -> Self {
            value.0
        }
    }
    impl From<&SphereId> for SphereId {
        fn from(value: &SphereId) -> Self {
            value.clone()
        }
    }
    impl From<uuid::Uuid> for SphereId {
        fn from(value: uuid::Uuid) -> Self {
            Self(value)
        }
    }
    impl std::str::FromStr for SphereId {
        type Err = <uuid::Uuid as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }
    impl std::convert::TryFrom<&str> for SphereId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for SphereId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for SphereId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl ToString for SphereId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }
    ///User
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "object",
      "properties": {
        "name": {
          "type": [
            "string",
            "null"
          ]
        }
      }
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct User {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl From<&User> for User {
        fn from(value: &User) -> Self {
            value.clone()
        }
    }
    impl User {
        pub fn builder() -> builder::User {
            Default::default()
        }
    }
    ///UserId
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /**{
      "type": "string",
      "format": "uuid"
    }*/
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UserId(pub uuid::Uuid);
    impl std::ops::Deref for UserId {
        type Target = uuid::Uuid;
        fn deref(&self) -> &uuid::Uuid {
            &self.0
        }
    }
    impl From<UserId> for uuid::Uuid {
        fn from(value: UserId) -> Self {
            value.0
        }
    }
    impl From<&UserId> for UserId {
        fn from(value: &UserId) -> Self {
            value.clone()
        }
    }
    impl From<uuid::Uuid> for UserId {
        fn from(value: uuid::Uuid) -> Self {
            Self(value)
        }
    }
    impl std::str::FromStr for UserId {
        type Err = <uuid::Uuid as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }
    impl std::convert::TryFrom<&str> for UserId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for UserId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for UserId {
        type Error = <uuid::Uuid as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }
    impl ToString for UserId {
        fn to_string(&self) -> String {
            self.0.to_string()
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
        pub struct GetMessagesResponse {
            items: Result<Option<super::ListPlanetMessages>, String>,
            limit: Result<Option<super::PagenationLimit>, String>,
            offset: Result<Option<super::PagenationOffset>, String>,
            total: Result<Option<i64>, String>,
        }
        impl Default for GetMessagesResponse {
            fn default() -> Self {
                Self {
                    items: Ok(Default::default()),
                    limit: Ok(Default::default()),
                    offset: Ok(Default::default()),
                    total: Ok(Default::default()),
                }
            }
        }
        impl GetMessagesResponse {
            pub fn items<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::ListPlanetMessages>>,
                T::Error: std::fmt::Display,
            {
                self.items = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for items: {}", e));
                self
            }
            pub fn limit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::PagenationLimit>>,
                T::Error: std::fmt::Display,
            {
                self.limit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for limit: {}", e));
                self
            }
            pub fn offset<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::PagenationOffset>>,
                T::Error: std::fmt::Display,
            {
                self.offset = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for offset: {}", e));
                self
            }
            pub fn total<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.total = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total: {}", e));
                self
            }
        }
        impl std::convert::TryFrom<GetMessagesResponse> for super::GetMessagesResponse {
            type Error = String;
            fn try_from(value: GetMessagesResponse) -> Result<Self, String> {
                Ok(Self {
                    items: value.items?,
                    limit: value.limit?,
                    offset: value.offset?,
                    total: value.total?,
                })
            }
        }
        impl From<super::GetMessagesResponse> for GetMessagesResponse {
            fn from(value: super::GetMessagesResponse) -> Self {
                Self {
                    items: Ok(value.items),
                    limit: Ok(value.limit),
                    offset: Ok(value.offset),
                    total: Ok(value.total),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct PlanetMessage {
            content: Result<String, String>,
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            id: Result<super::PlanetMessageId, String>,
            planet_id: Result<super::PlanetId, String>,
            updated_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            user_id: Result<super::UserId, String>,
        }
        impl Default for PlanetMessage {
            fn default() -> Self {
                Self {
                    content: Err("no value supplied for content".to_string()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    planet_id: Err("no value supplied for planet_id".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                    user_id: Err("no value supplied for user_id".to_string()),
                }
            }
        }
        impl PlanetMessage {
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::PlanetMessageId>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn planet_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::PlanetId>,
                T::Error: std::fmt::Display,
            {
                self.planet_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for planet_id: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::UserId>,
                T::Error: std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {}", e));
                self
            }
        }
        impl std::convert::TryFrom<PlanetMessage> for super::PlanetMessage {
            type Error = String;
            fn try_from(value: PlanetMessage) -> Result<Self, String> {
                Ok(Self {
                    content: value.content?,
                    created_at: value.created_at?,
                    id: value.id?,
                    planet_id: value.planet_id?,
                    updated_at: value.updated_at?,
                    user_id: value.user_id?,
                })
            }
        }
        impl From<super::PlanetMessage> for PlanetMessage {
            fn from(value: super::PlanetMessage) -> Self {
                Self {
                    content: Ok(value.content),
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    planet_id: Ok(value.planet_id),
                    updated_at: Ok(value.updated_at),
                    user_id: Ok(value.user_id),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct PostPlanet {
            description: Result<Option<String>, String>,
            name: Result<String, String>,
            sphere_id: Result<super::SphereId, String>,
        }
        impl Default for PostPlanet {
            fn default() -> Self {
                Self {
                    description: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    sphere_id: Err("no value supplied for sphere_id".to_string()),
                }
            }
        }
        impl PostPlanet {
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
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn sphere_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::SphereId>,
                T::Error: std::fmt::Display,
            {
                self.sphere_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sphere_id: {}", e));
                self
            }
        }
        impl std::convert::TryFrom<PostPlanet> for super::PostPlanet {
            type Error = String;
            fn try_from(value: PostPlanet) -> Result<Self, String> {
                Ok(Self {
                    description: value.description?,
                    name: value.name?,
                    sphere_id: value.sphere_id?,
                })
            }
        }
        impl From<super::PostPlanet> for PostPlanet {
            fn from(value: super::PostPlanet) -> Self {
                Self {
                    description: Ok(value.description),
                    name: Ok(value.name),
                    sphere_id: Ok(value.sphere_id),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct PostPlanetMessage {
            content: Result<Option<String>, String>,
        }
        impl Default for PostPlanetMessage {
            fn default() -> Self {
                Self {
                    content: Ok(Default::default()),
                }
            }
        }
        impl PostPlanetMessage {
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
        }
        impl std::convert::TryFrom<PostPlanetMessage> for super::PostPlanetMessage {
            type Error = String;
            fn try_from(value: PostPlanetMessage) -> Result<Self, String> {
                Ok(Self {
                    content: value.content?,
                })
            }
        }
        impl From<super::PostPlanetMessage> for PostPlanetMessage {
            fn from(value: super::PostPlanetMessage) -> Self {
                Self {
                    content: Ok(value.content),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct User {
            name: Result<Option<String>, String>,
        }
        impl Default for User {
            fn default() -> Self {
                Self {
                    name: Ok(Default::default()),
                }
            }
        }
        impl User {
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
        }
        impl std::convert::TryFrom<User> for super::User {
            type Error = String;
            fn try_from(value: User) -> Result<Self, String> {
                Ok(Self { name: value.name? })
            }
        }
        impl From<super::User> for User {
            fn from(value: super::User) -> Self {
                Self {
                    name: Ok(value.name),
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
pub trait ClientPlanetsExt {
    /**Sends a `GET` request to `/planets/{planet_id}/messages`

    ```ignore
    let response = client.get_messages()
        .planet_id(planet_id)
        .limit(limit)
        .offset(offset)
        .send()
        .await;
    ```*/
    fn get_messages(&self) -> builder::GetMessages;
    /**Sends a `POST` request to `/planets/{planet_id}/messages`

    Arguments:
    - `planet_id`
    - `body`: post planet message
    ```ignore
    let response = client.post_message()
        .planet_id(planet_id)
        .body(body)
        .send()
        .await;
    ```*/
    fn post_message(&self) -> builder::PostMessage;
}
impl ClientPlanetsExt for Client {
    fn get_messages(&self) -> builder::GetMessages {
        builder::GetMessages::new(self)
    }
    fn post_message(&self) -> builder::PostMessage {
        builder::PostMessage::new(self)
    }
}
pub trait ClientRootExt {
    /**Sends a `GET` request to `/me`

    ```ignore
    let response = client.me()
        .send()
        .await;
    ```*/
    fn me(&self) -> builder::Me;
}
impl ClientRootExt for Client {
    fn me(&self) -> builder::Me {
        builder::Me::new(self)
    }
}
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    /**Builder for [`ClientRootExt::me`]

    [`ClientRootExt::me`]: super::ClientRootExt::me*/
    #[derive(Debug, Clone)]
    pub struct Me<'a> {
        client: &'a super::Client,
    }
    impl<'a> Me<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }
        ///Sends a `GET` request to `/me`
        pub async fn send(self) -> Result<ResponseValue<types::User>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/me", client.baseurl,);
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
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`ClientPlanetsExt::get_messages`]

    [`ClientPlanetsExt::get_messages`]: super::ClientPlanetsExt::get_messages*/
    #[derive(Debug, Clone)]
    pub struct GetMessages<'a> {
        client: &'a super::Client,
        planet_id: Result<types::PlanetId, String>,
        limit: Result<Option<types::PagenationLimit>, String>,
        offset: Result<Option<types::PagenationOffset>, String>,
    }
    impl<'a> GetMessages<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                planet_id: Err("planet_id was not initialized".to_string()),
                limit: Ok(None),
                offset: Ok(None),
            }
        }
        pub fn planet_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PlanetId>,
        {
            self.planet_id = value
                .try_into()
                .map_err(|_| "conversion to `PlanetId` for planet_id failed".to_string());
            self
        }
        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PagenationLimit>,
        {
            self.limit = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `PagenationLimit` for limit failed".to_string());
            self
        }
        pub fn offset<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PagenationOffset>,
        {
            self.offset = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `PagenationOffset` for offset failed".to_string());
            self
        }
        ///Sends a `GET` request to `/planets/{planet_id}/messages`
        pub async fn send(self) -> Result<ResponseValue<types::GetMessagesResponse>, Error<()>> {
            let Self {
                client,
                planet_id,
                limit,
                offset,
            } = self;
            let planet_id = planet_id.map_err(Error::InvalidRequest)?;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let offset = offset.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/planets/{}/messages",
                client.baseurl,
                encode_path(&planet_id.to_string()),
            );
            let mut query = Vec::with_capacity(2usize);
            if let Some(v) = &limit {
                query.push(("limit", v.to_string()));
            }
            if let Some(v) = &offset {
                query.push(("offset", v.to_string()));
            }
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`ClientPlanetsExt::post_message`]

    [`ClientPlanetsExt::post_message`]: super::ClientPlanetsExt::post_message*/
    #[derive(Debug, Clone)]
    pub struct PostMessage<'a> {
        client: &'a super::Client,
        planet_id: Result<types::PlanetId, String>,
        body: Result<types::builder::PostPlanetMessage, String>,
    }
    impl<'a> PostMessage<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                planet_id: Err("planet_id was not initialized".to_string()),
                body: Ok(types::builder::PostPlanetMessage::default()),
            }
        }
        pub fn planet_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PlanetId>,
        {
            self.planet_id = value
                .try_into()
                .map_err(|_| "conversion to `PlanetId` for planet_id failed".to_string());
            self
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PostPlanetMessage>,
            <V as std::convert::TryInto<types::PostPlanetMessage>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `PostPlanetMessage` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::PostPlanetMessage,
            ) -> types::builder::PostPlanetMessage,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/planets/{planet_id}/messages`
        pub async fn send(self) -> Result<ResponseValue<types::PlanetMessage>, Error<()>> {
            let Self {
                client,
                planet_id,
                body,
            } = self;
            let planet_id = planet_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::PostPlanetMessage>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/planets/{}/messages",
                client.baseurl,
                encode_path(&planet_id.to_string()),
            );
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
                401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}
pub mod prelude {
    pub use super::Client;
    pub use super::ClientPlanetsExt;
    pub use super::ClientRootExt;
}
