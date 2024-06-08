use chrono::Duration;

pub(crate) struct ValidationConfig {
    pub iss: Vec<String>,
    pub aud: Vec<String>,
}

pub(crate) struct CacheConfig {
    pub jwks_max_age: Option<Duration>,
}
