use ethers::prelude::*;
use std::error::Error;

const RPC: &str = "https://eth.llamarpc.com";

pub struct Ethereum_client {
    provider: Provider<Http>,
}

impl Ethereum_client {
    pub async fn new() -> Result<Ethereum_client, Box<dyn Error>> {
        Ok(Ethereum_client {
            provider: Provider::<Http>::try_from(RPC)?,
        })
    }

    pub async fn get_block_number(&self) -> Result<U64, Box<dyn Error>> {
        Ok(self.provider.get_block_number().await?)
    }

    pub async fn get_chain_id(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.provider.get_chainid().await?)
    }

    pub async fn get_mempool(&self) -> Result<TxpoolContent, Box<dyn Error>> {
        Ok(self.provider.txpool_content().await?)
    }
}
