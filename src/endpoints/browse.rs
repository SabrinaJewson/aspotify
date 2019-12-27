//! Endpoint functions related to categories, featured playlists, recommendations, and new
//! releases.

use crate::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

fn format_language(locale: (LanguageCode, CountryCode)) -> String {
    format!("{}_{}", locale.0.code(), locale.1.alpha2())
}

/// Get information about a category.
///
/// If no locale is given or Spotify does not support the given locale, then it will default to
/// American English.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-category/).
pub async fn get_category(
    token: &AccessToken,
    name: &str,
    locale: Option<(LanguageCode, CountryCode)>,
    country: Option<CountryCode>,
) -> Result<Category, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/browse/categories/{}",
        path_params = [name],
        optional_query_params = {"locale": locale.map(format_language), "country": country.map(|c| c.alpha2())},
        ret = Category
    ))
}

/// Get information about several categories.
///
/// You do not choose which categories to get. Limit must be in the range [1..50]. If no locale is
/// given or Spotify does not support the given locale, then it will default to American English.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-list-categories/).
pub async fn get_categories(
    token: &AccessToken,
    limit: usize,
    offset: usize,
    locale: Option<(LanguageCode, CountryCode)>,
    country: Option<CountryCode>,
) -> Result<Page<Category>, EndpointError<Error>> {
    #[derive(Deserialize)]
    struct CategoryPage {
        categories: Page<Category>,
    };

    Ok(request!(
        token,
        GET "/v1/browse/categories",
        query_params = {"limit": limit.to_string(), "offset": offset.to_string()},
        optional_query_params = {"locale": locale.map(format_language), "country": country.map(|c| c.alpha2())},
        ret = CategoryPage
    ).categories)
}

/// Get a category's playlists.
///
/// Limit must be in the range [1..50].
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-categorys-playlists/).
pub async fn get_category_playlists(
    token: &AccessToken,
    name: &str,
    limit: usize,
    offset: usize,
    country: Option<CountryCode>,
) -> Result<Page<PlaylistSimplified>, EndpointError<Error>> {
    #[derive(Deserialize)]
    struct Playlists {
        playlists: Page<PlaylistSimplified>,
    };

    Ok(request!(
        token,
        GET "/v1/browse/categories/{}/playlists",
        path_params = [name],
        query_params = {"limit": limit.to_string(), "offset": offset.to_string()},
        optional_query_params = {"country": country.map(|c| c.alpha2())},
        ret = Playlists
    )
    .playlists)
}

/// Get featured playlists.
///
/// Limit must be in the range [1..50]. The locale will default to American English and the
/// timestamp will default to the current UTC time.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-list-featured-playlists/).
pub async fn get_featured_playlists(
    token: &AccessToken,
    limit: usize,
    offset: usize,
    locale: Option<(LanguageCode, CountryCode)>,
    time: Option<DateTime<Utc>>,
    country: Option<CountryCode>,
) -> Result<FeaturedPlaylists, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/browse/featured-playlists",
        query_params = {"limit": limit.to_string(), "offset": offset.to_string()},
        optional_query_params = {"locale": locale.map(format_language), "timestamp": time.map(|t| t.to_rfc3339()), "country": country.map(|c| c.alpha2())},
        ret = FeaturedPlaylists
    ))
}

/// Get new releases.
///
/// Limit must be in the range [1..50]. The documentation claims to also return a message string,
/// but in reality the API does not.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-list-new-releases/).
pub async fn get_new_releases(
    token: &AccessToken,
    limit: usize,
    offset: usize,
    country: Option<CountryCode>,
) -> Result<Page<AlbumSimplified>, EndpointError<Error>> {
    #[derive(Deserialize)]
    struct NewReleases {
        albums: Page<AlbumSimplified>,
    };

    Ok(request!(
        token,
        GET "/v1/browse/new-releases",
        query_params = {"limit": limit.to_string(), "offset": offset.to_string()},
        optional_query_params = {"country": country.map(|c| c.alpha2())},
        ret = NewReleases
    )
    .albums)
}

