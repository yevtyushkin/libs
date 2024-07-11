# id_token_verifier

A tiny library for validating ID tokens using JWKS.

# Usage

1. Create a new `IdTokenVerifier` with your configuration:

```rust
// Auto discover from an OpenID Connect metadata document
let jwks_uri_type = JwksUriType::AutoDiscover;
let jwks_uri = "https://accounts.google.com/.well-known/openid-configuration".parse()?;

// Provide your custom HTTP client
let http_client = Some(Client::new());

// Provide a default `max-age` value for caching JWKS. 
// Note: if the JWKS source provides a `Cache-Control` header, its value will have a higher priority than the given `jwks_max_age`.
let jwks_max_age = Some(Duration::seconds(3600));

// Provide valid issuers and audience
let iss = vec![
    "accounts.google.com".to_string(),
    "https://accounts.google.com".to_string(),
];
let aud = vec![
    "my app id".to_string()
];

let config = IdTokenVerifierConfig {
    jwks_uri_type,
    jwks_uri,
    jwks_max_age,
    iss,
    aud
};

let id_token_verifier = IdTokenVerifier::new(config, http_client)?;
```

2. Define your token's payload that has a `serde::Deserialize` impl, or just use a `DefaultIdTokenPayload`:

```rust
#[derive(Deserialize)]
pub struct DefaultIdTokenPayload {
    exp: i64,
    sub: String,
}
```

3. Verify your id token:

```rust
let my_id_token = "my_token";
let payload = id_token_verifier.verify::<MyIdTokenPayload>(my_id_token).await?;
```
