use std::collections::HashMap;

use crate::model::{Followers, Image, TypeArtist};

macro_rules! inherit_artist_simplified {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        to_struct!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// Known external URLs for this artist.
            external_urls: HashMap<String, String>,
            /// The name of the artist.
            name: String,
            /// The object type; `artist`.
            #[serde(rename = "type")]
            item_type: TypeArtist,
        });
    }
}

inherit_artist_simplified!(
    /// A simplified artist object.
    ArtistSimplified {
        /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
        /// for the artist. Only `None` for local tracks on a playlist.
        id: Option<String>,
    }
);
inherit_artist_simplified!(
    /// An artist object.
    Artist {
        /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
        /// for the artist.
        id: String,
        /// Information about the followers of this artist.
        followers: Followers,
        /// A list of the genres this artist is associated with. For example: "Prog Rock",
        /// "Post-Grunge". If not yet classified, the array is empty.
        genres: Vec<String>,
        /// Images of the artist in various sizes, widest first.
        images: Vec<Image>,
        /// The popularity of the artist. The value will be between 0 and 100, with 100 being the most
        /// popular. The artist's popularity is calculated from the popularity of all the artist's
        /// tracks.
        popularity: u32,
    }
);

impl From<Artist> for ArtistSimplified {
    fn from(artist: Artist) -> Self {
        Self {
            external_urls: artist.external_urls,
            id: Some(artist.id),
            name: artist.name,
            item_type: TypeArtist,
        }
    }
}
