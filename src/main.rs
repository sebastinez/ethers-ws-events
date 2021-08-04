mod org;
use ethers::{
    prelude::StreamExt,
    providers::{Provider, Ws},
    types::Address,
};
use org::{AnchoredFilter, Org};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Defining and parsing org address
    let address = "0x797Ff53d759e80193c91ACd486E7742a271DD23E".parse::<Address>()?;

    // Connecting to open Websocket Server, polling every 7 seconds
    let ws = Ws::connect("ws://localhost:8546").await?;

    // Instantiating new Provider
    let client = Provider::new(ws);

    // Instantiating a new Org contract at the defined address
    let org_contract = Org::new(address, client.into());

    // Creating a new Event Filter with the defined event name and a stream to listen to
    let event = org_contract.event_for_name::<AnchoredFilter>("Anchored")?;
    let mut stream = event.stream().await?;

    // Wait for websocket messages on this filter and decode and print them
    while let Some(item) = stream.next().await {
        match item {
            Ok(log) => {
                println!("{:?}", log);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    Ok(())
}
