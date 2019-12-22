//! Endpoint functions to the Spotify API.
// handy macros and things to reduce duplicate code

macro_rules! request {
    (
        $token:expr,
        $method:ident $path:expr
        $(, path_params = [$($path_param:expr),*])?
        $(, query_params = {$($query_param_name:literal: $query_param_value:expr),*})?
        $(, optional_query_params = {$($optional_query_param_name:literal: $optional_query_param_value:expr),*})?
        $(, additional_query_params = $additional_query_params:expr)?
        $(, ret = $type:ty)?
    ) => {{
        #[allow(unused_mut)]
        let mut request = crate::CLIENT.request(
            reqwest::Method::$method,
            &format!(concat!("https://api.spotify.com", $path)$($(, $path_param)*)?)
        )$(.query(&[$(($query_param_name, $query_param_value)),*]))?.bearer_auth($token.get_token());
        $(
            $(
                if let Some(optional_query_param) = $optional_query_param_value {
                    request = request.query(&[($optional_query_param_name, optional_query_param)]);
                }
            )*
        )?
        $(
            for (name, value) in $additional_query_params.into_iter() {
                request = request.query(&[(name, value)]);
            }
        )?
        let response = loop {
            let response = request.try_clone().unwrap().send().await?;
            if response.status() != 429 {
                break response;
            }
            let wait = response.headers().get("Retry-After").and_then(|val| val.to_str().ok()).and_then(|secs| secs.parse::<u64>().ok());
            // 2 seconds is default retry after time; should never be used if the Spotify API and
            // my code are both correct.
            let wait = wait.unwrap_or(2);
            tokio::timer::delay_for(std::time::Duration::from_secs(wait)).await;
        };
        if !response.status().is_success() {
            return Err(response.json::<Error>().await?.into());
        }
        if cfg!(test) {
            let text = response.text().await?;
            println!("Text is {}", text);
            serde_json::from_str(&text).unwrap()
        } else {
            response.json$(::<$type>)?().await?
        }
    }};
}

mod albums;
mod artists;
mod browse;
//mod follow;
//mod library;
//mod personalization;
//mod player;
//mod playlists;
//mod search;
//mod tracks;
//mod users_profile;

pub use albums::*;
pub use artists::*;
pub use browse::*;

#[cfg(test)]
async fn token() -> crate::CCToken {
    dotenv::dotenv().unwrap();
    crate::ClientCredentials::from_env()
        .unwrap()
        .send()
        .await
        .unwrap()
}
