//! Endpoint functions related to controlling what is playing on the current user's Spotify account. (Beta)
//!
//! All endpoints in here are in Beta, and so are more likely to break.
//!
//! The `device_id` parameter seen in this module is the device to perform the request on. If not
//! specified, it will default to the current user's currenttly active device.

use crate::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Get the current user's available devices (Beta).
///
/// Requires `user-read-playback-state`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-a-users-available-devices/).
pub async fn get_devices(token: &AccessToken) -> Result<Vec<Device>, EndpointError<PlayerError>> {
    #[derive(Deserialize)]
    struct Devices {
        devices: Vec<Device>,
    }

    Ok(request!(
        token,
        GET "/v1/me/player/devices",
        ret = Devices
    )
    .devices)
}

/// Get information about the current user's current playback (Beta).
///
/// Requires `user-read-playback-state`. Returns None if nothing is currently playing.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-information-about-the-users-current-playback/).
pub async fn get_playback(
    token: &AccessToken,
    market: Option<Market>,
) -> Result<Option<CurrentPlayback>, EndpointError<PlayerError>> {
    Ok(Some(request!(
        token,
        GET "/v1/me/player",
        query_params = {"additional_types": "episode,track"},
        optional_query_params = {"market": market.map(|m| m.as_str())},
        ret = CurrentPlayback,
        or_else = None
    )))
}

/// Get current user's recently played tracks (Beta).
///
/// Note that a track needs to be played for >30seconds to be included in the play history.
/// Requires `user-read-recently-played`. Will return None if a private session is enabled.
///
/// `after` and `before` are Cursor values given the previous time this endpoint was called, to
/// move forward or back in time respectively. Both `after` and `before` must _not_ be Some.
/// `after` is a Unix milliseconds timestamp, and will return everything played after that
/// position, `before` is the same but returns everything before that position.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-recently-played/).
pub async fn get_recently_played(
    token: &AccessToken,
    limit: usize,
    after: Option<String>,
    before: Option<String>,
) -> Result<Option<TwoWayCursorPage<PlayHistory>>, EndpointError<PlayerError>> {
    Ok(Some(request!(
        token,
        GET "/v1/me/player/recently-played",
        query_params = {"limit": limit.to_string()},
        optional_query_params = {"after": after, "before": before},
        ret = TwoWayCursorPage<PlayHistory>,
        or_else = None
    )))
}

/// Get the current user's currently playing track (Beta).
///
/// Requires `user-read-currently-playing` and/or `user-read-playback-state`. Returns None if no
/// available devices are found, no tracks are playing, or a private session is enabled.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-the-users-currently-playing-track/).
pub async fn get_playing_track(
    token: &AccessToken,
    market: Option<Market>,
) -> Result<Option<CurrentlyPlaying>, EndpointError<PlayerError>> {
    Ok(Some(request!(
        token,
        GET "/v1/me/player/currently-playing",
        query_params = {"additional_types": "episode,track"},
        optional_query_params = {"market": market.map(|m| m.as_str())},
        ret = CurrentlyPlaying,
        or_else = None
    )))
}

/// Pause the current user's playback (Beta).
///
/// Requires `user-modify-playback-state`. This action completes asynchronously, meaning you will
/// not know if it succeeded unless you check.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/pause-a-users-playback/).
pub async fn pause(
    token: &AccessToken,
    device_id: Option<&str>,
) -> Result<(), EndpointError<PlayerError>> {
    request!(
        token,
        PUT "/v1/me/player/pause",
        optional_query_params = {"device_id": device_id},
        body = "{}"
    );
    Ok(())
}

/// Seek to position in currently playing track (Beta).
///
/// Requires `user-modify-playback-state`. This action completes asynchronously, meaning you will
/// not know if it succeeded unless you check.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/seek-to-position-in-currently-playing-track/).
pub async fn seek(
    token: &AccessToken,
    position: Duration,
    device_id: Option<&str>,
) -> Result<(), EndpointError<PlayerError>> {
    request!(
        token,
        PUT "/v1/me/player/seek",
        query_params = {"position_ms": position.as_millis().to_string()},
        optional_query_params = {"device_id": device_id},
        body = "{}"
    );
    Ok(())
}

/// Set repeat mode on current playback (Beta).
///
/// Requires `user-modify-playback-state`. This action complete asynchronously, meaning you will
/// not know if it succeeded unless you check.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/set-repeat-mode-on-users-playback/).
pub async fn set_repeat(
    token: &AccessToken,
    state: RepeatState,
    device_id: Option<&str>,
) -> Result<(), EndpointError<PlayerError>> {
    request!(
        token,
        PUT "/v1/me/player/repeat",
        query_params = {"state": state.as_str()},
        optional_query_params = {"device_id": device_id},
        body = "{}"
    );
    Ok(())
}

