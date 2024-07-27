use chrono::Duration;
use std::error::Error;
use tokio;

use id_token_verifier::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = IdTokenVerifierConfig {
        jwks_uri_type: JwksUriType::AutoDiscover,
        jwks_uri: "https://accounts.google.com/.well-known/openid-configuration".parse()?,
        jwks_max_age: Some(Duration::seconds(3600)),
        iss: vec![
            "accounts.google.com".to_string(),
            "https://accounts.google.com".to_string(),
        ],
        aud: vec![],
        allow_unsafe_configuration: true,
    };

    let id_token_verifier = IdTokenVerifierImpl::new(config, None)?;

    // Paste the token from OAuth playground here
    let id_token = "";
    let payload: DefaultIdTokenPayload = id_token_verifier.verify(id_token).await?;

    println!("{payload:?}");

    Ok(())
}
