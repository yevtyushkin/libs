use std::sync::Arc;

use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;

use crate::internal::cache::Cache;
use crate::internal::client::Client;
use crate::internal::config::{CacheConfig, ValidationConfig};
use crate::prelude::*;

#[derive(Clone)]
pub struct IdTokenVerifier {
    inner: Arc<Inner>,
}

struct Inner {
    client: Client,
    cache: Cache,
    validation_config: ValidationConfig,
}

impl IdTokenVerifier {
    pub fn new(config: IdTokenVerifierConfig) -> IdTokenVerifier {
        let client = Client {
            jwks_uri: config.jwks_uri,
            http_client: config.http_client.unwrap_or_else(|| HttpClient::new()),
        };

        let cache = Cache {
            value: Mutex::new(None),
            cache_config: CacheConfig {
                jwks_max_age: config.jwks_max_age,
            },
        };

        let validation_config = ValidationConfig {
            iss: config.iss,
            aud: config.aud,
        };

        let inner = Inner {
            validation_config,
            cache,
            client,
        };

        IdTokenVerifier {
            inner: Arc::new(inner),
        }
    }

    pub async fn verify<Payload>(&self, token: &str) -> Result<Payload, IdTokenVerifierError>
    where
        Payload: DeserializeOwned,
    {
        let header = jsonwebtoken::decode_header(token).map_err(malformed_header)?;
        let kid = header.kid.ok_or_else(|| missing_kid())?;

        let jwks = self
            .inner
            .cache
            .get_or_fetch(|| self.inner.client.fetch())
            .await?;
        let jwk = jwks.find(&kid).ok_or_else(|| key_not_found())?;
        let decoding_key = jsonwebtoken::DecodingKey::from_jwk(jwk).map_err(invalid_key)?;

        let mut validation = jsonwebtoken::Validation::new(header.alg);
        if !self.inner.validation_config.iss.is_empty() {
            validation.set_issuer(&self.inner.validation_config.iss);
        }
        if self.inner.validation_config.aud.is_empty() {
            validation.validate_aud = false;
        } else {
            validation.set_audience(&self.inner.validation_config.aud);
        }

        Ok(
            jsonwebtoken::decode::<Payload>(token, &decoding_key, &validation)
                .map_err(invalid_token)?
                .claims,
        )
    }
}

fn malformed_header(e: jsonwebtoken::errors::Error) -> IdTokenVerifierError {
    IdTokenVerifierError::ValidationFailed(ValidationFailed::MalformedHeader(e))
}

const fn missing_kid() -> IdTokenVerifierError {
    IdTokenVerifierError::ValidationFailed(ValidationFailed::MissingKid)
}

const fn key_not_found() -> IdTokenVerifierError {
    IdTokenVerifierError::ValidationFailed(ValidationFailed::KeyNotFound)
}

fn invalid_key(e: jsonwebtoken::errors::Error) -> IdTokenVerifierError {
    IdTokenVerifierError::ValidationFailed(ValidationFailed::InvalidKey(e))
}

fn invalid_token(e: jsonwebtoken::errors::Error) -> IdTokenVerifierError {
    IdTokenVerifierError::ValidationFailed(ValidationFailed::InvalidToken(e))
}
