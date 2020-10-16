//! Endpoint functions relating to albums.

use std::fmt::Display;

use itertools::Itertools as _;
use serde::Deserialize;

use super::chunked_sequence;
use crate::{Album, Client, Error, Market, Page, Response, TrackSimplified};

/// Album-related endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Albums<'a>(pub &'a Client);

impl Albums<'_> {
    /// Get information about an album.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/albums/get-album/).
    pub async fn get_album(
        self,
        id: &str,
        market: Option<Market>,
    ) -> Result<Response<Album>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/albums/{}", id))
                    .query(&[market.map(Market::query)]),
            )
            .await
    }

    /// Get information about several albums.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/albums/get-several-albums/).
    pub async fn get_albums<I: Iterator>(
        self,
        ids: impl IntoIterator<IntoIter = I, Item = I::Item>,
        market: Option<Market>,
    ) -> Result<Response<Vec<Album>>, Error>
    where
        I::Item: Display,
    {
        #[derive(Deserialize)]
        struct Albums {
            albums: Vec<Album>,
        }

        chunked_sequence(&ids.into_iter().chunks(20), |mut ids| async move {
            Ok(self
                .0
                .send_json::<Albums>(
                    self.0
                        .client
                        .get(endpoint!("/v1/albums"))
                        .query(&(("ids", ids.join(",")), market.map(Market::query))),
                )
                .await?
                .map(|res| res.albums))
        })
        .await
    }

    /// Get an album's tracks.
    ///
    /// It does not return all the tracks, but a page of tracks. Limit and offset determine
    /// attributes of the page. Limit has a maximum of 50.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/albums/get-albums-tracks/).
    pub async fn get_album_tracks(
        self,
        id: &str,
        limit: usize,
        offset: usize,
        market: Option<Market>,
    ) -> Result<Response<Page<TrackSimplified>>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/albums/{}/tracks", id))
                    .query(&(
                        ("limit", limit),
                        ("offset", offset),
                        market.map(Market::query),
                    )),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::client;

    #[tokio::test]
    async fn test_get_album() {
        let album = client()
            .albums()
            .get_album("03JPFQvZRnHHysSZrSFmKY", None)
            .await
            .unwrap()
            .data;
        assert_eq!(album.name, "Inside In / Inside Out");
        assert_eq!(album.artists.len(), 1);
        assert_eq!(album.artists[0].name, "The Kooks");
        assert_eq!(album.tracks.total, 14);
        assert_eq!(album.tracks.items[0].name, "Seaside");
    }

    #[tokio::test]
    async fn test_get_albums() {
        let albums = client()
            .albums()
            .get_albums(&["29Xikj6r9kQDSbnZWCCW2s", "0axbvqBOAejn8DgTUcJAp1"], None)
            .await
            .unwrap()
            .data;
        assert_eq!(albums.len(), 2);
        assert_eq!(albums[0].name, "Neotheater");
        assert_eq!(albums[1].name, "Absentee");
    }

    #[tokio::test]
    async fn test_get_album_tracks() {
        let tracks = client()
            .albums()
            .get_album_tracks("62U7xIHcID94o20Of5ea4D", 3, 1, None)
            .await
            .unwrap()
            .data;
        assert_eq!(tracks.limit, 3);
        assert_eq!(tracks.total, 10);
        assert_eq!(tracks.offset, 1);
        assert_eq!(tracks.items.len(), 3);
        assert_eq!(tracks.items[0].name, "Make Believe");
        assert_eq!(tracks.items[1].name, "I Won't Hold You Back");
        assert_eq!(tracks.items[2].name, "Good for You");
    }
}
