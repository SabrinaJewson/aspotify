use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
// See line 38+120
//use isolanguage_1::LanguageCode;
use chrono::{DateTime, NaiveDate, Utc};

use crate::model::{Copyright, DatePrecision, Image, Page, TypeEpisode, TypeShow};
use crate::util;

macro_rules! inherit_show_simplified {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        to_struct!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// A list of countries in which the show can be played. These are ISO 3166 2-letter
            /// country codes.
            available_markets: Vec<String>,
            /// The copyright statements of the show.
            copyrights: Vec<Copyright>,
            /// A description of the show.
            description: String,
            /// Whether the show is explicit.
            explicit: bool,
            /// Known externals URLs for this show.
            external_urls: HashMap<String, String>,
            /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
            /// for this show.
            id: String,
            /// The cover art for the show in various sizes, widest first.
            images: Vec<Image>,
            /// Whether the episode is hosted outside of Spotify's CDN. Can be None.
            is_externally_hosted: Option<bool>,
            /// The list of languages used in the show. These are ISO 639 codes.
            // TODO: it can be en-US/en-GB
            languages: Vec<String>,
            /// The media type of the show.
            media_type: String,
            /// The name of the show.
            name: String,
            /// The publisher of the show.
            publisher: String,
            /// The item type; `show`.
            #[serde(rename = "type")]
            item_type: TypeShow,
        });
    }
}

inherit_show_simplified!(
    /// A simplified show object.
    ShowSimplified {}
);

inherit_show_simplified!(
    /// A show object.
    Show {
        /// A list of the show's episodes.
        episodes: Page<EpisodeSimplified>,
    }
);

impl From<Show> for ShowSimplified {
    fn from(show: Show) -> Self {
        Self {
            available_markets: show.available_markets,
            copyrights: show.copyrights,
            description: show.description,
            explicit: show.explicit,
            external_urls: show.external_urls,
            id: show.id,
            images: show.images,
            is_externally_hosted: show.is_externally_hosted,
            languages: show.languages,
            media_type: show.media_type,
            name: show.name,
            publisher: show.publisher,
            item_type: TypeShow,
        }
    }
}

/// Information about a show that has been saved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedShow {
    /// When the show was saved.
    pub added_at: DateTime<Utc>,
    /// Information about the show.
    pub show: ShowSimplified,
}

macro_rules! inherit_episode_simplified {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        to_struct!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// The URL of a 30 second MP3 preview of the episode, or None.
            audio_preview_url: Option<String>,
            /// A description of the episode.
            description: String,
            /// The length of the episode.
            #[serde(rename = "duration_ms", with = "serde_millis")]
            duration: Duration,
            /// Whether the episode is explicit.
            explicit: bool,
            /// Externals URLs for this episode.
            external_urls: HashMap<String, String>,
            /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
            /// for this episode.
            id: String,
            /// The cover art for this episode in sizes, widest first.
            images: Vec<Image>,
            /// Whether the episode is hosted outside of Spotify's CDN.
            is_externally_hosted: bool,
            /// Whether the episode is playable in the given market.
            is_playable: bool,
            /// The list of languages used in this episode.
            // TODO: it can be en-US/en-GB
            languages: Vec<String>,
            /// The name of the episode.
            name: String,
            /// When the episode was released.
            #[serde(deserialize_with = "util::de_date_any_precision")]
            release_date: NaiveDate,
            /// How precise the release date is: precise to the year, month or day.
            release_date_precision: DatePrecision,
            /// The user's most recent position in the episode. None if there is no user.
            resume_point: Option<ResumePoint>,
            /// The item type; `episode`.
            #[serde(rename = "type")]
            item_type: TypeEpisode,
        });
    }
}

inherit_episode_simplified!(
    /// A simplified episode object.
    EpisodeSimplified {}
);

inherit_episode_simplified!(
    /// An episode object.
    Episode {
        /// The show on which the episode belongs.
        show: ShowSimplified,
    }
);

impl From<Episode> for EpisodeSimplified {
    fn from(episode: Episode) -> Self {
        Self {
            audio_preview_url: episode.audio_preview_url,
            description: episode.description,
            duration: episode.duration,
            explicit: episode.explicit,
            external_urls: episode.external_urls,
            id: episode.id,
            images: episode.images,
            is_externally_hosted: episode.is_externally_hosted,
            is_playable: episode.is_playable,
            languages: episode.languages,
            name: episode.name,
            release_date: episode.release_date,
            release_date_precision: episode.release_date_precision,
            resume_point: episode.resume_point,
            item_type: TypeEpisode,
        }
    }
}

/// A position to resume from in an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResumePoint {
    /// Whether the user has fully played the object.
    pub fully_played: bool,
    /// The user's most recent position in the object.
    #[serde(rename = "resume_position_ms", with = "serde_millis")]
    pub resume_position: Duration,
}
