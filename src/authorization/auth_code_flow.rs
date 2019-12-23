//! The [Authorization
//! Code](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow)
//! Spotify authorization flow.

use crate::CLIENT;
use crate::model::*;
use super::{ClientCredentials, AccessToken};
use lazy_static::lazy_static;
use rand::Rng;
use reqwest::Url;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::time::Instant;
use std::{error, str};
use tokio::sync::Mutex;

// TODO: MAKE THIS BETTER LOL XD
const STATE_LEN: usize = 12;
const STATE_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";
lazy_static! {
    static ref STATE: &'static str = {
        let mut rng = rand::thread_rng();

        unsafe {
            // Mutable static as this function is only called once
            static mut STATE: [u8; STATE_LEN] = [0; STATE_LEN];
            for c in STATE.iter_mut() {
                *c = STATE_CHARS[rng.gen_range(0, STATE_CHARS.len())];
            }

            // String only contains ASCII, so it is always valid UTF-8.
            str::from_utf8_unchecked(&STATE)
        }
    };
}

/// A scope that the user can grant access to.
///
/// [Reference](https://developer.spotify.com/documentation/general/guides/scopes/).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scope {
    UgcImageUpload,
    UserReadPlaybackState,
    UserModifyPlaybackState,
    UserReadCurrentlyPlaying,
    Streaming,
    AppRemoteControl,
    UserReadEmail,
    UserReadPrivate,
    PlaylistReadCollaborative,
    PlaylistModifyPublic,
    PlaylistReadPrivate,
    PlaylistModifyPrivate,
    UserLibraryModify,
    UserLibraryRead,
    UserTopRead,
    UserReadRecentlyPlayed,
    UserFollowRead,
    UserFollowModify,
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::UgcImageUpload => "ugc-image-upload",
            Self::UserReadPlaybackState => "user-read-playback-state",
            Self::UserModifyPlaybackState => "user-modify-playback-state",
            Self::UserReadCurrentlyPlaying => "user-read-currently-playing",
            Self::Streaming => "streaming",
            Self::AppRemoteControl => "app-remote-control",
            Self::UserReadEmail => "user-read-email",
            Self::UserReadPrivate => "user-read-private",
            Self::PlaylistReadCollaborative => "playlist-read-collaborative",
            Self::PlaylistModifyPublic => "playlist-modify-public",
            Self::PlaylistReadPrivate => "playlist-read-private",
            Self::PlaylistModifyPrivate => "playlist-modify-private",
            Self::UserLibraryModify => "user-library-modify",
            Self::UserLibraryRead => "user-library-read",
            Self::UserTopRead => "user-top-read",
            Self::UserReadRecentlyPlayed => "user-read-recently-played",
            Self::UserFollowRead => "user-follow-read",
            Self::UserFollowModify => "user-follow-modify",
        })
    }
}

