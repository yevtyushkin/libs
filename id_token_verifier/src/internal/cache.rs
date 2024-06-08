use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::jwk::JwkSet as Jwks;
use tokio::sync::Mutex;

use crate::id_token_verifier_error::IdTokenVerifierError;
use crate::internal::client::JwksFetchResult;
use crate::internal::config::CacheConfig;

pub(crate) struct Cache {
    pub value: Mutex<Option<State>>,
    pub cache_config: CacheConfig,
}

pub(crate) struct State {
    jwks: Arc<Jwks>,
    expires_on: DateTime<Utc>,
}

impl Cache {
    pub async fn get_or_fetch<F, Fut>(&self, fetch: F) -> Result<Arc<Jwks>, IdTokenVerifierError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<JwksFetchResult, IdTokenVerifierError>>,
    {
        let mut guard = self.value.lock().await;

        let now = Utc::now();

        match guard.deref() {
            Some(State { jwks, expires_on }) if expires_on > &now => Ok(jwks.clone()),
            _ => {
                let fetch_result = fetch().await?;
                let jwks = Arc::new(fetch_result.jwks);
                let max_age = fetch_result
                    .max_age
                    .or_else(|| self.cache_config.jwks_max_age)
                    .unwrap_or_else(|| Duration::zero());
                let expires_on = Utc::now() + max_age;

                *(guard.deref_mut()) = Some(State {
                    jwks: jwks.clone(),
                    expires_on,
                });

                Ok(jwks.clone())
            }
        }
    }
}
