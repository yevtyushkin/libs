use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DefaultIdTokenPayload {
    exp: i64,
    sub: String,
}
