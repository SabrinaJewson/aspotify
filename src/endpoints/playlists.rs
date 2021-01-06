use std::fmt::Display;
#[cfg(feature = "base64")]
use std::{fs, path::Path};

use reqwest::header;

use crate::{
    Client, Error, Image, Market, Page, Playlist, PlaylistItem, PlaylistItemType,
    PlaylistSimplified, Response,
};

/// Endpoint functions relating to playlists.
///
/// The parameter `snapshot_id` is the snapshot of the playlist to perform the operation on to
/// prevent concurrent accesses causing problems.
///
/// Take this example; person A gets playlist X. Person B removes track N from playlist X. Person A
/// tries to do something to playlist X, assuming track N still exists, but it causes unexpected
/// behaviour because track N doesn't actually exist any more. However, with snapshot ids, person A
/// will have used the snapshot ID they received from the initial request in their request. Spotify
/// knows that person A is attempting to operate on an older playlist, and adjusts accordingly,
/// causing no unexpected behaviour.
///
/// One feature of Spotify is that you cannot delete playlists; you can only unfollow them, hence
/// there is no `delete_playlist` function.
#[derive(Debug, Clone, Copy)]
pub struct Playlists<'a>(pub &'a Client);

impl Playlists<'_> {
    /// Add tracks to a playlist.
    ///
    /// Requires `playist-modify-public` if the playlist is public, and `playlist-modify-private` if it
    /// is private. `position` is the zero-indexed position to insert the tracks; if None it appends to
    /// the playlist. A maximum of 100 tracks can be specified.
    ///
    /// This function returns the `snapshot_id` of the created playlist, which you should hold on to to
    /// stop concurrent accesses to the playlist interfering with each other.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/add-tracks-to-playlist/).
    pub async fn add_to_playlist<T: Display, E: Display>(
        self,
        id: &str,
        tracks: impl IntoIterator<Item = PlaylistItemType<T, E>>,
        position: Option<usize>,
    ) -> Result<String, Error> {
        self.0
            .send_snapshot_id(
                self.0
                    .client
                    .post(endpoint!("/v1/playlists/{}/tracks", id))
                    .json(&serde_json::json!({
                        "uris": tracks.into_iter().map(|track| track.uri()).collect::<Vec<_>>(),
                        "position": position,
                    })),
            )
            .await
    }

    /// Change a playlist's details.
    ///
    /// Requires `playist-modify-public` if the playlist is public, and `playlist-modify-private` if it
    /// is private. Each parameter, when Some, changes an attribute of the playlist. A playlist cannot
    /// be both public and collaborative.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/change-playlist-details/).
    pub async fn change_playlist(
        self,
        id: &str,
        name: Option<&str>,
        public: Option<bool>,
        collaborative: Option<bool>,
        description: Option<&str>,
    ) -> Result<(), Error> {
        self.0
            .send_empty(self.0.client.put(endpoint!("/v1/playlists/{}", id)).json(
                &serde_json::json!({
                    "name": name,
                    "public": public,
                    "collaborative": collaborative,
                    "description": description,
                }),
            ))
            .await
    }

    /// Create a playlist.
    ///
    /// Requires `playlist-modify-public` if creating a public playlist, requires
    /// `playlist-modify-private` if creating a private one.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/create-playlist/).
    pub async fn create_playlist(
        self,
        name: &str,
        public: bool,
        collaborative: bool,
        description: &str,
    ) -> Result<Response<Playlist>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .post(endpoint!("/v1/me/playlists"))
                    .json(&serde_json::json!({
                        "name": name,
                        "public": public,
                        "collaborative": collaborative,
                        "description": description,
                    })),
            )
            .await
    }

    /// Get current user's playlists.
    ///
    /// Gets a list of playlists owned or followed by the current Spotify user. Requires
    /// `playlist-read-private` in order to get private playlists, and `playlist-read-collaborative` to
    /// get collaborative playlists. Limit must be in the range [1..50], and offset must be in the
    /// range [0..100,000].
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/get-a-list-of-current-users-playlists/).
    pub async fn current_users_playlists(
        self,
        limit: usize,
        offset: usize,
    ) -> Result<Response<Page<PlaylistSimplified>>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/me/playlists"))
                    .query(&(("limit", limit.to_string()), ("offset", offset.to_string()))),
            )
            .await
    }

    /// Get a user's playlists.
    ///
    /// Gets a list of playlists owned or followed by a Spotify user.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/get-list-users-playlists/).
    pub async fn get_users_playlists(
        self,
        id: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Response<Page<PlaylistSimplified>>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/users/{}/playlists", id))
                    .query(&(("limit", limit.to_string()), ("offset", offset.to_string()))),
            )
            .await
    }

    /// Get information about a playlist.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/get-playlist/).
    pub async fn get_playlist(
        self,
        id: &str,
        market: Option<Market>,
    ) -> Result<Response<Playlist>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/playlists/{}", id))
                    .query(&(
                        market.map(Market::query),
                        ("additional_types", "track,episode"),
                    )),
            )
            .await
    }

    /// Get a playlist's cover images.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/get-playlist-cover/).
    pub async fn get_playlists_images(self, id: &str) -> Result<Response<Vec<Image>>, Error> {
        self.0
            .send_json(self.0.client.get(endpoint!("/v1/playlists/{}/images", id)))
            .await
    }

    /// Get a playlist's items.
    ///
    /// Limit must be in the range [1..100].
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/get-playlist-tracks/).
    pub async fn get_playlists_items(
        self,
        id: &str,
        limit: usize,
        offset: usize,
        market: Option<Market>,
    ) -> Result<Response<Page<PlaylistItem>>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/playlists/{}/tracks", id))
                    .query(&(
                        ("limit", limit.to_string()),
                        ("offset", offset.to_string()),
                        market.map(Market::query),
                        ("additional_types", "track,episode"),
                    )),
            )
            .await
    }

    /// Remove tracks from a playlist.
    ///
    /// Requires `playlist-modify-public` if the playlist is public, requires `playlist-modify-private`
    /// if it is private.
    ///
    /// Items is a list of items that will be removed, and optionally multiple positions where the
    /// items will be removed, otherwise it will remove them from the entire playlist. It is not
    /// possible to remove items only by position. There is a maximum of 100 items you can remove at
    /// once.
    ///
    /// This function returns the `snapshot_id` of the created playlist, which you should hold on to to
    /// stop concurrent accesses to the playlist interfering with each other.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/remove-tracks-playlist/).
    pub async fn remove_from_playlist<T: Display, E: Display>(
        self,
        id: &str,
        items: impl IntoIterator<Item = (PlaylistItemType<T, E>, Option<&[usize]>)>,
        snapshot_id: &str,
    ) -> Result<String, Error> {
        let mut items = items.into_iter().peekable();
        if items.peek().is_none() {
            return Ok(snapshot_id.to_owned());
        }

        self.0
            .send_snapshot_id(
                self.0
                    .client
                    .delete(endpoint!("/v1/playlists/{}/tracks", id))
                    .json(&serde_json::json!({
                        "tracks": items.map(|(item, positions)| if let Some(positions) = positions {
                            serde_json::json!({
                                "uri": item.uri(),
                                "positions": positions,
                            })
                        } else {
                            serde_json::json!({
                                "uri": item.uri(),
                            })
                        }).collect::<Vec<_>>(),
                        "snapshot_id": snapshot_id,
                    })),
            )
            .await
    }

    /// Reorder items in a playlist.
    ///
    /// Requires `playlist-modify-public` if the playlist is public, requires `playlist-modify-private`
    /// if it is private.
    ///
    /// `range_start` and `range_length` specify a slice of the playlist to move around. This slice
    /// will always remain in the same order. It will be moved before the position in `insert_before`
    /// (e.g. it will be moved to start if `insert_before == 0` and it will be moved to the end if
    /// `insert_before == the playlist's length`).
    ///
    /// ![A helpful diagram from Spotify explaining
    /// this](https://developer.spotify.com/assets/visualization-reordering-tracks.png)
    ///
    /// This function does nothing if `range_length == 0` or if `range_start + range_length ==
    /// insert_before`.
    ///
    /// This function returns the `snapshot_id` of the created playlist, which you should hold on to to
    /// stop concurrent accesses to the playlist interfering with each other.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/reorder-playlists-tracks/).
    pub async fn reorder_playlist(
        self,
        id: &str,
        range_start: usize,
        range_length: usize,
        insert_before: usize,
        snapshot_id: &str,
    ) -> Result<String, Error> {
        if range_length == 0 || range_start + range_length == insert_before {
            return Ok(snapshot_id.to_owned());
        }

        self.0
            .send_snapshot_id(
                self.0
                    .client
                    .put(endpoint!("/v1/playlists/{}/tracks", id))
                    .json(&serde_json::json!({
                        "range_start": range_start,
                        "range_length": range_length,
                        "insert_before": insert_before,
                        "snapshot_id": snapshot_id,
                    })),
            )
            .await
    }

    /// Replace a playlist's items.
    ///
    /// Requires `playlist-modify-public` if the playlist is public, requires
    /// `playlist-modify-private` if it is private.
    ///
    /// This function removes all the items from the given playlist, and replaces them with the
    /// given items. The maximum number of tracks is 100, if you need more you can use
    /// [`add_to_playlist`](Self::add_to_playlist).
    ///
    /// This function returns the `snapshot_id` of the created playlist, which you should hold on to to
    /// stop concurrent accesses to the playlist interfering with each other.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/replace-playlists-tracks/).
    pub async fn replace_playlists_items<T: Display, E: Display>(
        self,
        id: &str,
        items: impl IntoIterator<Item = PlaylistItemType<T, E>>,
    ) -> Result<String, Error> {
        self.0
            .send_snapshot_id(
                self.0
                    .client
                    .put(endpoint!("/v1/playlists/{}/tracks", id))
                    .json(&serde_json::json!({
                        "uris": items.into_iter().map(|id| id.uri()).collect::<Vec<_>>(),
                    })),
            )
            .await
    }

    /// Upload a custom playlist cover image.
    ///
    /// Requires `playlist-modify-public` if the playlist is public, requires
    /// `playlist-modify-private` if it is private, and also requires `ugc-image-upload`.
    ///
    /// `image` must be a base64-encoded JPEG image under 256KB. If you want to pass in JPEG data,
    /// see [`upload_playlist_cover_jpeg`](Self::upload_playlist_cover_jpeg); if you want to pass in
    /// a filename, see [`upload_playlist_cover_file`](Self::upload_playlist_cover_file).
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/upload-custom-playlist-cover/).
    pub async fn upload_playlist_cover(self, id: &str, image: String) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .put(endpoint!("/v1/playlists/{}/images", id))
                    .header(header::CONTENT_TYPE, "image/jpeg")
                    .body(image),
            )
            .await
    }

    /// Upload a custom playlist cover image.
    ///
    /// Requires `playlist-modify-public` if the playlist is public, requires
    /// `playlist-modify-private` if it is private, and also requires `ugc-image-upload`.
    ///
    /// `image` must be JPEG data. If you want to pass in a filename, see
    /// [`upload_playlist_cover_file`](Self::upload_playlist_cover_file).
    ///
    /// This function is only available when the `base64` feature of this library is enabled.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/upload-custom-playlist-cover/).
    #[cfg(feature = "base64")]
    pub async fn upload_playlist_cover_jpeg<T: ?Sized + AsRef<[u8]>>(
        self,
        id: &str,
        image: &T,
    ) -> Result<(), Error> {
        self.upload_playlist_cover(id, base64::encode(image)).await
    }

    /// Upload a custom playlist cover image.
    ///
    /// Requires `playlist-modify-public` if the playlist is public, requires
    /// `playlist-modify-private` if it is private, and also requires `ugc-image-upload`.
    ///
    /// `image` must be a JPEG filename.
    ///
    /// This function is only available when the `base64` feature of this library is enabled.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/upload-custom-playlist-cover/).
    #[cfg(feature = "base64")]
    pub async fn upload_playlist_cover_file<P: AsRef<Path>>(
        self,
        id: &str,
        image: P,
    ) -> Result<(), Error> {
        self.upload_playlist_cover_jpeg(id, &fs::read(image)?).await
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "base64")]
    use std::time::Duration;

    #[cfg(feature = "base64")]
    use tokio::time;

    use crate::endpoints::client;
    use crate::{Client, Followers, PlaylistItemType};

    #[tokio::test]
    async fn test() {
        let client = client();
        let playlists = client.playlists();

        let mut playlist = playlists
            .create_playlist("Testing Playlist", true, false, "Test Description")
            .await
            .unwrap()
            .data;
        assert_eq!(playlist.name, "Testing Playlist");
        assert_eq!(playlist.public, Some(true));
        assert_eq!(playlist.collaborative, false);
        assert_eq!(playlist.description.as_ref().unwrap(), "Test Description");
        assert_eq!(playlist.followers, Followers { total: 0 });
        assert!(playlist.images.is_empty());
        assert_eq!(playlist.tracks.total, 0);

        let got_playlist = playlists
            .get_playlist(&playlist.id, None)
            .await
            .unwrap()
            .data;
        playlist.snapshot_id = got_playlist.snapshot_id.clone();
        assert_eq!(playlist, got_playlist);

        let users_playlists = playlists.current_users_playlists(50, 0).await.unwrap().data;
        if users_playlists.total <= 50 {
            assert!(users_playlists.items.iter().any(|p| p.id == playlist.id));
        }

        playlists
            .change_playlist(
                &playlist.id,
                Some("New Name"),
                Some(false),
                Some(true),
                Some("New Description"),
            )
            .await
            .unwrap();
        let playlist = playlists
            .get_playlist(&playlist.id, None)
            .await
            .unwrap()
            .data;
        assert_eq!(playlist.name, "New Name");
        assert_eq!(playlist.public, Some(false));
        assert_eq!(playlist.collaborative, true);
        assert_eq!(playlist.description.unwrap(), "New Description");
        assert_eq!(playlist.followers, Followers { total: 0 });
        assert!(playlist.images.is_empty());
        assert_eq!(playlist.tracks.total, 0);

        // Add "Ten Tonne Skeleton" and "The Middle"
        let snapshot = playlists
            .add_to_playlist(
                &playlist.id,
                [
                    PlaylistItemType::<_, u8>::Track("0vjYxBDAcflD0358arIVZG"),
                    PlaylistItemType::Track("6GG73Jik4jUlQCkKg9JuGO"),
                ]
                .iter()
                .cloned(),
                None,
            )
            .await
            .unwrap();
        assert_ne!(playlist.snapshot_id, snapshot);
        let playlist = playlists
            .get_playlist(&playlist.id, None)
            .await
            .unwrap()
            .data;
        assert_eq!(playlist.snapshot_id, snapshot);
        assert_eq!(playlist.tracks.total, 2);

        let tracks = playlists
            .get_playlists_items(&playlist.id, 1, 1, None)
            .await
            .unwrap()
            .data;
        assert_eq!(tracks.limit, 1);
        assert_eq!(tracks.offset, 1);
        assert_eq!(tracks.total, 2);
        assert_eq!(tracks.items.len(), 1);
        let track = match tracks.items.into_iter().next().unwrap().item {
            Some(PlaylistItemType::Track(track)) => track,
            _ => panic!(),
        };
        assert_eq!(track.is_local, false);
        assert_eq!(track.id.unwrap(), "6GG73Jik4jUlQCkKg9JuGO");

        // "Blue", "Tredje rikets knarkande granskas", "Mr. Brightside"
        let items: &[PlaylistItemType<_, _>] = &[
            PlaylistItemType::Track("22wRQVOHzHAppfKsDs38nj"),
            PlaylistItemType::Episode("512ojhOuo1ktJprKbVcKyQ"),
            PlaylistItemType::Track("7d8GetOsjbxYnlo6Y9e5Kw"),
        ];

        async fn assert_playlist_order(
            client: &Client,
            id: &str,
            order: &[PlaylistItemType<&str, &str>],
        ) {
            let tracks = client
                .playlists()
                .get_playlists_items(id, order.len(), 0, None)
                .await
                .unwrap()
                .data;
            assert_eq!(tracks.total, order.len());
            assert_eq!(
                tracks
                    .items
                    .iter()
                    .map(|item| match item.item.as_ref().unwrap() {
                        PlaylistItemType::Track(track) =>
                            PlaylistItemType::Track(track.id.as_deref().unwrap()),
                        PlaylistItemType::Episode(episode) =>
                            PlaylistItemType::Episode(&*episode.id),
                    })
                    .collect::<Vec<_>>(),
                order
            );
        };

        // Replace
        let mut snapshot = playlists
            .replace_playlists_items(&playlist.id, items.iter().cloned())
            .await
            .unwrap();
        assert_playlist_order(&client, &playlist.id, &[items[0], items[1], items[2]]).await;

        // Reorder
        snapshot = playlists
            .reorder_playlist(&playlist.id, 1, 1, 0, &snapshot)
            .await
            .unwrap();
        assert_playlist_order(&client, &playlist.id, &[items[1], items[0], items[2]]).await;
        playlists
            .reorder_playlist(&playlist.id, 0, 2, 3, &snapshot)
            .await
            .unwrap();
        assert_playlist_order(&client, &playlist.id, &[items[2], items[1], items[0]]).await;

        // Add
        snapshot = playlists
            .add_to_playlist(&playlist.id, [items[0], items[1]].iter().cloned(), Some(1))
            .await
            .unwrap();
        assert_playlist_order(
            &client,
            &playlist.id,
            &[items[2], items[0], items[1], items[1], items[0]],
        )
        .await;

        // Remove
        playlists
            .remove_from_playlist(
                &playlist.id,
                [
                    (items[0], None),
                    (items[2], Some(&[0][..])),
                    (items[1], Some(&[2, 3][..])),
                ]
                .iter()
                .cloned(),
                &snapshot,
            )
            .await
            .unwrap();
        let playlist = playlists
            .get_playlist(&playlist.id, None)
            .await
            .unwrap()
            .data;
        assert_eq!(playlist.tracks.items, &[]);

        // Upload image
        #[cfg(feature = "base64")]
        {
            playlists
                .upload_playlist_cover_file(&playlist.id, "example_image.jpeg")
                .await
                .unwrap();
            time::sleep(Duration::from_secs(5)).await;
            let images = playlists
                .get_playlists_images(&playlist.id)
                .await
                .unwrap()
                .data;
            assert_eq!(images.len(), 1);
            if let Some(height) = images[0].height {
                assert_eq!(height, 512);
            }
            if let Some(width) = images[0].width {
                assert_eq!(width, 512);
            }
        }

        // Unfollow playlist
        client
            .follow()
            .unfollow_playlist(&playlist.id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_users_playlists() {
        client()
            .playlists()
            .get_users_playlists("wizzler", 2, 1)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_playlist_with_episodes() {
        client()
            .playlists()
            .get_playlist("37i9dQZF1DXacZOGa5EAdH", None)
            .await
            .unwrap();
    }
}
