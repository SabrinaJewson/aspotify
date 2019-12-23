//! aspotify is an asynchronous client to the [Spotify
//! API](https://developer.spotify.com/documentation/web-api/). It is similar to rspotify, but is
//! asynchronous and has a very different API.
//!
//! Everything is in modules, but that is just to make documentation cleaner; re-exports make
//! everything flat in the crate root.
//!
//! # Examples
//! ```no_run
//! # async {
//! use aspotify::{ClientCredentials, CCFlow};
//!
//! // CCFlow is an object that holds your client credentials, and caches access tokens if it can.
//! // This from_env function tries to read the CLIENT_ID and CLIENT_SECRET environment variables.
//! // You can use the dotenv crate to read it from a file.
//! let flow = CCFlow::new(ClientCredentials::from_env()
//!     .expect("CLIENT_ID and CLIENT_SECRET not found.")
//! );
//!
//! // Gets the album "Favourite Worst Nightmare" from Spotify, with no specified market.
//! let album = aspotify::get_album(
//!     &flow.send().await.unwrap(),
//!     "1XkGORuUX2QGOEIL4EbJKm",
//!     None
//! ).await.unwrap();
//! # };
//! ```

pub mod authorization;
pub mod endpoints;
pub mod model;
mod util;

pub use authorization::*;
pub use endpoints::*;
pub use model::*;

use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}
