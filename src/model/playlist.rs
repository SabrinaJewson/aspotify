use crate::model::*;

macro_rules! inherit_playlist_simplified {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        to_struct!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// Whether the owner allows other people to modify the playlist. Always is false from a search
            /// context.
            collaborative: bool,
            /// Known external URLs for this playlist.
            external_urls: HashMap<String, String>,
            /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids)
            /// for this playlist.
            id: String,
            /// Images for the playlist. It may be empty, or contain up to three images, in descending
            /// order of size. The URLs are temporary and will expire in less than a day.
            images: Vec<Image>,
            /// The name of the playlist.
            name: String,
            /// The user who owns the playlist. This is a UserPublic according to the
            /// documentation, but in practice it is not.
            owner: UserSimplified,
            /// Whether the playlist is public; None if not relevant.
            public: Option<bool>,
            /// The version identifier of the playlist.
            snapshot_id: String,
        });
    }
}

inherit_playlist_simplified!(
    /// A simplified playlist object.
    PlaylistSimplified {
        /// The number of tracks in the playlist.
        tracks: Tracks,
    }
);
inherit_playlist_simplified!(
    /// A playlist object.
    Playlist {
        /// The playlist description, only for modified and verified playlists.
        description: Option<String>,
        /// The followers of the playlist.
        followers: Followers,
        /// Information about the tracks of the playlist.
        tracks: Page<PlaylistTrack>,
    }
);

/// A track inside a playlist.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PlaylistTrack {
    /// The date and time that the track was added. Some very old playlists might have None.
    pub added_at: Option<DateTime<Utc>>,
    /// The Spotify user who added the track. Some very old playlists might have None. This is a
    /// UserPublic according to the documentation, but in practice it is not.
    pub added_by: Option<UserSimplified>,
    /// Whether the track is a local file or not.
    pub is_local: bool,
    /// Information about the track.
    pub track: Track,
}

/// A list of featured playlists, and a message.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FeaturedPlaylists {
    /// A message about the featured playlists.
    pub message: String,
    /// The list of featured playlists.
    pub playlists: Page<PlaylistSimplified>,
}
