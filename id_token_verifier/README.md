# id_token_verifier

A tiny library for validating ID tokens using JWKS.

# Usage

1. Create a new `IdTokenVerifier` with your configuration:

```rust
// Auto discover from an OpenID Connect metadata document
let auto_discover = JwksUri::AutoDiscover(
    "https://accounts.google.com/.well-known/openid-configuration".parse()?,
);

// Or just use a direct source
let direct = JwksUri::Direct(
    "https://www.googleapis.com/oauth2/v3/certs".parse()?,
);

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
    jwks_uri: auto_discover,
    http_client,
    jwks_max_age,
    iss,
    aud
};

let id_token_verifier = IdTokenVerifier::new(config);
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


