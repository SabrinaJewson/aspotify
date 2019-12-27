//! Endpoint functions relating to playlists.
//!
//! The parameter `snapshot_id` is the snapshot of the playlist to perform the operation on to
//! prevent concurrent accesses causing problems.
//!
//! Take this example; person A gets playlist X. Person B removes song N from playlist X. Person A
//! tries to do something to playlist X, assuming song N still exists, but it causes unexpected
//! behaviour because song N doesn't actually exist any more. However, with snapshot ids, person A
//! will have used the snapshot ID they received from the initial request in their request. Spotify
//! knows that person A is attempting to operate on an older playlist, and adjusts accordingly,
//! causing no unexpected behaviour.
//!
//! One feature of Spotify is that you cannot delete playlists; you can only unfollow them, hence
//! there is no `delete_playlist` function.

use crate::*;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct SnapshotId {
    snapshot_id: String,
}

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
pub async fn add_to_playlist(
    token: &AccessToken,
    id: &str,
    tracks: &[&str],
    position: Option<usize>,
) -> Result<String, EndpointError<Error>> {
    Ok(request!(
        token,
        POST "/v1/playlists/{}/tracks",
        path_params = [id],
        header_params = {"Content-Type": "application/json"},
        body = serde_json::json!({
            "uris": tracks.iter().map(|track| format!("spotify:track:{}", track)).collect::<Vec<_>>(),
            "position": position,
        }).to_string(),
        ret = SnapshotId
    ).snapshot_id)
}

/// Change a playlist's details.
///
/// Requires `playist-modify-public` if the playlist is public, and `playlist-modify-private` if it
/// is private. Each parameter, when Some, changes an attribute of the playlist. A playlist cannot
/// be both public and collaborative.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/change-playlist-details/).
pub async fn change_playlist(
    token: &AccessToken,
    id: &str,
    name: Option<&str>,
    public: Option<bool>,
    collaborative: Option<bool>,
    description: Option<&str>,
) -> Result<(), EndpointError<Error>> {
    request!(
        token,
        PUT "/v1/playlists/{}",
        path_params = [id],
        header_params = {"Content-Type": "application/json"},
        body = serde_json::json!({
            "name": name,
            "public": public,
            "collaborative": collaborative,
            "description": description,
        }).to_string()
    );
    Ok(())
}

/// Create a playlist.
///
/// Requires `playlist-modify-public` if creating a public playlist, requires
/// `playlist-modify-private` if creating a private one.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/create-playlist/).
pub async fn create_playlist(
    token: &AccessToken,
    name: &str,
    public: bool,
    collaborative: bool,
    description: &str,
) -> Result<Playlist, EndpointError<Error>> {
    Ok(request!(
        token,
        POST "/v1/me/playlists",
        header_params = {"Content-Type": "application/json"},
        body = serde_json::json!({
            "name": name,
            "public": public,
            "collaborative": collaborative,
            "description": description,
        }).to_string(),
        ret = Playlist
    ))
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
    token: &AccessToken,
    limit: usize,
    offset: usize,
) -> Result<Page<PlaylistSimplified>, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/me/playlists",
        query_params = {"limit": limit.to_string(), "offset": offset.to_string()},
        ret = Page<PlaylistSimplified>
    ))
}

/// Get a user's playlists.
///
/// Gets a list of playlists owned or followed by a Spotify user.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/get-list-users-playlists/).
pub async fn get_users_playlists(
    token: &AccessToken,
    id: &str,
    limit: usize,
    offset: usize,
) -> Result<Page<PlaylistSimplified>, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/users/{}/playlists",
        path_params = [id],
        query_params = {"limit": limit.to_string(), "offset": offset.to_string()},
        ret = Page<PlaylistSimplified>
    ))
}

/// Get information about a playlist.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/get-playlist/).
pub async fn get_playlist(
    token: &AccessToken,
    id: &str,
    market: Option<Market>,
) -> Result<Playlist, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/playlists/{}",
        path_params = [id],
        optional_query_params = {"market": market.map(|m| m.as_str())},
        ret = Playlist
    ))
}

