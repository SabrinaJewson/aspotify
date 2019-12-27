use crate::model::*;
// See line 50
//use chrono::serde::ts_milliseconds;

/// A device object.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Device {
    /// The device id. It can be None, and I don't know why.
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
    /// The current volume in percent. It can be None, and I don't know why.
    pub volume_percent: Option<u32>,
}

/// A type of device.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CurrentlyPlaying {
    /// The context of the currently playing track. Is None for example if a private session is
    /// enabled.
    pub context: Option<Context>,
    // Spotify gave me negative timestamps for some reason so I had to disable this.
    // /// When data was fetched.
    // #[serde(with = "ts_milliseconds")]
    // pub timestamp: DateTime<Utc>,
    /// Progress into the currently playing track. Is None for example if a private session is
    /// enabled.
    #[serde(
        rename = "progress_ms",
        deserialize_with = "duration_from_millis_option"
    )]
    pub progress: Option<Duration>,
    /// If something is currently playing.
    pub is_playing: bool,
    /// The currently playing track. Is None for example if a private session is enabled.
    pub item: Option<Track>,
    /// The object type of the currently playing item.
    pub currently_playing_type: TrackType,
    /// Which actions are disallowed in the current context.
    pub actions: Actions,
}

/// Information about a user's current playback state.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Actions {
    #[serde(deserialize_with = "deserialize_disallows")]
    pub disallows: Vec<Disallow>,
}

/// An action that is currently not able to be performed.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
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

/// The type of track.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackType {
    Track,
    Episode,
    Ad,
    Unknown,
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
    #[serde(rename = "uri", deserialize_with = "uri_to_id")]
    pub id: String,
}

/// Repeating the track, the context or not at all.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Deserialize)]
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
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Track => "track",
            Self::Context => "context",
        }
    }
}
