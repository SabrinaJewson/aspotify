//! The [Authorization
//! Code](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow)
//! Spotify authorization flow.

use super::{AccessToken, ClientCredentials};
use crate::model::*;
use crate::CLIENT;
use lazy_static::lazy_static;
use rand::Rng;
use reqwest::Url;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display, Formatter};
use std::time::Instant;
use std::{error, str};
use tokio::sync::Mutex;

lazy_static! {
    static ref VALID_STATES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}
const STATE_LEN: usize = 16;
const STATE_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";

fn random_state() -> String {
    let mut rng = rand::thread_rng();
    let mut state = String::with_capacity(STATE_LEN);
    for _ in 0..STATE_LEN {
        state.push(STATE_CHARS[rng.gen_range(0, STATE_CHARS.len())].into());
    }
    state
}

/// A scope that the user can grant access to.
///
/// [Reference](https://developer.spotify.com/documentation/general/guides/scopes/).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
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
    UserReadPlaybackPosition,
    UserFollowRead,
    UserFollowModify,
}

impl Scope {
    /// Get the scope as a string.
    pub fn as_str(self) -> &'static str {
        match self {
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
            Self::UserReadPlaybackPosition => "user-read-playback-position",
            Self::UserFollowRead => "user-follow-read",
            Self::UserFollowModify => "user-follow-modify",
        }
    }
}

/// Get the URL to redirect the user's browser to. Only URLs from this function can be used to make
/// a new AuthCodeFlow.
///
/// `force_approve`, if set, forces the user to approve the app again even if they already have.
/// Make sure that you have whitelisted the redirect_uri in your Spotify dashboard, and
/// `redirect_uri` must not contain any query strings.
///
/// This method automatically sets the state parameter parameter which
/// [`AuthCodeFlow::from_redirect`](struct.AuthCodeFlow.html#method.from_redirect) then checks,
/// ensuring that fake redirect requests cannnot be done.
///
/// [Reference](https://developer.spotify.com/documentation/general/guides/authorization-guide/#1-have-your-application-request-authorization-the-user-logs-in-and-authorizes-access).
pub async fn get_authorization_url(
    client_id: &str,
    scopes: &[Scope],
    force_approve: bool,
    redirect_uri: &str,
) -> String {
    let mut valid_states = VALID_STATES.lock().await;
    let state = loop {
        let state = random_state();
        if !valid_states.contains(&state) {
            break state;
        }
    };

    let url = Url::parse_with_params(
        "https://accounts.spotify.com/authorize",
        &[
            ("response_type", "code"),
            ("state", &state),
            ("client_id", client_id),
            (
                "scope",
                &scopes
                    .iter()
                    .map(|&scope| scope.as_str())
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
            ("show_dialog", if force_approve { "true" } else { "false" }),
            ("redirect_uri", redirect_uri),
        ],
    )
    .unwrap()
    .into_string();

    valid_states.insert(state);
    url
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
/// let url = aspotify::get_authorization_url(&credentials.id, &[], false, "http://non.existant/").await;
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
        let (token, refresh_token) = auth_code_send(&credentials, redirected_to).await?;

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
        let mut cache = self.cache.lock().await;
        if Instant::now() < cache.expires {
            return Ok(cache.clone());
        }
        let token = refresh_token_send(&self.credentials, &self.refresh_token).await?;
        *cache = token.clone();
        Ok(token)
    }
}

/// Get an Authorization Flow access token and refresh token without creating an AuthFlow from a
/// redirect URL.
pub async fn auth_code_send(
    credentials: &ClientCredentials,
    redirected_to: &str,
) -> Result<(AccessToken, String), FromRedirectError> {
    let url = Url::parse(redirected_to).map_err(|_| FromRedirectError::InvalidRedirect)?;

    let pairs: HashMap<_, _> = url.query_pairs().collect();
    if !VALID_STATES.lock().await.remove(
        &pairs
            .get("state")
            .ok_or(FromRedirectError::InvalidRedirect)?[..],
    ) {
        return Err(FromRedirectError::InvalidRedirect);
    }
    if let Some(error) = pairs.get("error") {
        return Err(FromRedirectError::SpotifyError(SpotifyRedirectError::from(
            error.to_string(),
        )));
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
    } = serde_json::from_str(&response.text().await?)?;

    Ok((token, refresh_token))
}

/// Get an Authorization Flow access token without creating an AuthFlow from a refresh token.
pub async fn refresh_token_send(
    credentials: &ClientCredentials,
    refresh_token: &str,
) -> Result<AccessToken, EndpointError<AuthenticationError>> {
    let response = CLIENT
        .post("https://accounts.spotify.com/api/token")
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .basic_auth(&credentials.id, Some(&credentials.secret))
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;
    if !status.is_success() {
        return Err(EndpointError::SpotifyError(serde_json::from_str(&text)?));
    }
    Ok(serde_json::from_str::<AccessToken>(&text)?)
}

/// An error generated by Spotify after a redirect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotifyRedirectError(String);

impl Display for SpotifyRedirectError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl error::Error for SpotifyRedirectError {}

impl From<String> for SpotifyRedirectError {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// An error generated from the `AuthCodeFlow::from_redirect` function.
#[derive(Debug)]
pub enum FromRedirectError {
    /// The redirect URL was invalid.
    InvalidRedirect,
    /// Parsing Spotify's response failed.
    ParseError(serde_json::error::Error),
    /// The Spotify server failed.
    SpotifyError(SpotifyRedirectError),
    /// An error occurred in the HTTP transport.
    HttpError(reqwest::Error),
}

impl From<serde_json::error::Error> for FromRedirectError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::ParseError(e)
    }
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
            Self::ParseError(e) => write!(f, "{}", e),
            Self::SpotifyError(e) => write!(f, "{}", e),
            Self::HttpError(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for FromRedirectError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::HttpError(e) => Some(e),
            Self::SpotifyError(e) => Some(e),
            _ => None,
        }
    }
}
