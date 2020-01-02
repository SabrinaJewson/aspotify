//! Endpoint functions related to tracks and audio analysis.

use crate::*;
use serde::Deserialize;

/// Get audio analysis for a track.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-analysis/).
pub async fn get_audio_analysis_track(
    token: &AccessToken,
    id: &str,
) -> Result<AudioAnalysis, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/audio-analysis/{}",
        path_params = [id],
        ret = AudioAnalysis
    ))
}

/// Get audio features for a track.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-features/).
pub async fn get_audio_features_track(
    token: &AccessToken,
    id: &str,
) -> Result<AudioFeatures, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/audio-features/{}",
        path_params = [id],
        ret = AudioFeatures
    ))
}

/// Get audio features for several tracks.
///
/// Maximum 100 ids.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-audio-features/).
pub async fn get_audio_features_tracks(
    token: &AccessToken,
    ids: &[&str],
) -> Result<Vec<AudioFeatures>, EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    #[derive(Deserialize)]
    struct ManyAudioFeatures {
        audio_features: Vec<AudioFeatures>,
    }

    Ok(request!(
        token,
        GET "/v1/audio-features",
        query_params = {"ids": ids.join(",")},
        ret = ManyAudioFeatures
    )
    .audio_features)
}

/// Get information about several tracks.
///
/// Maximum 50 ids.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-tracks/).
pub async fn get_tracks(
    token: &AccessToken,
    ids: &[&str],
    market: Option<Market>,
) -> Result<Vec<Track>, EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    #[derive(Deserialize)]
    struct Tracks {
        tracks: Vec<Track>,
    };

    Ok(request!(
        token,
        GET "/v1/tracks",
        query_params = {"ids": ids.join(",")},
        optional_query_params = {"market": market.map(|m| m.as_str())},
        ret = Tracks
    )
    .tracks)
}

/// Get information about a track.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-tracks/).
pub async fn get_track(
    token: &AccessToken,
    id: &str,
    market: Option<Market>,
) -> Result<Track, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/tracks/{}",
        path_params = [id],
        optional_query_params = {"market": market.map(|m| m.as_str())},
        ret = Track
    ))
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;

    #[tokio::test]
    async fn test_get_track() {
        // "Walk Like an Egyptian"
        let track = get_track(&token().await, "1Jwc3ODLQxtbnS8M9TflSP", None)
            .await
            .unwrap();
        assert_eq!(track.id, "1Jwc3ODLQxtbnS8M9TflSP");
        assert_eq!(track.name, "Walk Like an Egyptian");
        assert_eq!(track.artists[0].name, "The Bangles");
    }

    #[tokio::test]
    async fn test_get_tracks() {
        // "Walk Like an Egyptian", "Play that Funky Music"
        let tracks = get_tracks(
            &token().await,
            &["1Jwc3ODLQxtbnS8M9TflSP", "5uuJruktM9fMdN9Va0DUMl"],
            None,
        )
        .await
        .unwrap();
        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[0].name, "Walk Like an Egyptian");
        assert_eq!(tracks[1].name, "Play That Funky Music");
    }

    #[tokio::test]
    async fn test_relink() {
        // Test track relinking with "Heaven and Hell"
        let relinked = get_track(
            &token().await,
            "6kLCHFM39wkFjOuyPGLGeQ",
            Some(Market::Country(CountryCode::USA)),
        )
        .await
        .unwrap();
        assert_eq!(relinked.name, "Heaven and Hell");
        assert!(relinked.is_playable.unwrap());
        let from = relinked.linked_from.as_ref().unwrap();
        assert_eq!(from.id, "6kLCHFM39wkFjOuyPGLGeQ");
    }

    #[tokio::test]
    async fn test_analysis() {
        // Get analysis of "Walk Like an Egyptian"
        get_audio_analysis_track(&token().await, "1Jwc3ODLQxtbnS8M9TflSP")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_features() {
        // Get features of "Walk Like an Egyptian"
        let features = get_audio_features_track(&token().await, "1Jwc3ODLQxtbnS8M9TflSP")
            .await
            .unwrap();
        assert_eq!(features.id, "1Jwc3ODLQxtbnS8M9TflSP");
        assert_eq!(features.key, 11);
        assert_eq!(features.mode, Mode::Major);
        assert_eq!(features.tempo, 103.022);
    }

    #[tokio::test]
    async fn test_features_tracks() {
        // Get features of "Walk Like an Egyptian" and "Play that Funky Music"
        let features = get_audio_features_tracks(
            &token().await,
            &["1Jwc3ODLQxtbnS8M9TflSP", "5uuJruktM9fMdN9Va0DUMl"],
        )
        .await
        .unwrap();
        assert_eq!(features.len(), 2);
        assert_eq!(features[0].id, "1Jwc3ODLQxtbnS8M9TflSP");
        assert_eq!(features[1].id, "5uuJruktM9fMdN9Va0DUMl");
    }
}
