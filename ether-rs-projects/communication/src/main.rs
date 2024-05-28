// connect with ethereum node
mod eth_rpc;

use dotenv::dotenv;
use eth_rpc::Ethereum_client;
use ethers::prelude::*;
use ethers::{core::utils::Anvil, providers::Http, types::H160, utils::parse_ether};
use eyre::Result;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv().ok();

    let _: String = std::env::var("SIGNER_PRIVET_KEY").expect("Failed to read signer privet key");

    let test = Ethereum_client::new().unwrap();

    let to_adr: H160 = "0x000000000000000000000000000000000000dead"
        .parse()
        .unwrap();

    let val = parse_ether(1u64).unwrap();

    // let _ = test.get_block(10).await;
    let _ = test.create_tx(to_adr, val).await.unwrap();

    Ok(())
}
