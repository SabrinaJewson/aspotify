use crate::{Artist, Client, Error, Page, Response, TimeRange, Track};

/// Endpoint functions relating to a user's top artists and tracks.
#[derive(Debug, Clone, Copy)]
pub struct Personalization<'a>(pub &'a Client);

impl Personalization<'_> {
    /// Get a user's top artists.
    ///
    /// Requires `user-top-read`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/personalization/get-users-top-artists-and-tracks/).
    pub async fn get_top_artists(
        self,
        limit: usize,
        offset: usize,
        time_range: TimeRange,
    ) -> Result<Response<Page<Artist>>, Error> {
        self.0
            .send_json(self.0.client.get(endpoint!("/v1/me/top/artists")).query(&(
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
                ("time_range", time_range.as_str()),
            )))
            .await
    }

    /// Get a user's top tracks.
    ///
    /// Requires `user-top-read`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/personalization/get-users-top-tracks-and-tracks/).
    pub async fn get_top_tracks(
        self,
        limit: usize,
        offset: usize,
        time_range: TimeRange,
    ) -> Result<Response<Page<Track>>, Error> {
        self.0
            .send_json(self.0.client.get(endpoint!("/v1/me/top/tracks")).query(&(
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
                ("time_range", time_range.as_str()),
            )))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::client;
    use crate::TimeRange;

    #[tokio::test]
    async fn test() {
        let client = client();
        let personalization = client.personalization();

        let top = personalization
            .get_top_artists(5, 2, TimeRange::Short)
            .await
            .unwrap()
            .data;
        assert_eq!(top.limit, 5);
        assert_eq!(top.offset, 2);
        assert!(top.items.len() <= 5);

        let top = personalization
            .get_top_tracks(2, 8, TimeRange::Long)
            .await
            .unwrap()
            .data;
        assert_eq!(top.limit, 2);
        assert_eq!(top.offset, 8);
        assert!(top.items.len() <= 2);
    }
}