/// Get a playlist's cover images.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/get-playlist-cover/).
pub async fn get_playlists_images(
    token: &AccessToken,
    id: &str,
) -> Result<Vec<Image>, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/playlists/{}/images",
        path_params = [id],
        ret = Vec<Image>
    ))
}

/// Get a playlist's tracks.
///
/// Limit must be in the range [1..100].
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/get-playlist-cover/).
pub async fn get_playlists_tracks(
    token: &AccessToken,
    id: &str,
    limit: usize,
    offset: usize,
    market: Option<Market>,
) -> Result<Page<PlaylistTrack>, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/playlists/{}/tracks",
        path_params = [id],
        query_params = {"limit": limit.to_string(), "offset": offset.to_string()},
        optional_query_params = {"market": market.map(|m| m.as_str())},
        ret = Page<PlaylistTrack>
    ))
}

/// Remove tracks from a playlist.
///
/// Requires `playlist-modify-public` if the playlist is public, requires `playlist-modify-private`
/// if it is private.
///
/// Tracks is a list of tracks that will be removed, and optionally multiple positions where the
/// tracks will be removed, otherwise it will remove them from the entire playlist. It is not
/// possible to remove tracks only by position. There is a maximum of 100 tracks you can remove at
/// once.
///
/// This function returns the `snapshot_id` of the created playlist, which you should hold on to to
/// stop concurrent accesses to the playlist interfering with each other.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/remove-tracks-playlist/).
pub async fn remove_from_playlist(
    token: &AccessToken,
    id: &str,
    tracks: &[(&str, Option<&[usize]>)],
    snapshot_id: &str,
) -> Result<String, EndpointError<Error>> {
    if tracks.is_empty() {
        return Ok(String::from(snapshot_id));
    }

    Ok(request!(
        token,
        DELETE "/v1/playlists/{}/tracks",
        path_params = [id],
        header_params = {"Content-Type": "application/json"},
        body = serde_json::json!({
            "tracks": tracks.iter().map(|(id, positions)| if let Some(position) = positions {
                serde_json::json!({
                    "uri": format!("spotify:track:{}", id),
                    "positions": position,
                })
            } else {
                serde_json::json!({
                    "uri": format!("spotify:track:{}", id),
                })
            }).collect::<Vec<_>>(),
            "snapshot_id": snapshot_id,
        }).to_string(),
        ret = SnapshotId
    )
    .snapshot_id)
}

/// Reorder tracks in a playlist.
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
    token: &AccessToken,
    id: &str,
    range_start: usize,
    range_length: usize,
    insert_before: usize,
    snapshot_id: &str,
) -> Result<String, EndpointError<Error>> {
    if range_length == 0 || range_start + range_length == insert_before {
        return Ok(String::from(snapshot_id));
    }

    Ok(request!(
        token,
        PUT "/v1/playlists/{}/tracks",
        path_params = [id],
        header_params = {"Content-Type": "application/json"},
        body = format!(
            r#"{{"range_start":{},"range_length":{},"insert_before":{},"snapshot_id":"{}"}}"#,
            range_start, range_length, insert_before, snapshot_id
        ),
        ret = SnapshotId
    )
    .snapshot_id)
}

/// Replace a playlist's tracks.
///
/// Requires `playlist-modify-public` if the playlist is public, requires `playlist-modify-private`
/// if it is private.
///
/// This function removes all the songs from the given playlist, and replaces them with the given
/// tracks. The maximum number of tracks is 100, if you need more you can use
/// [add_to_playlist](fn.add_to_playlist.html).
///
/// This function returns the `snapshot_id` of the created playlist, which you should hold on to to
/// stop concurrent accesses to the playlist interfering with each other.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/replace-playlists-tracks/).
pub async fn replace_playlists_tracks(
    token: &AccessToken,
    id: &str,
    tracks: &[&str],
) -> Result<String, EndpointError<Error>> {
    Ok(request!(
        token,
        PUT "/v1/playlists/{}/tracks",
        path_params = [id],
        header_params = {"Content-Type": "application/json"},
        body = serde_json::json!({
            "uris": tracks.iter().map(|id| format!("spotify:track:{}", id)).collect::<Vec<_>>(),
        }).to_string(),
        ret = SnapshotId
    )
    .snapshot_id)
}

