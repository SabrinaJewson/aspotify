use std::fmt::Display;

use chrono::{DateTime, Utc};
use isocountry::CountryCode;
use isolanguage_1::LanguageCode;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    AlbumSimplified, Category, Client, Error, FeaturedPlaylists, Market, Page, PlaylistSimplified,
    Recommendations, Response,
};

/// Endpoint functions related to categories, featured playlists, recommendations, and new
/// releases.
#[derive(Debug, Clone, Copy)]
pub struct Browse<'a>(pub &'a Client);

impl Browse<'_> {
    /// Get information about a category.
    ///
    /// If no locale is given or Spotify does not support the given locale, then it will default to
    /// American English.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-category/).
    pub async fn get_category(
        self,
        name: &str,
        locale: Option<(LanguageCode, CountryCode)>,
        country: Option<CountryCode>,
    ) -> Result<Response<Category>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/browse/categories/{}", name))
                    .query(&(
                        locale.map(|locale| ("locale", format_language(locale))),
                        country.map(|c| ("country", c.alpha2())),
                    )),
            )
            .await
    }

    /// Get information about several categories.
    ///
    /// You do not choose which categories to get. Limit must be in the range [1..50]. If no locale
    /// is given or Spotify does not support the given locale, then it will default to American
    /// English.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-list-categories/).
    pub async fn get_categories(
        self,
        limit: usize,
        offset: usize,
        locale: Option<(LanguageCode, CountryCode)>,
        country: Option<CountryCode>,
    ) -> Result<Response<Page<Category>>, Error> {
        #[derive(Deserialize)]
        struct CategoryPage {
            categories: Page<Category>,
        };

        Ok(self
            .0
            .send_json::<CategoryPage>(self.0.client.get(endpoint!("/v1/browse/categories")).query(
                &(
                    ("limit", limit.to_string()),
                    ("offset", offset.to_string()),
                    locale.map(|l| ("locale", format_language(l))),
                    country.map(|c| ("country", c.alpha2())),
                ),
            ))
            .await?
            .map(|res| res.categories))
    }

    /// Get a category's playlists.
    ///
    /// Limit must be in the range [1..50].
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-categorys-playlists/).
    pub async fn get_category_playlists(
        self,
        name: &str,
        limit: usize,
        offset: usize,
        country: Option<CountryCode>,
    ) -> Result<Response<Page<PlaylistSimplified>>, Error> {
        #[derive(Deserialize)]
        struct Playlists {
            playlists: Page<PlaylistSimplified>,
        };

        Ok(self
            .0
            .send_json::<Playlists>(
                self.0
                    .client
                    .get(endpoint!("/v1/browse/categories/{}/playlists", name))
                    .query(&(
                        ("limit", limit.to_string()),
                        ("offset", offset.to_string()),
                        country.map(|c| ("country", c.alpha2())),
                    )),
            )
            .await?
            .map(|res| res.playlists))
    }

    /// Get featured playlists.
    ///
    /// Limit must be in the range [1..50]. The locale will default to American English and the
    /// timestamp will default to the current UTC time.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-list-featured-playlists/).
    pub async fn get_featured_playlists(
        self,
        limit: usize,
        offset: usize,
        locale: Option<(LanguageCode, CountryCode)>,
        time: Option<DateTime<Utc>>,
        country: Option<CountryCode>,
    ) -> Result<Response<FeaturedPlaylists>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/browse/featured-playlists"))
                    .query(&(
                        ("limit", limit.to_string()),
                        ("offset", offset.to_string()),
                        locale.map(|l| ("locale", format_language(l))),
                        time.map(|t| ("timestamp", t.to_rfc3339())),
                        country.map(|c| ("country", c.alpha2())),
                    )),
            )
            .await
    }

    /// Get new releases.
    ///
    /// Limit must be in the range [1..50]. The documentation claims to also return a message string,
    /// but in reality the API does not.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-list-new-releases/).
    pub async fn get_new_releases(
        self,
        limit: usize,
        offset: usize,
        country: Option<CountryCode>,
    ) -> Result<Response<Page<AlbumSimplified>>, Error> {
        #[derive(Deserialize)]
        struct NewReleases {
            albums: Page<AlbumSimplified>,
        };

        Ok(self
            .0
            .send_json::<NewReleases>(
                self.0
                    .client
                    .get(endpoint!("/v1/browse/new-releases"))
                    .query(&(
                        ("limit", limit.to_string()),
                        ("offset", offset.to_string()),
                        country.map(|c| ("country", c.alpha2())),
                    )),
            )
            .await?
            .map(|res| res.albums))
    }

    /// Get recommendations.
    ///
    /// Up to 5 seed values may be provided, that can be distributed in `seed_artists`,
    /// `seed_genres` and `seed_tracks` in any way. Limit must be in the range [1..100] and this
    /// target number of tracks may not always be met.
    ///
    /// `attributes` must serialize to a string to string map or sequence of key-value tuples. See
    /// the reference for more info on this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-recommendations/).
    pub async fn get_recommendations<AI: IntoIterator, GI: IntoIterator, TI: IntoIterator>(
        self,
        seed_artists: AI,
        seed_genres: GI,
        seed_tracks: TI,
        attributes: &impl Serialize,
        limit: usize,
        market: Option<Market>,
    ) -> Result<Response<Recommendations>, Error>
    where
        AI::Item: Display,
        GI::Item: Display,
        TI::Item: Display,
    {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/recommendations"))
                    .query(&(
                        ("seed_artists", seed_artists.into_iter().join(",")),
                        ("seed_genres", seed_genres.into_iter().join(",")),
                        ("seed_tracks", seed_tracks.into_iter().join(",")),
                        ("limit", limit.to_string()),
                        market.map(Market::query),
                    ))
                    .query(attributes),
            )
            .await
    }
}

