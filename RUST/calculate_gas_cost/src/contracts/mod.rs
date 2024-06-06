use ethers::prelude::*;
use std::{error::Error, io};

struct Connect {
    source: Provider<Http>,
    dest: Provider<Http>,
    tx: Option<TransactionReceipt>,
}

impl Connect {
    fn new(source: String, dest: String) -> Result<Connect, Box<dyn Error>> {
        let source = Provider::<Http>::try_from(source)?;
        let dest = Provider::<Http>::try_from(dest)?;

        Ok(Connect {
            source,
            dest,
            tx: None,
        })
    }

    async fn fetch_tx(&mut self, tx: TxHash) -> Result<(), Box<dyn Error>> {
        let recepit = self.source.get_transaction_receipt(tx).await?.unwrap();
        self.tx = Some(recepit);
        Ok(())
    }

    async fn get_dest_gas_price(&self) -> Result<U256, Box<dyn Error>> {
        let gas_price = self.dest.get_gas_price().await?;
        Ok(gas_price)
    }

    fn get_used_gas(&self) -> Result<U256, Box<dyn Error>> {
        let gas_used = self.tx.as_ref().unwrap().gas_used.unwrap();
        Ok(gas_used)
    }
}

fn read_input(msg: &str) -> Option<String> {
    let mut buf = String::new();

    println!("{msg}");
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read data");

    if buf.trim().is_empty() {
        return None;
    }
    println!();

    Some(buf.trim().to_string())
}

fn wei_to_gwei(wei: U256) -> U256 {
    let gwei_divisor = U256::from(1_000_000_000u64);
    wei / gwei_divisor
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn Error>> {
    let source = read_input("Enter Source RPC")
        .unwrap_or_else(|| "https://data-seed-prebsc-1-s2.binance.org:8545/".to_string());

    let dest = read_input("Enter destination RPC")
        .unwrap_or_else(|| "https://bsc-dataseed1.binance.org/".to_string());

    let tx_hash = read_input("Enter TX hash from source").unwrap();

    let tx_hash: TxHash = tx_hash.parse()?;

    let mut connection = Connect::new(source, dest)?;

    connection.fetch_tx(tx_hash).await?;

    let gas_used = connection.get_used_gas()?;

    let gas_price = connection.get_dest_gas_price().await?;

    let gas_price = wei_to_gwei(gas_price);

    let contract_deployment_multiplier: U256 = read_input("Deployment multiplier")
        .unwrap_or_else(|| "2".to_string())
        .parse()
        .unwrap();

    let gas_multiplier: U256 = read_input("Enter TX hash from source")
        .unwrap_or_else(|| "2".to_string())
        .parse()
        .unwrap();

    let latest_price = (gas_used * contract_deployment_multiplier) * (gas_price * gas_multiplier);

    println!("Current Destination GWEI is {}", latest_price);

    println!(
        "Current Destination GWEI is {gas_price} \n
        current deployment gas used = {gas_used},
        number of time to deploy this same contract = {contract_deployment_multiplier},
        gas multipliter = {gas_multiplier},
        using formula = (used gas * deployment count) * (current gas * gas multiplier) = deployment cost\n
        ({gas_used} * {contract_deployment_multiplier}) * ({gas_price} * {gas_multiplier}) = {latest_price}"
    );

    Ok(())
}
