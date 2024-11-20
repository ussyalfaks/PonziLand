use starknet::ContractAddress;

#[derive(Copy, Drop, Serde, Debug)]
#[dojo::model]
pub struct land {
    #[key]
    pub location: u64, // 64 x 64 land
    pub remaining: u8,
    pub can_move: bool,
}