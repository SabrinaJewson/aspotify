use crate::model::*;

macro_rules! inherit_artist_simplified {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        to_struct!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// Known external URLs for this artist.
            external_urls: HashMap<String, String>,
            /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
            /// for the artist.
            id: String,
            /// The name of the artist.
            name: String,
        });
    }
}

inherit_artist_simplified!(
    /// A simplified artist object.
    ArtistSimplified {}
);
inherit_artist_simplified!(
    /// An artist object.
    Artist {
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
