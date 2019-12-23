//! Endpoint functions to the Spotify API.
//!
//! A parameter named `id` always refers to the [Spotify
//! ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) of the required
//! resource.
//!
//! `market` and `country` parameters limit the request to one particular country (so resources not
//! available in the country will not appear in the results).
//!
//! `locale` parameters determine the language of the response, and consist of an ISO-639 language
//! code and an ISO-3166 country code (for example, En and GBR is British English).
//!
//! When the response is a Page object, the `limit` and `offset` parameters are included; `limit`
//! is the maximum number of resources per page, and `offset` is the offset into the list. You can
//! think of a Page as a slice of a larger list; limit is the length of the slice and offset is the
//! index of the slice.
//!
//! CursorPage objects work slightly differently; instead of an `offset` parameter they have a
//! `cursor` parameter that points to the next element in the list. It can be fed back in to the
//! endpoint to get the next element in the list, and a cursor to the one after that (if it
//! exists).
//!
//! TODO: Make cursorpage Stream (async Iterator)

pub use albums::*;
pub use artists::*;
pub use browse::*;
pub use follow::*;
pub use library::*;
pub use personalization::*;
pub use player::*;
pub use playlists::*;
pub use search::*;
pub use tracks::*;
pub use users_profile::*;
use std::fmt::{self, Display, Formatter};
use isocountry::CountryCode;

macro_rules! request {
    (
        $token:expr,
        $method:ident $path:expr
        $(, path_params = [$($path_param:expr),*])?
        $(, header_params = {$($header_param_name:literal: $header_param_value:expr),*})?
        $(, query_params = {$($query_param_name:literal: $query_param_value:expr),*})?
        $(, optional_query_params = {$($optional_query_param_name:literal: $optional_query_param_value:expr),*})?
        $(, additional_query_params = $additional_query_params:expr)?
        $(, body = $body:expr)?
        $(, ret = $type:ty)?
    ) => {{
        #[allow(unused_mut)]
        let mut request = crate::CLIENT.request(
            reqwest::Method::$method,
            &format!(concat!("https://api.spotify.com", $path)$($(, $path_param)*)?)
        )
            $($(.header($header_param_name, $header_param_value))*)?
            $(.query(&[$(($query_param_name, $query_param_value)),*]))?
            $(.body($body))?
            .bearer_auth(&$token.token);
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
            let text = response.text().await?;
            println!("Request is {:?}", request);
            println!("EText is {}", text);
            return Err(serde_json::from_str::<Error>(&text).unwrap().into());
            //return Err(response.json::<Error>().await?.into());
        } else {
            ()
        }
        $(
            if cfg!(test) {
                let text = response.text().await?;
                println!("Text is {}", text);
                serde_json::from_str::<$type>(&text).unwrap()
            } else {
                response.json::<$type>().await?
            }
        )?
    }};
}

mod albums;
mod artists;
mod browse;
mod follow;
mod library;
mod personalization;
mod player;
mod playlists;
mod search;
mod tracks;
mod users_profile;

/// A market in which to limit the request to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Market {
    /// A country code.
    Country(CountryCode),
    /// Deduce the current country from the access token. Not valid for client credentials.
    FromToken,
}

impl Display for Market {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Market::Country(code) => code.alpha2(),
            Market::FromToken => "from_token",
        })
    }
}

/// A time range from which to calculate the response.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeRange {
    /// Use several years of data.
    LongTerm,
    /// Use approximately the last 6 months of data.
    MediumTerm,
    /// Use approximately the last 4 weeks of data.
    ShortTerm,
}

impl Display for TimeRange {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::LongTerm => "long_term",
            Self::MediumTerm => "medium_term",
            Self::ShortTerm => "short_term",
        })
    }
}

#[cfg(test)]
async fn token() -> crate::AccessToken {
    dotenv::dotenv().unwrap();
    crate::AuthCodeFlow::from_refresh(
        crate::ClientCredentials::from_env().unwrap(),
        std::fs::read_to_string(".refresh_token").unwrap(),
    )
    .send()
    .await
    .unwrap()
}
