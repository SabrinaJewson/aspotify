use aspotify::{Client, ClientCredentials};

#[tokio::main]
async fn main() {
    // Read the client credentials from the .env file
    dotenv::dotenv().unwrap();

    // Make the Spotify client
    let client = Client::with_refresh(
        ClientCredentials::from_env().unwrap(),
        std::fs::read_to_string(".refresh_token").unwrap(),
    );

    let recent = client
        .player()
        .get_recently_played(50, None, None)
        .await
        .unwrap()
        .data;

    // Print the results
    match recent {
        Some(aspotify::TwoWayCursorPage { items, .. }) => {
            println!("Play history:");
            for item in items {
                println!(
                    "{}: '{}' by {}",
                    item.played_at,
                    item.track.name,
                    item.track
                        .artists
                        .into_iter()
                        .map(|artist| artist.name)
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }
        None => {
            println!("Nothing in play history.");
        }
    }
}
