use ethers::prelude::*;

pub fn ether_to_wei(ether: f64) -> U256 {
    let wei_per_ether = U256::exp10(18); // 10^18
    let wei_value = ether * wei_per_ether.as_u64() as f64;
    U256::from(wei_value as u64)
}

pub fn float_to_wei(float: f64) -> U256 {
    let wei: U256 = U256::from((float * 1e18) as u64);
    wei
}
