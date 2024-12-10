// This represents the Cairo smart contract model for Land NFTs in the Realms game
// The data structure below mirrors the one in Cairo, which we'll use to fetch and display land data
import { derived, writable } from 'svelte/store';

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

export const mockPlayerAddress = '0x1234567890abcdef';

// Add token address constants
const TOKEN_ADDRESSES = {
    'LORDS': '0x124afc6f5320456789fed23432d11f11acc11111',
    'ETH': '0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7',
    'STARK': '0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d'
};

// Then use it in mockLandData
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
            token_address: null,
            pool_key: null
        };
    }
    
    // 1% chance for occupied land to be owned by mock player (should result in ~10 lands on average)
    const isMockPlayerOwner = Math.random() < 0.01;
    const tokenType = ['LORDS', 'ETH', 'STARK'][Math.floor(Math.random() * 3)] as keyof typeof TOKEN_ADDRESSES;
    
    return {
        location: index,
        block_date_bought: Math.floor(Date.now() / 1000) - Math.floor(Math.random() * 10000000),
        owner: isMockPlayerOwner ? mockPlayerAddress : `0x${Math.random().toString(16).slice(2, 42)}`,
        sell_price: Math.floor(Math.random() * 1000000),
        token_used: tokenType,
        token_address: TOKEN_ADDRESSES[tokenType],
        pool_key: ['pool_lords', 'pool_eth', 'pool_stark'][Math.floor(Math.random() * 3)]
    };
});

export const landStore = writable(mockLandData);

export const playerLands = derived(landStore, $landStore => 
    $landStore.filter(land => land.owner === mockPlayerAddress)
);
