use std::collections::HashMap;
use std::time::Duration;

use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
// See line 50
//use chrono::serde::ts_milliseconds;

use crate::model::{Episode, ItemType, Track};
use crate::util;

/// A device object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    /// The device id. It can be [`None`], and I don't know why.
    pub id: Option<String>,
    /// Whether this device is the currently active device.
    pub is_active: bool,
    /// Whether this device is currently in a private session.
    pub is_private_session: bool,
    /// Whether controlling this device is restricted; if set to true, no Web API commands will be
    /// accepted by it.
    pub is_restricted: bool,
    /// The name of the device.
    pub name: String,
    /// The type of the device.
    #[serde(rename = "type")]
    pub device_type: DeviceType,
    /// The current volume in percent. It can be [`None`], and I don't know why.
    pub volume_percent: Option<u32>,
}

/// A type of device.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum DeviceType {
    Computer,
    Tablet,
    Smartphone,
    Speaker,
    TV,
    AVR,
    STB,
    AudioDongle,
    GameConsole,
    CastVideo,
    CastAudio,
    Automobile,
    Unknown,
}

/// Information about the currently playing track.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentlyPlaying {
    /// The context of the currently playing track. Is [`None`] for example if a private session is
    /// enabled.
    pub context: Option<Context>,
    // Spotify gave me negative timestamps for some reason so I had to disable this.
    // /// When data was fetched.
    // #[serde(with = "ts_milliseconds")]
    // pub timestamp: DateTime<Utc>,
    /// Progress into the currently playing track. Is [`None`] for example if a private session is
    /// enabled.
    #[serde(rename = "progress_ms", with = "util::serde_duration_millis_option")]
    pub progress: Option<Duration>,
    /// If something is currently playing.
    pub is_playing: bool,
    /// The currently playing item. Is [`None`] for example if a private session is enabled.
    #[serde(flatten)]
    pub item: Option<PlayingType>,
    /// Which actions are disallowed in the current context.
    pub actions: Actions,
}

/// Information about a user's current playback state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentPlayback {
    /// The currently active device.
    pub device: Device,
    /// The repeat state.
    pub repeat_state: RepeatState,
    /// Whether shuffle is on.
    pub shuffle_state: bool,
    /// The currently playing track.
    #[serde(flatten)]
    pub currently_playing: CurrentlyPlaying,
}

/// Actions that are disallowed in the current context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Actions {
    /// The list of disallowed actions.
    #[serde(with = "util::serde_disallows")]
    pub disallows: Vec<Disallow>,
}

/// An action that is currently not able to be performed.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Disallow {
    InterruptingPlayback,
    Pausing,
    Resuming,
    Seeking,
    SkippingNext,
    SkippingPrev,
    TogglingRepeatContext,
    TogglingShuffle,
    TogglingRepeatTrack,
    TransferringPlayback,
}

/// The type of a currently playing item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    rename_all = "snake_case",
    tag = "currently_playing_type",
    content = "item"
)]
pub enum PlayingType {
    /// A track.
    Track(Track),
    /// An episode of a show.
    Episode(Episode),
    /// An advert.
    Ad(Track),
    /// An unknown track type.
    Unknown(Track),
}

/// The context of the current playing track.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Context {
    /// The type of context; album, artist, playlist, track.
    #[serde(rename = "type")]
    pub context_type: ItemType,
    /// External URLs for this context.
    pub external_urls: HashMap<String, String>,
    /// The [Spotify
    /// ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
    /// for the context.
    #[serde(rename = "uri", deserialize_with = "util::de_any_uri")]
    pub id: String,
}

impl Serialize for Context {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut context = serializer.serialize_struct("Context", 3)?;
        context.serialize_field("type", &self.context_type)?;
        context.serialize_field("external_urls", &self.external_urls)?;
        context.serialize_field("uri", {
            struct UriSerialize<'a> {
                context_type: ItemType,
                id: &'a str,
            }
            impl Serialize for UriSerialize<'_> {
                fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    serializer.serialize_str(&format!(
                        "spotify:{}:{}",
                        self.context_type.as_str(),
                        self.id
                    ))
                }
            }
            &UriSerialize {
                context_type: self.context_type,
                id: &self.id,
            }
        })?;
        context.end()
    }
}

/// Repeating the track, the context or not at all.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepeatState {
    /// Not repeating.
    Off,
    /// Repeating the current track.
    Track,
    /// Repeating the current context (e.g. playlist, album, etc).
    Context,
}

impl RepeatState {
    /// Get the state of repeating as a lowercase string.
    ///
    /// # Examples
    ///
    /// ```
    /// let state = aspotify::RepeatState::Track;
    ///
    /// assert_eq!(state.as_str(), "track");
    /// ```
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Track => "track",
            Self::Context => "context",
        }
    }
}
