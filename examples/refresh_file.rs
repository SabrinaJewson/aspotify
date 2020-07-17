use std::fs;
use std::io::{self, Write};

use aspotify::{Client, ClientCredentials, Scope};

#[tokio::main]
async fn main() {
    // Read .env file into environment variables.
    dotenv::dotenv().unwrap();

    // Create the Spotify client from the credentials in the env variables.
    let client = Client::new(ClientCredentials::from_env().unwrap());

    // Get the URL to send the user to, requesting all the scopes and redirecting to a non-existant website.
    let (url, state) = aspotify::authorization_url(
        &client.credentials.id,
        [
            Scope::UgcImageUpload,
            Scope::UserReadPlaybackState,
            Scope::UserModifyPlaybackState,
            Scope::UserReadCurrentlyPlaying,
            Scope::Streaming,
            Scope::AppRemoteControl,
            Scope::UserReadEmail,
            Scope::UserReadPrivate,
            Scope::PlaylistReadCollaborative,
            Scope::PlaylistModifyPublic,
            Scope::PlaylistReadPrivate,
            Scope::PlaylistModifyPrivate,
            Scope::UserLibraryModify,
            Scope::UserLibraryRead,
            Scope::UserTopRead,
            Scope::UserReadRecentlyPlayed,
            Scope::UserFollowRead,
            Scope::UserFollowModify,
        ]
        .iter()
        .copied(),
        false,
        "http://non.existant/",
    )
    .await;

    // Get the user to authorize our application.
    println!("Go to this website: {}", url);

    // Receive the URL that was redirected to.
    print!("Enter the URL that you were redirected to: ");
    io::stdout().flush().unwrap();
    let mut redirect = String::new();
    io::stdin().read_line(&mut redirect).unwrap();

    // Create the refresh token from the redirected URL.
    client.redirected(&redirect, &state).await.unwrap();

    // Put the refresh token in a file.
    fs::write(".refresh_token", client.refresh_token().await.unwrap()).unwrap();
}
