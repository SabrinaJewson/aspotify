//! Endpoint functions relating to artists.

use crate::*;
use serde::Deserialize;

/// Get information about an artist.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/artists/get-artist/).
pub async fn get_artist(token: &AccessToken, id: &str) -> Result<Artist, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/artists/{}",
        path_params = [id],
        ret = Artist
    ))
}

/// Get information about several artists.
///
/// Maximum number of artists is 50.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/artists/get-several-artists/).
pub async fn get_artists(
    token: &AccessToken,
    ids: &[&str],
) -> Result<Vec<Artist>, EndpointError<Error>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    #[derive(Deserialize)]
    struct Artists {
        artists: Vec<Artist>,
    };

    Ok(request!(
        token,
        GET "/v1/artists",
        query_params = {"ids": ids.join(",")},
        ret = Artists
    )
    .artists)
}

/// Get an artist's albums.
///
/// The include_groups parameter can specify which groups to include (album, single, appears_on,
/// compilation). If not specified it includes them all. Limit and offset control the attributes of
/// the resulting Page. Limit has a maximum of 50.
///
/// If no market is specified this function is likely to give duplicate albums, one for each
/// market, so it is advised to provide a market.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/artists/get-artists-albums/).
pub async fn get_artist_albums(
    token: &AccessToken,
    id: &str,
    include_groups: Option<&[AlbumGroup]>,
    limit: usize,
    offset: usize,
    country: Option<CountryCode>,
) -> Result<Page<ArtistsAlbum>, EndpointError<Error>> {
    Ok(request!(
        token,
        GET "/v1/artists/{}/albums",
        path_params = [id],
        query_params = {"limit": limit.to_string(), "offset": offset.to_string()},
        optional_query_params = {
            "include_groups": include_groups.map(|groups|
                groups
                    .iter()
                    .map(|&group| group.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            "country": country.map(|c| c.alpha2())
        },
        ret = Page<ArtistsAlbum>
    ))
}

/// Get an artist's top tracks.
///
/// Unlike most other endpoints, the country code is required. The response contains up to 10 tracks which are the artist's top tracks.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/artists/get-artists-top-tracks/).
pub async fn get_artist_top(
    token: &AccessToken,
    id: &str,
    market: Market,
) -> Result<Vec<Track>, EndpointError<Error>> {
    #[derive(Deserialize)]
    struct Tracks {
        tracks: Vec<Track>,
    };

    Ok(request!(
        token,
        GET "/v1/artists/{}/top-tracks",
        path_params = [id],
        query_params = {"country": market.as_str()},
        ret = Tracks
    )
    .tracks)
}

/// Get an artist's related artists.
///
/// These artists are similar in style to the given artist.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/artists/get-related-artists/).
pub async fn get_related_artists(
    token: &AccessToken,
    id: &str,
) -> Result<Vec<Artist>, EndpointError<Error>> {
    #[derive(Deserialize)]
    struct Artists {
        artists: Vec<Artist>,
    };

    Ok(request!(
        token,
        GET "/v1/artists/{}/related-artists",
        path_params = [id],
        ret = Artists
    )
    .artists)
}

#[cfg(test)]
mod tests {
    use crate::endpoints::token;
    use crate::*;

    #[tokio::test]
    async fn test_get_artist() {
        let artist = get_artist(&token().await, "0L8ExT028jH3ddEcZwqJJ5")
            .await
            .unwrap();
        assert_eq!(artist.id, "0L8ExT028jH3ddEcZwqJJ5");
        assert_eq!(artist.name, "Red Hot Chili Peppers");
    }

    #[tokio::test]
    async fn test_get_artists() {
        let artists = get_artists(
            &token().await,
            &["0L8ExT028jH3ddEcZwqJJ5", "0gxyHStUsqpMadRV0Di1Qt"],
        )
        .await
        .unwrap();
        assert_eq!(artists.len(), 2);
        assert_eq!(artists[0].name, "Red Hot Chili Peppers");
        assert_eq!(artists[1].name, "Rick Astley");
    }

    #[tokio::test]
    async fn test_get_artist_albums() {
        let albums = get_artist_albums(
            &token().await,
            "0L8ExT028jH3ddEcZwqJJ5",
            Some(&[AlbumGroup::Single]),
            2,
            1,
            Some(CountryCode::GBR),
        )
        .await
        .unwrap();
        assert_eq!(albums.limit, 2);
        assert_eq!(albums.offset, 1);
        assert_eq!(albums.items.len(), 2);
        assert!(albums
            .items
            .iter()
            .all(|album| album.album_group == AlbumGroup::Single));
        assert!(albums.items.iter().all(|album| album
            .artists
            .iter()
            .find(|artist| artist.name == "Red Hot Chili Peppers")
            .is_some()));
    }

    #[tokio::test]
    async fn test_get_artist_top() {
        let top = get_artist_top(
            &token().await,
            "0L8ExT028jH3ddEcZwqJJ5",
            Market::Country(CountryCode::GBR),
        )
        .await
        .unwrap();
        assert!(top.iter().all(|track| track
            .artists
            .iter()
            .find(|artist| artist.name == "Red Hot Chili Peppers")
            .is_some()));
    }
}
