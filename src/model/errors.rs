use crate::model::*;
use reqwest::StatusCode;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::{error, io};

/// A marker trait for an error caused by a Spotify endpoint.
///
/// The [`AnyError`](enum.AnyError.html) type is an enum of all these.
pub trait SpotifyError: error::Error {}

/// An error caused by one of the Web API endpoints relating to authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationError {
    /// A high level description of the error.
    pub error: String,
    /// A more detailed description of the error.
    pub error_description: String,
}

impl Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error, self.error_description)
    }
}

impl error::Error for AuthenticationError {}
impl SpotifyError for AuthenticationError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ErrorWrapper<T> {
    error: T,
}

/// A regular error object returns by endpoints of the API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    from = "ErrorWrapper<ErrorInternal>",
    into = "ErrorWrapper<ErrorInternal>"
)]
pub struct Error {
    /// The HTTP status code of the error.
    pub status: StatusCode,
    /// A short description of the error's cause.
    pub message: String,
}
impl From<PlayerError> for Error {
    fn from(e: PlayerError) -> Self {
        Self {
            status: e.status,
            message: e.message,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ErrorInternal {
    #[serde(with = "serde_status_code")]
    status: StatusCode,
    message: String,
}

impl From<ErrorWrapper<ErrorInternal>> for Error {
    fn from(error: ErrorWrapper<ErrorInternal>) -> Self {
        Self {
            status: error.error.status,
            message: error.error.message,
        }
    }
}

impl From<Error> for ErrorWrapper<ErrorInternal> {
    fn from(error: Error) -> Self {
        Self {
            error: ErrorInternal {
                status: error.status,
                message: error.message,
            },
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error {}: {}", self.status, self.message)
    }
}

impl error::Error for Error {}
impl SpotifyError for Error {}

/// An error returned by the player. It is an extension of Error, with an additional reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    from = "ErrorWrapper<PlayerErrorInternal>",
    into = "ErrorWrapper<PlayerErrorInternal>"
)]
pub struct PlayerError {
    /// The HTTP status code of the error.
    pub status: StatusCode,
    /// A short description of the error's cause.
    pub message: String,
    /// A reason for the error.
    pub reason: PlayerErrorReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PlayerErrorInternal {
    #[serde(with = "serde_status_code")]
    status: StatusCode,
    message: String,
    reason: PlayerErrorReason,
}

impl From<ErrorWrapper<PlayerErrorInternal>> for PlayerError {
    fn from(error: ErrorWrapper<PlayerErrorInternal>) -> Self {
        Self {
            status: error.error.status,
            message: error.error.message,
            reason: error.error.reason,
        }
    }
}

impl From<PlayerError> for ErrorWrapper<PlayerErrorInternal> {
    fn from(error: PlayerError) -> Self {
        Self {
            error: PlayerErrorInternal {
                status: error.status,
                message: error.message,
                reason: error.reason,
            },
        }
    }
}

impl Display for PlayerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.reason)
    }
}

impl error::Error for PlayerError {}
impl SpotifyError for PlayerError {}

/// An error returned by authentication endpoints, regular endpoints or player endpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyError {
    /// The error was with authentication.
    Authentication(AuthenticationError),
    /// The error was a regular error.
    Regular(Error),
    /// The error was a player error.
    Player(PlayerError),
}

impl From<AuthenticationError> for AnyError {
    fn from(e: AuthenticationError) -> Self {
        Self::Authentication(e)
    }
}
impl From<Error> for AnyError {
    fn from(e: Error) -> Self {
        Self::Regular(e)
    }
}
impl From<PlayerError> for AnyError {
    fn from(e: PlayerError) -> Self {
        Self::Player(e)
    }
}

impl Display for AnyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Authentication(e) => write!(f, "authentication error: {}", e),
            Self::Regular(e) => e.fmt(f),
            Self::Player(e) => write!(f, "player error: {}", e),
        }
    }
}

impl error::Error for AnyError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Self::Authentication(e) => e,
            Self::Regular(e) => e,
            Self::Player(e) => e,
        })
    }
}

