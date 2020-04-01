use aspotify::{AuthCodeFlow, ClientCredentials};

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let flow = AuthCodeFlow::from_refresh(
        ClientCredentials::from_env().unwrap(),
        std::fs::read_to_string(".refresh_token").unwrap(),
    );

    let recent = aspotify::get_recently_played(&flow.send().await.unwrap(), 50, None, None)
        .await
        .unwrap();

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
