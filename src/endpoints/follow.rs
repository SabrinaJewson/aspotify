//! Endpoint functions relating to following and unfollowing artists, users and playlists.

use crate::*;
use serde::Deserialize;

/// Check if the current user follows some artists.
///
/// Returns vector of bools that is in the same order as the given ids. Maximum 50 ids. Requires
/// `user-follow-read`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/check-current-user-follows/).
pub async fn user_follows_artists(
    token: &AccessToken,
    ids: &[&str],
) -> Result<Vec<bool>, EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(request!(
        token,
        GET "/v1/me/following/contains",
        query_params = {"type": "artist", "ids": &&ids.join(",")},
        ret = Vec<bool>
    ))
}

/// Check if the current user follows some users.
///
/// Return vector of bools that is in the same order as the given ids. Maximum 50 ids. Requires
/// `user-follow-read`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/check-current-user-follows/).
pub async fn user_follows_users(
    token: &AccessToken,
    ids: &[&str],
) -> Result<Vec<bool>, EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(request!(
        token,
        GET "/v1/me/following/contains",
        query_params = {"type": "user", "ids": &&ids.join(",")},
        ret = Vec<bool>
    ))
}

/// Check if some users follow a playlist.
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
    if user_ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(request!(
        token,
        GET "/v1/playlists/{}/followers/contains",
        path_params = [id],
        query_params = {"ids": &&user_ids.join(",")},
        ret = Vec<bool>
    ))
}

/// Follow artists.
///
/// Maximum 50 ids. Requires `user-follow-modify`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/follow-artists-users/).
pub async fn follow_artists(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(());
    }

    request!(
        token,
        PUT "/v1/me/following",
        query_params = {"type": "artist", "ids": &ids.join(",")},
        body = "{}"
    );
    Ok(())
}

/// Follow users.
///
/// Maximum 50 ids. Requires `user-follow-modify`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/follow-artists-users/).
pub async fn follow_users(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(());
    }

    request!(
        token,
        PUT "/v1/me/following",
        query_params = {"type": "user", "ids": &ids.join(",")},
        body = "{}"
    );
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
    request!(
        token,
        PUT "/v1/playlists/{}/followers",
        path_params = [id],
        header_params = {"Content-Type": "application/json"},
        body = "{\"public\": true}"
    );
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
    request!(
        token,
        PUT "/v1/playlists/{}/followers",
        path_params = [id],
        header_params = {"Content-Type": "application/json"},
        body = "{\"public\": false}"
    );
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

    Ok(request!(
        token,
        GET "/v1/me/following",
        query_params = {"type": "artist", "limit": &limit.to_string()},
        optional_query_params = {"after": after},
        ret = Response
    )
    .artists)
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
    if ids.is_empty() {
        return Ok(());
    }

    request!(
        token,
        DELETE "/v1/me/following",
        query_params = {"type": "artist", "ids": &ids.join(",")},
        body = "{}"
    );
    Ok(())
}

/// Unfollow users.
///
/// Maximum 50 ids. Requires `user-follow-modify`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/unfollow-artists-users/).
pub async fn unfollow_users(token: &AccessToken, ids: &[&str]) -> Result<(), EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(());
    }

    request!(
        token,
        DELETE "/v1/me/following",
        query_params = {"type": "user", "ids": &ids.join(",")},
        body = "{}"
    );
    Ok(())
}

/// Unfollow a playlist.
///
/// If the user follows it publicly you need `playlist-modify-public`, if the user follows it
/// privately you need `playlist-modiy-private`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/unfollow-playlist/).
pub async fn unfollow_playlist(token: &AccessToken, id: &str) -> Result<(), EndpointError<Error>> {
    request!(
        token,
        DELETE "/v1/playlists/{}/followers",
        path_params = [id],
        body = "{}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;

    #[tokio::test]
    async fn test_follow_artists() {
        // NOTE: This test only works if you follow < 49 artists as it only requests the first page.
        // You also must not follow Lemon Demon.
        let token = token().await;

        // TOTO, Eminem and Lemon Demon
        let artists = &[
            "0PFtn5NtBbbUNbU9EAmIWF",
            "7dGJo4pcD2V6oG8kP0tJRR",
            "4llAOeA6kEF4ytaB2fsmcW",
        ];
        let split = 2;
        let (followed_artists, unfollowed_artists) = artists.split_at(split);

        // Store old
        let old = user_follows_artists(&token, artists).await.unwrap();

        // Following and unfollowing
        follow_artists(&token, followed_artists).await.unwrap();
        unfollow_artists(&token, unfollowed_artists).await.unwrap();

        // Check
        let check = user_follows_artists(&token, artists).await.unwrap();
        let (follow_check, unfollow_check) = check.split_at(split);
        assert!(follow_check.iter().all(|&followed| followed));
        assert!(unfollow_check.iter().all(|&followed| !followed));

        // Check by finding in list
        let followed = get_followed_artists(&token, 50, None).await.unwrap();
        if followed.total <= 50 {
            for followed_artist in followed_artists {
                assert!(followed
                    .items
                    .iter()
                    .any(|artist| artist.id == *followed_artist));
            }
            for unfollowed_artist in unfollowed_artists {
                assert!(followed
                    .items
                    .iter()
                    .all(|artist| artist.id != *unfollowed_artist));
            }
        }

        // Restore
        let mut old_followed = Vec::with_capacity(artists.len());
        let mut old_unfollowed = Vec::with_capacity(artists.len());
        for i in 0..artists.len() {
            if old[i] {
                &mut old_followed
            } else {
                &mut old_unfollowed
            }
            .push(artists[i]);
        }
        follow_artists(&token, &old_followed).await.unwrap();
        unfollow_artists(&token, &old_unfollowed).await.unwrap();
    }

    #[tokio::test]
    async fn test_follow_playlists() {
        let token = token().await;

        // Follow "Sing-Along Indie Hits" playlist
        follow_playlist_public(&token, "37i9dQZF1DWYBF1dYDPlHw")
            .await
            .unwrap();

        // Check whether following playlist
        let id = get_current_user(&token).await.unwrap().id;
        let followers = users_follow_playlist(&token, "37i9dQZF1DWYBF1dYDPlHw", &["spotify", &id])
            .await
            .unwrap();
        assert_eq!(followers, &[false, true]);

        // Unfollow
        unfollow_playlist(&token, "37i9dQZF1DWYBF1dYDPlHw")
            .await
            .unwrap();
    }
}
