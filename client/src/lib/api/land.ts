// This represents the Cairo smart contract model for Land NFTs in the Realms game
// The data structure below mirrors the one in Cairo, which we'll use to fetch and display land data

// Cairo struct reference:
// pub struct land {
//     #[key]
//     pub location: u64, // 64 x 64 land grid - location calculated as: y * 64 + x
//     pub block_date_bought: u64,
//     pub owner: ContractAddress,
//     pub sell_price: u64,
//     pub token_used: ContractAddress, // Currency token used (LORDS, ETH, etc.)
//     pub pool_key: ContractAddress, // The Liquidity Pool Key (pool_lords, pool_eth, etc.)
// }

// Mock data for testing/development
export const mockLandData = Array.from({ length: 64 * 64 }, (_, index) => ({
    location: index, // 0 to 4095 (64*64 - 1)
    block_date_bought: Math.floor(Date.now() / 1000) - Math.floor(Math.random() * 10000000), // Random past timestamp
    owner: `0x${Math.random().toString(16).slice(2, 42)}`, // Random address
    sell_price: Math.floor(Math.random() * 1000000), // Random price
    token_used: ['LORDS', 'ETH', 'STARK'][Math.floor(Math.random() * 3)], // Random token
    pool_key: ['pool_lords', 'pool_eth', 'pool_stark'][Math.floor(Math.random() * 3)], // Random pool
}));