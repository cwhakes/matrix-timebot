use dotenv;
use ruma_client::Client;
use ruma_client::api::r0;
use ruma_client::events::{
    room::message::{MessageEventContent, TextMessageEventContent},
    EventType,
};
use ruma_client::identifiers::RoomAliasId;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    dotenv::from_filename("creds.env")?;
    let env: HashMap<_,_> = std::env::vars().collect();

    let homeserver_url = "https://matrix.org".parse()?;
    let client = Client::https(homeserver_url, None);
    println!("Client created");

    client
        .log_in(
            env.get("MATRIX_USERNAME").ok_or("Missing username")?.clone(),
            env.get("MATRIX_PASSWORD").ok_or("Missing password")?.clone(),
            None, None)
        .await?;

    let room_id = client
        .request(r0::alias::get_alias::Request {
            room_alias: RoomAliasId::try_from("#cwhakes:matrix.org")?,
        })
        .await?.room_id;
    
    client
        .request(r0::membership::join_room_by_id::Request {
            room_id: room_id.clone(),
            third_party_signed: None,
        })
        .await?;

    client.request(r0::message::create_message_event::Request {
        room_id: room_id.clone(),
        event_type: EventType::RoomMessage,
        txn_id: "1".to_owned(),
        data: MessageEventContent::Text(TextMessageEventContent {
            body: "Hello World!".to_owned(),
            format: None,
            formatted_body: None,
            relates_to: None,
        }),
    })
    .await?;
    
    println!("{}", room_id);

    Ok(())
}
