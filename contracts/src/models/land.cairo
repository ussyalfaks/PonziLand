use starknet::ContractAddress;

#[derive(Copy, Drop, Serde, Debug)]
#[dojo::model]
pub struct land {
    #[key]
    pub index: u64, // 64 x 64 land => value from 0 to 4095
    pub sell_price: u128,
    pub sell_token: ContractAddress,
    pub liquidity_pool: ContractAddress, // must be a pair between sell_token and main_currency(stark / eth / usdc / lords)
}
