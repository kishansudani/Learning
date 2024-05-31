mod db;
mod rpc_client;

use db::ClientConnection;
use rpc_client::RpcClient;

const DB_URI: &str = "mongodb://localhost:27017";
const RPC: &str = "https://rpc-testnet.vii.exchange/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientConnection::new(DB_URI).await?;
    let rpc = RpcClient::new(RPC)?;

    let block = rpc.get_block(10).await?;

    println!("{:?}", block);

    Ok(())
}
