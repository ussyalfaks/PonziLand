export interface Token {
    name: string;
    address: string;
    lpAddress: string;
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