/// Set volume on current playback (Beta).
///
/// Requires `user-modify-playback-state`. This action complete asynchronously, meaning you will
/// not know if it succeeded unless you check.
///
/// `volume_percent` is the volume as a percentage, from 0 to 100 inclusive.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/set-volume-for-users-playback/).
pub async fn set_volume(
    token: &AccessToken,
    volume_percent: i32,
    device_id: Option<&str>,
) -> Result<(), EndpointError<PlayerError>> {
    request!(
        token,
        PUT "/v1/me/player/volume",
        query_params = {"volume_percent": volume_percent.to_string()},
        optional_query_params = {"device_id": device_id},
        body = "{}"
    );
    Ok(())
}

/// Skip to next track (Beta).
///
/// Requires `user-modify-playback-state`. This action complete asynchronously, meaning you will
/// not know if it succeeded unless you check.
///
/// After a successful skip operation, playback will automatically start.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/skip-users-playback-to-next-track/).
pub async fn skip_next(
    token: &AccessToken,
    device_id: Option<&str>,
) -> Result<(), EndpointError<PlayerError>> {
    request!(
        token,
        POST "/v1/me/player/next",
        optional_query_params = {"device_id": device_id},
        body = "{}"
    );
    Ok(())
}

/// Skip to previous track (Beta).
///
/// Requires `user-modify-playback-state`. This action complete asynchronously, meaning you will
/// not know if it succeeded unless you check.
///
/// After a successful skip operation, playback will automatically start. This action will always
/// skip to the previous track, regardless of the current track's progress; to go to the start of
/// the track, use `seek`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/skip-users-playback-to-previous-track/).
pub async fn skip_prev(
    token: &AccessToken,
    device_id: Option<&str>,
) -> Result<(), EndpointError<PlayerError>> {
    request!(
        token,
        POST "/v1/me/player/previous",
        optional_query_params = {"device_id": device_id},
        body = "{}"
    );
    Ok(())
}

/// Request to play something.
#[derive(Debug, Clone)]
pub enum Play<'s, 'i> {
    /// Play from a context (must not be track) with a specified 0-indexed offset to start playing at.
    Context(ItemType, &'i str, usize),
    /// Play a list of tracks.
    Tracks(&'s [&'s str]),
}

/// Start or resume playback (Beta).
///
/// Requires `user-modify-playback-state`. This action complete asynchronously, meaning you will
/// not know if it succeeded unless you check.
///
/// `play`, when set, controls what to play, and what offset in the context to start playing at.
/// `position` controls how far into the current track to play; if it is longer than the current
/// track, then the next track will play.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/start-a-users-playback/).
pub async fn play(
    token: &AccessToken,
    play: Option<Play<'_, '_>>,
    position: Option<Duration>,
    device_id: Option<&str>,
) -> Result<(), EndpointError<PlayerError>> {
    #[derive(Serialize)]
    struct Offset {
        position: usize,
    }

    #[derive(Serialize)]
    struct Body {
        context_uri: Option<String>,
        offset: Option<Offset>,
        uris: Option<Vec<String>>,
        position_ms: Option<u128>,
    }

    let mut body = Body {
        context_uri: None,
        offset: None,
        uris: None,
        position_ms: position.map(|duration| duration.as_millis()),
    };

    if let Some(play) = play {
        match play {
            Play::Context(context_type, id, position) => {
                body.context_uri = Some(format!("spotify:{}:{}", context_type.as_str(), id));
                body.offset = Some(Offset { position });
            }
            Play::Tracks(ids) => {
                body.uris = Some(ids.iter().map(|s| format!("spotify:track:{}", s)).collect());
            }
        }
    }

    request!(
        token,
        PUT "/v1/me/player/play",
        optional_query_params = {"device_id": device_id},
        body = serde_json::to_string(&body)?
    );
    Ok(())
}

/// Enable or disable shuffle (Beta).
///
/// Requires `user-modify-playback-state`. This action complete asynchronously, meaning you will
/// not know if it succeeded unless you check.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/toggle-shuffle-for-users-playback/).
pub async fn set_shuffle(
    token: &AccessToken,
    shuffle: bool,
    device_id: Option<&str>,
) -> Result<(), EndpointError<PlayerError>> {
    request!(
        token,
        PUT "/v1/me/player/shuffle",
        query_params = {"state": if shuffle {"true"} else {"false"}},
        optional_query_params = {"deivce_id": device_id},
        body = "{}"
    );
    Ok(())
}

