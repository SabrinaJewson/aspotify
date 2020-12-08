use std::fmt::Display;

use isocountry::CountryCode;
use itertools::Itertools;
use serde::Deserialize;

use super::chunked_sequence;
use crate::{Client, EpisodeSimplified, Error, Page, Response, Show, ShowSimplified};

/// Endpoint functions relating to shows.
///
/// For all the below endpoints, the market parameter must be specified if the token is not a
/// user's. If the token is a user's and the market parameter is specified, the user's token will
/// take precedence.
#[derive(Debug, Clone, Copy)]
pub struct Shows<'a>(pub &'a Client);

impl Shows<'_> {
    /// Get information about a show.
    ///
    /// Either the client must have a refresh token or the `market` parameter must be provided,
    /// otherwise this will fail. If both are provided, then the user's market will take
    /// precendence.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/shows/get-a-show/).
    pub async fn get_show(
        self,
        id: &str,
        market: Option<CountryCode>,
    ) -> Result<Response<Show>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/shows/{}", id))
                    .query(&(market.map(|c| ("market", c.alpha2())),)),
            )
            .await
    }

    /// Get several shows.
    ///
    /// Either the client must have a refresh token or the `market` parameter must be provided,
    /// otherwise this will fail. If both are provided, then the user's market will take
    /// precendence.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/shows/get-several-shows/).
    pub async fn get_shows<I: Iterator>(
        self,
        ids: impl IntoIterator<IntoIter = I, Item = I::Item>,
        market: Option<CountryCode>,
    ) -> Result<Response<Vec<ShowSimplified>>, Error>
    where
        I::Item: Display,
    {
        #[derive(Deserialize)]
        struct Shows {
            shows: Vec<ShowSimplified>,
        }

        chunked_sequence(ids, 50, |mut ids| {
            let req = self.0.client.get(endpoint!("/v1/shows")).query(&(
                ("ids", ids.join(",")),
                market.map(|c| ("market", c.alpha2())),
            ));
            async move { Ok(self.0.send_json::<Shows>(req).await?.map(|res| res.shows)) }
        })
        .await
    }

    /// Get a show's episodes.
    ///
    /// Either the client must have a refresh token or the `market` parameter must be provided,
    /// otherwise this will fail. If both are provided, then the user's market will take
    /// precendence.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/shows/get-shows-episodes/).
    pub async fn get_show_episodes(
        self,
        id: &str,
        limit: usize,
        offset: usize,
        market: Option<CountryCode>,
    ) -> Result<Response<Page<EpisodeSimplified>>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/shows/{}/episodes", id))
                    .query(&(
                        ("limit", limit.to_string()),
                        ("offset", offset.to_string()),
                        market.map(|c| ("market", c.alpha2())),
                    )),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use isocountry::CountryCode;

    use crate::endpoints::client;

    #[tokio::test]
    async fn test_get_show() {
        let show = client()
            .shows()
            .get_show("38bS44xjbVVZ3No3ByF1dJ", Some(CountryCode::AUS))
            .await
            .unwrap()
            .data;
        assert_eq!(show.name, "Vetenskapsradion Historia");
    }

    #[tokio::test]
    async fn test_get_shows() {
        let shows = client()
            .shows()
            .get_shows(&["5CfCWKI5pZ28U0uOzXkDHe"], None)
            .await
            .unwrap()
            .data;
        assert_eq!(shows.len(), 1);
        assert_eq!(shows[0].name, "Without Fail");
    }

    #[tokio::test]
    async fn test_get_show_episodes() {
        let episodes = client()
            .shows()
            .get_show_episodes("38bS44xjbVVZ3No3ByF1dJ", 2, 1, None)
            .await
            .unwrap()
            .data;
        assert_eq!(episodes.limit, 2);
        assert_eq!(episodes.offset, 1);
        assert_eq!(episodes.items.len(), 2);
    }
}
