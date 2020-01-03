use crate::model::*;
use reqwest::StatusCode;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::{error, io};

/// An error caused by a Spotify endpoint.
pub trait SpotifyError: error::Error {}

/// An error caused by one of the Web API endpoints relating to authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationError {
    /// A high level description of the error.
    pub error: String,
    /// A mroe detailed description of the error.
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

/// An HTTP error or an error from the endpoint.
#[derive(Debug)]
pub enum EndpointError<E: SpotifyError> {
    HttpError(reqwest::Error),
    ParseError(serde_json::error::Error),
    SpotifyError(E),
    IoError(io::Error),
}

impl<E: SpotifyError> Display for EndpointError<E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::HttpError(e) => write!(f, "{}", e),
            Self::ParseError(e) => write!(f, "{}", e),
            Self::SpotifyError(e) => write!(f, "{}", e),
            Self::IoError(e) => write!(f, "{}", e),
        }
    }
}

impl<E: SpotifyError> error::Error for EndpointError<E> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::HttpError(e) => Some(e),
            Self::ParseError(e) => Some(e),
            Self::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl<E: SpotifyError> From<reqwest::Error> for EndpointError<E> {
    fn from(error: reqwest::Error) -> Self {
        Self::HttpError(error)
    }
}

impl<E: SpotifyError> From<serde_json::error::Error> for EndpointError<E> {
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
