use std::fmt::Display;

use itertools::Itertools;

use super::{chunked_requests, chunked_sequence};
use crate::{Client, Error, Market, Page, Response, SavedAlbum, SavedShow, SavedTrack};

/// Endpoints relating to saving albums and tracks.
#[derive(Debug, Clone, Copy)]
pub struct Library<'a>(pub &'a Client);

impl Library<'_> {
    /// Check if the current user has saved some albums.
    ///
    /// Returns vector of bools that is in the same order as the given ids, telling whether the user
    /// has saved each album. Requires `user-library-read`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/check-users-saved-albums/).
    pub async fn user_saved_albums<I: Iterator>(
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
                        .get(endpoint!("/v1/me/albums/contains"))
                        .query(&(("ids", ids.join(",")),)),
                )
                .await
        })
        .await
    }

    /// Check if the current user has saved some shows.
    ///
    /// Returns vector of bools that is in the same order as the given ids, telling whether the user
    /// has saved each album. Requires `user-library-read`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/check-users-saved-shows/).
    pub async fn user_saved_shows<I: Iterator>(
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
                        .get(endpoint!("/v1/me/shows/contains"))
                        .query(&(("ids", ids.join(",")),)),
                )
                .await
        })
        .await
    }

    /// Check if the current user has saved some tracks.
    ///
    /// Returns vector of bools that is in the same order as the given ids, telling whether the user
    /// has saved each track. Requires `user-library-read`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/check-users-saved-tracks/).
    pub async fn user_saved_tracks<I: Iterator>(
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
                        .get(endpoint!("/v1/me/tracks/contains"))
                        .query(&(("ids", ids.join(",")),)),
                )
                .await
        })
        .await
    }

    /// Get the current user's saved albums.
    ///
    /// Requires `user-library-read`. Limit must be in the range [1..50].
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/get-users-saved-albums/).
    pub async fn get_saved_albums(
        self,
        limit: usize,
        offset: usize,
        market: Option<Market>,
    ) -> Result<Response<Page<SavedAlbum>>, Error> {
        self.0
            .send_json(self.0.client.get(endpoint!("/v1/me/albums")).query(&(
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
                market.map(Market::query),
            )))
            .await
    }

    /// Get the current user's saved shows.
    ///
    /// Requires `user-library-read`. Limit must be in the range [1..50].
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/get-users-saved-shows/).
    pub async fn get_saved_shows(
        self,
        limit: usize,
        offset: usize,
    ) -> Result<Response<Page<SavedShow>>, Error> {
        self.0
            .send_json(
                self.0
                    .client
                    .get(endpoint!("/v1/me/shows"))
                    .query(&(("limit", limit.to_string()), ("offset", offset.to_string()))),
            )
            .await
    }

    /// Get the current user's saved tracks.
    ///
    /// Requires `user-library-read`. Limit must be in the range [1..50].
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/get-users-saved-tracks/).
    pub async fn get_saved_tracks(
        self,
        limit: usize,
        offset: usize,
        market: Option<Market>,
    ) -> Result<Response<Page<SavedTrack>>, Error> {
        self.0
            .send_json(self.0.client.get(endpoint!("/v1/me/tracks")).query(&(
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
                market.map(Market::query),
            )))
            .await
    }

    /// Unsave some of the current user's saved albums.
    ///
    /// Requires `user-library-modify`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/remove-albums-user/).
    pub async fn unsave_albums<I: Iterator>(
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
                        .delete(endpoint!("/v1/me/albums"))
                        .query(&(("ids", ids.join(",")),))
                        .body("{}"),
                )
                .await
        })
        .await
    }

    /// Unsave some of the current user's saved shows.
    ///
    /// Requires `user-library-modify`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/remove-shows-user/).
    pub async fn unsave_shows<I: Iterator>(
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
                        .delete(endpoint!("/v1/me/shows"))
                        .query(&(("ids", ids.join(",")),))
                        .body("{}"),
                )
                .await
        })
        .await
    }

    /// Unsave some of the current user's saved tracks.
    ///
    /// Requires `user-library-modify`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/remove-tracks-user/).
    pub async fn unsave_tracks<I: Iterator>(
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
                        .delete(endpoint!("/v1/me/tracks"))
                        .query(&(("ids", ids.join(",")),))
                        .body("{}"),
                )
                .await
        })
        .await
    }

    /// Save albums for the current user.
    ///
    /// Requires `user-library-modify`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/save-albums-user/).
    pub async fn save_albums<I: Iterator>(
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
                        .put(endpoint!("/v1/me/albums"))
                        .query(&(("ids", ids.join(",")),))
                        .body("{}"),
                )
                .await
        })
        .await
    }

    /// Save shows for the current user.
    ///
    /// Requires `user-library-modify`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/save-shows-user/).
    pub async fn save_shows<I: Iterator>(
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
                        .put(endpoint!("/v1/me/shows"))
                        .query(&(("ids", ids.join(",")),))
                        .body("{}"),
                )
                .await
        })
        .await
    }

    /// Save tracks for the current user.
    ///
    /// Requires `user-library-modify`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/library/save-albums-user/).
    pub async fn save_tracks<I: Iterator>(
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
                        .put(endpoint!("/v1/me/tracks"))
                        .query(&(("ids", ids.join(",")),))
                        .body("{}"),
                )
                .await
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::client;

    #[tokio::test]
    async fn test_save_albums() {
        let client = client();
        let library = client.library();

        // "Wish", "The Black Parade", and "Spirit Phone"
        let albums = &[
            "0aEL0zQ4XLuxQP0j7sLlS1",
            "0FZK97MXMm5mUQ8mtudjuK",
            "4ocal2JegUDVQdP6KN1roI",
        ];
        let split = 2;
        let (saved_albums, unsaved_albums) = albums.split_at(split);

        // Store old saved status to restore
        let old = library.user_saved_albums(albums).await.unwrap().data;

        // Saving and unsaving
        library.save_albums(saved_albums).await.unwrap();
        library.unsave_albums(unsaved_albums).await.unwrap();

        // Check
        let check = library.user_saved_albums(albums).await.unwrap().data;
        let (save_check, unsave_check) = check.split_at(split);
        assert!(save_check.iter().all(|&saved| saved));
        assert!(unsave_check.iter().all(|&saved| !saved));

        // Check by finding in list
        let saved = library.get_saved_albums(50, 0, None).await.unwrap().data;
        if saved.total <= 50 {
            for saved_album in saved_albums {
                assert!(saved
                    .items
                    .iter()
                    .any(|album| album.album.id == *saved_album));
            }
            for unsaved_album in unsaved_albums {
                assert!(saved
                    .items
                    .iter()
                    .all(|album| album.album.id != *unsaved_album));
            }
        }

        // Restore
        let mut old_saved = Vec::with_capacity(albums.len());
        let mut old_unsaved = Vec::with_capacity(albums.len());
        for i in 0..albums.len() {
            if old[i] {
                &mut old_saved
            } else {
                &mut old_unsaved
            }
            .push(albums[i]);
        }
        if !old_saved.is_empty() {
            library.save_albums(&old_saved).await.unwrap();
        }
        if !old_unsaved.is_empty() {
            library.unsave_albums(&old_unsaved).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_save_shows() {
        let client = client();
        let library = client.library();

        let shows = &["5CfCWKI5pZ28U0uOzXkDHe", "6ups0LMt1G8n81XLlkbsPo"];
        let split = 1;
        let (saved_shows, unsaved_shows) = shows.split_at(split);

        // Store old saved status to restore
        let old = library.user_saved_shows(shows).await.unwrap().data;

        // Saving and unsaving
        library.save_shows(saved_shows).await.unwrap();
        library.unsave_shows(unsaved_shows).await.unwrap();

        // Check
        let check = library.user_saved_shows(shows).await.unwrap().data;
        let (save_check, unsave_check) = check.split_at(split);
        assert!(save_check.iter().all(|&saved| saved));
        assert!(unsave_check.iter().all(|&saved| !saved));

        // Check by finding in list, only if it has them all
        let saved = library.get_saved_shows(50, 0).await.unwrap().data;
        if saved.total <= 50 {
            for saved_show in saved_shows {
                assert!(saved.items.iter().any(|show| show.show.id == *saved_show));
            }
            for unsaved_show in unsaved_shows {
                assert!(saved.items.iter().all(|show| show.show.id != *unsaved_show));
            }
        }

        // Restore
        let mut old_saved = Vec::with_capacity(shows.len());
        let mut old_unsaved = Vec::with_capacity(shows.len());
        for i in 0..shows.len() {
            if old[i] {
                &mut old_saved
            } else {
                &mut old_unsaved
            }
            .push(shows[i]);
        }
        if !old_saved.is_empty() {
            library.save_shows(&old_saved).await.unwrap();
        }
        if !old_unsaved.is_empty() {
            library.unsave_shows(&old_unsaved).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_save_tracks() {
        let client = client();
        let library = client.library();

        // "Friday I'm In Love" and "Spiral of Ants"
        let tracks = &["4QlzkaRHtU8gAdwqjWmO8n", "77hzctaLvLRLAh71LwNPE3"];
        let split = 1;
        let (saved_tracks, unsaved_tracks) = tracks.split_at(split);

        // Store old saved status to restore
        let old = library.user_saved_tracks(tracks).await.unwrap().data;

        // Saving and unsaving
        library.save_tracks(saved_tracks).await.unwrap();
        library.unsave_tracks(unsaved_tracks).await.unwrap();

        // Check
        let check = library.user_saved_tracks(tracks).await.unwrap().data;
        let (save_check, unsave_check) = check.split_at(split);
        assert!(save_check.iter().all(|&saved| saved));
        assert!(unsave_check.iter().all(|&saved| !saved));

        // Check by finding in list, only if it has them all
        let saved = library.get_saved_tracks(50, 0, None).await.unwrap().data;
        if saved.total <= 50 {
            for saved_track in saved_tracks {
                assert!(saved
                    .items
                    .iter()
                    .any(|track| track.track.id.as_ref().unwrap() == *saved_track));
            }
            for unsaved_track in unsaved_tracks {
                assert!(saved
                    .items
                    .iter()
                    .all(|track| track.track.id.as_ref().unwrap() != *unsaved_track));
            }
        }

        // Restore
        let mut old_saved = Vec::with_capacity(tracks.len());
        let mut old_unsaved = Vec::with_capacity(tracks.len());
        for i in 0..tracks.len() {
            if old[i] {
                &mut old_saved
            } else {
                &mut old_unsaved
            }
            .push(tracks[i]);
        }
        if !old_saved.is_empty() {
            library.save_tracks(&old_saved).await.unwrap();
        }
        if !old_unsaved.is_empty() {
            library.unsave_tracks(&old_unsaved).await.unwrap();
        }
    }
}
