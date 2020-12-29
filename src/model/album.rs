use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::model::{
    ArtistSimplified, Copyright, DatePrecision, Image, Page, Restrictions, TrackSimplified,
    TypeAlbum,
};
use crate::util;

macro_rules! inherit_album_simplified {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        to_struct!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// The list of artists who made this album.
            artists: Vec<ArtistSimplified>,
            /// The markets in which at least 1 of the album's tracks is available. Only Some if
            /// the market parameter is not supplied in the request. This is an ISO 3166 2-letter
            /// country code.
            available_markets: Option<Vec<String>>,
            /// Known external URLs for this album.
            external_urls: HashMap<String, String>,
            /// The cover art for the album in various sizes, widest first.
            images: Vec<Image>,
            /// The name of the album; if the album has been taken down, this is an empty string.
            name: String,
            /// When [track
            /// relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/)
            /// is applied, the original track isn't available in the given market and Spotify didn't have
            /// any tracks to relink it with, then this is Some.
            restrictions: Option<Restrictions>,
            /// The item type; `album`.
            #[serde(rename = "type")]
            item_type: TypeAlbum,
        });
    }
}

inherit_album_simplified!(
    /// A simplified album object.
    AlbumSimplified {
        /// The type of album: album, single or compilation. This can only be not present for the
        /// album of a local track, which can only ever be obtained from a playlist.
        album_type: Option<AlbumType>,
        /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
        /// for this album. This can only be [`None`] for the album of a local track, which can only
        /// ever be obtained from a playlist.
        id: Option<String>,
        /// When the album was released. This can only be `None` for the album of a local track,
        /// which can only ever be obtained from a playlist.
        #[serde(deserialize_with = "util::de_date_any_precision_option")]
        release_date: Option<NaiveDate>,
        /// How precise the release date is: precise to the year, month or day. This can only be
        /// [`None`] for the album of a local track,which can only ever be obtained from a playlist.
        release_date_precision: Option<DatePrecision>,
    }
);

macro_rules! inherit_album_not_local {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        inherit_album_simplified!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// The type of album: album, single or compilation.
            album_type: AlbumType,
            /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
            /// for this album.
            id: String,
            /// When the album was released.
            #[serde(deserialize_with = "util::de_date_any_precision")]
            release_date: NaiveDate,
            /// How precise the release date is: precise to the year, month or day.
            release_date_precision: DatePrecision,
        });
    }
}

inherit_album_not_local!(
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
inherit_album_not_local!(
    /// A simplified album object from the context of an artist.
    ArtistsAlbum {
        /// Similar to AlbumType, but also includes if the artist features on the album, and didn't
        /// create it as an album, single or compilation.
        album_group: AlbumGroup,
    }
);

impl Album {
    /// Convert to an `AlbumSimplified`.
    #[must_use]
    pub fn simplify(self) -> AlbumSimplified {
        AlbumSimplified {
            album_type: Some(self.album_type),
            artists: self.artists,
            available_markets: self.available_markets,
            external_urls: self.external_urls,
            id: Some(self.id),
            images: self.images,
            name: self.name,
            release_date: Some(self.release_date),
            release_date_precision: Some(self.release_date_precision),
            restrictions: self.restrictions,
            item_type: TypeAlbum,
        }
    }
}
impl From<Album> for AlbumSimplified {
    fn from(album: Album) -> Self {
        album.simplify()
    }
}
impl ArtistsAlbum {
    /// Convert to an `AlbumSimplified`.
    #[must_use]
    pub fn simplify(self) -> AlbumSimplified {
        AlbumSimplified {
            album_type: Some(self.album_type),
            artists: self.artists,
            available_markets: self.available_markets,
            external_urls: self.external_urls,
            id: Some(self.id),
            images: self.images,
            name: self.name,
            release_date: Some(self.release_date),
            release_date_precision: Some(self.release_date_precision),
            restrictions: self.restrictions,
            item_type: TypeAlbum,
        }
    }
}
impl From<ArtistsAlbum> for AlbumSimplified {
    fn from(album: ArtistsAlbum) -> Self {
        album.simplify()
    }
}

/// The type of album.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    /// An album.
    #[serde(alias = "ALBUM")]
    Album,
    /// A single.
    #[serde(alias = "SINGLE")]
    Single,
    /// A compilation album.
    #[serde(alias = "COMPILATION")]
    Compilation,
}

/// Similar to `AlbumType`, but with an extra variant.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlbumGroup {
    /// An album.
    Album,
    /// A single.
    Single,
    /// A compilation album.
    Compilation,
    /// When getting all an artist's albums, if the artist didn't release the album but instead
    /// appeared on it, it is this value.
    AppearsOn,
}

impl AlbumGroup {
    /// Get the album's type as a string.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            AlbumGroup::Album => "album",
            AlbumGroup::Single => "single",
            AlbumGroup::Compilation => "compilation",
            AlbumGroup::AppearsOn => "appears_on",
        }
    }
}

/// Information about an album that has been saved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedAlbum {
    /// When the album was saved.
    pub added_at: DateTime<Utc>,
    /// Information about the album.
    pub album: Album,
}
