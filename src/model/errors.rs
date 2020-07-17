use std::fmt::{self, Display, Formatter};
use std::{error, io};

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::util;

/// An error caused by one of the Web API endpoints relating to authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthError {
    /// A high level description of the error.
    pub error: String,
    /// A more detailed description of the error.
    pub error_description: String,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error, self.error_description)
    }
}

impl error::Error for AuthError {}

/// A regular error object returned by endpoints of the API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "EndpointErrorWrapper", into = "EndpointErrorWrapper")]
pub struct EndpointError {
    /// The HTTP status code of the error.
    pub status: StatusCode,
    /// A short description of the error's cause.
    pub message: String,
    /// The reason for the error. Only present for player endpoints.
    pub reason: Option<PlayerErrorReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct EndpointErrorWrapper {
    error: EndpointErrorInternal,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct EndpointErrorInternal {
    #[serde(with = "util::serde_status_code")]
    status: StatusCode,
    message: String,
    #[serde(default)]
    reason: Option<PlayerErrorReason>,
}
impl From<EndpointErrorWrapper> for EndpointError {
    fn from(error: EndpointErrorWrapper) -> Self {
        Self {
            status: error.error.status,
            message: error.error.message,
            reason: error.error.reason,
        }
    }
}
impl From<EndpointError> for EndpointErrorWrapper {
    fn from(error: EndpointError) -> Self {
        Self {
            error: EndpointErrorInternal {
                status: error.status,
                message: error.message,
                reason: error.reason,
            },
        }
    }
}

impl Display for EndpointError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(reason) = self.reason {
            write!(f, "{}: {}", self.message, reason)
        } else {
            write!(f, "Error {}: {}", self.status, self.message)
        }
    }
}

impl error::Error for EndpointError {}

/// An error from a Spotify endpoint.
#[derive(Debug)]
pub enum Error {
    /// An error caused when sending the HTTP request.
    Http(reqwest::Error),
    /// An error caused parsing the response.
    Parse(serde_json::error::Error),
    /// An error caused in authentication.
    Auth(AuthError),
    /// An error caused by a Spotify endpoint.
    Endpoint(EndpointError),
    /// Any other IO error.
    Io(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Http(e) => write!(f, "{}", e),
            Self::Parse(e) => write!(f, "{}", e),
            Self::Auth(e) => write!(f, "{}", e),
            Self::Endpoint(e) => write!(f, "{}", e),
            Self::Io(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Self::Http(e) => e,
            Self::Parse(e) => e,
            Self::Auth(e) => e,
            Self::Endpoint(e) => e,
            Self::Io(e) => e,
        })
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}
impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Self::Parse(error)
    }
}
impl From<AuthError> for Error {
    fn from(error: AuthError) -> Self {
        Self::Auth(error)
    }
}
impl From<EndpointError> for Error {
    fn from(error: EndpointError) -> Self {
        Self::Endpoint(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// A reason for an error caused by the Spotify player.
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
