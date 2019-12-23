//! The Spotify [Object
//! Model](https://developer.spotify.com/documentation/web-api/reference/object-model/), in
//! deserializable Rust structures.

pub use album::*;
pub use artist::*;
pub use errors::*;
pub use playlist::*;
pub use track::*;
pub use user::*;

use crate::util::*;
use chrono::{DateTime, NaiveDate, Utc};
use isocountry::CountryCode;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use std::collections::HashMap;
use std::fmt::{self, Formatter};
use std::time::Duration;

macro_rules! to_struct {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        $(#[$attr])*
        #[derive(Debug, Clone, serde::Deserialize)]
        pub struct $name {
            $(
                $(#[$f_attr])*
                pub $f_name: $f_ty,
            )*
        }
    }
}

mod album;
mod artist;
mod errors;
mod playlist;
mod track;
mod user;

/// A category of music, for example "Mood", "Top Lists", "Workout", et cetera.
#[derive(Debug, Clone, Deserialize)]
pub struct Category {
    /// The category icon, in various sizes, probably with widest first (although this is not
    /// specified by the Web API documentation).
    pub icons: Vec<Image>,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
    /// for the category.
    pub id: String,
    /// The name of the category.
    pub name: String,
}

/// The context of the current playing track.
#[derive(Debug, Clone, Deserialize)]
pub struct Context {
    /// The type of object; album, artist, playlist, et cetera.
    pub object_type: ObjectType,
    /// External URLs for this context.
    pub external_urls: HashMap<String, String>,
    /// The [Spotify
    /// URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
    /// for the context.
    pub uri: String,
}

/// A type of object in the Spotify model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    Album,
    Artist,
    AudioFeatures,
    Playlist,
    Track,
    User,
}

/// The copyright information for a resource.
#[derive(Debug, Clone, Deserialize)]
pub struct Copyright {
    /// The copyright text.
    pub text: String,
    /// Whether the copyright is for the performance of the piece, not the piece.
    #[serde(rename = "type", deserialize_with = "is_p")]
    pub performance_copyright: bool,
}

fn is_p<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct CopyrightType;

    impl<'de> Visitor<'de> for CopyrightType {
        type Value = bool;
        fn expecting(&self, f: &mut Formatter) -> fmt::Result {
            f.write_str("P or C")
        }
        fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
            match s {
                "P" => Ok(true),
                "C" => Ok(false),
                _ => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
            }
        }
    }

    deserializer.deserialize_str(CopyrightType)
}

/// An action that is currently not able to be performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
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

/// Information about the followers of an item. Currently only contains the number of followers.
#[derive(Debug, Clone, Deserialize)]
pub struct Followers {
    /// The total number of followers.
    pub total: usize,
}

/// An image with a URL and an optional width and height.
#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    /// The source URL of the image.
    pub url: String,
    /// The height of the image in pixels, if known.
    pub height: Option<usize>,
    /// The width of the image in pixels, if known.
    pub width: Option<usize>,
}

/// A page of items.
#[derive(Debug, Clone, Deserialize)]
pub struct Page<T> {
    /// The items in the page.
    pub items: Vec<T>,
    /// The maximum number of items in the page, as set by the request or a default value.
    pub limit: usize,
    /// The offset of the page in the items.
    pub offset: usize,
    /// The total number of items.
    pub total: usize,
}

/// A page of items, using a cursor to find the next page.
#[derive(Debug, Clone, Deserialize)]
pub struct CursorPage<T> {
    /// The items in the page.
    pub items: Vec<T>,
    /// The maximum number of items in the page, as set by the request or a default value.
    pub limit: usize,
    /// The cursor used to find the next set of items.
    pub cursors: Cursor,
    /// The total number of items.
    pub total: usize,
}

/// Object that contains the next CursorPage.
#[derive(Debug, Clone, Deserialize)]
pub struct Cursor {
    pub after: Option<String>,
}

/// Recommended tracks for the user.
#[derive(Debug, Clone, Deserialize)]
pub struct Recommendations {
    /// An array of recommendation seeds.
    pub seeds: Vec<RecommendationSeed>,
    /// An array of simplified track objects.
    pub tracks: Vec<TrackSimplified>,
}

/// How the recommendation was chosen.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationSeed {
    /// The number of tracks available after min_* and max_* filters have been applied.
    pub after_filtering_size: usize,
    /// The number of tracks available after relinking for regional availability.
    pub after_relinking_size: usize,
    /// The id used to select this seed, given by the user.
    pub id: String,
    /// The number of recommended tracks available for this seed.
    pub initial_pool_size: usize,
    /// The entity type of this seed; Artist, Track or Genre.
    #[serde(rename = "type")]
    pub entity_type: SeedType,
}

/// The context from which the recommendation was chosen; artist, track or genre.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeedType {
    Artist,
    Track,
    Genre,
}

/// How precise a date measurement is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatePrecision {
    Year,
    Month,
    Day,
}

/// Restrictions applied to a track due to markets.
#[derive(Debug, Clone, Deserialize)]
pub struct Restrictions {
    pub reason: String,
}
