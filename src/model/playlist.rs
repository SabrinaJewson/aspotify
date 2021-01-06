use std::collections::HashMap;
use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::{Episode, Followers, Image, Page, Track, Tracks, TypePlaylist, UserSimplified};

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
            /// The user who owns the playlist. This is a [`UserPublic`](crate::UserPublic)
            /// according to the documentation, but in practice it is not.
            owner: UserSimplified,
            /// Whether the playlist is public; None if not relevant.
            public: Option<bool>,
            /// The version identifier of the playlist.
            snapshot_id: String,
            /// The item type; `playlist`.
            #[serde(rename = "type")]
            item_type: TypePlaylist,
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
        /// Information about the tracks and episodes of the playlist.
        tracks: Page<PlaylistItem>,
    }
);

impl Playlist {
    /// Convert to a `PlaylistSimplified`.
    #[must_use]
    pub fn simplify(self) -> PlaylistSimplified {
        PlaylistSimplified {
            collaborative: self.collaborative,
            external_urls: self.external_urls,
            id: self.id,
            images: self.images,
            name: self.name,
            owner: self.owner,
            public: self.public,
            snapshot_id: self.snapshot_id,
            tracks: Tracks {
                total: self.tracks.total,
            },
            item_type: TypePlaylist,
        }
    }
}
impl From<Playlist> for PlaylistSimplified {
    fn from(playlist: Playlist) -> Self {
        playlist.simplify()
    }
}

/// Information about an item inside a playlist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaylistItem {
    /// The date and time that the item was added. Some very old playlists might have [`None`].
    pub added_at: Option<DateTime<Utc>>,
    /// The Spotify user who added the item. Some very old playlists might have [`None`]. This is a
    /// [`UserPublic`](crate::UserPublic) according to the documentation, but in practice it is not.
    pub added_by: Option<UserSimplified>,
    /// Whether the item is a local file or not.
    pub is_local: bool,
    /// The item itself. Spotify API sometimes returns null for this, and I don't know why.
    #[serde(rename = "track")]
    pub item: Option<PlaylistItemType<Track, Episode>>,
}

/// The types of item that can go in a playlist.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlaylistItemType<T, E> {
    /// A track.
    Track(T),
    /// An episode.
    Episode(E),
}

impl<T: Display, E: Display> PlaylistItemType<T, E> {
    /// Formats a Spotify URI using the [`Display`] implementations of the track and episode types.
    pub fn uri(&self) -> String {
        match self {
            Self::Track(track) => format!("spotify:track:{}", track),
            Self::Episode(episode) => format!("spotify:episode:{}", episode),
        }
    }
}

/// A list of featured playlists, and a message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeaturedPlaylists {
    /// A message about the featured playlists.
    pub message: String,
    /// The list of featured playlists.
    pub playlists: Page<PlaylistSimplified>,
}
