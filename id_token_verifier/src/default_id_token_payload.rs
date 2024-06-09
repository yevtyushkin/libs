use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DefaultIdTokenPayload {
    pub exp: i64,
    pub sub: String,
}
