use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::{Followers, Image, TypeUser};

macro_rules! inherit_user_simplified {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        to_struct!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// The name of the user; can be not available.
            display_name: Option<String>,
            /// Known public external URLs for this user.
            external_urls: HashMap<String, String>,
            /// The [Spotify user
            /// ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the
            /// user.
            id: String,
            /// The item type; `user`.
            #[serde(rename = "type")]
            item_type: TypeUser,
        });
    }
}

inherit_user_simplified!(
    /// A user object that contains less fields than UserPublic and is not documented anywhere, but
    /// is returned by some endpoints.
    UserSimplified {}
);

macro_rules! inherit_user_public {
    ($(#[$attr:meta])* $name:ident { $($(#[$f_attr:meta])* $f_name:ident : $f_ty:ty,)* }) => {
        inherit_user_simplified!($(#[$attr])* $name {
            $(
                $(#[$f_attr])*
                $f_name: $f_ty,
            )*
            /// Information about the followers of the user.
            followers: Followers,
            /// The user's profile image.
            images: Vec<Image>,
        });
    }
}

inherit_user_public!(
    /// A user object that is accessible to everyone.
    UserPublic {}
);
inherit_user_public!(
    /// A user object only accessible to the user themselves; does not work with Client Credentials
    /// flow.
    UserPrivate {
        /// The country of the user, as set in their account profile. Requires `user-read-private`.
        /// This is an ISO 3166 2-letter country code.
        country: Option<String>,
        /// The user's email address, which is not necessarily a real email address. Requires
        /// `user-read-email`.
        email: Option<String>,
        /// The user's Spotify subscription level. Requires `user-read-private`.
        product: Option<Subscription>,
    }
);

impl UserPublic {
    /// Convert to a `UserSimplified`.
    #[must_use]
    pub fn simplify(self) -> UserSimplified {
        UserSimplified {
            display_name: self.display_name,
            external_urls: self.external_urls,
            id: self.id,
            item_type: TypeUser,
        }
    }
}
impl From<UserPublic> for UserSimplified {
    fn from(user: UserPublic) -> Self {
        user.simplify()
    }
}
impl UserPrivate {
    /// Convert to a `UserPublic`.
    #[must_use]
    pub fn publicize(self) -> UserPublic {
        UserPublic {
            display_name: self.display_name,
            external_urls: self.external_urls,
            id: self.id,
            followers: self.followers,
            images: self.images,
            item_type: TypeUser,
        }
    }
    /// Convert to a `UserSimplified`.
    #[must_use]
    pub fn simplify(self) -> UserSimplified {
        self.publicize().simplify()
    }
}
impl From<UserPrivate> for UserSimplified {
    fn from(user: UserPrivate) -> Self {
        user.simplify()
    }
}

/// The subscription level; premium or free.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Subscription {
    /// The user is subscribed to Spotify Premium.
    Premium,
    /// The user isn't subscribed to Spotify Premium. Also known as `open`.
    #[serde(alias = "open")]
    Free,
}