/// Upload a custom playlist cover image.
///
/// Requires `playlist-modify-public` if the playlist is public, requires `playlist-modify-private`
/// if it is private, and also requires `ugc-image-upload`.
///
/// `image` must be a base64-encoded JPEG image under 256KB. If you want to pass in JPEG data, see
/// [upload_playlist_cover_jpeg](fn.upload_playlist_cover_jpeg.html); if you want to pass in a
/// filename, see [upload_playlist_cover_file](fn.upload_playlist_cover_file.html).
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/upload-custom-playlist-cover/).
pub async fn upload_playlist_cover(
    token: &AccessToken,
    id: &str,
    image: &str,
) -> Result<(), EndpointError<Error>> {
    request!(
        token,
        PUT "/v1/playlists/{}/images",
        path_params = [id],
        header_params = {"Content-Type": "image/jpeg"},
        body = String::from(image)
    );
    Ok(())
}

/// Upload a custom playlist cover image.
///
/// Requires `playlist-modify-public` if the playlist is public, requires `playlist-modify-private`
/// if it is private, and also requires `ugc-image-upload`.
///
/// `image` must be JPEG data. If you want to pass in a filename, see
/// [upload_playlist_cover_file](fn.upload_playlist_cover_file.html).
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/upload-custom-playlist-cover/).
pub async fn upload_playlist_cover_jpeg<T: ?Sized + AsRef<[u8]>>(
    token: &AccessToken,
    id: &str,
    image: &T,
) -> Result<(), EndpointError<Error>> {
    upload_playlist_cover(token, id, &base64::encode(image)).await
}

