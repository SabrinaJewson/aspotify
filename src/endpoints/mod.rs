//! Endpoint types.
//!
//! These types are transparent, short-lived wrappers around `Client`. They avoid having an
//! enormous number of methods on the `Client` itself. They can be created from methods on
//! `Client`, so you generally won't ever need to name them.
//!
//! # Common Parameters
//!
//! These are some common parameters used in endpoint functions.
//!
//! | Parameter | Use |
//! | --- | --- |
//! | `id(s)` | The [Spotify ID(s)](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) of the required resource. |
//! | `country` | Limits the request to one particular country, so that resources not available in the country will not appear in the results. |
//! | `market` | Limits the request to one particular country, and applies [Track Relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/). |
//! | `locale` | The language of the response. It consists of an ISO-639 language code and an ISO-3166 country code (for, example, En and GBR is British English). |
//! | `limit` | When the function returns a [`Page`](../model/struct.Page.html), [`CursorPage`](../model/struct.CursorPage.html) or [`TwoWayCursorPage`](../model/struct.TwoWayCursorPage.html), this determines the maximum length of the page. |
//! | `offset` | When the function returns a [`Page`](../model/struct.Page.html), this determines what index in the larger list the page starts at. |
//! | `cursor`, `before` and `after` | When the function returns a [`CursorPage`](../model/struct.CursorPage.html) or [`TwoWayCursorPage`](../model/struct.TwoWayCursorPage.html), this determines to give the next (`cursor` or `after`) or previous (`before`) page. |
#![allow(clippy::missing_errors_doc)]

use std::future::Future;
use std::iter;
use std::time::Instant;

use futures_util::stream::{FuturesOrdered, FuturesUnordered, StreamExt, TryStreamExt};
use isocountry::CountryCode;

use crate::{Client, Error, Response};

pub use albums::*;
pub use artists::*;
pub use browse::*;
pub use episodes::*;
pub use follow::*;
pub use library::*;
pub use personalization::*;
pub use player::*;
pub use playlists::*;
pub use search::*;
pub use shows::*;
pub use tracks::*;
pub use users_profile::*;

macro_rules! endpoint {
    ($path:literal) => {
        concat!("https://api.spotify.com", $path)
    };
    ($path:literal, $($fmt:tt)*) => {
        &format!(endpoint!($path), $($fmt)*)
    };
}

mod albums;
mod artists;
mod browse;
mod episodes;
mod follow;
mod library;
mod personalization;
mod player;
mod playlists;
mod search;
mod shows;
mod tracks;
mod users_profile;

/// Endpoint function namespaces.
impl Client {
    /// Album-related endpoints.
    #[must_use]
    pub const fn albums(&self) -> Albums {
        Albums(self)
    }

    /// Artist-related endpoints.
    #[must_use]
    pub const fn artists(&self) -> Artists {
        Artists(self)
    }

    /// Endpoint functions related to categories, featured playlists, recommendations, and new
    /// releases.
    #[must_use]
    pub const fn browse(&self) -> Browse {
        Browse(self)
    }

    /// Episode-related endpoints.
    #[must_use]
    pub const fn episodes(&self) -> Episodes {
        Episodes(self)
    }

    /// Endpoint functions related to following and unfollowing artists, users and playlists.
    #[must_use]
    pub const fn follow(&self) -> Follow {
        Follow(self)
    }

    /// Endpoints relating to saving albums and tracks.
    #[must_use]
    pub const fn library(&self) -> Library {
        Library(self)
    }

    /// Endpoint functions relating to a user's top artists and tracks.
    #[must_use]
    pub const fn personalization(&self) -> Personalization {
        Personalization(self)
    }

    /// Endpoint functions related to controlling what is playing on the current user's Spotify
    /// account. (Beta)
    #[must_use]
    pub const fn player(&self) -> Player {
        Player(self)
    }

    /// Endpoint functions related to playlists.
    #[must_use]
    pub const fn playlists(&self) -> Playlists {
        Playlists(self)
    }

    /// Endpoint functions related to searches.
    #[must_use]
    pub const fn search(&self) -> Search {
        Search(self)
    }

    /// Endpoint functions related to shows.
    #[must_use]
    pub const fn shows(&self) -> Shows {
        Shows(self)
    }

    /// Endpoint functions related to tracks and audio analysis.
    #[must_use]
    pub const fn tracks(&self) -> Tracks {
        Tracks(self)
    }

    /// Endpoint functions related to users' profiles.
    #[must_use]
    pub const fn users_profile(&self) -> UsersProfile {
        UsersProfile(self)
    }
}

/// A market in which to limit the request to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Market {
    /// A country code.
    Country(CountryCode),
    /// Deduce the current country from the access token. Requires `user-read-private`.
    FromToken,
}

impl Market {
    fn as_str(self) -> &'static str {
        match self {
            Market::Country(code) => code.alpha2(),
            Market::FromToken => "from_token",
        }
    }
    fn query(self) -> (&'static str, &'static str) {
        ("market", self.as_str())
    }
}

/// A time range from which to calculate the response.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeRange {
    /// Use approximately the last 4 weeks of data.
    Short,
    /// Use approximately the last 6 months of data.
    Medium,
    /// Use several years of data.
    Long,
}

impl TimeRange {
    fn as_str(self) -> &'static str {
        match self {
            Self::Long => "long_term",
            Self::Medium => "medium_term",
            Self::Short => "short_term",
        }
    }
}

type Chunk<'a, I> = iter::Take<&'a mut iter::Peekable<I>>;

async fn chunked_sequence<I: IntoIterator, Fut, T>(
    items: I,
    chunk_size: usize,
    mut f: impl FnMut(Chunk<'_, I::IntoIter>) -> Fut,
) -> Result<Response<Vec<T>>, Error>
where
    Fut: Future<Output = Result<Response<Vec<T>>, Error>>,
{
    let mut items = items.into_iter().peekable();
    let mut futures = FuturesOrdered::new();

    while items.peek().is_some() {
        futures.push(f(items.by_ref().take(chunk_size)));
    }

    let mut response = Response {
        data: Vec::new(),
        expires: Instant::now(),
    };

    while let Some(mut r) = futures.next().await.transpose()? {
        response.data.append(&mut r.data);
        response.expires = r.expires;
    }

    Ok(response)
}

async fn chunked_requests<I: IntoIterator, Fut>(
    items: I,
    chunk_size: usize,
    mut f: impl FnMut(Chunk<'_, I::IntoIter>) -> Fut,
) -> Result<(), Error>
where
    Fut: Future<Output = Result<(), Error>>,
{
    let mut items = items.into_iter().peekable();
    let futures = FuturesUnordered::new();

    while items.peek().is_some() {
        futures.push(f(items.by_ref().take(chunk_size)));
    }

    futures.try_collect().await
}

#[cfg(test)]
fn client() -> crate::Client {
    dotenv::dotenv().unwrap();
    let mut client = crate::Client::with_refresh(
        crate::ClientCredentials::from_env().unwrap(),
        std::fs::read_to_string(".refresh_token").unwrap(),
    );
    client.debug = true;
    client
}
