use aspotify::{AuthCodeFlow, ClientCredentials, Scope};
use std::fs;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // Read .env file into environment variables.
    dotenv::dotenv().unwrap();

    // Get client credentials from environment variables.
    let credentials = ClientCredentials::from_env().unwrap();

    // Get the URL to send the user to, requesting all the scopes and redirecting to a non-existant website.
    let url = aspotify::get_authorization_url(
        &credentials.id,
        &[
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
        ],
        false,
        "http://non.existant/",
    );

    // Get the user to authorize our application.
    println!("Go to this website: {}", url);

    // Receive the URL that was redirected to.
    print!("Enter the URL that you were redirected to: ");
    io::stdout().flush().unwrap();
    let mut redirect = String::new();
    io::stdin().read_line(&mut redirect).unwrap();

    // Create the authorization flow from that redirect.
    let flow = AuthCodeFlow::from_redirect(credentials, &redirect)
        .await
        .unwrap();

    // Put the refresh token in a file.
    fs::write(".refresh_token", flow.get_refresh_token()).unwrap();
}
