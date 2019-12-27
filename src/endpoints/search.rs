//! The search function.

use crate::*;

/// Search for an item.
///
/// `include_external` specifies whether to include audio content that is hosted externally.
/// Playlist results are not affected by `market`. `limit` must be in the range [1..50], and is
/// applied individually to each type specified in `types`, not the whole response. `offset` has a
/// maximum of 10,000.
///
/// See
/// [here](https://developer.spotify.com/documentation/web-api/reference/search/search/#writing-a-query---guidelines)
/// on how to write a query. The only difference is that you shouldn't encode spaces as `%20` or
/// `+`, as that is done by this function automatically.
///
/// # Limitations
///
/// - You cannot fetch sorted results.
/// - You cannot search for playlists that contain a track.
/// - You can only search for one genre at a time.
/// - You cannot search for playlists in a user's library.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/search/search/).
pub async fn search(
    token: &AccessToken,
    query: &str,
    types: &[ItemType],
    include_external: bool,
    limit: usize,
    offset: usize,
    market: Option<Market>,
) -> Result<SearchResults, EndpointError<Error>> {
    let types = if types.is_empty() {
        &[
            ItemType::Album,
            ItemType::Artist,
            ItemType::Playlist,
            ItemType::Track,
        ]
    } else {
        types
    };

    Ok(request!(
        token,
        GET "/v1/search",
        query_params = {
            "q": query,
            "type": &types.iter().map(|&item| item.as_str()).collect::<Vec<_>>().join(","),
            "limit": &limit.to_string(),
            "offset": &offset.to_string()
        },
        optional_query_params = {
            "include_external": if include_external {
                Some("audio")
            } else {
                None
            },
            "market": market.map(|m| m.as_str())
        },
        ret = SearchResults
    ))
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;

    #[tokio::test]
    async fn test_search_artist() {
        let res = search(
            &token().await,
            "tania bowra",
            &[ItemType::Artist],
            false,
            1,
            0,
            None,
        )
        .await
        .unwrap();
        assert_eq!(res.albums, None);
        assert_eq!(res.tracks, None);
        assert_eq!(res.playlists, None);
        let artists = res.artists.unwrap();
        assert_eq!(artists.limit, 1);
        assert_eq!(artists.offset, 0);
        assert_eq!(artists.items.len(), 1);
        assert_eq!(artists.items[0].name, "Tania Bowra");
    }

    #[tokio::test]
    async fn test_search_album_tracks() {
        search(
            &token().await,
            "abba",
            &[ItemType::Album, ItemType::Track],
            true,
            1,
            0,
            Some(Market::FromToken),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_search_playlist() {
        search(
            &token().await,
            "doom metal",
            &[ItemType::Playlist],
            false,
            1,
            0,
            None,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_search_all() {
        search(&token().await, "test", &[], false, 3, 2, None)
            .await
            .unwrap();
    }
}
