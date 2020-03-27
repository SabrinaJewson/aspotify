use crate::model::*;
use serde::Serialize;

macro_rules! inherit_show_simplified {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        to_struct!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// A list of countries in which the show can be played.
            available_markets: Vec<CountryCode>,
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
            /// The list of languages used in the show.
            languages: Vec<LanguageCode>,
            /// The media type of the show.
            media_type: String,
            /// The name of the show.
            name: String,
            /// The publisher of the show.
            publisher: String,
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
            languages: Vec<LanguageCode>,
            /// The name of the episode.
            name: String,
            /// When the episode was released.
            #[serde(deserialize_with = "de_date_any_precision")]
            release_date: NaiveDate,
            /// How precise the release date is: precise to the year, month or day.
            release_date_precision: DatePrecision,
            /// The user's most recent position in the episode. None if there is no user.
            resume_point: Option<ResumePoint>,
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

/// A position to resume from in an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResumePoint {
    /// Whether the user has fully played the object.
    pub fully_played: bool,
    /// The user's most recent position in the object.
    #[serde(rename = "resume_position_ms", with = "serde_millis")]
    pub resume_position: Duration,
}
