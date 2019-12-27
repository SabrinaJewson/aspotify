//! Everything relating to authorization for the Spotify endpoints.

pub mod auth_code_flow;
pub mod c_c_flow;

use crate::util::*;
pub use auth_code_flow::*;
pub use c_c_flow::*;
use serde::Deserialize;
use std::env::{self, VarError};
use std::ffi::OsStr;
use std::time::{Duration, Instant};

/// An object that holds your Spotify Client ID and Client Secret.
///
/// # Examples
/// ```no_run
/// # async {
/// use aspotify::ClientCredentials;
///
/// // Create from inside the program.
/// let credentials = ClientCredentials {
///     id: String::from("your client id here"),
///     secret: String::from("your client secret here")
/// };
///
/// // Create from environment variables
/// let credentials = ClientCredentials::from_env()
///     .expect("CLIENT_ID or CLIENT_SECRET environment variables not set");
///
/// // The above line is equivalent to:
/// let flow = ClientCredentials::from_env_vars("CLIENT_ID", "CLIENT_SECRET")
///     .expect("CLIENT ID or CLIENT_SECRET environment variables not set");
/// # };
/// ```
#[derive(Debug, Clone)]
pub struct ClientCredentials {
    pub id: String,
    pub secret: String,
}

impl ClientCredentials {
    /// Attempts to create a ClientCredentials by reading environment variables.
    pub fn from_env_vars<I: AsRef<OsStr>, S: AsRef<OsStr>>(
        client_id: I,
        client_secret: S,
    ) -> Result<Self, VarError> {
        Ok(Self {
            id: env::var(client_id)?,
            secret: env::var(client_secret)?,
        })
    }
    /// Attempts to create a ClientCredentials by reading the CLIENT_ID and CLIENT_SECRET
    /// environment variables.
    ///
    /// Equivalent to `ClientCredentials::from_env_vars("CLIENT_ID", "CLIENT_SECRET")`.
    pub fn from_env() -> Result<Self, VarError> {
        Self::from_env_vars("CLIENT_ID", "CLIENT_SECRET")
    }
}

/// An access token for the Spotify API.
///
/// Generate these with CCFlow or AuthCodeFlow.
#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    #[serde(rename = "access_token")]
    pub token: String,
    #[serde(rename = "expires_in", deserialize_with = "instant_from_seconds")]
    pub expires: Instant,
}

impl Default for AccessToken {
    /// An already-expired token.
    fn default() -> Self {
        Self {
            token: String::new(),
            expires: Instant::now() - Duration::from_secs(1),
        }
    }
}
