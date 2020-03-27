//! Endpoint functions relating to episodes.
//!
//! For all the below endpoints, the market parameter must be specified if the token is not a
//! user's. If the token is a user's and the market parameter is specified, the user's token will
//! take precedence.

use crate::*;
use serde::Deserialize;

/// Get information about an episode.
///
/// Reading the user's playback points requires `user-read-playback-position`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/episodes/get-an-episode/).
pub async fn get_episode(
    token: &AccessToken,
    id: &str,
    market: Option<CountryCode>,
) -> Result<Episode, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/episodes/{}",
        path_params = [id],
        optional_query_params = {"market": market.map(|m| m.alpha2())},
        ret = Episode
    ))
}

/// Get information about several episodes.
///
/// Maximum 50 IDs. Reading the user's playback points requires `user-read-playback-position`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/episodes/get-several-episodes/).
pub async fn get_episodes(
    token: &AccessToken,
    ids: &[&str],
    market: Option<CountryCode>,
) -> Result<Vec<Episode>, EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    #[derive(Deserialize)]
    struct Episodes {
        episodes: Vec<Episode>,
    }

    Ok(request!(
        token,
        GET "/v1/episodes",
        query_params = {"ids": ids.join(",")},
        optional_query_params = {"market": market.map(|m| m.alpha2())},
        ret = Episodes
    )
    .episodes)
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;

    #[tokio::test]
    async fn test_get_episode() {
        let episode = get_episode(
            &token().await,
            "512ojhOuo1ktJprKbVcKyQ",
            Some(CountryCode::ESP),
        )
        .await
        .unwrap();
        assert_eq!(episode.name, "Tredje rikets knarkande granskas");
    }

    #[tokio::test]
    async fn test_get_episodes() {
        let episodes = get_episodes(
            &token().await,
            &["77o6BIVlYM3msb4MMIL1jH", "0Q86acNRm6V9GYx55SXKwf"],
            Some(CountryCode::SOM),
        )
        .await
        .unwrap();
        assert_eq!(episodes.len(), 2);
        assert_eq!(episodes[0].name, "Riddarnas vapensköldar under lupp");
        assert_eq!(episodes[1].name, "Okända katedralen i Dalsland");
    }
}
