use std::fmt::Display;

use itertools::Itertools;
use reqwest::header;
use serde::Deserialize;

use super::{chunked_requests, chunked_sequence};
use crate::{Artist, Client, CursorPage, Error, Response};

/// Endpoint functions relating to following and unfollowing artists, users and playlists.
#[derive(Debug, Clone, Copy)]
pub struct Follow<'a>(pub &'a Client);

impl Follow<'_> {
    /// Check if the current user follows some artists.
    ///
    /// Returns vector of bools that is in the same order as the given ids. Requires
    /// `user-follow-read`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/check-current-user-follows/).
    pub async fn user_follows_artists<I: Iterator>(
        self,
        ids: impl IntoIterator<IntoIter = I, Item = I::Item>,
    ) -> Result<Response<Vec<bool>>, Error>
    where
        I::Item: Display,
    {
        chunked_sequence(&ids.into_iter().chunks(50), |mut ids| async move {
            self.0
                .send_json(
                    self.0
                        .client
                        .get(endpoint!("/v1/me/following/contains"))
                        .query(&(("type", "artist"), ("ids", ids.join(",")))),
                )
                .await
        }).await
    }

    /// Check if the current user follows some users.
    ///
    /// Returns vector of bools that is in the same order as the given ids. Requires
    /// `user-follow-read`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/check-current-user-follows/).
    pub async fn user_follows_users<I: Iterator>(
        self,
        ids: impl IntoIterator<IntoIter = I, Item = I::Item>,
    ) -> Result<Response<Vec<bool>>, Error>
    where
        I::Item: Display,
    {
        chunked_sequence(&ids.into_iter().chunks(50), |mut ids| async move {
            self.0
                .send_json(
                    self.0
                        .client
                        .get(endpoint!("/v1/me/following/contains"))
                        .query(&(("type", "user"), ("ids", ids.join(",")))),
                )
                .await
        }).await
    }

    /// Check if some users follow a playlist.
    ///
    /// `id` is the id of the playlist and `user_ids` is the users who you want to check. Users can
    /// publicly or privately follow playlists; checking whether a user privately follows a playlist
    /// requires `playlist-read-private`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/check-user-following-playlist/).
    pub async fn users_follow_playlist<I: Iterator>(
        self,
        id: &str,
        user_ids: impl IntoIterator<IntoIter = I, Item = I::Item>,
    ) -> Result<Response<Vec<bool>>, Error>
    where
        I::Item: Display,
    {
        chunked_sequence(&user_ids.into_iter().chunks(5), |mut user_ids| async move {
            self.0
                .send_json(
                    self.0
                        .client
                        .get(endpoint!("/v1/playlists/{}/followers/contains", id))
                        .query(&(("ids", user_ids.join(",")),)),
                )
                .await
        }).await
    }

    /// Follow artists.
    ///
    /// Requires `user-follow-modify`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/follow-artists-users/).
    pub async fn follow_artists<I: Iterator>(
        self,
        ids: impl IntoIterator<IntoIter = I, Item = I::Item>,
    ) -> Result<(), Error>
    where
        I::Item: Display,
    {
        chunked_requests(&ids.into_iter().chunks(50), |mut ids| async move {
            self.0
                .send_empty(
                    self.0
                        .client
                        .put(endpoint!("/v1/me/following"))
                        .query(&(("type", "artist"), ("ids", ids.join(","))))
                        .body("{}"),
                )
                .await
        }).await
    }

    /// Follow users.
    ///
    /// Requires `user-follow-modify`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/follow-artists-users/).
    pub async fn follow_users<I: Iterator>(
        self,
        ids: impl IntoIterator<IntoIter = I, Item = I::Item>,
    ) -> Result<(), Error>
    where
        I::Item: Display,
    {
        chunked_requests(&ids.into_iter().chunks(50), |mut ids| async move {
            self.0
                .send_empty(
                    self.0
                        .client
                        .put(endpoint!("/v1/me/following"))
                        .query(&(("type", "user"), ("ids", ids.join(","))))
                        .body("{}"),
                )
                .await
        }).await
    }

    /// Follow a playlist publicly.
    ///
    /// Requires `playlist-modify-public`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/follow-playlist/).
    pub async fn follow_playlist_public(self, id: &str) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .put(endpoint!("/v1/playlists/{}/followers", id))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(r#"{"public":true}"#),
            )
            .await
    }

    /// Follow a playlist privately.
    ///
    /// Requires `playlist-modify-private`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/follow-playlist/).
    pub async fn follow_playlist_private(self, id: &str) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .put(endpoint!("/v1/playlists/{}/followers", id))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(r#"{"public":false}"#),
            )
            .await
    }

    /// Get followed artists.
    ///
    /// Limit must be in the range [1..50]. `after` is the Cursor value given the previous time this
    /// endpoint was called. It is used to get the next page of items.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/get-followed/).
    pub async fn get_followed_artists(
        self,
        limit: usize,
        after: Option<&str>,
    ) -> Result<Response<CursorPage<Artist>>, Error> {
        #[derive(Deserialize)]
        struct Response {
            artists: CursorPage<Artist>,
        };

        Ok(self
            .0
            .send_json::<Response>(self.0.client.get(endpoint!("/v1/me/following")).query(&(
                ("type", "artist"),
                ("limit", limit.to_string()),
                after.map(|after| ("after", after)),
            )))
            .await?
            .map(|res| res.artists))
    }

    /// Unfollow artists.
    ///
    /// Requires `user-follow-modify`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/unfollow-artists-users/).
    pub async fn unfollow_artists<I: Iterator>(
        self,
        ids: impl IntoIterator<IntoIter = I, Item = I::Item>,
    ) -> Result<(), Error>
    where
        I::Item: Display,
    {
        chunked_requests(&ids.into_iter().chunks(50), |mut ids| async move {
            self.0
                .send_empty(
                    self.0
                        .client
                        .delete(endpoint!("/v1/me/following"))
                        .query(&(("type", "artist"), ("ids", ids.join(","))))
                        .body("{}"),
                )
                .await
        }).await
    }

    /// Unfollow users.
    ///
    /// Requires `user-follow-modify`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/unfollow-artists-users/).
    pub async fn unfollow_users<I: Iterator>(
        self,
        ids: impl IntoIterator<IntoIter = I, Item = I::Item>,
    ) -> Result<(), Error>
    where
        I::Item: Display,
    {
        chunked_requests(&ids.into_iter().chunks(50), |mut ids| async move {
            self.0
                .send_empty(
                    self.0
                        .client
                        .delete(endpoint!("/v1/me/following"))
                        .query(&(("type", "users"), ("ids", ids.join(","))))
                        .body("{}"),
                )
                .await
        }).await
    }

    /// Unfollow a playlist.
    ///
    /// If the user follows it publicly you need `playlist-modify-public`, if the user follows it
    /// privately you need `playlist-modiy-private`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/unfollow-playlist/).
    pub async fn unfollow_playlist(self, id: &str) -> Result<(), Error> {
        self.0
            .send_empty(
                self.0
                    .client
                    .delete(endpoint!("/v1/playlists/{}/followers", id))
                    .body("{}"),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::client;

    #[tokio::test]
    async fn test_follow_artists() {
        // NOTE: This test only works if you follow < 49 artists as it only requests the first page.
        // You also must not follow Lemon Demon.
        let client = client();
        let follow = client.follow();

        // TOTO, Eminem and Lemon Demon
        let artists = &[
            "0PFtn5NtBbbUNbU9EAmIWF",
            "7dGJo4pcD2V6oG8kP0tJRR",
            "4llAOeA6kEF4ytaB2fsmcW",
        ];
        let split = 2;
        let (followed_artists, unfollowed_artists) = artists.split_at(split);

        // Store old
        let old = follow.user_follows_artists(artists).await.unwrap().data;

        // Following and unfollowing
        follow.follow_artists(followed_artists).await.unwrap();
        follow.unfollow_artists(unfollowed_artists).await.unwrap();

        // Check
        let check = follow.user_follows_artists(artists).await.unwrap().data;
        let (follow_check, unfollow_check) = check.split_at(split);
        assert!(follow_check.iter().all(|&followed| followed));
        assert!(unfollow_check.iter().all(|&followed| !followed));

        // Check by finding in list
        let followed = follow.get_followed_artists(50, None).await.unwrap().data;
        if followed.total <= 50 {
            for followed_artist in followed_artists {
                assert!(followed
                    .items
                    .iter()
                    .any(|artist| artist.id == *followed_artist));
            }
            for unfollowed_artist in unfollowed_artists {
                assert!(followed
                    .items
                    .iter()
                    .all(|artist| artist.id != *unfollowed_artist));
            }
        }

        // Restore
        let mut old_followed = Vec::with_capacity(artists.len());
        let mut old_unfollowed = Vec::with_capacity(artists.len());
        for i in 0..artists.len() {
            if old[i] {
                &mut old_followed
            } else {
                &mut old_unfollowed
            }
            .push(artists[i]);
        }
        if !old_followed.is_empty() {
            follow.follow_artists(&old_followed).await.unwrap();
        }
        if !old_unfollowed.is_empty() {
            follow.unfollow_artists(&old_unfollowed).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_follow_playlists() {
        let client = client();
        let follow = client.follow();

        // Follow "Sing-Along Indie Hits" playlist
        follow
            .follow_playlist_public("37i9dQZF1DWYBF1dYDPlHw")
            .await
            .unwrap();

        // Check whether following playlist
        let id = client
            .users_profile()
            .get_current_user()
            .await
            .unwrap()
            .data
            .id;
        let followers = follow
            .users_follow_playlist("37i9dQZF1DWYBF1dYDPlHw", &["spotify", &id])
            .await
            .unwrap()
            .data;
        assert_eq!(followers, &[false, true]);

        // Unfollow
        follow
            .unfollow_playlist("37i9dQZF1DWYBF1dYDPlHw")
            .await
            .unwrap();
    }
}