/// Get the URL to redirect the user's browser to. Only URLs from this function can be used to make
/// a new AuthCodeFlow.
///
/// `force_approve`, if set, forces the user to approve the app again even if they already have.
/// Make sure that you have whitelisted the redirect_uri in your Spotify dashboard, and
/// `redirect_uri` must not contain any query strings.
///
/// [Reference](https://developer.spotify.com/documentation/general/guides/authorization-guide/#1-have-your-application-request-authorization-the-user-logs-in-and-authorizes-access).
pub fn get_authorization_url(
    client_id: &str,
    scopes: &[Scope],
    force_approve: bool,
    redirect_uri: &str,
) -> String {
    Url::parse_with_params(
        "https://accounts.spotify.com/authorize",
        &[
            ("response_type", "code"),
            ("state", *STATE),
            ("client_id", client_id),
            (
                "scope",
                &scopes
                    .iter()
                    .map(Scope::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
            ("show_dialog", if force_approve { "true" } else { "false" }),
            ("redirect_uri", redirect_uri),
        ],
    )
    .unwrap()
    .into_string()
}

/// An object that holds your client credentials, and caches access tokens with the Authorization
/// Code authorization flow.
///
/// # Examples
/// ```no_run
/// # async {
/// use std::io::{self, Write};
/// use aspotify::{ClientCredentials, AuthCodeFlow};
///
/// // Get client credentials from environment variables.
/// let credentials = ClientCredentials::from_env().unwrap();
///
/// // Get the URL to send the user to, requesting no scopes and redirecting to a non-existant
/// // website (make sure that the non-existant website is whitelisted on the Spotify dashboard).
/// let url = aspotify::get_authorization_url(&credentials.id, &[], false, "http://non.existant/");
///
/// // Get the user to authorize our application.
/// println!("Go to this website: {}", url);
///
/// // Receive the URL that was redirected to.
/// print!("Enter the URL that you were redirected to: ");
/// io::stdout().flush().unwrap();
/// let mut redirect = String::new();
/// io::stdin().read_line(&mut redirect).unwrap();
///
/// // Create the authorization flow from that redirect.
/// let flow = AuthCodeFlow::from_redirect(credentials, &redirect).await.unwrap();
///
/// // Now you can get access tokens on the user's behalf with:
/// let token = flow.send().await.unwrap();
/// # };
/// ```
#[derive(Debug)]
pub struct AuthCodeFlow {
    credentials: ClientCredentials,
    refresh_token: String,
    cache: Mutex<AccessToken>,
}

impl AuthCodeFlow {
    /// Creates a new Authorization flow from the URL that the user was redirected to after they
    /// visited the URL given by `get_authorization_url`.
    pub async fn from_redirect(
        credentials: ClientCredentials,
        redirected_to: &str,
    ) -> Result<Self, FromRedirectError> {
        let url = Url::parse(redirected_to).map_err(|_| FromRedirectError::InvalidRedirect)?;

        let pairs: HashMap<_, _> = url.query_pairs().collect();
        if pairs
            .get("state")
            .ok_or(FromRedirectError::InvalidRedirect)?
            != *STATE
        {
            return Err(FromRedirectError::InvalidRedirect);
        }
        if let Some(error) = pairs.get("error") {
            return Err(FromRedirectError::SpotifyError(error.to_string()));
        }
        let code = pairs
            .get("code")
            .ok_or(FromRedirectError::InvalidRedirect)?;

        let orig_url = &url.as_str()[0..url
            .as_str()
            .find('?')
            .ok_or(FromRedirectError::InvalidRedirect)?];
        let response = CLIENT
            .post("https://accounts.spotify.com/api/token")
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", orig_url),
            ])
            .basic_auth(&credentials.id, Some(&credentials.secret))
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Response {
            refresh_token: String,
            #[serde(flatten)]
            token: AccessToken,
        }
        let Response {
            refresh_token,
            token,
        } = response.json().await?;

        Ok(Self {
            credentials,
            refresh_token,
            cache: Mutex::new(token),
        })
    }
    /// Creates a new Authorization flow given credentials and a refresh token.
    pub fn from_refresh(credentials: ClientCredentials, refresh_token: String) -> Self {
        Self {
            credentials,
            refresh_token,
            cache: Mutex::default(),
        }
    }
    /// Get the client credentials.
    pub fn get_credentials(&self) -> &ClientCredentials {
        &self.credentials
    }
    /// Get the refresh token.
    pub fn get_refresh_token(&self) -> &str {
        &self.refresh_token
    }
    /// Destructure into client credentials and refresh token.
    pub fn into(self) -> (ClientCredentials, String) {
        (self.credentials, self.refresh_token)
    }
    /// Returns the cache or sends a request.
    pub async fn send(&self) -> Result<AccessToken, EndpointError<AuthenticationError>> {
        let cache = self.cache.lock().await;
        if Instant::now() < cache.expires {
            return Ok(cache.clone());
        }

        let request = CLIENT
            .post("https://accounts.spotify.com/api/token")
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", &self.refresh_token),
            ])
            .basic_auth(&self.credentials.id, Some(&self.credentials.secret));
        drop(cache);

        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(response.json::<AuthenticationError>().await?.into());
        }
        let token = response.json::<AccessToken>().await?;
        *self.cache.lock().await = token.clone();
        Ok(token)
    }
}

/// An error generated from the `AuthCodeFlow::from_redirect` function.
#[derive(Debug)]
pub enum FromRedirectError {
    InvalidRedirect,
    SpotifyError(String),
    HttpError(reqwest::Error),
}

impl From<reqwest::Error> for FromRedirectError {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpError(e)
    }
}

impl Display for FromRedirectError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidRedirect => f.write_str("Invalid redirect URL"),
            Self::SpotifyError(s) => f.write_str(&s),
            Self::HttpError(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for FromRedirectError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::HttpError(e) => Some(e),
            _ => None,
        }
    }
}
