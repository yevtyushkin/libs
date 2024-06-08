use chrono::Duration;
use std::error::Error;
use tokio;

use id_token_verifier::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = IdTokenVerifierConfig {
        jwks_uri: JwksUri::AutoDiscover(
            "https://accounts.google.com/.well-known/openid-configuration".parse()?,
        ),
        http_client: None,
        jwks_max_age: Some(Duration::seconds(3600)),
        iss: vec![
            "accounts.google.com".to_string(),
            "https://accounts.google.com".to_string(),
        ],
        aud: vec![],
    };

    let id_token_verifier = IdTokenVerifier::new(config);

    // Paste the token from OAuth playground here
    let id_token = "";
    let payload = id_token_verifier
        .verify::<DefaultIdTokenPayload>(id_token)
        .await?;

    println!("{payload:?}");

    Ok(())
}
