use crate::*;
use serde::Deserialize;

/// Get information about an album.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/albums/get-album/).
pub async fn get_album(
    token: &AccessToken,
    id: &str,
    market: Option<Market>,
) -> Result<Album, EndpointError<Error>> {
    Ok(
        request!(token, GET "/v1/albums/{}", path_params = [id], optional_query_params = {"market": market.map(|m| m.to_string())}, ret = Album),
    )
}

/// Get information about several albums.
///
/// Maximum number of albums is 20.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/albums/get-several-albums/).
pub async fn get_albums(
    token: &AccessToken,
    ids: &[&str],
    market: Option<Market>,
) -> Result<Vec<Album>, EndpointError<Error>> {
    #[derive(Deserialize)]
    struct Albums {
        albums: Vec<Album>,
    }

    Ok(request!(token, GET "/v1/albums", query_params = {"ids": ids.join(",")}, optional_query_params = {"market": market.map(|m| m.to_string())}, ret = Albums).albums)
}

/// Get an album's tracks.
///
/// It does not return all the tracks, but a page of tracks. Limit and offset determine attributes
/// of the page. Limit has a maximum of 50.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/albums/get-albums-tracks/).
pub async fn get_album_tracks(
    token: &AccessToken,
    id: &str,
    limit: usize,
    offset: usize,
    market: Option<Market>,
) -> Result<Page<TrackSimplified>, EndpointError<Error>> {
    Ok(
        request!(token, GET "/v1/albums/{}/tracks", path_params = [id], query_params = {"limit": limit, "offset": offset}, optional_query_params = {"market": market.map(|m| m.to_string())}, ret = Page<TrackSimplified>),
    )
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::endpoints::token;

    #[tokio::test]
    async fn test_get_album() {
        let album = get_album(&token().await, "03JPFQvZRnHHysSZrSFmKY", None)
            .await
            .unwrap();
        assert_eq!(album.name, "Inside In / Inside Out");
        assert_eq!(album.artists.len(), 1);
        assert_eq!(album.artists[0].name, "The Kooks");
        assert_eq!(album.tracks.total, 14);
        assert_eq!(album.tracks.items[0].name, "Seaside");
    }

    #[tokio::test]
    async fn test_get_albums() {
        let albums = get_albums(
            &token().await,
            &["29Xikj6r9kQDSbnZWCCW2s", "0axbvqBOAejn8DgTUcJAp1"],
            None,
        )
        .await
        .unwrap();
        assert_eq!(albums.len(), 2);
        assert_eq!(albums[0].name, "Neotheater");
        assert_eq!(albums[1].name, "Absentee");
    }

    #[tokio::test]
    async fn test_get_album_tracks() {
        let tracks = get_album_tracks(&token().await, "62U7xIHcID94o20Of5ea4D", 3, 1, None)
            .await
            .unwrap();
        assert_eq!(tracks.limit, 3);
        assert_eq!(tracks.total, 10);
        assert_eq!(tracks.offset, 1);
        assert_eq!(tracks.items.len(), 3);
        assert_eq!(tracks.items[0].name, "Make Believe");
        assert_eq!(tracks.items[1].name, "I Won't Hold You Back");
        assert_eq!(tracks.items[2].name, "Good for You");
    }
}
