use aspotify::{Client, ClientCredentials, CurrentlyPlaying, PlayingType};

#[tokio::main]
async fn main() {
    // Read the client credentials from the .env file
    dotenv::dotenv().unwrap();

    // Make the Spotify client
    let client = Client::with_refresh(
        ClientCredentials::from_env().unwrap(),
        std::fs::read_to_string(".refresh_token").unwrap(),
    );

    // Call the Spotify API to get the playing track
    let playing = client.player().get_playing_track(None).await.unwrap().data;

    // Print out the results
    match playing {
        Some(CurrentlyPlaying {
            item: Some(item), ..
        }) => {
            print!("Currently playing ");
            match item {
                PlayingType::Track(track) => print!("the track {}", track.name),
                PlayingType::Episode(ep) => print!("the episode {}", ep.name),
                PlayingType::Ad(ad) => print!("the advert {}", ad.name),
                PlayingType::Unknown(item) => print!("an unknown track {}", item.name),
            }
            println!(".");
        }
        Some(CurrentlyPlaying { item: None, .. }) => println!("Currently playing an unknown item."),
        None => println!("Nothing currently playing."),
    }
}
