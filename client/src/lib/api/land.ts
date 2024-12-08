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
export const mockLandData = Array.from({ length: 64 * 64 }, (_, index) => {
    // 30% chance of land being occupied
    const isOccupied = Math.random() < 0.3;
    
    if (!isOccupied) {
        return {
            location: index,
            block_date_bought: 0,
            owner: null,
            sell_price: 0,
            token_used: null,
            pool_key: null
        };
    }
    
    // 15% chance for occupied land to be owned by mock player (should result in ~10 lands on average)
    const isMockPlayerOwner = Math.random() < 0.15;
    
    return {
        location: index,
        block_date_bought: Math.floor(Date.now() / 1000) - Math.floor(Math.random() * 10000000),
        owner: isMockPlayerOwner ? mockPlayerAddress : `0x${Math.random().toString(16).slice(2, 42)}`,
        sell_price: Math.floor(Math.random() * 1000000),
        token_used: ['LORDS', 'ETH', 'STARK'][Math.floor(Math.random() * 3)],
        pool_key: ['pool_lords', 'pool_eth', 'pool_stark'][Math.floor(Math.random() * 3)]
    };
});

export const mockPlayerAddress = '0x1234567890abcdef';