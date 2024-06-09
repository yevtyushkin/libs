use chrono::Duration;
use reqwest::Client as HttpClient;
use url::Url;

pub struct IdTokenVerifierConfig {
    pub jwks_uri: JwksUri,
    pub jwks_max_age: Option<Duration>,
    pub iss: Vec<String>,
    pub aud: Vec<String>,
    pub http_client: Option<HttpClient>,
}

#[derive(Debug)]
pub enum JwksUri {
    AutoDiscover(Url),
    Direct(Url),
}
