use aspotify::{Client, ClientCredentials};

#[tokio::main]
async fn main() {
    // Read the client credentials from the .env file
    dotenv::dotenv().unwrap();

    // Make the Spotify client using client credentials flow
    let client = Client::new(ClientCredentials::from_env().unwrap());

    // Call the Spotify API to get information on an artist
    let artist = client
        .artists()
        .get_artist("2WX2uTcsvV5OnS0inACecP")
        .await
        .unwrap()
        .data;

    println!(
        "Found artist named {} with {} followers",
        artist.name, artist.followers.total
    );
}
