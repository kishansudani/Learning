// connect with ethereum node
mod eth_rpc;

use eth_rpc::Ethereum_client;
use ethers::prelude::*;
use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let provider = Ethereum_client::new().await?;
    

    Ok(())
}
