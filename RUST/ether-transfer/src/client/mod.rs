mod utils;
use dotenv::dotenv;
// use ethers::prelude::*;
use ethers::middleware::{MiddlewareBuilder, NonceManagerMiddleware, SignerMiddleware};
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{TransactionReceipt, TransactionRequest, H160, U256};
use ethers::utils::format_units;
use std::{error::Error, thread, time::Duration};

const MINIMUM_BALANCE: f64 = 0.0001; // in ether
const NATIVE_TRANSFER_COST: u64 = 21000 * 3;

struct EtherumTx {
    from: H160,
    to: H160,
    min_balance: U256,
    signer: NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl EtherumTx {
    async fn new(p_key: String, to: String, rpc: String) -> Result<EtherumTx, Box<dyn Error>> {
        let to: H160 = to.parse()?;
        let min_balance = utils::ether_to_wei(MINIMUM_BALANCE);

        let provider = Provider::<Http>::try_from(rpc)?;

        let chain_id = provider.get_chainid().await?;
        let wallet: LocalWallet = p_key.parse::<LocalWallet>()?;
        let from: H160 = wallet.address();

        let singer = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id.as_u64()));

        let nonce_mgr = singer.nonce_manager(from);

        Ok(EtherumTx {
            from,
            to,
            min_balance,
            signer: nonce_mgr,
        })
    }

    async fn get_gas_price(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.signer.get_gas_price().await?)
    }

    async fn fetch_balance(&self) -> Result<U256, Box<dyn Error>> {
        Ok(self.signer.get_balance(self.from, None).await?)
    }

    async fn send_tx(&self) -> Result<Option<TransactionReceipt>, Box<dyn Error>> {
        let current_balance = self.fetch_balance().await?;

        if current_balance < self.min_balance {
            return Ok(None);
        }

        let current_gas_price = self.get_gas_price().await?;

        let current_gas_price: f64 = format_units(current_gas_price, "ether")
            .unwrap()
            .parse()
            .unwrap();

        let gas_price = NATIVE_TRANSFER_COST as f64 * current_gas_price;

        let gas_price = utils::float_to_wei(gas_price);

        println!("Sending Balance {}", current_balance - gas_price);

        let tx = TransactionRequest::new()
            .from(self.from)
            .to(self.to)
            .value(current_balance - gas_price);

        let tx_recepit = self.signer.send_transaction(tx, None).await?.await?;
        Ok(tx_recepit)
    }

    async fn start(&self) -> Result<(), Box<dyn Error>> {
        let mut counter: u64 = 0;
        loop {
            println!("Running this bot: {counter}...");
            counter += 1;
            let receipt = self.send_tx().await?;

            if receipt.is_none() {
                thread::sleep(Duration::from_secs(1));
                continue;
            }

            let tx_receipt = receipt.unwrap();
            println!("{:?}", tx_receipt);

            thread::sleep(Duration::from_secs(1));
        }
    }
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let p_key = std::env::var("PRIVET_KEY")?;
    let rpc = std::env::var("RPC_URL")?;
    let to = std::env::var("TO")?;

    let obj = EtherumTx::new(p_key, to, rpc).await?;

    obj.start().await?;

    Ok(())
}
