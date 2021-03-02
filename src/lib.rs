//! aspotify is an asynchronous client to the [Spotify
//! API](https://developer.spotify.com/documentation/web-api/).
//!
//! # Examples
//! ```
//! # async {
//! use aspotify::{Client, ClientCredentials};
//!
//! // This from_env function tries to read the CLIENT_ID and CLIENT_SECRET environment variables.
//! // You can use the dotenv crate to read it from a file.
//! let credentials = ClientCredentials::from_env()
//!     .expect("CLIENT_ID and CLIENT_SECRET not found.");
//!
//! // Create a Spotify client.
//! let client = Client::new(credentials);
//!
//! // Gets the album "Favourite Worst Nightmare" from Spotify, with no specified market.
//! let album = client.albums().get_album("1XkGORuUX2QGOEIL4EbJKm", None).await.unwrap();
//! # };
//! ```
//!
//! # Notes
//! - Spotify often imposes limits on endpoints, for example you can't get more than 50 tracks at
//! once. This crate removes this limit by making multiple requests when necessary.
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::non_ascii_literal,
    clippy::items_after_statements,
    clippy::filter_map
)]
#![cfg_attr(test, allow(clippy::float_cmp))]

use std::collections::HashMap;
use std::env::{self, VarError};
use std::error::Error as StdError;
use std::ffi::OsStr;
use std::fmt::{self, Display, Formatter};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use reqwest::{header, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub use authorization_url::*;
pub use endpoints::*;
/// Re-export from [`isocountry`].
pub use isocountry::CountryCode;
/// Re-export from [`isolanguage_1`].
pub use isolanguage_1::LanguageCode;
pub use model::*;

mod authorization_url;
pub mod endpoints;
pub mod model;
mod util;

#[async_trait]
pub trait Authenticator {
    async fn get_token(&self, client: &reqwest::Client) -> Result<String, Error>;
}

pub struct ApiAuthenticator {
    credentials: ClientCredentials,
    cache: Mutex<AccessToken>,
}

impl ApiAuthenticator {
    pub fn with_credentials(credentials: ClientCredentials) -> Self {
        Self {
            credentials,
            cache: Mutex::new(AccessToken::expired()),
        }
    }

    pub fn with_refresh_token(credentials: ClientCredentials, refresh_token: String) -> Self {
        Self {
            credentials,
            cache: Mutex::new(AccessToken::expired_with_refresh(refresh_token)),
        }
    }

    /// Set the refresh token from the URL the client was redirected to and the state that was used
    /// to send them there.
    ///
    /// Use the [`authorization_url()`] function to generate the URL to which you can send the
    /// client to to generate the URL here.
    ///
    /// # Errors
    ///
    /// Fails if the URL is invalid in some way, the state was incorrect for the URL or Spotify
    /// fails.
    pub async fn redirected(&self, url: &str, state: &str) -> Result<(), RedirectedError> {
        let url = Url::parse(url)?;

        let pairs: HashMap<_, _> = url.query_pairs().collect();

        if pairs
            .get("state")
            .map_or(true, |url_state| url_state != state)
        {
            return Err(RedirectedError::IncorrectState);
        }

        if let Some(error) = pairs.get("error") {
            return Err(RedirectedError::AuthFailed(error.to_string()));
        }

        let code = pairs
            .get("code")
            .ok_or_else(|| RedirectedError::AuthFailed(String::new()))?;

        let token = self
            .token_request(TokenRequest::AuthorizationCode {
                code: &*code,
                redirect_uri: &url[..url::Position::AfterPath],
            })
            .await?;
        *self.cache.lock().await = token;

        Ok(())
    }

    async fn request_token(
        &self,
        client: &reqwest::Client,
        params: TokenRequest<'_>,
    ) -> Result<AccessToken, Error> {
        let request = client
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(&self.credentials.id, Some(&self.credentials.secret))
            .form(&params)
            .build()?;
        if cfg!(test) {
            dbg!(&request, body_str(&request));
        }

        let response = client.execute(request).await?;
        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            if cfg!(test) {
                eprintln!("Authentication response body is '{}'", text);
            }
            let token = serde_json::from_str(&text)?;
            Ok(token)
        } else {
            if cfg!(test) {
                eprintln!(
                    "Authentication failed ({}). Response body is '{}'",
                    status, text
                );
            }
            let auth_error = serde_json::from_str(&text)?;
            Err(Error::Auth(auth_error))
        }
    }
}

#[async_trait]
impl Authenticator for ApiAuthenticator {
    async fn get_token(&self, client: &reqwest::Client) -> Result<String, Error> {
        let mut cache = self.cache.lock().await;

        if cache.is_expired() {
            let token_request = match cache.refresh_token.as_ref() {
                // Use refresh token for re-newing the access token.
                Some(refresh) => TokenRequest::RefreshToken {
                    refresh_token: refresh,
                },
                // Use credential authentication.
                None => TokenRequest::ClientCredentials,
            };
            *cache = self.request_token(client, token_request).await?;
        }

        Ok(cache.token.clone())
    }
}

