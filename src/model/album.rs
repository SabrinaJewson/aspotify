use crate::model::*;
use std::fmt::{self, Display, Formatter};

macro_rules! inherit_album_simplified {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        to_struct!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// The type of album: album, single or compilation.
            album_type: AlbumType,
            /// The list of artists who made this album.
            artists: Vec<ArtistSimplified>,
            /// The markets in which at least 1 of the album's tracks is available. Only Some if
            /// the market parameter is not supplied in the request.
            available_markets: Option<Vec<CountryCode>>,
            /// Known external URLs for this album.
            external_urls: HashMap<String, String>,
            /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
            /// for this album.
            id: String,
            /// The cover art for the album in various sizes, widest first.
            images: Vec<Image>,
            /// The name of the album; if the album has been taken down, this is an empty string.
            name: String,
            /// When the album was released.
            release_date: NaiveDate,
            /// How precise the release date is: precise to the year, month or day.
            release_date_precision: DatePrecision,
            /// When [track
            /// relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/)
            /// is applied, the original track isn't available in the given market and Spotify didn't have
            /// any tracks to relink it with, then this is Some.
            restrictions: Option<Restrictions>,
        });
    }
}

inherit_album_simplified!(
    /// A simplified album object.
    AlbumSimplified {}
);
inherit_album_simplified!(
    /// An album object.
    Album {
        /// The known copyrights of this album.
        copyrights: Vec<Copyright>,
        /// Known external IDs for this album.
        external_ids: HashMap<String, String>,
        /// A list of the genres used to classify the album. For example: "Prog Rock", "Post-Grunge".
        /// If not yet classified, the array is empty.
        genres: Vec<String>,
        /// The label of the album.
        label: String,
        /// The popularity of the album. The value will be between 0 and 100, with 100 being the most
        /// popular. The popularity is calculated from the popularity of the album's individual tracks.
        popularity: u32,
        /// A page of tracks in the album.
        tracks: Page<TrackSimplified>,
    }
);
inherit_album_simplified!(
    /// A simplified album object from the context of an artist.
    ArtistsAlbum {
        /// Similar to AlbumType, but also includes if the artist features on the album, and didn't
        /// create it as an album, single or compilation.
        album_group: AlbumGroup,
    }
);

/// The type of album.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    Compilation,
}

/// When getting all an artist's albums, if the artist didn't release the album but instead
/// appeared on it, this is set to AppearsOn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlbumGroup {
    Album,
    Single,
    Compilation,
    AppearsOn,
}

impl Display for AlbumGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            AlbumGroup::Album => "album",
            AlbumGroup::Single => "single",
            AlbumGroup::Compilation => "compilation",
            AlbumGroup::AppearsOn => "appears_on",
        })
    }
}

/// Information about an album that has been saved.
#[derive(Debug, Clone, Deserialize)]
pub struct SavedAlbum {
    /// When the album was saved.
    pub added_at: DateTime<Utc>,
    /// Information about the album.
    pub album: Album,
}
