use itertools::Itertools;
use url::Url;

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
    /// Get the scope as a string (in `kebab-case` like Spotify requires).
    #[must_use]
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

/// Like [`authorization_url`](fn.authorization_url.html), but you supply your own state.
///
/// It is recommended to use randomly generated state for security, so use this if you wish to use
/// your own random state generator.
///
/// This function, unlike `authorization_url` does not require features to be activated.
///
/// See the docs of the other function for information about the parameters.
pub fn authorization_url_with_state(
    client_id: &str,
    scopes: impl IntoIterator<Item = Scope>,
    force_approve: bool,
    redirect_uri: &str,
    state: &str,
) -> String {
    Url::parse_with_params(
        "https://accounts.spotify.com/authorize",
        &[
            ("response_type", "code"),
            ("state", &state),
            ("client_id", client_id),
            ("scope", &scopes.into_iter().map(Scope::as_str).join(" ")),
            ("show_dialog", if force_approve { "true" } else { "false" }),
            ("redirect_uri", redirect_uri),
        ],
    )
    .unwrap()
    .into_string()
}

/// Get the URL to redirect the user's browser to so that the URL can be generated for the
/// `Client::redirected` function.
///
/// `force_approve`, if set, forces the user to approve the app again even if they already have.
/// Make sure that you have whitelisted the redirect uri in your Spotify dashboard, and
/// `redirect_uri` must not contain any query strings.
///
/// This method returns a tuple of the generated url and the state parameter, which is randomly
/// generated for security.
///
/// This function is only available when the `rand` feature of this library is activated, and it is
/// activated by default.
///
/// [Reference](https://developer.spotify.com/documentation/general/guides/authorization-guide/#1-have-your-application-request-authorization-the-user-logs-in-and-authorizes-access).
#[cfg(feature = "rand")]
pub fn authorization_url(
    client_id: &str,
    scopes: impl IntoIterator<Item = Scope>,
    force_approve: bool,
    redirect_uri: &str,
) -> (String, String) {
    use rand::Rng as _;

    const STATE_LEN: usize = 16;
    const STATE_CHARS: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";

    let mut rng = rand::thread_rng();
    let mut state = String::with_capacity(STATE_LEN);
    for _ in 0..STATE_LEN {
        state.push(STATE_CHARS[rng.gen_range(0, STATE_CHARS.len())].into());
    }

    (
        authorization_url_with_state(client_id, scopes, force_approve, redirect_uri, &state),
        state,
    )
}