pub struct ClientBuilder<T: Authenticator> {
    client: Option<reqwest::Client>,
    authenticator: Option<T>,
}

impl<T: Authenticator> ClientBuilder<T> {
    pub fn new() -> Self {
        Self {
            client: None,
            authenticator: None,
        }
    }

    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client.replace(client);
        self
    }

    pub fn authenticator(mut self, authenticator: T) -> Self {
        self.authenticator.replace(authenticator);
        self
    }

    pub fn build(self) -> Client<T> {
        Client {
            client: self.client.unwrap_or_default(),
            authenticator: self.authenticator.unwrap(),
        }
    }
}

/// A client to the Spotify API.
///
/// By default it will use the [client credentials
/// flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#client-credentials-flow)
/// to send requests to the Spotify API. The [`set_refresh_token`](Client::set_refresh_token) and
/// [`redirected`](Client::redirected) methods tell it to use the [authorization code
/// flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow)
/// instead.
#[derive(Debug)]
pub struct Client<T: Authenticator> {
    client: reqwest::Client,
    authenticator: T,
}

impl<T: Authenticator> Client<T> {
    /// Create a new client from your Spotify client credentials.
    #[must_use]
    pub fn new(credentials: ClientCredentials) -> Self {
        ClientBuilder::new()
            .authenticator(ApiAuthenticator::with_credentials(credentials))
            .build()
    }

    /// Create a new client with your Spotify client credentials and a refresh token.
    #[must_use]
    pub fn with_refresh(credentials: ClientCredentials, refresh_token: String) -> Self {
        ClientBuilder::new()
            .authenticator(ApiAuthenticator::with_refresh_token(
                credentials,
                refresh_token,
            ))
            .build()
    }

    pub fn authenticator(&self) -> &T {
        &self.authenticator
    }

    async fn send_text(&self, request: RequestBuilder) -> Result<Response<String>, Error> {
        let request = request
            .bearer_auth(&self.authenticator.get_token(&self.client).await?.token)
            .build()?;

        if cfg!(test) {
            dbg!(&request, body_str(&request));
        }

        let response = loop {
            let response = self.client.execute(request.try_clone().unwrap()).await?;
            if response.status() != 429 {
                break response;
            }
            let wait = response
                .headers()
                .get(header::RETRY_AFTER)
                .and_then(|val| val.to_str().ok())
                .and_then(|secs| secs.parse::<u64>().ok());
            // 2 seconds is default retry after time; should never be used if the Spotify API and
            // my code are both correct.
            let wait = wait.unwrap_or(2);
            tokio::time::sleep(Duration::from_secs(wait)).await;
        };
        let status = response.status();
        let cache_control = Duration::from_secs(
            response
                .headers()
                .get_all(header::CACHE_CONTROL)
                .iter()
                .filter_map(|value| value.to_str().ok())
                .flat_map(|value| value.split(|c| c == ','))
                .find_map(|value| {
                    let mut parts = value.trim().splitn(2, '=');
                    if parts.next().unwrap().eq_ignore_ascii_case("max-age") {
                        parts.next().and_then(|max| max.parse::<u64>().ok())
                    } else {
                        None
                    }
                })
                .unwrap_or_default(),
        );

        let data = response.text().await?;
        if !status.is_success() {
            if cfg!(test) {
                eprintln!("Failed ({}). Response body is '{}'", status, data);
            }
            return Err(Error::Endpoint(serde_json::from_str(&data)?));
        }

        if cfg!(test) {
            dbg!(status);
            eprintln!("Response body is '{}'", data);
        }

        Ok(Response {
            data,
            expires: Instant::now() + cache_control,
        })
    }

    async fn send_empty(&self, request: RequestBuilder) -> Result<(), Error> {
        self.send_text(request).await?;
        Ok(())
    }

    async fn send_opt_json<U: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<Response<Option<U>>, Error> {
        let res = self.send_text(request).await?;
        Ok(Response {
            data: if res.data.is_empty() {
                None
            } else {
                serde_json::from_str(&res.data)?
            },
            expires: res.expires,
        })
    }

    async fn send_json<U: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<Response<U>, Error> {
        let res = self.send_text(request).await?;
        Ok(Response {
            data: serde_json::from_str(&res.data)?,
            expires: res.expires,
        })
    }

    async fn send_snapshot_id(&self, request: RequestBuilder) -> Result<String, Error> {
        #[derive(Deserialize)]
        struct SnapshotId {
            snapshot_id: String,
        }
        Ok(self
            .send_json::<SnapshotId>(request)
            .await?
            .data
            .snapshot_id)
    }
}

/// The result of a request to a Spotify endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Response<T> {
    /// The data itself.
    pub data: T,
    /// When the cache expires.
    pub expires: Instant,
}

