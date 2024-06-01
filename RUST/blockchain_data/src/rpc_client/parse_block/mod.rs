use ethers::prelude::*;
use std::error::Error;

pub struct Blocks {
    hash: Option<H256>,
    parent_hash: Option<H256>,
    uncles_hash: Option<H256>,
    author: Option<H160>,
    state_root: Option<H256>,
    transactions_root: Option<H256>,
    receipts_root: Option<H256>,
    number: Option<U64>,
    gas_used: Option<U256>,
    gas_limit: Option<U256>,
    extra_data: Option<Bytes>,
    logs_bloom: Option<Bloom>,
    timestamp: Option<U256>,
    difficulty: Option<U256>,
    total_difficulty: Option<U256>,
    uncles: Option<Vec<TxHash>>,
    transactions: Option<Vec<TxHash>>,
    size: Option<U256>,
    mix_hash: Option<TxHash>,
    nonce: Option<H64>,
    base_fee_per_gas: Option<U256>,
    blob_gas_used: Option<U256>,
    excess_blob_gas: Option<U256>,
    withdrawals: Option<Vec<Withdrawal>>,
    parent_beacon_block_root: Option<TxHash>,
}

impl Default for Blocks {
    fn default() -> Self {
        Blocks {
            hash: Default::default(),
            parent_hash: None,
            uncles_hash: None,
            author: Default::default(),
            state_root: None,
            transactions_root: None,
            receipts_root: None,
            number: Default::default(),
            gas_used: None,
            gas_limit: None,
            extra_data: None,
            logs_bloom: Default::default(),
            timestamp: None,
            difficulty: None,
            total_difficulty: Default::default(),
            uncles: None,
            transactions: None,
            size: Default::default(),
            mix_hash: Default::default(),
            nonce: Default::default(),
            base_fee_per_gas: Default::default(),
            blob_gas_used: Default::default(),
            excess_blob_gas: Default::default(),
            withdrawals: Default::default(),
            parent_beacon_block_root: Default::default(),
        }
    }
}

impl Blocks {
    pub fn new(block: Block<H256>) -> Result<Self, Box<dyn Error>> {
        Ok(Blocks {
            hash: block.hash,
            parent_hash: Some(block.parent_hash),
            uncles_hash: Some(block.uncles_hash),
            author: block.author,
            state_root: Some(block.state_root),
            transactions_root: Some(block.transactions_root),
            receipts_root: Some(block.receipts_root),
            number: block.number,
            gas_used: Some(block.gas_used),
            gas_limit: Some(block.gas_limit),
            extra_data: Some(block.extra_data),
            logs_bloom: block.logs_bloom,
            timestamp: Some(block.timestamp),
            difficulty: Some(block.difficulty),
            total_difficulty: block.total_difficulty,
            uncles: Some(block.uncles),
            transactions: Some(block.transactions),
            size: block.size,
            mix_hash: block.mix_hash,
            nonce: block.nonce,
            base_fee_per_gas: block.base_fee_per_gas,
            blob_gas_used: block.blob_gas_used,
            excess_blob_gas: block.excess_blob_gas,
            withdrawals: block.withdrawals,
            parent_beacon_block_root: block.parent_beacon_block_root,
            ..Default::default()
        })
    }

    pub fn get_hash(&self) -> Result<TxHash, Box<dyn Error>> {
        Ok(self.hash.unwrap())
    }
    pub fn get_parent_hash(&self) -> Result<TxHash, Box<dyn Error>> {
        Ok(self.parent_hash.unwrap())
    }

    pub fn get_uncles_hash(&self) -> Result<TxHash, Box<dyn Error>> {
        Ok(self.uncles_hash.unwrap())
    }

    pub fn get_author(&self) -> Result<H160, Box<dyn Error>> {
        Ok(self.author.unwrap())
    }

    pub fn get_state_root(&self) -> Result<TxHash, Box<dyn Error>> {
        Ok(self.state_root.unwrap())
    }

    pub fn get_transactions_root(&self) -> Result<TxHash, Box<dyn Error>> {
        Ok(self.transactions_root.unwrap())
    }

    pub fn get_receipts_root(&self) -> Result<TxHash, Box<dyn Error>> {
        Ok(self.receipts_root.unwrap())
    }

    pub fn get_number(&self) -> Result<U64, Box<dyn Error>> {
        Ok(self.number.unwrap())
    }

    pub fn get_gas_used(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.gas_used.unwrap())
    }

    pub fn get_gas_limit(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.gas_limit.unwrap())
    }

    pub fn get_extra_data(&self) -> Result<Bytes, Box<dyn Error>> {
        Ok(self.extra_data.clone().unwrap())
    }

    pub fn get_logs_bloom(&self) -> Result<Bloom, Box<dyn Error>> {
        Ok(self.logs_bloom.unwrap())
    }

    pub fn get_timestamp(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.timestamp.unwrap())
    }

    pub fn get_difficulty(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.difficulty.unwrap())
    }

    pub fn get_total_difficulty(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.total_difficulty.unwrap())
    }

    pub fn get_uncles(&self) -> Result<Vec<TxHash>, Box<dyn Error>> {
        Ok(self.uncles.clone().unwrap())
    }

    pub fn get_transactions(&self) -> Result<Vec<TxHash>, Box<dyn Error>> {
        Ok(self.transactions.clone().unwrap())
    }

    pub fn get_size(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.size.unwrap())
    }

    pub fn get_mix_hash(&self) -> Result<TxHash, Box<dyn Error>> {
        Ok(self.mix_hash.unwrap())
    }

    pub fn get_nonce(&self) -> Result<H64, Box<dyn Error>> {
        Ok(self.nonce.unwrap())
    }

    pub fn get_base_fee_per_gas(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.base_fee_per_gas.unwrap())
    }

    pub fn get_blob_gas_used(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.blob_gas_used.unwrap())
    }

    pub fn get_excess_blob_gas(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.excess_blob_gas.unwrap())
    }

    pub fn get_withdrawals(&self) -> Result<Vec<Withdrawal>, Box<dyn Error>> {
        Ok(self.withdrawals.clone().unwrap())
    }

    pub fn get_parent_beacon_block_root(&self) -> Result<TxHash, Box<dyn Error>> {
        Ok(self.parent_beacon_block_root.unwrap())
    }

    pub fn get_print_all_transaction_hash(&self) -> Result<(), Box<dyn Error>> {
        for tx in self.transactions.iter() {
            println!("{:?}", tx);
        }

        Ok(())
    }
}