/// Get recommendations.
///
/// Up to 5 seed values may be provided, that can be distributed in seed_artists, seed_genres and
/// seed_tracks in any way. Limit must be in the range [1..100] and this target number of tracks
/// may not always be met.
///
/// `attributes` is a map of keys and values. See the reference for more info on this.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-recommendations/).
pub async fn get_recommendations<'iter, I>(
    token: &AccessToken,
    seed_artists: &[&str],
    seed_genres: &[&str],
    seed_tracks: &[&str],
    attributes: I,
    limit: usize,
    market: Option<Market>,
) -> Result<Recommendations, EndpointError<Error>>
where
    I: IntoIterator<Item = &'iter (&'iter str, &'iter str)>,
{
    Ok(request!(
        token,
        GET "/v1/recommendations",
        query_params = {"seed_artists": seed_artists.join(","), "seed_genres": seed_genres.join(","), "seed_tracks": seed_tracks.join(","), "limit": limit.to_string()},
        optional_query_params = {"market": market.map(|m| m.as_str())},
        additional_query_params = attributes,
        ret = Recommendations
    ))
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;
    use chrono::DateTime;

    #[tokio::test]
    async fn test_get_category() {
        let category = get_category(
            &token().await,
            "pop",
            Some((LanguageCode::En, CountryCode::GBR)),
            Some(CountryCode::GBR),
        )
        .await
        .unwrap();
        assert_eq!(category.id, "pop");
        assert_eq!(category.name, "Pop");
    }

    #[tokio::test]
    async fn test_get_categories() {
        let categories = get_categories(&token().await, 2, 0, None, None)
            .await
            .unwrap();
        assert_eq!(categories.limit, 2);
        assert_eq!(categories.offset, 0);
        assert!(categories.items.len() <= 2);
    }

    #[tokio::test]
    async fn test_get_category_playlists() {
        let playlists =
            get_category_playlists(&token().await, "chill", 1, 3, Some(CountryCode::GBR))
                .await
                .unwrap();
        assert_eq!(playlists.limit, 1);
        assert_eq!(playlists.offset, 3);
        assert!(playlists.items.len() <= 1);
    }

    #[tokio::test]
    async fn test_get_featured_playlists() {
        let playlists = get_featured_playlists(
            &token().await,
            2,
            0,
            None,
            Some(
                DateTime::parse_from_rfc3339("2015-05-02T19:25:47Z")
                    .unwrap()
                    .into(),
            ),
            None,
        )
        .await
        .unwrap()
        .playlists;
        assert_eq!(playlists.limit, 2);
        assert_eq!(playlists.offset, 0);
        assert!(playlists.items.len() <= 2);
    }

    #[tokio::test]
    async fn test_get_new_releases() {
        let releases = get_new_releases(&token().await, 1, 0, None).await.unwrap();
        assert_eq!(releases.limit, 1);
        assert_eq!(releases.offset, 0);
        assert!(releases.items.len() <= 1);
    }

    #[tokio::test]
    async fn test_get_recommendations() {
        let recommendations = get_recommendations(
            &token().await,
            &[],
            &["rock"],
            &["2RTkebdbPFyg4AMIzJZql1", "6fTt0CH2t0mdeB2N9XFG5r"],
            &[
                ("max_acousticness", "0.8"),
                ("min_loudness", "-40"),
                ("target_popularity", "100"),
            ],
            3,
            Some(Market::Country(CountryCode::GBR)),
        )
        .await
        .unwrap();
        assert!(recommendations.seeds.len() <= 3);
        assert_eq!(
            recommendations
                .seeds
                .iter()
                .filter(|seed| seed.entity_type == SeedType::Artist)
                .count(),
            0
        );
        assert_eq!(
            recommendations
                .seeds
                .iter()
                .filter(|seed| seed.entity_type == SeedType::Genre)
                .count(),
            1
        );
        assert_eq!(
            recommendations
                .seeds
                .iter()
                .filter(|seed| seed.entity_type == SeedType::Track)
                .count(),
            2
        );
        assert!(recommendations.tracks.len() <= 3);
    }
}