fn format_language(locale: (LanguageCode, CountryCode)) -> String {
    format!("{}_{}", locale.0.code(), locale.1.alpha2())
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use isocountry::CountryCode;
    use isolanguage_1::LanguageCode;

    use crate::endpoints::client;
    use crate::{Market, SeedType};

    #[tokio::test]
    async fn test_get_category() {
        let category = client()
            .browse()
            .get_category(
                "pop",
                Some((LanguageCode::En, CountryCode::GBR)),
                Some(CountryCode::GBR),
            )
            .await
            .unwrap()
            .data;
        assert_eq!(category.id, "pop");
        assert_eq!(category.name, "Pop");
    }

    #[tokio::test]
    async fn test_get_categories() {
        let categories = client()
            .browse()
            .get_categories(2, 0, None, None)
            .await
            .unwrap()
            .data;
        assert_eq!(categories.limit, 2);
        assert_eq!(categories.offset, 0);
        assert!(categories.items.len() <= 2);
    }

    #[tokio::test]
    async fn test_get_category_playlists() {
        let playlists = client()
            .browse()
            .get_category_playlists("chill", 1, 3, Some(CountryCode::GBR))
            .await
            .unwrap()
            .data;
        assert_eq!(playlists.limit, 1);
        assert_eq!(playlists.offset, 3);
        assert!(playlists.items.len() <= 1);
    }

    #[tokio::test]
    async fn test_get_featured_playlists() {
        let playlists = client()
            .browse()
            .get_featured_playlists(
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
            .data
            .playlists;
        assert_eq!(playlists.limit, 2);
        assert_eq!(playlists.offset, 0);
        assert!(playlists.items.len() <= 2);
    }

    #[tokio::test]
    async fn test_get_new_releases() {
        let releases = client()
            .browse()
            .get_new_releases(1, 0, None)
            .await
            .unwrap()
            .data;
        assert_eq!(releases.limit, 1);
        assert_eq!(releases.offset, 0);
        assert!(releases.items.len() <= 1);
    }

    #[tokio::test]
    async fn test_get_recommendations() {
        let recommendations = client()
            .browse()
            .get_recommendations(
                &["unused"; 0],
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
            .unwrap()
            .data;
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
