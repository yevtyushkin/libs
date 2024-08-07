use chrono::{Duration, TimeDelta};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use url::Url;

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct IdTokenVerifierConfig {
    pub jwks_uri_type: JwksUriType,
    pub jwks_uri: Url,
    #[serde(default, deserialize_with = "from_seconds")]
    pub jwks_max_age: Option<Duration>,
    #[serde(default, deserialize_with = "from_comma_separated_string")]
    pub iss: Vec<String>,
    #[serde(default, deserialize_with = "from_comma_separated_string")]
    pub aud: Vec<String>,
    #[serde(default)]
    pub allow_unsafe_configuration: bool,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum JwksUriType {
    AutoDiscover,
    Direct,
}

fn from_seconds<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds: Option<i64> = Deserialize::deserialize(deserializer)?;
    Ok(seconds.map(TimeDelta::seconds))
}

fn from_comma_separated_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let maybe_comma_separated_string: Option<Cow<'_, str>> =
        Deserialize::deserialize(deserializer)?;
    Ok(maybe_comma_separated_string
        .map(|comma_separated_string| {
            comma_separated_string
                .split(",")
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .unwrap_or(vec![]))
}
