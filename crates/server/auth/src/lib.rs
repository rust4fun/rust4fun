pub mod error;

use chrono::{Duration, Utc};
pub use error::AuthError as Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use uuid::Uuid;

static KEYS: OnceLock<KeySet> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct JWT(String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // issuer
    iss: String,
    // subject
    sub: Uuid,
    // audience
    aud: String,
    // expiration time
    exp: i64,
    // 	Issued At
    iat: i64,
    // 	JWT ID
    jti: Uuid,
}

struct KeySet {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JWT {
    pub fn new(token: String) -> Self {
        Self(token)
    }

    pub fn access_token(&self) -> &str {
        &self.0
    }

    pub fn create(iss: String, sub: Uuid, aud: String, duration_hours: i64) -> Result<Self, Error> {
        let iat = Utc::now();
        let exp = iat + Duration::hours(duration_hours);
        let claims = Claims {
            iss,
            sub,
            aud,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
            jti: Uuid::new_v4(),
        };
        let header = Header::new(Algorithm::HS512);
        let key = &KEYS.get().ok_or(Error::KeySetNotInitialize)?.encoding;
        let token = encode(&header, &claims, key).map_err(Error::EoncodeInvalidJwt)?;

        Ok(JWT(token))
    }

    pub fn validate(&self, aud: &str) -> Result<Claims, Error> {
        let key = &KEYS.get().ok_or(Error::KeySetNotInitialize)?.decoding;
        let mut validate = Validation::new(Algorithm::HS512);
        validate.set_audience(&[aud]);
        let token_data = decode::<Claims>(self.access_token(), key, &validate)
            .map_err(Error::DecodeInvalidJwt)?;

        Ok(token_data.claims)
    }
}

impl Claims {
    pub fn sub(&self) -> Uuid {
        self.sub
    }
}

pub fn init(secret: &[u8]) -> Result<(), Error> {
    let key_set = KeySet {
        encoding: EncodingKey::from_secret(secret),
        decoding: DecodingKey::from_secret(secret),
    };

    KEYS.set(key_set).map_err(|_| Error::KeySetNotInitialize)
}
