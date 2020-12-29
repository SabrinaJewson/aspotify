//! Endpoint functions relating to artists.

use std::fmt::Display;

use itertools::Itertools;
use serde::Deserialize;

use super::chunked_sequence;
use crate::{AlbumGroup, Artist, ArtistsAlbum, Client, Error, Market, Page, Response, Track};

/// Artist-related endpoints.
#[derive(Debug, Clone, Copy)]
pub struct Artists<'a>(pub &'a Client);

impl Artists<'_> {
    /// Get information about an artist.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/artists/get-artist/).
    pub async fn get_artist(self, id: &str) -> Result<Response<Artist>, Error> {
        self.0
            .send_json(self.0.client.get(endpoint!("/v1/artists/{}", id)))
            .await
    }

    /// Get information about several artists.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/artists/get-several-artists/).
    pub async fn get_artists<I: IntoIterator>(self, ids: I) -> Result<Response<Vec<Artist>>, Error>
    where
        I::Item: Display,
    {
        #[derive(Deserialize)]
        struct Artists {
            artists: Vec<Artist>,
        };

        chunked_sequence(ids, 50, |mut ids| {
            let req = self
                .0
                .client
                .get(endpoint!("/v1/artists"))
                .query(&(("ids", ids.join(",")),));
            async move {
                Ok(self
                    .0
                    .send_json::<Artists>(req)
                    .await?
                    .map(|res| res.artists))
            }
        })
        .await
    }

    /// Get an artist's albums.
    ///
    /// The `include_groups` parameter can specify which groups to include (`album`, `single`,
    /// `appears_on`, `compilation`). If not specified it includes them all. Limit and offset
    /// control the attributes of the resulting Page. Limit has a maximum of 50.
    ///
    /// If no market is specified this function is likely to give duplicate albums, one for each
    /// market, so it is advised to provide a market.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/artists/get-artists-albums/).
    pub async fn get_artist_albums(
        self,
        id: &str,
        include_groups: Option<&[AlbumGroup]>,
        limit: usize,
        offset: usize,
        country: Option<Market>,
    ) -> Result<Response<Page<ArtistsAlbum>>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/artists/{}/albums", id))
                    .query(&(
                        ("limit", limit.to_string()),
                        ("offset", offset.to_string()),
                        include_groups.map(|groups| {
                            (
                                "include_groups",
                                groups.iter().map(|group| group.as_str()).join(","),
                            )
                        }),
                        country.map(|m| ("country", m.as_str())),
                    )),
            )
            .await
    }

    /// Get an artist's top tracks.
    ///
    /// Unlike most other endpoints, the country code is required. The response contains up to 10
    /// tracks which are the artist's top tracks.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/artists/get-artists-top-tracks/).
    pub async fn get_artist_top(
        self,
        id: &str,
        market: Market,
    ) -> Result<Response<Vec<Track>>, Error> {
        #[derive(Deserialize)]
        struct Tracks {
            tracks: Vec<Track>,
        };

        Ok(self
            .0
            .send_json::<Tracks>(
                self.0
                    .client
                    .get(endpoint!("/v1/artists/{}/top-tracks", id))
                    .query(&(("country", market.as_str()),)),
            )
            .await?
            .map(|res| res.tracks))
    }

    /// Get an artist's related artists.
    ///
    /// These artists are similar in style to the given artist.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/artists/get-related-artists/).
    pub async fn get_related_artists(self, id: &str) -> Result<Response<Vec<Artist>>, Error> {
        #[derive(Deserialize)]
        struct Artists {
            artists: Vec<Artist>,
        };

        Ok(self
            .0
            .send_json::<Artists>(
                self.0
                    .client
                    .get(endpoint!("/v1/artists/{}/related-artists", id)),
            )
            .await?
            .map(|res| res.artists))
    }
}

#[cfg(test)]
mod tests {
    use isocountry::CountryCode;

    use crate::endpoints::client;
    use crate::{AlbumGroup, Market};

    #[tokio::test]
    async fn test_get_artist() {
        let artist = client()
            .artists()
            .get_artist("0L8ExT028jH3ddEcZwqJJ5")
            .await
            .unwrap()
            .data;
        assert_eq!(artist.id, "0L8ExT028jH3ddEcZwqJJ5");
        assert_eq!(artist.name, "Red Hot Chili Peppers");
    }

    #[tokio::test]
    async fn test_get_artists() {
        let artists = client()
            .artists()
            .get_artists(&["0L8ExT028jH3ddEcZwqJJ5", "0gxyHStUsqpMadRV0Di1Qt"])
            .await
            .unwrap()
            .data;
        assert_eq!(artists.len(), 2);
        assert_eq!(artists[0].name, "Red Hot Chili Peppers");
        assert_eq!(artists[1].name, "Rick Astley");
    }

    #[tokio::test]
    async fn test_get_artist_albums() {
        let albums = client()
            .artists()
            .get_artist_albums(
                "0L8ExT028jH3ddEcZwqJJ5",
                Some(&[AlbumGroup::Single]),
                2,
                1,
                Some(Market::Country(CountryCode::GBR)),
            )
            .await
            .unwrap()
            .data;
        assert_eq!(albums.limit, 2);
        assert_eq!(albums.offset, 1);
        assert_eq!(albums.items.len(), 2);
        assert!(albums
            .items
            .iter()
            .all(|album| album.album_group == AlbumGroup::Single));
        assert!(albums.items.iter().all(|album| album
            .artists
            .iter()
            .any(|artist| artist.name == "Red Hot Chili Peppers")));
    }

    #[tokio::test]
    async fn test_get_artist_top() {
        let top = client()
            .artists()
            .get_artist_top("0L8ExT028jH3ddEcZwqJJ5", Market::Country(CountryCode::GBR))
            .await
            .unwrap()
            .data;
        assert!(top.iter().all(|track| track
            .artists
            .iter()
            .any(|artist| artist.name == "Red Hot Chili Peppers")));
    }
}
