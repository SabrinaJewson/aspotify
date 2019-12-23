use crate::*;

/// Check if the current user has saved some albums.
///
/// Returns vector of bools that is in the same order as the given ids, telling whether the user
/// has saved each album. Requires `user-library-read`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/check-users-saved-albums/).
pub async fn user_saved_albums(token: &AccessToken, ids: &[&str]) -> Result<Vec<bool>, EndpointError<Error>> {
    Ok(request!(token, GET "/v1/me/albums/contains", query_params = {"ids": ids.join(",")}, ret = Vec<bool>))
}

/// Check if the current user has saved some tracks.
///
/// Returns vector of bools that is in the same order as the given ids, telling whether the user
/// has saved each track. Requires `user-library-read`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/check-users-saved-tracks/).
pub async fn user_saved_tracks(token: &AccessToken, ids: &[&str]) -> Result<Vec<bool>, EndpointError<Error>> {
    Ok(request!(token, GET "/v1/me/tracks/contains", query_params = {"ids": ids.join(",")}, ret = Vec<bool>))
}

/// Get the current user's saved albums.
///
/// Requires `user-library-read`. Limit must be in the range [1..50].
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/get-users-saved-albums/).
pub async fn get_saved_albums(token: &AccessToken, limit: usize, offset: usize, market: Option<Market>) -> Result<Page<SavedAlbum>, EndpointError<Error>> {
    Ok(request!(token, GET "/v1/me/albums", query_params = {"limit": limit.to_string(), "offset": offset.to_string()}, optional_query_params = {"market": market.map(|m| m.to_string())}, ret = Page<SavedAlbum>))
}

/// Get the current user's saved tracks.
///
/// Requires `user-library-read`. Limit must be in the range [1..50].
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/get-users-saved-tracks/).
pub async fn get_saved_tracks(token: &AccessToken, limit: usize, offset: usize, market: Option<Market>) -> Result<Page<SavedTrack>, EndpointError<Error>> {
    Ok(request!(token, GET "/v1/me/tracks", query_params = {"limit": limit.to_string(), "offset": offset.to_string()}, optional_query_params = {"market": market.map(|m| m.to_string())}, ret = Page<SavedTrack>))
}

/// Unsave some of the current user's saved albums.
///
/// Requires `user-library-modify`. Maximum of 50 ids.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/remove-albums-user/).
pub async fn unsave_albums(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    request!(token, DELETE "/v1/me/albums", query_params = {"ids": ids.join(",")}, body = "{}");
    Ok(())
}

/// Unsave some of the current user's saved tracks.
///
/// Requires `user-library-modify`. Maximum of 50 ids.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/remove-tracks-user/).
pub async fn unsave_tracks(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    request!(token, DELETE "/v1/me/tracks", query_params = {"ids": ids.join(",")}, body = "{}");
    Ok(())
}

/// Save albums for the current user.
///
/// Requires `user-library-modify`. Maximum of 50 ids.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/save-albums-user/).
pub async fn save_albums(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    request!(token, PUT "/v1/me/albums", query_params = {"ids": ids.join(",")}, body = "{}");
    Ok(())
}

/// Save tracks for the current user.
///
/// Requires `user-library-modify`. Maximum of 50 ids.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/save-albums-user/).
pub async fn save_tracks(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    request!(token, PUT "/v1/me/tracks", query_params = {"ids": ids.join(",")}, body = "{}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::endpoints::token;

    #[tokio::test]
    async fn test_save_albums() {
        // NOTE: This test only works if you have < 49 albums saved as it only requests the first
        // page.
        // You also must not have the album "Spirit Phone" by Lemon Demon saved.
        let token = token().await;

        // Save "Wish" and "The Black Parade"
        save_albums(&token, &["0aEL0zQ4XLuxQP0j7sLlS1", "0FZK97MXMm5mUQ8mtudjuK"]).await.unwrap();

        // Check "Spirit Phone" and "The Black Parade"
        assert_eq!(user_saved_albums(&token, &["4ocal2JegUDVQdP6KN1roI", "0FZK97MXMm5mUQ8mtudjuK"]).await.unwrap(), &[false, true]);
        // Check "Wish" and "The Black Parade"
        let saved = get_saved_albums(&token, 50, 0, None).await.unwrap().items;
        assert!(saved.iter().any(|album| album.album.name == "Wish"));
        assert!(saved.iter().any(|album| album.album.name == "The Black Parade"));

        // Unsave "Wish" and "The Black Parade"
        unsave_albums(&token, &["0aEL0zQ4XLuxQP0j7sLlS1", "0FZK97MXMm5mUQ8mtudjuK"]).await.unwrap();

        // Same checks as before but inverted
        assert_eq!(user_saved_albums(&token, &["4ocal2JegUDVQdP6KN1roI", "0FZK97MXMm5mUQ8mtudjuK"]).await.unwrap(), &[false, false]);
        let saved = get_saved_albums(&token, 50, 0, None).await.unwrap().items;
        assert!(saved.iter().all(|album| album.album.name != "Wish"));
        assert!(saved.iter().all(|album| album.album.name != "The Black Parade"));
    }

    #[tokio::test]
    async fn test_save_tracks() {
        // NOTE: This test only works if you have < 50 tracks saved as it only requests the first
        // page.
        // You also must not have the track "Spiral of Ants" by Lemon Demon saved.
        let token = token().await;

        // Save "Friday I'm In Love"
        save_tracks(&token, &["4QlzkaRHtU8gAdwqjWmO8n"]).await.unwrap();

        // Check "Friday I'm In Love" and "Spiral of Ants"
        assert_eq!(user_saved_tracks(&token, &["4QlzkaRHtU8gAdwqjWmO8n", "77hzctaLvLRLAh71LwNPE3"]).await.unwrap(), &[true, false]);
        // Check "Friday I'm In Love"
        let saved = get_saved_tracks(&token, 50, 0, None).await.unwrap().items;
        assert!(saved.iter().any(|track| track.track.name == "Friday I'm In Love"));

        // Unsave "Friday I'm In Love"
        unsave_tracks(&token, &["4QlzkaRHtU8gAdwqjWmO8n"]).await.unwrap();

        // Same checks as before but inverted
        // Check "Friday I'm In Love" and "Spiral of Ants"
        assert_eq!(user_saved_tracks(&token, &["4QlzkaRHtU8gAdwqjWmO8n", "77hzctaLvLRLAh71LwNPE3"]).await.unwrap(), &[false, false]);
        // Check "Friday I'm In Love"
        let saved = get_saved_tracks(&token, 50, 0, None).await.unwrap().items;
        assert!(saved.iter().all(|track| track.track.name != "Friday I'm In Love"));
    }
}
