use std::fmt::Display;

use itertools::Itertools;
use serde::Deserialize;

use super::chunked_sequence;
use crate::{Client, CountryCode, Episode, Error, Response};

/// Endpoint functions relating to episodes.
///
/// For all the below endpoints, the market parameter must be specified if a refresh token is not
/// provided. If a refresh token is provided and the market parameter is specified, the user's
/// market will take precedence.
#[derive(Debug, Clone, Copy)]
pub struct Episodes<'a>(pub &'a Client);

impl Episodes<'_> {
    /// Get information about an episode.
    ///
    /// Reading the user's playback points requires `user-read-playback-position`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/episodes/get-an-episode/).
    pub async fn get_episode(
        self,
        id: &str,
        market: Option<CountryCode>,
    ) -> Result<Response<Episode>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/episodes/{}", id))
                    .query(&(market.map(|c| ("market", c.alpha2())),)),
            )
            .await
    }

    /// Get information about several episodes.
    ///
    /// Reading the user's playback points requires `user-read-playback-position`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/episodes/get-several-episodes/).
    pub async fn get_episodes<I: IntoIterator>(
        self,
        ids: I,
        market: Option<CountryCode>,
    ) -> Result<Response<Vec<Option<Episode>>>, Error>
    where
        I::Item: Display,
    {
        #[derive(Deserialize)]
        struct Episodes {
            episodes: Vec<Option<Episode>>,
        }

        chunked_sequence(ids, 50, |mut ids| {
            let req = self.0.client.get(endpoint!("/v1/episodes")).query(&(
                ("ids", ids.join(",")),
                market.map(|m| ("market", m.alpha2())),
            ));
            async move {
                Ok(self
                    .0
                    .send_json::<Episodes>(req)
                    .await?
                    .map(|res| res.episodes))
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use isocountry::CountryCode;

    use crate::endpoints::client;

    #[tokio::test]
    async fn test_get_episode() {
        let episode = client()
            .episodes()
            .get_episode("512ojhOuo1ktJprKbVcKyQ", Some(CountryCode::ESP))
            .await
            .unwrap()
            .data;
        assert_eq!(episode.name, "Tredje rikets knarkande granskas");
    }

    #[tokio::test]
    async fn test_get_episodes() {
        let episodes = client()
            .episodes()
            .get_episodes(
                &["77o6BIVlYM3msb4MMIL1jH", "0Q86acNRm6V9GYx55SXKwf"],
                Some(CountryCode::CHL),
            )
            .await
            .unwrap()
            .data;

        assert_eq!(episodes.len(), 2);

        let mut episodes = episodes.into_iter();
        assert_eq!(
            episodes.next().unwrap().unwrap().name,
            "Riddarnas vapensköldar under lupp"
        );
        assert_eq!(
            episodes.next().unwrap().unwrap().name,
            "Okända katedralen i Dalsland"
        );
    }
}