/// Upload a custom playlist cover image.
///
/// Requires `playlist-modify-public` if the playlist is public, requires `playlist-modify-private`
/// if it is private, and also requires `ugc-image-upload`.
///
/// `image` must be a JPEG filename.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/upload-custom-playlist-cover/).
pub async fn upload_playlist_cover_file<P: AsRef<Path>>(
    token: &AccessToken,
    id: &str,
    image: P,
) -> Result<(), EndpointError<Error>> {
    upload_playlist_cover_jpeg(token, id, &fs::read(image)?).await
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;
    use std::time::Duration;
    use tokio::timer;

    #[tokio::test]
    async fn test() {
        let token = token().await;

        let mut playlist =
            create_playlist(&token, "Testing Playlist", true, false, "Test Description")
                .await
                .unwrap();
        assert_eq!(playlist.name, "Testing Playlist");
        assert_eq!(playlist.public, Some(true));
        assert_eq!(playlist.collaborative, false);
        assert_eq!(playlist.description.as_ref().unwrap(), "Test Description");
        assert_eq!(playlist.followers, Followers { total: 0 });
        assert!(playlist.images.is_empty());
        assert_eq!(playlist.tracks.total, 0);

        let got_playlist = get_playlist(&token, &playlist.id, None).await.unwrap();
        playlist.snapshot_id = got_playlist.snapshot_id.clone();
        assert_eq!(playlist, got_playlist);

        let playlists = current_users_playlists(&token, 50, 0).await.unwrap();
        if playlists.total <= 50 {
            assert!(playlists.items.iter().any(|p| p.id == playlist.id));
        }

        change_playlist(
            &token,
            &playlist.id,
            Some("New Name"),
            Some(false),
            Some(true),
            Some("New Description"),
        )
        .await
        .unwrap();
        let playlist = get_playlist(&token, &playlist.id, None).await.unwrap();
        assert_eq!(playlist.name, "New Name");
        assert_eq!(playlist.public, Some(false));
        assert_eq!(playlist.collaborative, true);
        assert_eq!(playlist.description.unwrap(), "New Description");
        assert_eq!(playlist.followers, Followers { total: 0 });
        assert!(playlist.images.is_empty());
        assert_eq!(playlist.tracks.total, 0);

        // Add "Ten Tonne Skeleton" and "The Middle"
        let snapshot = add_to_playlist(
            &token,
            &playlist.id,
            &["0vjYxBDAcflD0358arIVZG", "6GG73Jik4jUlQCkKg9JuGO"],
            None,
        )
        .await
        .unwrap();
        assert_ne!(playlist.snapshot_id, snapshot);
        let playlist = get_playlist(&token, &playlist.id, None).await.unwrap();
        assert_eq!(playlist.snapshot_id, snapshot);
        assert_eq!(playlist.tracks.total, 2);

        let tracks = get_playlists_tracks(&token, &playlist.id, 1, 1, None)
            .await
            .unwrap();
        assert_eq!(tracks.items.len(), 1);
        assert_eq!(tracks.items[0].is_local, false);
        assert_eq!(tracks.items[0].track.id, "6GG73Jik4jUlQCkKg9JuGO");
        assert_eq!(tracks.limit, 1);
        assert_eq!(tracks.offset, 1);
        assert_eq!(tracks.total, 2);

        // "Blue", "Friday I'm In Love", "Mr. Brightside"
        let tracks = &[
            "22wRQVOHzHAppfKsDs38nj",
            "4QlzkaRHtU8gAdwqjWmO8n",
            "7d8GetOsjbxYnlo6Y9e5Kw",
        ];

        async fn assert_playlist_order(token: &AccessToken, id: &str, order: &[&str]) {
            let tracks = get_playlists_tracks(token, id, order.len(), 0, None)
                .await
                .unwrap();
            assert_eq!(tracks.total, order.len());
            assert_eq!(
                tracks
                    .items
                    .iter()
                    .map(|track| &track.track.id)
                    .collect::<Vec<_>>(),
                order
            );
        }

        // Replace
        let mut snapshot = replace_playlists_tracks(&token, &playlist.id, tracks)
            .await
            .unwrap();
        assert_playlist_order(&token, &playlist.id, &[tracks[0], tracks[1], tracks[2]]).await;

        // Reorder
        snapshot = reorder_playlist(&token, &playlist.id, 1, 1, 0, &snapshot)
            .await
            .unwrap();
        assert_playlist_order(&token, &playlist.id, &[tracks[1], tracks[0], tracks[2]]).await;
        reorder_playlist(&token, &playlist.id, 0, 2, 3, &snapshot)
            .await
            .unwrap();
        assert_playlist_order(&token, &playlist.id, &[tracks[2], tracks[1], tracks[0]]).await;

        // Add
        snapshot = add_to_playlist(&token, &playlist.id, &[tracks[0], tracks[1]], Some(1))
            .await
            .unwrap();
        assert_playlist_order(
            &token,
            &playlist.id,
            &[tracks[2], tracks[0], tracks[1], tracks[1], tracks[0]],
        )
        .await;

        // Remove
        remove_from_playlist(
            &token,
            &playlist.id,
            &[
                (tracks[0], None),
                (tracks[2], Some(&[0])),
                (tracks[1], Some(&[2, 3])),
            ],
            &snapshot,
        )
        .await
        .unwrap();
        let playlist = get_playlist(&token, &playlist.id, None).await.unwrap();
        assert_eq!(playlist.tracks.items, &[]);

        // Upload image
        upload_playlist_cover_file(&token, &playlist.id, "example_image.jpeg")
            .await
            .unwrap();
        timer::delay_for(Duration::from_secs(5)).await;
        let images = get_playlists_images(&token, &playlist.id).await.unwrap();
        assert_eq!(images.len(), 1);
        if let Some(height) = images[0].height {
            assert_eq!(height, 512);
        }
        if let Some(width) = images[0].width {
            assert_eq!(width, 512);
        }

        // Unfollow playlist
        unfollow_playlist(&token, &playlist.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_users_playlists() {
        get_users_playlists(&token().await, "wizzler", 2, 1)
            .await
            .unwrap();
    }
}
