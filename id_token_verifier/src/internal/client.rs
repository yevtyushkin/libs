use chrono::Duration;
use jsonwebtoken::jwk::JwkSet as Jwks;
use reqwest::header::{HeaderValue, CACHE_CONTROL};
use reqwest::Client as HttpClient;
use serde_json::Value;
use url::Url;

use crate::prelude::*;

pub(crate) struct Client {
    pub http_client: HttpClient,
    pub jwks_uri_type: JwksUriType,
    pub jwks_uri: Url,
}

pub(crate) struct JwksFetchResult {
    pub jwks: Jwks,
    pub max_age: Option<Duration>,
}

impl Client {
    pub async fn fetch(&self) -> Result<JwksFetchResult, IdTokenVerifierError> {
        let jwks_uri = self.get_jwks_uri().await?;

        self.fetch_jwks(jwks_uri).await
    }

    async fn get_jwks_uri(&self) -> Result<Url, IdTokenVerifierError> {
        match &self.jwks_uri_type {
            JwksUriType::AutoDiscover => self.auto_discover_jwks_uri().await,
            JwksUriType::Direct => Ok(self.jwks_uri.clone()),
        }
    }

    async fn auto_discover_jwks_uri(&self) -> Result<Url, IdTokenVerifierError> {
        match self
            .http_client
            .get(self.jwks_uri.clone())
            .send()
            .await
            .map_err(failed_auto_discover_request)?
            .json::<Value>()
            .await
            .map_err(invalid_auto_discover_response)?
            .get("jwks_uri")
            .and_then(Value::as_str)
        {
            Some(uri) => Url::parse(uri).map_err(invalid_jwks_uri),
            None => Err(missing_jwks_uri()),
        }
    }

    async fn fetch_jwks(&self, jwks_uri: Url) -> Result<JwksFetchResult, IdTokenVerifierError> {
        let response = self
            .http_client
            .get(jwks_uri)
            .send()
            .await
            .map_err(failed_jwks_request)?;

        let max_age = response
            .headers()
            .get(CACHE_CONTROL)
            .and_then(Self::parse_max_age);

        let jwks = response
            .json::<Jwks>()
            .await
            .map_err(invalid_jwks_response)?;

        Ok(JwksFetchResult { jwks, max_age })
    }

    fn parse_max_age(value: &HeaderValue) -> Option<Duration> {
        value.to_str().ok().and_then(|s| {
            s.split(",").into_iter().find_map(|directive| {
                let mut split = directive.split("=").map(str::trim);
                if let Some("max-age") = split.next() {
                    split
                        .next()
                        .and_then(|s| str::parse::<i64>(s).ok())
                        .and_then(Duration::try_seconds)
                } else {
                    None
                }
            })
        })
    }
}

fn failed_auto_discover_request(e: reqwest::Error) -> IdTokenVerifierError {
    IdTokenVerifierError::FailedToFetchJwks(FailedToFetchJwks::FailedAutoDiscoverRequest(e))
}

fn invalid_auto_discover_response(e: reqwest::Error) -> IdTokenVerifierError {
    IdTokenVerifierError::FailedToFetchJwks(FailedToFetchJwks::InvalidAutoDiscoverResponse(e))
}

fn invalid_jwks_uri(e: url::ParseError) -> IdTokenVerifierError {
    IdTokenVerifierError::FailedToFetchJwks(FailedToFetchJwks::InvalidJwksUri(e))
}

const fn missing_jwks_uri() -> IdTokenVerifierError {
    IdTokenVerifierError::FailedToFetchJwks(FailedToFetchJwks::MissingJwksUri)
}

fn failed_jwks_request(e: reqwest::Error) -> IdTokenVerifierError {
    IdTokenVerifierError::FailedToFetchJwks(FailedToFetchJwks::FailedJwksRequest(e))
}

fn invalid_jwks_response(e: reqwest::Error) -> IdTokenVerifierError {
    IdTokenVerifierError::FailedToFetchJwks(FailedToFetchJwks::InvalidJwksResponse(e))
}