/// An HTTP error or an error from the endpoint.
///
/// See [`SpotifyError`](trait.SpotifyError.html) for the errors that this could contain.
#[derive(Debug)]
pub enum EndpointError<E> {
    /// An error caused when sending the HTTP request.
    HttpError(reqwest::Error),
    /// An error caused parsing the response
    ParseError(serde_json::error::Error),
    /// An error caused by the Spotify server.
    SpotifyError(E),
    /// Any other IO error.
    IoError(io::Error),
}

impl<E: Display> Display for EndpointError<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::HttpError(e) => write!(f, "{}", e),
            Self::ParseError(e) => write!(f, "{}", e),
            Self::SpotifyError(e) => write!(f, "{}", e),
            Self::IoError(e) => write!(f, "{}", e),
        }
    }
}

impl<E: error::Error + 'static> error::Error for EndpointError<E> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::HttpError(e) => Some(e),
            Self::ParseError(e) => Some(e),
            Self::IoError(e) => Some(e),
            Self::SpotifyError(e) => Some(e),
        }
    }
}

impl<E: SpotifyError + Into<AnyError>> From<EndpointError<E>> for EndpointError<AnyError> {
    fn from(error: EndpointError<E>) -> Self {
        match error {
            EndpointError::HttpError(e) => Self::HttpError(e),
            EndpointError::ParseError(e) => Self::ParseError(e),
            EndpointError::SpotifyError(e) => Self::SpotifyError(e.into()),
            EndpointError::IoError(e) => Self::IoError(e),
        }
    }
}

impl<E> From<reqwest::Error> for EndpointError<E> {
    fn from(error: reqwest::Error) -> Self {
        Self::HttpError(error)
    }
}

impl<E> From<serde_json::error::Error> for EndpointError<E> {
    fn from(error: serde_json::error::Error) -> Self {
        Self::ParseError(error)
    }
}

impl<E: SpotifyError> From<E> for EndpointError<E> {
    fn from(error: E) -> Self {
        Self::SpotifyError(error)
    }
}

impl<E: SpotifyError> From<io::Error> for EndpointError<E> {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

/// A reason for an PlayerError.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayerErrorReason {
    /// There is no previous track in the context.
    NoPrevTrack,
    /// There is no next track in the context.
    NoNextTrack,
    /// The requested track does not exist.
    NoSpecificTrack,
    /// Playback is paused.
    AlreadyPaused,
    /// Playback is not paused.
    NotPaused,
    /// Playback is not on the local device.
    NotPlayingLocally,
    /// No track is currently playing.
    NotPlayingTrack,
    /// No context is currently playing.
    NotPlayingContext,
    /// The current context is endless, so the shuffle command cannot be applied.
    EndlessContext,
    /// The command cannot be performed on the current context.
    ContextDisallow,
    /// The command requested a new track and context to play, but it is the same as the old one
    /// and there is a resume point.
    AlreadyPlaying,
    /// Too frequent track play.
    RateLimited,
    /// The context cannot be remote controlled.
    RemoteControlDisallow,
    /// It is not possible to remote control the device.
    DeviceNotControllable,
    /// It is not possible to remote control the device's volume.
    VolumeControlDisallow,
    /// The user does not have an active device.
    NoActiveDevice,
    /// The action requires premium, which the user doesn't have.
    PremiumRequired,
    /// The action is restricted due to unknown reasons.
    Unknown,
}

impl Display for PlayerErrorReason {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::NoPrevTrack => "There is no previous track",
            Self::NoNextTrack => "There is no next track",
            Self::NoSpecificTrack => "The requested track does not exist",
            Self::AlreadyPaused => "Playback is paused",
            Self::NotPaused => "Playback is not paused",
            Self::NotPlayingLocally => "Playback is not on the local device",
            Self::NotPlayingTrack => "No track is currently playing",
            Self::NotPlayingContext => "No context is currently playing",
            Self::EndlessContext => "The current context is endless",
            Self::ContextDisallow => "The action cannot be performed on the current context",
            Self::AlreadyPlaying => "The same track is already playing",
            Self::RateLimited => "Too frequent track play",
            Self::RemoteControlDisallow => "The context cannot be remote controlled",
            Self::DeviceNotControllable => "It is not possible to control the device",
            Self::VolumeControlDisallow => "It is not possible to control the device's volume",
            Self::NoActiveDevice => "The user does not have an active device",
            Self::PremiumRequired => "The action requires premium",
            Self::Unknown => "The action is restricted for unknown reasons",
        })
    }
}
