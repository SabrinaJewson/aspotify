use crate::*;
use serde::Deserialize;

/// Check if current user follows artists.
///
/// Returns vector of bools that is in the same order as the given ids. Maximum 50 IDs. Requires
/// `user-follow-read`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/check-current-user-follows/).
pub async fn user_follows_artists(
    token: &AccessToken,
    ids: &[&str],
) -> Result<Vec<bool>, EndpointError<Error>> {
    Ok(
        request!(token, GET "/v1/me/following/contains", query_params = {"type": "artist", "ids": &&ids.join(",")}, ret = Vec<bool>),
    )
}

/// Check if current user follows users.
///
/// Return vector of bools that is in the same order as the given ids. Maximum 50 IDs. Requires
/// `user-follow-read`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/check-current-user-follows/).
pub async fn user_follows_users(
    token: &AccessToken,
    ids: &[&str],
) -> Result<Vec<bool>, EndpointError<Error>> {
    Ok(
        request!(token, GET "/v1/me/following/contains", query_params = {"type": "user", "ids": &&ids.join(",")}, ret = Vec<bool>),
    )
}

/// Check if users follow a playlist.
///
/// `id` is the id of the playlist and `user_ids` is the users who you want to check, maximum 5.
/// Users can publicly or privately follow playlists; checking whether a user privately follows a
/// playlist requires `playlist-read-private`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/check-user-following-playlist/).
pub async fn users_follow_playlist(
    token: &AccessToken,
    id: &str,
    user_ids: &[&str],
) -> Result<Vec<bool>, EndpointError<Error>> {
    Ok(
        request!(token, GET "/v1/playlists/{}/followers/contains", path_params = [id], query_params = {"ids": &&user_ids.join(",")}, ret = Vec<bool>),
    )
}

/// Follow artists.
///
/// Maximum 50 ids. Requires `user-follow-modify`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/follow-artists-users/).
pub async fn follow_artists(
    token: &AccessToken,
    ids: &[&str],
) -> Result<(), EndpointError<Error>> {
    request!(token, PUT "/v1/me/following", query_params = {"type": "artist", "ids": &ids.join(",")}, body = "{}");
    Ok(())
}

/// Follow users.
///
/// Maximum 50 ids. Requires `user-follow-modify`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/follow-artists-users/).
pub async fn follow_users(
    token: &AccessToken,
    ids: &[&str],
) -> Result<(), EndpointError<Error>> {
    request!(token, PUT "/v1/me/following", query_params = {"type": "user", "ids": &ids.join(",")}, body = "{}");
    Ok(())
}

/// Follow a playlist publicly.
///
/// Requires `playlist-modify-public`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/follow-playlist/).
pub async fn follow_playlist_public(
    token: &AccessToken,
    id: &str,
) -> Result<(), EndpointError<Error>> {
    request!(token, PUT "/v1/playlists/{}/followers", path_params = [id], header_params = {"Content-Type": "application/json"}, body = "{\"public\": true}");
    Ok(())
}

/// Follow a playlist privately.
///
/// Requires `playlist-modify-private`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/follow-playlist/).
pub async fn follow_playlist_private(
    token: &AccessToken,
    id: &str,
) -> Result<(), EndpointError<Error>> {
    request!(token, PUT "/v1/playlists/{}/followers", path_params = [id], header_params = {"Content-Type": "application/json"}, body = "{\"public\": false}");
    Ok(())
}

/// Get followed artists.
///
/// Limit must be in the range [1..50]. `after` is the Cursor value given the previous time this
/// endpoint was called. It is used to get the next page of items.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/get-followed/).
pub async fn get_followed_artists(
    token: &AccessToken,
    limit: usize,
    after: Option<&str>,
) -> Result<CursorPage<Artist>, EndpointError<Error>> {
    #[derive(Deserialize)]
    struct Response {
        artists: CursorPage<Artist>,
    };

    Ok(request!(token, GET "/v1/me/following", query_params = {"type": "artist", "limit": &limit.to_string()}, optional_query_params = {"after": after}, ret = Response).artists)
}

/// Unfollow artists.
///
/// Maximum 50 ids. Requires `user-follow-modify`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/unfollow-artists-users/).
pub async fn unfollow_artists(
    token: &AccessToken,
    ids: &[&str],
) -> Result<(), EndpointError<Error>> {
    request!(token, DELETE "/v1/me/following", query_params = {"type": "artist", "ids": &ids.join(",")}, body = "{}");
    Ok(())
}

/// Unfollow users.
///
/// Maximum 50 ids. Requires `user-follow-modify`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/unfollow-artists-users/).
pub async fn unfollow_users(
    token: &AccessToken,
    ids: &[&str],
) -> Result<(), EndpointError<Error>> {
    request!(token, DELETE "/v1/me/following", query_params = {"type": "user", "ids": &ids.join(",")}, body = "{}");
    Ok(())
}

/// Unfollow a playlist.
///
/// If the user follows it publicly you need `playlist-modify-public`, if the user follows it
/// privately you need `playlist-modiy-private`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/unfollow-playlist/).
pub async fn unfollow_playlist(
    token: &AccessToken,
    id: &str,
) -> Result<(), EndpointError<Error>> {
    request!(token, DELETE "/v1/playlists/{}/followers", path_params = [id], body = "{}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::endpoints::token;

    #[tokio::test]
    async fn test_follow_artists() {
        // NOTE: this test only works if you follow < 49 artists as it only requests the first page.
        // You also must not follow Lemon Demon.
        let token = token().await;

        // Follow TOTO and Eminem
        follow_artists(&token, &["0PFtn5NtBbbUNbU9EAmIWF", "7dGJo4pcD2V6oG8kP0tJRR"]).await.unwrap();

        // Get followed artists
        let following = get_followed_artists(&token, 50, None).await.unwrap().items;
        assert!(following.iter().any(|artist| artist.name == "TOTO"));
        assert!(following.iter().any(|artist| artist.name == "Eminem"));

        // Check if you follow those artists
        let follows = user_follows_artists(&token, &["0PFtn5NtBbbUNbU9EAmIWF", "4llAOeA6kEF4ytaB2fsmcW"]).await.unwrap();
        assert_eq!(follows, &[true, false]);

        // Unfollow TOTO and Eminem
        unfollow_artists(&token, &["0PFtn5NtBbbUNbU9EAmIWF", "7dGJo4pcD2V6oG8kP0tJRR"]).await.unwrap();

        // Get followed artists
        let following = get_followed_artists(&token, 50, None).await.unwrap().items;
        assert!(following.iter().all(|artist| artist.name != "TOTO"));
        assert!(following.iter().all(|artist| artist.name != "Eminem"));
    }

    #[tokio::test]
    async fn test_follow_playlists() {
        let token = token().await;

        // Follow "Sing-Along Indie Hits" playlist
        follow_playlist_public(&token, "37i9dQZF1DWYBF1dYDPlHw").await.unwrap();

        // Check whether following playlist
        // TODO: Check whether current user follows this
        let followers = users_follow_playlist(&token, "37i9dQZF1DWYBF1dYDPlHw", &["spotify"]).await.unwrap();
        assert_eq!(followers, &[false]);

        // Unfollow
        unfollow_playlist(&token, "37i9dQZF1DWYBF1dYDPlHw").await.unwrap();
    }
}
