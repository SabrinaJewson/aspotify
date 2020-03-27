//! Endpoints relating to saving albums and tracks.

use crate::*;

/// Check if the current user has saved some albums.
///
/// Returns vector of bools that is in the same order as the given ids, telling whether the user
/// has saved each album. Requires `user-library-read`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/check-users-saved-albums/).
pub async fn user_saved_albums(
    token: &AccessToken,
    ids: &[&str],
) -> Result<Vec<bool>, EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    Ok(request!(
        token,
        GET "/v1/me/albums/contains",
        query_params = {"ids": ids.join(",")},
        ret = Vec<bool>
    ))
}

/// Check if the current user has saved some shows.
///
/// Returns vector of bools that is in the same order as the given ids, telling whether the user
/// has saved each album. Requires `user-library-read`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/check-users-saved-shows/).
pub async fn user_saved_shows(
    token: &AccessToken,
    ids: &[&str],
) -> Result<Vec<bool>, EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    Ok(request!(
        token,
        GET "/v1/me/shows/contains",
        query_params = {"ids": ids.join(",")},
        ret = Vec<bool>
    ))
}

/// Check if the current user has saved some tracks.
///
/// Returns vector of bools that is in the same order as the given ids, telling whether the user
/// has saved each track. Requires `user-library-read`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/check-users-saved-tracks/).
pub async fn user_saved_tracks(
    token: &AccessToken,
    ids: &[&str],
) -> Result<Vec<bool>, EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    Ok(request!(
        token,
        GET "/v1/me/tracks/contains",
        query_params = {"ids": ids.join(",")},
        ret = Vec<bool>
    ))
}

/// Get the current user's saved albums.
///
/// Requires `user-library-read`. Limit must be in the range [1..50].
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/get-users-saved-albums/).
pub async fn get_saved_albums(
    token: &AccessToken,
    limit: usize,
    offset: usize,
    market: Option<Market>,
) -> Result<Page<SavedAlbum>, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/me/albums",
        query_params = {"limit": limit.to_string(), "offset": offset.to_string()},
        optional_query_params = {"market": market.map(|m| m.as_str())},
        ret = Page<SavedAlbum>
    ))
}

/// Get the current user's saved shows.
///
/// Requires `user-library-read`. Limit must be in the range [1..50].
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/get-users-saved-shows/).
pub async fn get_saved_shows(
    token: &AccessToken,
    limit: usize,
    offset: usize,
) -> Result<Page<SavedShow>, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/me/shows",
        query_params = {"limit": limit.to_string(), "offset": offset.to_string()},
        ret = Page<SavedShow>
    ))
}

/// Get the current user's saved tracks.
///
/// Requires `user-library-read`. Limit must be in the range [1..50].
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/get-users-saved-tracks/).
pub async fn get_saved_tracks(
    token: &AccessToken,
    limit: usize,
    offset: usize,
    market: Option<Market>,
) -> Result<Page<SavedTrack>, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/me/tracks",
        query_params = {"limit": limit.to_string(), "offset": offset.to_string()},
        optional_query_params = {"market": market.map(|m| m.as_str())},
        ret = Page<SavedTrack>
    ))
}

/// Unsave some of the current user's saved albums.
///
/// Requires `user-library-modify`. Maximum of 50 ids.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/remove-albums-user/).
pub async fn unsave_albums(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(());
    }
    request!(
        token,
        DELETE "/v1/me/albums",
        query_params = {"ids": ids.join(",")},
        body = "{}"
    );
    Ok(())
}

/// Unsave some of the current user's saved shows.
///
/// Requires `user-library-modify`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/remove-shows-user/).
pub async fn unsave_shows(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(());
    }
    request!(
        token,
        DELETE "/v1/me/shows",
        query_params = {"ids": ids.join(",")},
        body = "{}"
    );
    Ok(())
}

/// Unsave some of the current user's saved tracks.
///
/// Requires `user-library-modify`. Maximum of 50 ids.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/remove-tracks-user/).
pub async fn unsave_tracks(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(());
    }
    request!(
        token,
        DELETE "/v1/me/tracks",
        query_params = {"ids": ids.join(",")},
        body = "{}"
    );
    Ok(())
}

/// Save albums for the current user.
///
/// Requires `user-library-modify`. Maximum of 50 ids.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/save-albums-user/).
pub async fn save_albums(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(());
    }
    request!(
        token,
        PUT "/v1/me/albums",
        query_params = {"ids": ids.join(",")},
        body = "{}"
    );
    Ok(())
}

/// Save shows for the current user.
///
/// Requires `user-library-modify`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/save-shows-user/).
pub async fn save_shows(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(());
    }
    request!(
        token,
        PUT "/v1/me/shows",
        query_params = {"ids": ids.join(",")},
        body = "{}"
    );
    Ok(())
}

