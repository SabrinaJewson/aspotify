//! The [Client
//! Credentials](https://developer.spotify.com/documentation/general/guides/authorization-guide/#client-credentials-flow)
//! Spotify authorization flow.

use crate::CLIENT;
use crate::model::*;
use super::{ClientCredentials, AccessToken};
use std::time::Instant;
use tokio::sync::Mutex;

/// An object that holds your client credentials, and caches access tokens with the Client
/// Credentials authorization flow.
///
/// # Examples
/// ```no_run
/// # async {
/// use aspotify::{ClientCredentials, CCFlow};
///
/// // Create a flow, taking credentials from environment variables.
/// let flow = CCFlow::new(
///     ClientCredentials::from_env()
///         .expect("CLIENT_ID or CLIENT_SECRET environment variables not set")
/// );
///
/// // Get a client credentials access token with:
/// let token = flow.send().await.unwrap();
/// # };
/// ```
#[derive(Debug)]
pub struct CCFlow {
    credentials: ClientCredentials,
    cache: Mutex<AccessToken>,
}

impl CCFlow {
    /// Creates a new CCFlow from the client's credentials.
    pub fn new(credentials: ClientCredentials) -> Self {
        Self {
            credentials,
            cache: Mutex::default(),
        }
    }
    /// Get the client credentials.
    pub fn get_credentials(&self) -> &ClientCredentials {
        &self.credentials
    }
    /// Become the client credentials.
    pub fn into_credentials(self) -> ClientCredentials {
        self.credentials
    }
    /// Return the cache or send a request.
    pub async fn send(&self) -> Result<AccessToken, EndpointError<AuthenticationError>> {
        {
            let cache = self.cache.lock().await;
            if Instant::now() < cache.expires {
                return Ok(cache.clone());
            }
        }
        let response = CLIENT
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(&self.credentials.id, Some(&self.credentials.secret))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("grant_type=client_credentials")
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(response.json::<AuthenticationError>().await?.into());
        }
        let token = response.json::<AccessToken>().await?;
        *self.cache.lock().await = token.clone();
        Ok(token)
    }
}
