//! The Spotify [Object
//! Model](https://developer.spotify.com/documentation/web-api/reference/object-model/), in
//! deserializable Rust structures.

pub use album::*;
pub use analysis::*;
pub use artist::*;
pub use device::*;
pub use errors::*;
pub use playlist::*;
pub use show::*;
pub use track::*;
pub use user::*;

use crate::util::*;
use chrono::{DateTime, NaiveDate, Utc};
use isocountry::CountryCode;
use isolanguage_1::LanguageCode;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::collections::HashMap;
use std::fmt::{self, Formatter};
use std::time::Duration;

macro_rules! to_struct {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Deserialize)]
        pub struct $name {
            $(
                $(#[$f_attr])*
                pub $f_name: $f_ty,
            )*
        }
    }
}

mod album;
mod analysis;
mod artist;
mod device;
mod errors;
mod playlist;
mod show;
mod track;
mod user;

/// A category of music, for example "Mood", "Top Lists", "Workout", et cetera.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// The copyright information for a resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Copyright {
    /// The copyright text.
    pub text: String,
    /// Whether the copyright is for the performance of the piece, not the piece.
    #[serde(rename = "type", with = "serde_is_p")]
    pub performance_copyright: bool,
}

mod serde_is_p {
    use serde::{
        de::{self, Visitor},
        Deserializer, Serializer,
    };
    use std::fmt::{self, Formatter};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
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

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(v: &bool, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(if *v { "P" } else { "C" })
    }
}

/// Information about the followers of an item. Currently only contains the number of followers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Followers {
    /// The total number of followers.
    pub total: usize,
}

/// An image with a URL and an optional width and height.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    /// The source URL of the image.
    pub url: String,
    /// The height of the image in pixels, if known.
    pub height: Option<usize>,
    /// The width of the image in pixels, if known.
    pub width: Option<usize>,
}

/// A page of items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cursor {
    /// The cursor page after this one.
    pub after: Option<String>,
}

/// A page of items, using a cursor to move backwards and forwards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoWayCursorPage<T> {
    /// The items in the page.
    pub items: Vec<T>,
    /// The maximum number of items in the page, as set by the request or a default value.
    pub limit: usize,
    /// The cursor used to find the next set of items.
    pub cursors: TwoWayCursor,
}

/// Object that contains the next and previous CursorPage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoWayCursor {
    /// The cursor page after this one.
    pub after: Option<String>,
    /// The cursor page before this one.
    pub before: Option<String>,
}

/// Recommended tracks for the user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recommendations {
    /// An array of recommendation seeds.
    pub seeds: Vec<RecommendationSeed>,
    /// An array of simplified track objects.
    pub tracks: Vec<TrackSimplified>,
}

/// How the recommendation was chosen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum SeedType {
    Artist,
    Track,
    Genre,
}

/// How precise a date measurement is.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatePrecision {
    /// The measurement is precise to the nearest year.
    Year,
    /// The measurement is precise to the nearest month.
    Month,
    /// The measurement is precise to the nearest day.
    Day,
}

/// Restrictions applied to a track due to markets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Restrictions {
    /// Why the restriction was applied.
    pub reason: String,
}

/// A type of item in the Spotify model.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum ItemType {
    Album,
    Artist,
    Playlist,
    Track,
    Show,
    Episode,
}

impl ItemType {
    /// The type of item as a string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Album => "album",
            Self::Artist => "artist",
            Self::Playlist => "playlist",
            Self::Track => "track",
            Self::Show => "show",
            Self::Episode => "episode",
        }
    }
}

/// The results of a search.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResults {
    /// The resulting artists of the search.
    pub artists: Option<Page<Artist>>,
    /// The resulting albums of the search.
    pub albums: Option<Page<AlbumSimplified>>,
    /// The resulting tracks of the search.
    pub tracks: Option<Page<Track>>,
    /// The resulting playlists of the search.
    pub playlists: Option<Page<PlaylistSimplified>>,
    /// The resulting shows of the search.
    pub shows: Option<Page<ShowSimplified>>,
    /// The resulting episodes of the search.
    pub episodes: Option<Page<EpisodeSimplified>>,
}