impl<T> Response<T> {
    /// Map the contained data if there is any.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Response<U> {
        Response {
            data: f(self.data),
            expires: self.expires,
        }
    }
}

/// An object that holds your Spotify Client ID and Client Secret.
///
/// See [the Spotify guide on Spotify
/// apps](https://developer.spotify.com/documentation/general/guides/app-settings/) for how to get
/// these.
///
/// # Examples
///
/// ```no_run
/// use aspotify::ClientCredentials;
///
/// // Create from inside the program.
/// let credentials = ClientCredentials {
///     id: "your client id here".to_owned(),
///     secret: "your client secret here".to_owned()
/// };
///
/// // Create from CLIENT_ID and CLIENT_SECRET environment variables
/// let credentials = ClientCredentials::from_env()
///     .expect("CLIENT_ID or CLIENT_SECRET environment variables not set");
///
/// // Or use custom env var names
/// let credentials = ClientCredentials::from_env_vars("SPOTIFY_ID", "SPOTIFY_SECRET")
///     .expect("SPOTIFY_ID or SPOTIFY_SECRET environment variables not set");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientCredentials {
    /// The Client ID.
    pub id: String,
    /// The Client Secret.
    pub secret: String,
}

impl ClientCredentials {
    /// Attempts to create a `ClientCredentials` by reading environment variables.
    ///
    /// # Errors
    ///
    /// Fails if the environment variables are not present or are not unicode.
    pub fn from_env_vars<I: AsRef<OsStr>, S: AsRef<OsStr>>(
        client_id: I,
        client_secret: S,
    ) -> Result<Self, VarError> {
        Ok(Self {
            id: env::var(client_id)?,
            secret: env::var(client_secret)?,
        })
    }
    /// Attempts to create a `ClientCredentials` by reading the `CLIENT_ID` and `CLIENT_SECRET`
    /// environment variables.
    ///
    /// Equivalent to `ClientCredentials::from_env_vars("CLIENT_ID", "CLIENT_SECRET")`.
    ///
    /// # Errors
    ///
    /// Fails if the environment variables are not present or are not unicode.
    pub fn from_env() -> Result<Self, VarError> {
        Self::from_env_vars("CLIENT_ID", "CLIENT_SECRET")
    }

    pub fn empty() -> Self {
        Self {
            id: String::new(),
            secret: String::new(),
        }
    }
}

/// An error caused by the [`Client::redirected`] function.
#[derive(Debug)]
pub enum RedirectedError {
    /// The URL is malformed.
    InvalidUrl(url::ParseError),
    /// The URL has no state parameter, or the state parameter was incorrect.
    IncorrectState,
    /// The user has not accepted the request or an error occured in Spotify.
    ///
    /// This contains the string returned by Spotify in the `error` parameter.
    AuthFailed(String),
    /// An error occurred getting the access token.
    Token(Error),
}

impl From<url::ParseError> for RedirectedError {
    fn from(error: url::ParseError) -> Self {
        Self::InvalidUrl(error)
    }
}

impl From<Error> for RedirectedError {
    fn from(error: Error) -> Self {
        Self::Token(error)
    }
}

impl Display for RedirectedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUrl(_) => f.write_str("malformed redirect URL"),
            Self::IncorrectState => f.write_str("state parameter not found or is incorrect"),
            Self::AuthFailed(_) => f.write_str("authorization failed"),
            Self::Token(e) => e.fmt(f),
        }
    }
}

impl StdError for RedirectedError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(match self {
            Self::InvalidUrl(e) => e,
            Self::Token(e) => e,
            _ => return None,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "grant_type", rename_all = "snake_case")]
enum TokenRequest<'a> {
    RefreshToken {
        refresh_token: &'a str,
    },
    ClientCredentials,
    AuthorizationCode {
        code: &'a str,
        redirect_uri: &'a str,
    },
}

#[derive(Debug, Deserialize)]
struct AccessToken {
    #[serde(rename = "access_token")]
    token: String,
    #[serde(
        rename = "expires_in",
        deserialize_with = "util::deserialize_instant_seconds"
    )]
    expires: Instant,
    #[serde(default)]
    refresh_token: Option<String>,
}

impl AccessToken {
    fn expired_with_refresh(refresh_token: String) -> Self {
        Self {
            token: String::new(),
            expires: Instant::now() - Duration::from_secs(1),
            refresh_token: Some(refresh_token),
        }
    }

    fn expired() -> Self {
        Self {
            token: String::new(),
            expires: Instant::now() - Duration::from_secs(1),
            refresh_token: None,
        }
    }

    fn is_expired(&self) -> bool {
        self.expires <= Instant::now()
    }
}

/// Get the contents of a request body as a string. This is only used for debugging purposes.
fn body_str(req: &reqwest::Request) -> Option<&str> {
    req.body().map(|body| {
        body.as_bytes().map_or("stream", |bytes| {
            std::str::from_utf8(bytes).unwrap_or("opaque bytes")
        })
    })
}
