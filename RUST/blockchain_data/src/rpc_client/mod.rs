pub mod parse_block;

use ethers::prelude::*;
use parse_block::Blocks;
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

    pub async fn get_block_range(&self, mut start: u64, end: u64) -> Result<(), Box<dyn Error>> {
        while start <= end {
            let block = self.get_block(start).await?;
            let parsed_block = Blocks::new(block)?;

            let number = parsed_block.get_number()?;
            println!("{:?}", number);
            parsed_block.get_print_all_transaction_hash()?;

            start += 1;
        }

        Ok(())
    }
}
