use std::fmt::{self, Formatter};
use std::time::Duration;

use serde::de::{self, Deserializer, Unexpected, Visitor};
use serde::{Deserialize, Serialize, Serializer};

use crate::model::TypeAudioFeatures;

/// Information and features of a track.
///
/// See [the Spotify Web API
/// reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#audio-features-object)
/// for more details on each on the items.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct AudioFeatures {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
    /// for the track.
    pub id: String,
    /// The length of the track.
    #[serde(rename = "duration_ms", with = "serde_millis")]
    pub duration: Duration,
    pub acousticness: f64,
    pub danceability: f64,
    pub energy: f64,
    pub instrumentalness: f64,
    pub key: u32,
    pub liveness: f64,
    pub loudness: f64,
    pub mode: Mode,
    pub speechiness: f64,
    pub tempo: f64,
    pub time_signature: u32,
    pub valence: f64,
    /// The item type; `audio_features`.
    #[serde(rename = "type")]
    pub item_type: TypeAudioFeatures,
}

/// The mode of a track (major or minor).
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Mode {
    /// The track is major.
    Major,
    /// The track is minor.
    Minor,
}

struct ModeVisitor;

impl<'de> Visitor<'de> for ModeVisitor {
    type Value = Mode;
    fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("a mode which is 0 (minor) or 1 (major)")
    }
    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(Mode::Minor),
            1 => Ok(Mode::Major),
            _ => Err(E::invalid_value(Unexpected::Unsigned(v), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for Mode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(ModeVisitor)
    }
}

impl Serialize for Mode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Major => serializer.serialize_u64(1),
            Self::Minor => serializer.serialize_u64(0),
        }
    }
}

mod serde_mode_opt {
    use super::{Mode, ModeVisitor};
    use serde::{
        de::{self, Visitor},
        Deserializer, Serialize, Serializer,
    };
    use std::fmt::{self, Formatter};
    use std::u64;

    struct ModeOptVisitor;
    impl<'de> Visitor<'de> for ModeOptVisitor {
        type Value = Option<Mode>;
        fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str("-1 or a mode")
        }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
            match v {
                -1 => Ok(None),
                _ => self.visit_u64(u64::MAX),
            }
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            ModeVisitor.visit_u64(v).map(Some)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Mode>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(ModeOptVisitor)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(v: &Option<Mode>, serializer: S) -> Result<S::Ok, S::Error> {
        match v {
            Some(mode) => mode.serialize(serializer),
            None => serializer.serialize_i64(-1),
        }
    }
}

mod serde_key_opt {
    use serde::{
        de::{self, Unexpected, Visitor},
        Deserializer, Serializer,
    };
    use std::convert::TryInto;
    use std::fmt::{self, Formatter};

    struct KeyOptVisitor;
    impl<'de> Visitor<'de> for KeyOptVisitor {
        type Value = Option<u32>;
        fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str("-1 or a key")
        }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
            match v {
                -1 => Ok(None),
                _ => Err(E::invalid_value(Unexpected::Signed(v), &self)),
            }
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            match v {
                0..=11 => Ok(Some(v.try_into().unwrap())),
                _ => Err(E::invalid_value(Unexpected::Unsigned(v), &self)),
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i64(KeyOptVisitor)
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(v: &Option<u32>, serializer: S) -> Result<S::Ok, S::Error> {
        match v {
            Some(v) => serializer.serialize_u32(*v),
            None => serializer.serialize_i32(-1),
        }
    }
}

/// Audio analysis of a track.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioAnalysis {
    /// The time intervals of bars throughout the track. A bar is a segment of time defined as a
    /// given number of beats. Bar offsets also indicate downbeats, the first beat of a bar.
    pub bars: Vec<TimeInterval>,
    /// The time intervals of beats throughout the track. A beat is the basic time unit of a piece
    /// of music; for example, each tick of a metronome. Beats are typically multiples of tatums.
    pub beats: Vec<TimeInterval>,
    /// A tatum represents the lowest regular pulse train that a listener intuitively infers from
    /// the timing of perceived musical events (segments). For more information about tatums, see
    /// Rhythm (below).
    pub tatums: Vec<TimeInterval>,
    /// Sections are defined by large variations in rhythm or timbre, e.g. chorus, verse, bridge,
    /// guitar solo, etc. Each section contains its own descriptions of tempo, key, mode,
    /// time_signature, and loudness.
    pub sections: Vec<Section>,
    /// Audio segments attempts to subdivide a track into many segments, with each segment
    /// containing a roughly consistent sound throughout its duration.
    pub segments: Vec<Segment>,
}

/// A time interval in a track.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeInterval {
    /// The starting point of the time interval.
    #[serde(with = "crate::util::serde_duration_secs")]
    pub start: Duration,
    /// The duration of the time interval.
    #[serde(with = "crate::util::serde_duration_secs")]
    pub duration: Duration,
    /// The confidence, from 0 to 1, of the reliability of the interval.
    pub confidence: f64,
}

/// A section of a track.
///
/// See [the Spotify docs for a section
/// object](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-analysis/#section-object)
/// for more information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Section {
    /// The interval of the section.
    #[serde(flatten)]
    pub interval: TimeInterval,
    pub loudness: f64,
    pub tempo: f64,
    pub tempo_confidence: f64,
    #[serde(with = "serde_key_opt")]
    pub key: Option<u32>,
    pub key_confidence: f64,
    #[serde(with = "serde_mode_opt")]
    pub mode: Option<Mode>,
    pub mode_confidence: f64,
    pub time_signature: u32,
    pub time_signature_confidence: f64,
}

/// A segment in a track.
///
/// See [the Spotify docs for a segment
/// object](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-analysis/#segment-object)
/// for more information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Segment {
    /// The interval of the segment.
    #[serde(flatten)]
    pub interval: TimeInterval,
    pub loudness_start: f64,
    pub loudness_max: f64,
    pub loudness_max_time: f64,
    pub pitches: Vec<f64>,
    pub timbre: Vec<f64>,
}
