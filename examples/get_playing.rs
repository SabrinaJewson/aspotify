use aspotify::{AuthCodeFlow, ClientCredentials, CurrentlyPlaying, PlayingType};

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let flow = AuthCodeFlow::from_refresh(
        ClientCredentials::from_env().unwrap(),
        std::fs::read_to_string(".refresh_token").unwrap(),
    );

    let playing = aspotify::get_playing_track(&flow.send().await.unwrap(), None)
        .await
        .unwrap();

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
