use starknet::ContractAddress;

#[derive(Copy, Drop, Serde, Debug)]
#[dojo::model]
pub struct land {
    #[key]
    pub location: u64, // 64 x 64 land
    pub block_date_bought: u64,
    pub owner: ContractAddress,
    pub sell_price: u64,
    pub token_used: ContractAddress,
    pub pool_key: ContractAddress, // The Liquidity Pool Key
}


// Auction model is only active if the land is not owned
// Auction is held with the liquid token(prob Lords)
#[derive(Copy, Drop, Serde, Debug)]
#[dojo::model]
pub struct auction_info {
    #[key]
    pub location: u64,
    pub auction_start_time: u64,
    pub last_bid_time: u64,
    pub auction_price: u64,
}