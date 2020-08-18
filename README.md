# aspotify

Asynchronous Rust Spotify client.

## Description

Aspotify is a Rust wrapper for the Spotify API. It provides Rust structures around all of Spotify's
[Object Model](https://developer.spotify.com/documentation/web-api/reference/object-model/) and
functions around all their endpoints.

## Authorization

All Spotify endpoints require authorization. There are two forms of authorization provided by this
library; Client Credentials and Authorization Code. To use either, you first need a [Spotify
Developer](https://developer.spotify.com/dashboard/applications) account, which is free. Then you
can use endpoints with your Client ID and Client Secret with Client Credentials, or perform actions
on behalf of a user with oauth2 and Authorization Code.

## Testing

In order to test, you first need to add `http://non.existant/` in your Spotify whitelisted URLs. Get
your Client ID and Client Secret and put them in a `.env` file in the crate root like this:
```
CLIENT_ID=some value
CLIENT_SECRET=some value
```
Then, run `cargo run --example refresh_file`. Follow the instructions shown. If everything went
successfully, you should see a file called `.refresh_token` in your crate root. This file contains a
refresh token that will be used to run all the tests. For more infomation about this process, see
`examples/refresh_file.rs`.

These tests will make temporary changes to your account, however they will all be reverted. You will
also need an unrestricted non-private Spotify client open to get all the tests to run successfully,
and you must not have any songs in your queue.

## TODO

Automatically send multiple requests when the limit is above Spotify's limit for functions that
return `Page`/`CursorPage`.