/// Transfer playback to another device (Beta).
///
/// Requires `user-modify-playback-state`. When `play == true`, playback will happen on the new
/// device. When `play == false`, playback will continue in its current state.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/transfer-a-users-playback/).
pub async fn transfer(
    token: &AccessToken,
    id: &str,
    play: bool,
) -> Result<(), EndpointError<PlayerError>> {
    request!(
        token,
        PUT "/v1/me/player",
        body = format!(r#"{{"device_ids":["{}"],"play":{}}}"#, id, play)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;
    use std::time::Duration;
    use tokio::time;

    #[tokio::test]
    async fn test() {
        let token = token().await;

        let mut devices = get_devices(&token).await.unwrap().into_iter();
        let device = loop {
            let device = devices
                .next()
                .expect("You must have at least one usable device for this test to work.");
            if !device.is_restricted && device.id.is_some() && !device.is_private_session {
                break device;
            }
        };
        let id = &device.id.as_ref().unwrap();
        if !device.is_active {
            println!("Transferring device to {}...", device.name);
            transfer(&token, id, false).await.unwrap();
        }

        // Time to wait to assume that the operation has completed
        let wait_time = Duration::from_millis(300);

        // Play 10 seconds into the 3rd track from RELAXER
        play(
            &token,
            Some(Play::Context(ItemType::Album, "3lBPyXvg1hhoJ1REnw80fZ", 2)),
            Some(Duration::from_secs(10)),
            None,
        )
        .await
        .unwrap();
        time::delay_for(wait_time).await;
        let playback = get_playback(&token, Some(Market::FromToken))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(playback.device.id, device.id);
        assert_eq!(playback.device.name, device.name);
        assert_eq!(playback.device.device_type, device.device_type);
        assert_eq!(playback.device.volume_percent, device.volume_percent);
        let context = playback.currently_playing.context.unwrap();
        assert_eq!(context.context_type, ItemType::Album);
        assert_eq!(context.id, "3lBPyXvg1hhoJ1REnw80fZ");
        assert!(playback.currently_playing.progress.unwrap() >= Duration::from_secs(10));
        assert!(playback.currently_playing.is_playing);
        let track = match playback.currently_playing.item.unwrap() {
            PlayingType::Track(item) => item,
            _ => panic!(),
        };
        assert_eq!(track.album.id, "3lBPyXvg1hhoJ1REnw80fZ");
        assert_eq!(track.track_number, 3);

        // Play "I am a Paleontologist" and "Ten Tonne Skeleton"
        play(
            &token,
            Some(Play::Tracks(&[
                "2Wbz0QcXCVYmuBgOwUV6KU",
                "0vjYxBDAcflD0358arIVZG",
            ])),
            None,
            None,
        )
        .await
        .unwrap();
        time::delay_for(wait_time).await;
        let playing = get_playing_track(&token, Some(Market::FromToken))
            .await
            .unwrap()
            .unwrap();
        assert!(playing.progress.unwrap() < Duration::from_secs(2));
        assert!(playing.is_playing);
        let track = match playing.item.unwrap() {
            PlayingType::Track(item) => item,
            _ => panic!(),
        };
        assert_eq!(track.id, "2Wbz0QcXCVYmuBgOwUV6KU");

        // Seek to 2ms before end
        seek(&token, Duration::from_millis(152_106 - 2), None)
            .await
            .unwrap();
        time::delay_for(wait_time).await;
        let playing = get_playing_track(&token, Some(Market::FromToken))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            match playing.item.unwrap() {
                PlayingType::Track(item) => item,
                _ => panic!(),
            }
            .id,
            "0vjYxBDAcflD0358arIVZG"
        );

        // Repeat, shuffle, volume
        set_repeat(&token, RepeatState::Track, None).await.unwrap();
        set_shuffle(&token, true, None).await.unwrap();
        set_volume(&token, 17, None).await.unwrap();
        time::delay_for(wait_time).await;
        let playback = get_playback(&token, Some(Market::FromToken))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(playback.repeat_state, RepeatState::Track);
        assert_eq!(playback.shuffle_state, true);
        assert_eq!(playback.device.volume_percent.unwrap(), 17);
        set_repeat(&token, RepeatState::Context, None)
            .await
            .unwrap();
        set_shuffle(&token, false, None).await.unwrap();
        set_volume(&token, 73, None).await.unwrap();
        time::delay_for(wait_time).await;
        let playback = get_playback(&token, Some(Market::FromToken))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(playback.repeat_state, RepeatState::Context);
        assert_eq!(playback.shuffle_state, false);
        assert_eq!(playback.device.volume_percent.unwrap(), 73);

        // Skip previous
        skip_prev(&token, None).await.unwrap();
        time::delay_for(wait_time).await;
        let playing = get_playing_track(&token, Some(Market::FromToken))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            match playing.item.unwrap() {
                PlayingType::Track(item) => item,
                _ => panic!(),
            }
            .id,
            "2Wbz0QcXCVYmuBgOwUV6KU"
        );

        // Skip next
        skip_next(&token, None).await.unwrap();
        time::delay_for(wait_time).await;
        let playing = get_playing_track(&token, Some(Market::FromToken))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            match playing.item.unwrap() {
                PlayingType::Track(item) => item,
                _ => panic!(),
            }
            .id,
            "0vjYxBDAcflD0358arIVZG"
        );

        // Play from playlist
        play(
            &token,
            Some(Play::Context(
                ItemType::Playlist,
                "37i9dQZF1DWSVtp02hITpN",
                0,
            )),
            None,
            None,
        )
        .await
        .unwrap();
        time::delay_for(wait_time).await;
        get_playing_track(&token, Some(Market::FromToken))
            .await
            .unwrap()
            .unwrap();

        // Pause
        pause(&token, None).await.unwrap();
        time::delay_for(wait_time).await;
        let playback = get_playback(&token, Some(Market::FromToken))
            .await
            .unwrap()
            .unwrap();
        assert!(!playback.currently_playing.is_playing);
    }

    #[tokio::test]
    async fn test_recent() {
        get_recently_played(&token().await, 3, None, None)
            .await
            .unwrap();
    }
}
