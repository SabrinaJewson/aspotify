use crate::model::*;

macro_rules! inherit_track_simplified {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        to_struct!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// The artists who performed the track.
            artists: Vec<ArtistSimplified>,
            /// The markets in which this track is available. Only Some if the market parameter is
            /// not supplied in the request.
            available_markets: Option<Vec<CountryCode>>,
            /// The disc number (1 unless the album contains more than one disc).
            disc_number: usize,
            /// The track length.
            #[serde(rename = "duration_ms", with = "serde_millis")]
            duration: Duration,
            /// Whether the track has explicit lyrics, false if unknown.
            explicit: bool,
            /// Known external URLs for this track.
            external_urls: HashMap<String, String>,
            /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
            /// for this track.
            id: String,
            /// When [track
            /// relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/)
            /// is applied, if the track is playable in the given market.
            is_playable: Option<bool>,
            /// When [track
            /// relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/)
            /// is applied and the requested track has been replaced by a different one.
            linked_from: Option<TrackLink>,
            /// When [track
            /// relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/)
            /// is applied, the original track isn't available in the given market and Spotify didn't have
            /// any tracks to relink it with, then this is Some.
            restrictions: Option<Restrictions>,
            /// The name of the track.
            name: String,
            /// Link to a 30 second MP3 preview of the track, doesn't have to be there.
            preview_url: Option<String>,
            /// The 1-indexed number of the track in its album; if the track has several discs,
            /// then it the number on the specified disc.
            track_number: usize,
            /// Whether the track is from a local file.
            is_local: bool,
        });
    }
}

inherit_track_simplified!(
    /// A simplified track object.
    TrackSimplified {}
);
inherit_track_simplified!(
    /// A track object.
    Track {
        /// The album on which this track appears.
        album: AlbumSimplified,
        /// Known external IDs for this track.
        external_ids: HashMap<String, String>,
        /// The popularity of the track. The value will be between 0 and 100, with 100 being the most
        /// popular. The popularity is calculated from the total number of plays and how recent they
        /// are.
        popularity: u32,
    }
);

/// A link to a track.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TrackLink {
    /// Known external URLs for this track.
    pub external_urls: HashMap<String, String>,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
    /// for this track.
    pub id: String,
}

/// When and how a track was played.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PlayHistory {
    /// The track the user listened to.
    pub track: TrackSimplified,
    /// When the track was played.
    pub played_at: DateTime<Utc>,
    /// The context from which the track was played.
    pub context: Option<Context>,
}

/// Information about a track that has been saved.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SavedTrack {
    /// When the track was saved.
    pub added_at: DateTime<Utc>,
    /// Information about the track.
    pub track: Track,
}

/// The number of tracks an object contains.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Tracks {
    pub total: usize,
}
