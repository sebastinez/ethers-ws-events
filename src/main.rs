mod org;
use ethers::{
    prelude::StreamExt,
    providers::{Provider, Ws},
    types::Address,
};
use org::{AnchoredFilter, Org};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Org address
    let address = "0x797Ff53d759e80193c91ACd486E7742a271DD23E".parse::<Address>()?;

    let ws = Ws::connect("ws://localhost:8546").await?;

    // connect to the network
    let client = Provider::new(ws);

    // create the contract object at the address
    let org_contract = Org::new(address, client.into());

    let event = org_contract.event_for_name::<AnchoredFilter>("Anchored")?;
    let mut stream = event.stream().await?;

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
