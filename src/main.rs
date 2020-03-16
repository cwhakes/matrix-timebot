use ruma_client::Client;

fn main() {
    let homeserver_url = "https://riot.im".parse().unwrap();
    let client = Client::https(homeserver_url, None);


    println!("Hello, world!");
}
