use std::fmt::Display;

use itertools::Itertools;
use serde::Deserialize;

use super::chunked_sequence;
use crate::{AudioAnalysis, AudioFeatures, Client, Error, Market, Response, Track};

/// Endpoint functions related to tracks and audio analysis.
#[derive(Debug, Clone, Copy)]
pub struct Tracks<'a>(pub &'a Client);

impl Tracks<'_> {
    /// Get audio analysis for a track.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-analysis/).
    pub async fn get_analysis(self, id: &str) -> Result<Response<AudioAnalysis>, Error> {
        self.0
            .send_json(self.0.client.get(endpoint!("/v1/audio-analysis/{}", id)))
            .await
    }

    /// Get audio features for a track.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-features/).
    pub async fn get_features_track(self, id: &str) -> Result<Response<AudioFeatures>, Error> {
        self.0
            .send_json(self.0.client.get(endpoint!("/v1/audio-features/{}", id)))
            .await
    }

    /// Get audio features for several tracks.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-audio-features/).
    pub async fn get_features_tracks<I: Iterator>(
        self,
        ids: impl IntoIterator<IntoIter = I, Item = I::Item>,
    ) -> Result<Response<Vec<AudioFeatures>>, Error>
    where
        I::Item: Display,
    {
        #[derive(Deserialize)]
        struct ManyAudioFeatures {
            audio_features: Vec<AudioFeatures>,
        }

        chunked_sequence(ids, 100, |mut ids| {
            let req = self
                .0
                .client
                .get(endpoint!("/v1/audio-features"))
                .query(&(("ids", ids.join(",")),));
            async move {
                Ok(self
                    .0
                    .send_json::<ManyAudioFeatures>(req)
                    .await?
                    .map(|res| res.audio_features))
            }
        })
        .await
    }

    /// Get information about several tracks.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-tracks/).
    pub async fn get_tracks<I: Iterator>(
        self,
        ids: impl IntoIterator<IntoIter = I, Item = I::Item>,
        market: Option<Market>,
    ) -> Result<Response<Vec<Track>>, Error>
    where
        I::Item: Display,
    {
        #[derive(Deserialize)]
        struct Tracks {
            tracks: Vec<Track>,
        };

        chunked_sequence(ids, 50, |mut ids| {
            let req = self
                .0
                .client
                .get(endpoint!("/v1/tracks"))
                .query(&(("ids", ids.join(",")), market.map(Market::query)));
            async move { Ok(self.0.send_json::<Tracks>(req).await?.map(|res| res.tracks)) }
        })
        .await
    }

    /// Get information about a track.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-tracks/).
    pub async fn get_track(
        self,
        id: &str,
        market: Option<Market>,
    ) -> Result<Response<Track>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/tracks/{}", id))
                    .query(&(market.map(Market::query),)),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use isocountry::CountryCode;

    use crate::endpoints::client;
    use crate::{Market, Mode};

    #[tokio::test]
    async fn test_get_track() {
        // "Walk Like an Egyptian"
        let track = client()
            .tracks()
            .get_track("1Jwc3ODLQxtbnS8M9TflSP", None)
            .await
            .unwrap()
            .data;
        assert_eq!(track.id.unwrap(), "1Jwc3ODLQxtbnS8M9TflSP");
        assert_eq!(track.name, "Walk Like an Egyptian");
        assert_eq!(track.artists[0].name, "The Bangles");
    }

    #[tokio::test]
    async fn test_get_tracks() {
        // "Walk Like an Egyptian", "Play that Funky Music"
        let tracks = client()
            .tracks()
            .get_tracks(&["1Jwc3ODLQxtbnS8M9TflSP", "5uuJruktM9fMdN9Va0DUMl"], None)
            .await
            .unwrap()
            .data;
        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[0].name, "Walk Like an Egyptian");
        assert_eq!(tracks[1].name, "Play That Funky Music");
    }

    #[tokio::test]
    async fn test_relink() {
        // Test track relinking with "Heaven and Hell"
        let relinked = client()
            .tracks()
            .get_track(
                "6kLCHFM39wkFjOuyPGLGeQ",
                Some(Market::Country(CountryCode::USA)),
            )
            .await
            .unwrap()
            .data;
        assert_eq!(relinked.name, "Heaven and Hell");
        assert!(relinked.is_playable.unwrap());
        let from = relinked.linked_from.as_ref().unwrap();
        assert_eq!(from.id, "6kLCHFM39wkFjOuyPGLGeQ");
    }

    #[tokio::test]
    async fn test_analysis() {
        // Get analysis of "Walk Like an Egyptian"
        client()
            .tracks()
            .get_analysis("1Jwc3ODLQxtbnS8M9TflSP")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_features() {
        // Get features of "Walk Like an Egyptian"
        let features = client()
            .tracks()
            .get_features_track("1Jwc3ODLQxtbnS8M9TflSP")
            .await
            .unwrap()
            .data;
        assert_eq!(features.id, "1Jwc3ODLQxtbnS8M9TflSP");
        assert_eq!(features.key, 11);
        assert_eq!(features.mode, Mode::Major);
        assert_eq!(features.tempo, 103.022);
    }

    #[tokio::test]
    async fn test_features_tracks() {
        // Get features of "Walk Like an Egyptian" and "Play that Funky Music"
        let features = client()
            .tracks()
            .get_features_tracks(&["1Jwc3ODLQxtbnS8M9TflSP", "5uuJruktM9fMdN9Va0DUMl"])
            .await
            .unwrap()
            .data;
        assert_eq!(features.len(), 2);
        assert_eq!(features[0].id, "1Jwc3ODLQxtbnS8M9TflSP");
        assert_eq!(features[1].id, "5uuJruktM9fMdN9Va0DUMl");
    }
}
