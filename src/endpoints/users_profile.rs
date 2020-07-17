use crate::{Client, Error, Response, UserPrivate, UserPublic};

/// Endpoint functions related to users' profiles.
#[derive(Debug, Clone, Copy)]
pub struct UsersProfile<'a>(pub &'a Client);

impl UsersProfile<'_> {
    /// Get current user's profile.
    ///
    /// Reading the user's email requires `user-read-email`, reading their country and product
    /// subscription level requires `user-read-private`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/).
    pub async fn get_current_user(self) -> Result<Response<UserPrivate>, Error> {
        self.0
            .send_json(self.0.client.get(endpoint!("/v1/me")))
            .await
    }

    /// Get a user's profile.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/users-profile/get-users-profile/).
    pub async fn get_user(self, id: &str) -> Result<Response<UserPublic>, Error> {
        self.0
            .send_json(self.0.client.get(endpoint!("/v1/users/{}", id)))
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::client;

    #[tokio::test]
    async fn test_get_user() {
        let user = client()
            .users_profile()
            .get_user("spotify")
            .await
            .unwrap()
            .data;
        assert_eq!(user.display_name.unwrap(), "Spotify");
        assert_eq!(user.external_urls.len(), 1);
        assert_eq!(
            user.external_urls["spotify"],
            "https://open.spotify.com/user/spotify"
        );
        assert_eq!(user.id, "spotify");
        assert_eq!(user.images.len(), 1);
        assert_eq!(
            user.images[0].url,
            "https://i.scdn.co/image/ab6775700000ee8555c25988a6ac314394d3fbf5"
        );
    }

    #[tokio::test]
    async fn test_get_current() {
        let user = client()
            .users_profile()
            .get_current_user()
            .await
            .unwrap()
            .data;
        assert_eq!(user.external_urls.len(), 1);
        assert_eq!(
            user.external_urls["spotify"],
            format!("https://open.spotify.com/user/{}", user.id)
        );
    }
}
