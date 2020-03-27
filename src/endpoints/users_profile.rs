//! Endpoint functions related to users' profiles.

use crate::*;

/// Get current user's profile.
///
/// Reading the user's email requires `user-read-email`, reading their country and product
/// subscription level requires `user-read-private`.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/).
pub async fn get_current_user(token: &AccessToken) -> Result<UserPrivate, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/me",
        ret = UserPrivate
    ))
}

/// Get a user's profile.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/users-profile/get-users-profile/).
pub async fn get_user(token: &AccessToken, id: &str) -> Result<UserPublic, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/users/{}",
        path_params = [id],
        ret = UserPublic
    ))
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;

    #[tokio::test]
    async fn test_get_user() {
        let user = get_user(&token().await, "spotify").await.unwrap();
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
        let user = get_current_user(&token().await).await.unwrap();
        assert_eq!(user.external_urls.len(), 1);
        assert_eq!(
            user.external_urls["spotify"],
            format!("https://open.spotify.com/user/{}", user.id)
        );
    }
}
