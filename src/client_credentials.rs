//! The [Client
//! Credentials](https://developer.spotify.com/documentation/general/guides/authorization-guide/#client-credentials-flow)
//! Spotify authorization flow.

use crate::util::*;
use crate::*;
use serde::Deserialize;
use std::env::{self, VarError};
use std::ffi::OsStr;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// An object that holds your Client ID and Client Secret, and caches access tokens.
///
/// # Examples
/// ```no_run
/// # async {
/// use aspotify::ClientCredentials;
///
/// // Create from inside the program.
/// let credentials = ClientCredentials::new(
///     String::from("your client id here"),
///     String::from("your client secret here")
/// );
///
/// // Create from environment variables
/// let credentials = ClientCredentials::from_env()
///     .expect("CLIENT_ID or CLIENT_SECRET environment variables not set");
///
/// // The above line is equivalent to:
/// let credentials = ClientCredentials::from_env_vars("CLIENT_ID", "CLIENT_SECRET")
///     .expect("CLIENT ID or CLIENT_SECRET environment variables not set");
///
/// // Get a client credentials access token with:
/// let token = credentials.send().await.unwrap();
/// # };
/// ```
pub struct ClientCredentials {
    client_id: String,
    client_secret: String,
    cache: Mutex<CCToken>,
}

impl ClientCredentials {
    /// Creates a ClientCredentials from client_id and client_secret strings.
    pub fn new(client_id: String, client_secret: String) -> ClientCredentials {
        ClientCredentials {
            client_id,
            client_secret,
            cache: Mutex::new(CCToken::default()),
        }
    }
    /// Attempts to create a ClientCredentials by reading environment variables.
    pub fn from_env_vars<I: AsRef<OsStr>, S: AsRef<OsStr>>(
        client_id: I,
        client_secret: S,
    ) -> Result<ClientCredentials, VarError> {
        Ok(ClientCredentials::new(
            env::var(client_id)?,
            env::var(client_secret)?,
        ))
    }
    /// Attempts to create a ClientCredentials by reading the CLIENT_ID and CLIENT_SECRET
    /// environment variables.
    ///
    /// Equivalent to `ClientCredentials::from_env_vars("CLIENT_ID", "CLIENT_SECRET")`.
    pub fn from_env() -> Result<ClientCredentials, VarError> {
        ClientCredentials::from_env_vars("CLIENT_ID", "CLIENT_SECRET")
    }
    /// Get the client id and client secret.
    pub fn get_credentials(&self) -> (&str, &str) {
        (&self.client_id, &self.client_secret)
    }
    /// Get the client id and client secret, consuming the object.
    pub fn into_credentials(self) -> (String, String) {
        (self.client_id, self.client_secret)
    }
    /// Return the cache or send a request.
    pub async fn send(&self) -> Result<CCToken, EndpointError<AuthenticationError>> {
        {
            let cache = self.cache.lock().await;
            if Instant::now() < cache.expires {
                return Ok(cache.clone());
            }
        }
        let response = CLIENT
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("grant_type=client_credentials")
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(response.json::<AuthenticationError>().await?.into());
        }
        let token = response.json::<CCToken>().await?;
        *self.cache.lock().await = token.clone();
        Ok(token)
    }
}

/// A client credentials access token.
#[derive(Debug, Clone, Deserialize)]
pub struct CCToken {
    #[serde(rename = "access_token")]
    pub token: String,
    #[serde(rename = "expires_in", deserialize_with = "from_seconds")]
    pub expires: Instant,
}

impl AccessToken for CCToken {
    fn get_token(&self) -> &str {
        &self.token
    }
}

impl Default for CCToken {
    /// An already-expired token.
    fn default() -> Self {
        Self {
            token: String::new(),
            expires: Instant::now() - Duration::from_secs(1),
        }
    }
}
