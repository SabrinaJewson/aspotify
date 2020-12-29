use std::fmt::Display;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    Client, CurrentPlayback, CurrentlyPlaying, Device, Error, ItemType, Market, PlayHistory,
    RepeatState, Response, TwoWayCursorPage,
};

/// Endpoint functions related to controlling what is playing on the current user's Spotify account.
/// (Beta)
///
/// All endpoints in here are in Beta, and so are more likely to break.
///
/// The `device_id` parameter seen in this module is the device to perform the request on. If not
/// specified, it will default to the current user's currenttly active device.
#[derive(Debug, Clone, Copy)]
pub struct Player<'a>(pub &'a Client);

impl Player<'_> {
    /// Get the current user's available devices (Beta).
    ///
    /// Requires `user-read-playback-state`
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-a-users-available-devices/).
    pub async fn get_devices(self) -> Result<Response<Vec<Device>>, Error> {
        #[derive(Deserialize)]
        struct Devices {
            devices: Vec<Device>,
        }

        Ok(self
            .0
            .send_json::<Devices>(self.0.client.get(endpoint!("/v1/me/player/devices")))
            .await?
            .map(|res| res.devices))
    }

    /// Get information about the current user's current playback (Beta).
    ///
    /// Requires `user-read-playback-state`. Returns None if nothing is currently playing.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-information-about-the-users-current-playback/).
    pub async fn get_playback(
        self,
        market: Option<Market>,
    ) -> Result<Response<Option<CurrentPlayback>>, Error> {
        self.0
            .send_opt_json(self.0.client.get(endpoint!("/v1/me/player")).query(&(
                ("additional_types", "episode,track"),
                market.map(Market::query),
            )))
            .await
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
        self,
        limit: usize,
        after: Option<String>,
        before: Option<String>,
    ) -> Result<Response<Option<TwoWayCursorPage<PlayHistory>>>, Error> {
        self.0
            .send_opt_json(
                self.0
                    .client
                    .get(endpoint!("/v1/me/player/recently-played"))
                    .query(&(
                        ("limit", limit.to_string()),
                        after.map(|after| ("after", after)),
                        before.map(|before| ("before", before)),
                    )),
            )
            .await
    }

    /// Get the current user's currently playing track (Beta).
    ///
    /// Requires `user-read-currently-playing` and/or `user-read-playback-state`. Returns None if no
    /// available devices are found, no tracks are playing, or a private session is enabled.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-the-users-currently-playing-track/).
    pub async fn get_playing_track(
        self,
        market: Option<Market>,
    ) -> Result<Response<Option<CurrentlyPlaying>>, Error> {
        self.0
            .send_opt_json(
                self.0
                    .client
                    .get(endpoint!("/v1/me/player/currently-playing"))
                    .query(&(
                        ("additional_types", "episode,track"),
                        market.map(Market::query),
                    )),
            )
            .await
    }

    /// Pause the current user's playback (Beta).
    ///
    /// Requires `user-modify-playback-state`. This action completes asynchronously, meaning you will
    /// not know if it succeeded unless you check.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/pause-a-users-playback/).
    pub async fn pause(self, device_id: Option<&str>) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .put(endpoint!("/v1/me/player/pause"))
                    .query(&(device_id.map(device_query)))
                    .body("{}"),
            )
            .await
    }

    /// Seek to position in currently playing track (Beta).
    ///
    /// Requires `user-modify-playback-state`. This action completes asynchronously, meaning you will
    /// not know if it succeeded unless you check.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/seek-to-position-in-currently-playing-track/).
    pub async fn seek(self, position: Duration, device_id: Option<&str>) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .put(endpoint!("/v1/me/player/seek"))
                    .query(&(
                        device_id.map(device_query),
                        ("position_ms", position.as_millis().to_string()),
                    ))
                    .body("{}"),
            )
            .await
    }

    /// Set repeat mode on current playback (Beta).
    ///
    /// Requires `user-modify-playback-state`. This action complete asynchronously, meaning you will
    /// not know if it succeeded unless you check.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/set-repeat-mode-on-users-playback/).
    pub async fn set_repeat(
        self,
        state: RepeatState,
        device_id: Option<&str>,
    ) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .put(endpoint!("/v1/me/player/repeat"))
                    .query(&(device_id.map(device_query), ("state", state.as_str())))
                    .body("{}"),
            )
            .await
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
        self,
        volume_percent: i32,
        device_id: Option<&str>,
    ) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .put(endpoint!("/v1/me/player/volume"))
                    .query(&(
                        device_id.map(device_query),
                        ("volume_percent", volume_percent.to_string()),
                    ))
                    .body("{}"),
            )
            .await
    }

    /// Skip to next track (Beta).
    ///
    /// Requires `user-modify-playback-state`. This action complete asynchronously, meaning you will
    /// not know if it succeeded unless you check.
    ///
    /// After a successful skip operation, playback will automatically start.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/skip-users-playback-to-next-track/).
    pub async fn skip_next(self, device_id: Option<&str>) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .post(endpoint!("/v1/me/player/next"))
                    .query(&(device_id.map(device_query),))
                    .body("{}"),
            )
            .await
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
    pub async fn skip_prev(self, device_id: Option<&str>) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .post(endpoint!("/v1/me/player/previous"))
                    .query(&(device_id.map(device_query),))
                    .body("{}"),
            )
            .await
    }

    /// Start or resume playback (Beta).
    ///
    /// Requires `user-modify-playback-state`. This action complete asynchronously, meaning you will
    /// not know if it succeeded unless you check.
    ///
    /// `play`, when set, controls what to play, and what offset in the context to start playing at.
    /// `position` controls how far into the current track to play; if it is longer than the current
    /// track, then the next track will play. To keep the existing content and position, use `resume`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/start-a-users-playback/).
    pub async fn play<I: Iterator>(
        self,
        play: Option<Play<'_, impl IntoIterator<IntoIter = I, Item = I::Item>>>,
        position: Option<Duration>,
        device_id: Option<&str>,
    ) -> Result<(), Error>
    where
        I::Item: Display,
    {
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
                    body.uris = Some(
                        ids.into_iter()
                            .map(|s| format!("spotify:track:{}", s))
                            .collect(),
                    );
                }
            }
        }

        self.0
            .send_empty(
                self.0
                    .client
                    .put(endpoint!("/v1/me/player/play"))
                    .query(&(device_id.map(device_query)))
                    .body(serde_json::to_string(&body)?),
            )
            .await
    }

    /// Resume playback (Beta).
    ///
    /// Requires `user-modify-playback-state`. This action complete asynchronously, meaning you will
    /// not know if it succeeded unless you check.
    ///
    /// Resumes playback where it was paused. To specify a content or offset, use `play` instead.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/start-a-users-playback/).
    pub async fn resume(self, device_id: Option<&str>) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .put(endpoint!("/v1/me/player/play"))
                    .query(&(device_id.map(device_query),))
                    .body("{}"),
            )
            .await
    }

    /// Enable or disable shuffle (Beta).
    ///
    /// Requires `user-modify-playback-state`. This action complete asynchronously, meaning you will
    /// not know if it succeeded unless you check.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/toggle-shuffle-for-users-playback/).
    pub async fn set_shuffle(self, shuffle: bool, device_id: Option<&str>) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .put(endpoint!("/v1/me/player/shuffle"))
                    .query(&(
                        ("state", if shuffle { "true" } else { "false" }),
                        device_id.map(device_query),
                    ))
                    .body("{}"),
            )
            .await
    }

    /// Transfer playback to another device (Beta).
    ///
    /// Requires `user-modify-playback-state`. When `play == true`, playback will happen on the new
    /// device. When `play == false`, playback will continue in its current state.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/transfer-a-users-playback/).
    pub async fn transfer(self, id: &str, play: bool) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .put(endpoint!("/v1/me/player"))
                    .body(format!(r#"{{"device_ids":["{}"],"play":{}}}"#, id, play)),
            )
            .await
    }
}