/// Save tracks for the current user.
///
/// Requires `user-library-modify`. Maximum of 50 ids.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/save-albums-user/).
pub async fn save_tracks(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(());
    }
    request!(
        token,
        PUT "/v1/me/tracks",
        query_params = {"ids": ids.join(",")},
        body = "{}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;

    #[tokio::test]
    async fn test_save_albums() {
        let token = token().await;

        // "Wish", "The Black Parade", and "Spirit Phone"
        let albums = &[
            "0aEL0zQ4XLuxQP0j7sLlS1",
            "0FZK97MXMm5mUQ8mtudjuK",
            "4ocal2JegUDVQdP6KN1roI",
        ];
        let split = 2;
        let (saved_albums, unsaved_albums) = albums.split_at(split);

        // Store old saved status to restore
        let old = user_saved_albums(&token, albums).await.unwrap();

        // Saving and unsaving
        save_albums(&token, saved_albums).await.unwrap();
        unsave_albums(&token, unsaved_albums).await.unwrap();

        // Check
        let check = user_saved_albums(&token, albums).await.unwrap();
        let (save_check, unsave_check) = check.split_at(split);
        assert!(save_check.into_iter().all(|&saved| saved));
        assert!(unsave_check.into_iter().all(|&saved| !saved));

        // Check by finding in list
        let saved = get_saved_albums(&token, 50, 0, None).await.unwrap();
        if saved.total <= 50 {
            for saved_album in saved_albums {
                assert!(saved
                    .items
                    .iter()
                    .any(|album| album.album.id == *saved_album));
            }
            for unsaved_album in unsaved_albums {
                assert!(saved
                    .items
                    .iter()
                    .all(|album| album.album.id != *unsaved_album));
            }
        }

        // Restore
        let mut old_saved = Vec::with_capacity(albums.len());
        let mut old_unsaved = Vec::with_capacity(albums.len());
        for i in 0..albums.len() {
            if old[i] {
                &mut old_saved
            } else {
                &mut old_unsaved
            }
            .push(albums[i]);
        }
        save_albums(&token, &old_saved).await.unwrap();
        unsave_albums(&token, &old_unsaved).await.unwrap();
    }

    #[tokio::test]
    async fn test_save_shows() {
        let token = token().await;

        let shows = &["5CfCWKI5pZ28U0uOzXkDHe", "6ups0LMt1G8n81XLlkbsPo"];
        let split = 1;
        let (saved_shows, unsaved_shows) = shows.split_at(split);

        // Store old saved status to restore
        let old = user_saved_shows(&token, shows).await.unwrap();

        // Saving and unsaving
        save_shows(&token, saved_shows).await.unwrap();
        unsave_shows(&token, unsaved_shows).await.unwrap();

        // Check
        let check = user_saved_shows(&token, shows).await.unwrap();
        let (save_check, unsave_check) = check.split_at(split);
        assert!(save_check.into_iter().all(|&saved| saved));
        assert!(unsave_check.into_iter().all(|&saved| !saved));

        // Check by finding in list, only if it has them all
        let saved = get_saved_shows(&token, 50, 0).await.unwrap();
        if saved.total <= 50 {
            for saved_show in saved_shows {
                assert!(saved.items.iter().any(|show| show.show.id == *saved_show));
            }
            for unsaved_show in unsaved_shows {
                assert!(saved.items.iter().all(|show| show.show.id != *unsaved_show));
            }
        }

        // Restore
        let mut old_saved = Vec::with_capacity(shows.len());
        let mut old_unsaved = Vec::with_capacity(shows.len());
        for i in 0..shows.len() {
            if old[i] {
                &mut old_saved
            } else {
                &mut old_unsaved
            }
            .push(shows[i]);
        }
        save_shows(&token, &old_saved).await.unwrap();
        unsave_shows(&token, &old_unsaved).await.unwrap();
    }

    #[tokio::test]
    async fn test_save_tracks() {
        let token = token().await;

        // "Friday I'm In Love" and "Spiral of Ants"
        let tracks = &["4QlzkaRHtU8gAdwqjWmO8n", "77hzctaLvLRLAh71LwNPE3"];
        let split = 1;
        let (saved_tracks, unsaved_tracks) = tracks.split_at(split);

        // Store old saved status to restore
        let old = user_saved_tracks(&token, tracks).await.unwrap();

        // Saving and unsaving
        save_tracks(&token, saved_tracks).await.unwrap();
        unsave_tracks(&token, unsaved_tracks).await.unwrap();

        // Check
        let check = user_saved_tracks(&token, tracks).await.unwrap();
        let (save_check, unsave_check) = check.split_at(split);
        assert!(save_check.into_iter().all(|&saved| saved));
        assert!(unsave_check.into_iter().all(|&saved| !saved));

        // Check by finding in list, only if it has them all
        let saved = get_saved_tracks(&token, 50, 0, None).await.unwrap();
        if saved.total <= 50 {
            for saved_track in saved_tracks {
                assert!(saved
                    .items
                    .iter()
                    .any(|track| track.track.id == *saved_track));
            }
            for unsaved_track in unsaved_tracks {
                assert!(saved
                    .items
                    .iter()
                    .all(|track| track.track.id != *unsaved_track));
            }
        }

        // Restore
        let mut old_saved = Vec::with_capacity(tracks.len());
        let mut old_unsaved = Vec::with_capacity(tracks.len());
        for i in 0..tracks.len() {
            if old[i] {
                &mut old_saved
            } else {
                &mut old_unsaved
            }
            .push(tracks[i]);
        }
        save_tracks(&token, &old_saved).await.unwrap();
        unsave_tracks(&token, &old_unsaved).await.unwrap();
    }
}
