# aspotify

Asynchronous Rust Spotify client.

## Testing

In order to test, you first need to make a Spotify Developer account. In your list of whitelisted URLs, add `http://non.existant/`. Get your Client ID and Client Secret and put them in a `.env` file in the crate root like this:
```
CLIENT_ID=some value
CLIENT_SECRET=some value
```
Then, run `cargo run --example refresh_file`. Follow the instructions shown. If everything went successfully, you should see a file called `.refresh_token` in your crate root. This file contains a refresh token that will be used to run all the tests. For more infomation about this process, see `examples/refresh_file.rs`.

These tests will make temporary changes to your account, however they will all be reverted. You will also need an unrestricted non-private Spotify client open to get all the tests to run successfully.
