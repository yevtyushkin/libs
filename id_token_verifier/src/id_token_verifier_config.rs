use chrono::Duration;
use url::Url;

pub struct IdTokenVerifierConfig {
    pub jwks_uri_type: JwksUriType,
    pub jwks_uri: Url,
    pub jwks_max_age: Option<Duration>,
    pub iss: Vec<String>,
    pub aud: Vec<String>,
}

#[derive(Debug)]
pub enum JwksUriType {
    AutoDiscover,
    Direct,
}
