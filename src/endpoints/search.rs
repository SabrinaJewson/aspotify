use itertools::Itertools;

use crate::{Client, Error, ItemType, Market, Response, SearchResults};

/// Endpoint functions related to searches.
#[derive(Debug, Clone, Copy)]
pub struct Search<'a>(pub &'a Client);

impl Search<'_> {
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
        self,
        query: &str,
        types: impl IntoIterator<Item = ItemType>,
        include_external: bool,
        limit: usize,
        offset: usize,
        market: Option<Market>,
    ) -> Result<Response<SearchResults>, Error> {
        let types = types.into_iter().map(ItemType::as_str).join(",");
        let types = if types.is_empty() {
            "album,artist,playlist,track,show,episode"
        } else {
            &types
        };

        self.0
            .send_json(self.0.client.get(endpoint!("/v1/search")).query(&(
                ("q", query),
                ("type", types),
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
                if include_external {
                    Some(("include_external", "audio"))
                } else {
                    None
                },
                market.map(Market::query),
            )))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::client;
    use crate::{ItemType, Market};

    #[tokio::test]
    async fn test_search_artist() {
        let res = client()
            .search()
            .search(
                "tania bowra",
                [ItemType::Artist].iter().copied(),
                false,
                1,
                0,
                None,
            )
            .await
            .unwrap()
            .data;
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
        client()
            .search()
            .search(
                "abba",
                [ItemType::Album, ItemType::Track].iter().copied(),
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
        client()
            .search()
            .search(
                "doom metal",
                [ItemType::Playlist].iter().copied(),
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
        client()
            .search()
            .search("test", [].iter().copied(), false, 3, 2, None)
            .await
            .unwrap();
    }
}
