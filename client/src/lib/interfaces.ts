export interface Token {
    name: string;
    address: string;
    lpAddress: string;
    images: {
        icon: string;
        castle: {
            basic: string;
            advanced: string;
            premium: string;
        }
    }
}

export interface ModalData {
    location: number;
    sellPrice: number;
    tokenUsed: string;
    owner?: string;
}

export interface TileInfo {
    location: number;
    sellPrice: number;
    tokenUsed: string;
    owner?: string;
    tokenAddress: string;
}

export interface BuyData {
    tokens: Array<{
        name: string;
        address: string;
        lpAddress: string;
    }>;
    stakeAmount: string;
    sellPrice: string;
}