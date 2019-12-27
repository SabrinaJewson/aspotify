//! Endpoint functions relating to a user's top artists and tracks.

use crate::*;

/// Get a user's top artists.
///
/// Requires `user-top-read`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/personalization/get-users-top-artists-and-tracks/).
pub async fn get_top_artists(
    token: &AccessToken,
    limit: usize,
    offset: usize,
    time_range: TimeRange,
) -> Result<Page<Artist>, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/me/top/artists",
        query_params = {"limit": &limit.to_string()[..], "offset": &offset.to_string()[..], "time_range": time_range.as_str()},
        ret = Page<Artist>
    ))
}

/// Get a user's top tracks.
///
/// Requires `user-top-read`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/personalization/get-users-top-tracks-and-tracks/).
pub async fn get_top_tracks(
    token: &AccessToken,
    limit: usize,
    offset: usize,
    time_range: TimeRange,
) -> Result<Page<Track>, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/me/top/tracks",
        query_params = {"limit": &limit.to_string()[..], "offset": &offset.to_string()[..], "time_range": time_range.as_str()},
        ret = Page<Track>
    ))
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;

    #[tokio::test]
    async fn test() {
        let token = token().await;

        let top = get_top_artists(&token, 5, 2, TimeRange::ShortTerm)
            .await
            .unwrap();
        assert_eq!(top.limit, 5);
        assert_eq!(top.offset, 2);
        assert!(top.items.len() <= 5);

        let top = get_top_tracks(&token, 2, 8, TimeRange::LongTerm)
            .await
            .unwrap();
        assert_eq!(top.limit, 2);
        assert_eq!(top.offset, 8);
        assert!(top.items.len() <= 2);
    }
}
