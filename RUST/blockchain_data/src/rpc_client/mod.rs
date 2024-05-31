use ethers::prelude::*;
use std::error::Error;

pub struct RpcClient {
    provider: Provider<Http>,
}

impl RpcClient {
    pub fn new(connection_str: &str) -> Result<RpcClient, Box<dyn Error>> {
        let connection = Provider::<Http>::try_from(connection_str)?;

        Ok(RpcClient {
            provider: connection,
        })
    }

    pub async fn get_block(&self, block_number: u64) -> Result<Block<H256>, Box<dyn Error>> {
        let block = self.provider.get_block(block_number).await?;

        let result = match block {
            Some(value) => Ok(value),
            None => Err(()),
        };

        Ok(result.unwrap())
    }

    pub fn parse_block(&self, block: Block<H256>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub async fn get_block_range(&self, start: u64, end: u64) {}

    pub async fn get_address_balance(&self, address: H160) {}
}
