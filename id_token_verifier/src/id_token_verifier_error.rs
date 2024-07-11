use std::fmt::Debug;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum IdTokenVerifierError {
    #[error("Unsafe configuration provided: iss empty = {iss_empty}, aud_empty = {aud_empty}")]
    UnsafeConfiguration { iss_empty: bool, aud_empty: bool },
    #[error("Failed to fetch jwks {0:?}")]
    FailedToFetchJwks(FailedToFetchJwks),
    #[error("Failed to validate the token {0:?}")]
    ValidationFailed(ValidationFailed),
}

#[derive(Debug)]
pub enum ConfigurationError {
    EmptyIssuers,
    EmptyAudience,
}

#[derive(Debug)]
pub enum FailedToFetchJwks {
    FailedAutoDiscoverRequest(reqwest::Error),
    InvalidAutoDiscoverResponse(reqwest::Error),
    InvalidJwksUri(url::ParseError),
    MissingJwksUri,
    FailedJwksRequest(reqwest::Error),
    InvalidJwksResponse(reqwest::Error),
}

#[derive(Debug)]
pub enum ValidationFailed {
    MalformedHeader(jsonwebtoken::errors::Error),
    MissingKid,
    KeyNotFound,
    InvalidKey(jsonwebtoken::errors::Error),
    InvalidToken(jsonwebtoken::errors::Error),
}
