use ethers::prelude::*;

const RPC_URL: &str = "https://rpc-testnet.guapcoinx.com/";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let block_number: U64 = provider.get_block_number().await?;
    println!("Current block number: {}", block_number);

    Ok(())
}