//! Endpoint functions relating to shows.
//!
//! For all the below endpoints, the market parameter must be specified if the token is not a
//! user's. If the token is a user's and the market parameter is specified, the user's token will
//! take precedence.
//!
//! **Note**: The `get_show` and `get_shows` endpoints can break in some cases due to an
//! undocumented feature in the Spotify API. In particular, the Spotify API claims to return ISO 639
//! language codes as the languages, however in practice it returns something different.

use crate::*;
use serde::Deserialize;

/// Get information about a show.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/shows/get-a-show/).
pub async fn get_show(
    token: &AccessToken,
    id: &str,
    market: Option<CountryCode>,
) -> Result<Show, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/shows/{}",
        path_params = [id],
        optional_query_params = {"market": market.map(|m| m.alpha2())},
        ret = Show
    ))
}

/// Get several shows.
///
/// In theory, this endpoint should return `Show`s; however, in practice it returns
/// `ShowSimplified`s.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/shows/get-several-shows/).
pub async fn get_shows(
    token: &AccessToken,
    ids: &[&str],
    market: Option<CountryCode>,
) -> Result<Vec<ShowSimplified>, EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    #[derive(Deserialize)]
    struct Shows {
        shows: Vec<ShowSimplified>,
    }

    Ok(request!(
        token,
        GET "/v1/shows",
        query_params = {"ids": ids.join(",")},
        optional_query_params = {"market": market.map(|m| m.alpha2())},
        ret = Shows
    )
    .shows)
}

/// Get a show's episodes.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/shows/get-shows-episodes/).
pub async fn get_show_episodes(
    token: &AccessToken,
    id: &str,
    limit: usize,
    offset: usize,
    market: Option<CountryCode>,
) -> Result<Page<EpisodeSimplified>, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/shows/{}/episodes",
        path_params = [id],
        query_params = {"limit": limit, "offset": offset},
        optional_query_params = {"market": market.map(|m| m.alpha2())},
        ret = Page<EpisodeSimplified>
    ))
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;

    #[tokio::test]
    async fn test_get_show() {
        let show = get_show(
            &token().await,
            "38bS44xjbVVZ3No3ByF1dJ",
            Some(CountryCode::AUS),
        )
        .await
        .unwrap();
        assert_eq!(show.name, "Vetenskapsradion Historia");
    }

    #[tokio::test]
    async fn test_get_shows() {
        let shows = get_shows(&token().await, &["5CfCWKI5pZ28U0uOzXkDHe"], None)
            .await
            .unwrap();
        assert_eq!(shows.len(), 1);
        assert_eq!(shows[0].name, "Without Fail");
    }

    #[tokio::test]
    async fn test_get_show_episodes() {
        let episodes = get_show_episodes(&token().await, "38bS44xjbVVZ3No3ByF1dJ", 2, 1, None)
            .await
            .unwrap();
        assert_eq!(episodes.limit, 2);
        assert_eq!(episodes.offset, 1);
        assert_eq!(episodes.items.len(), 2);
    }
}