/// Request to play something.
#[derive(Debug, Clone)]
pub enum Play<'c, I> {
    /// Play from a context (must not be track) with a specified 0-indexed offset to start playing
    /// at.
    Context(ItemType, &'c str, usize),
    /// Play a list of tracks.
    Tracks(I),
}

fn device_query(device: &str) -> (&'static str, &str) {
    ("device_id", device)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time;

    use crate::endpoints::client;
    use crate::{ItemType, Market, Play, PlayingType, RepeatState};

    #[tokio::test]
    async fn test() {
        let client = client();
        let player = client.player();

        let mut devices = player.get_devices().await.unwrap().data.into_iter();
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
            player.transfer(id, false).await.unwrap();
        }

        // Time to wait to assume that the operation has completed
        let wait_time = Duration::from_millis(300);

        // Play 10 seconds into the 3rd track from RELAXER
        player
            .play(
                Some(Play::<'_, &[u8]>::Context(
                    ItemType::Album,
                    "3lBPyXvg1hhoJ1REnw80fZ",
                    2,
                )),
                Some(Duration::from_secs(10)),
                None,
            )
            .await
            .unwrap();
        time::sleep(wait_time).await;

        let playback = player
            .get_playback(Some(Market::FromToken))
            .await
            .unwrap()
            .data
            .unwrap();
        assert_eq!(playback.device.id, device.id);
        assert_eq!(playback.device.name, device.name);
        assert_eq!(playback.device.device_type, device.device_type);
        assert_eq!(playback.device.volume_percent, device.volume_percent);
        let context = playback.currently_playing.context.unwrap();
        assert_eq!(context.context_type, ItemType::Album);
        assert_eq!(context.id, "3lBPyXvg1hhoJ1REnw80fZ");
        if playback.currently_playing.progress.unwrap() < Duration::from_secs(10) {
            panic!(
                "duration is {:?} (less than 10 seconds)",
                playback.currently_playing.progress.unwrap()
            );
        }
        assert!(playback.currently_playing.is_playing);
        let track = match playback.currently_playing.item.unwrap() {
            PlayingType::Track(item) => item,
            _ => panic!(),
        };
        assert_eq!(track.album.id.unwrap(), "3lBPyXvg1hhoJ1REnw80fZ");
        assert_eq!(track.track_number, 3);

        // Play "I am a Paleontologist" and "Ten Tonne Skeleton"
        player
            .play(
                Some(Play::Tracks(&[
                    "0MSqR4unoY5KReMoOP6E2D",
                    "0vjYxBDAcflD0358arIVZG",
                ])),
                None,
                None,
            )
            .await
            .unwrap();
        time::sleep(wait_time).await;
        let playing = player
            .get_playing_track(Some(Market::FromToken))
            .await
            .unwrap()
            .data
            .unwrap();
        assert!(playing.progress.unwrap() < Duration::from_secs(4));
        assert!(playing.is_playing);
        let track = match playing.item.unwrap() {
            PlayingType::Track(item) => item,
            _ => panic!(),
        };
        assert_eq!(track.id.unwrap(), "0MSqR4unoY5KReMoOP6E2D");

        // Seek to 2ms before end
        player
            .seek(Duration::from_millis(152_106 - 2), None)
            .await
            .unwrap();
        time::sleep(wait_time).await;
        let playing = player
            .get_playing_track(Some(Market::FromToken))
            .await
            .unwrap()
            .data
            .unwrap();
        assert_eq!(
            match playing.item.unwrap() {
                PlayingType::Track(item) => item,
                _ => panic!(),
            }
            .id
            .unwrap(),
            "0vjYxBDAcflD0358arIVZG"
        );

        // Repeat, shuffle, volume
        player.set_repeat(RepeatState::Track, None).await.unwrap();
        player.set_shuffle(true, None).await.unwrap();
        player.set_volume(17, None).await.unwrap();
        time::sleep(wait_time).await;
        let playback = player
            .get_playback(Some(Market::FromToken))
            .await
            .unwrap()
            .data
            .unwrap();
        assert_eq!(playback.repeat_state, RepeatState::Track);
        assert_eq!(playback.shuffle_state, true);
        assert_eq!(playback.device.volume_percent.unwrap(), 17);
        player.set_repeat(RepeatState::Context, None).await.unwrap();
        player.set_shuffle(false, None).await.unwrap();
        player.set_volume(73, None).await.unwrap();
        time::sleep(wait_time).await;
        let playback = player
            .get_playback(Some(Market::FromToken))
            .await
            .unwrap()
            .data
            .unwrap();
        assert_eq!(playback.repeat_state, RepeatState::Context);
        assert_eq!(playback.shuffle_state, false);
        assert_eq!(playback.device.volume_percent.unwrap(), 73);

        // Skip previous
        player.skip_prev(None).await.unwrap();
        time::sleep(wait_time).await;
        let playing = player
            .get_playing_track(Some(Market::FromToken))
            .await
            .unwrap()
            .data
            .unwrap();
        assert_eq!(
            match playing.item.unwrap() {
                PlayingType::Track(item) => item,
                _ => panic!(),
            }
            .id
            .unwrap(),
            "0MSqR4unoY5KReMoOP6E2D"
        );

        // Skip next
        player.skip_next(None).await.unwrap();
        time::sleep(wait_time).await;
        let playing = player
            .get_playing_track(Some(Market::FromToken))
            .await
            .unwrap()
            .data
            .unwrap();
        assert_eq!(
            match playing.item.unwrap() {
                PlayingType::Track(item) => item,
                _ => panic!(),
            }
            .id
            .unwrap(),
            "0vjYxBDAcflD0358arIVZG"
        );

        // Play from playlist
        player
            .play(
                Some(Play::<'_, &[u8]>::Context(
                    ItemType::Playlist,
                    "37i9dQZF1DWSVtp02hITpN",
                    0,
                )),
                None,
                None,
            )
            .await
            .unwrap();
        time::sleep(wait_time).await;
        player
            .get_playing_track(Some(Market::FromToken))
            .await
            .unwrap()
            .data
            .unwrap();

        // Pause
        player.pause(None).await.unwrap();
        time::sleep(wait_time).await;
        let playback = player
            .get_playback(Some(Market::FromToken))
            .await
            .unwrap()
            .data
            .unwrap();
        assert!(!playback.currently_playing.is_playing);
    }

    #[tokio::test]
    async fn test_recent() {
        client()
            .player()
            .get_recently_played(3, None, None)
            .await
            .unwrap();
    }
}
