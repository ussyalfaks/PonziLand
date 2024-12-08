interface Token {
    name: string;
    address: string;
    lpAddress: string;
}

interface ModalData {
    location: number;
    sellPrice: number;
    tokenUsed: string;
    owner?: string;
}

interface TileInfo {
    location: number;
    sellPrice: number;
    tokenUsed: string;
    owner?: string;
}

interface BuyData {
    tokens: Array<{
        name: string;
        address: string;
        lpAddress: string;
    }>;
    stakeAmount: string;
    sellPrice: string;
}