<script lang="ts">    

    import { tileHUD } from '$lib/stores/stores';

    let { type, location, owner, sellPrice, tokenUsed, tokenAddress } = $props<{
        type: string;
        location: number;
        owner: string | null;
        sellPrice: number;
        tokenUsed: string | null;
        tokenAddress: string | null;
    }>()

    function handleClick() {
        console.log('clicked');
        $tileHUD = {
            location: location,
            owner: owner,
            sellPrice: sellPrice,
            tokenUsed: tokenUsed,
            tokenAddress: tokenAddress
        };
    }
</script>

<div on:click={handleClick} class="tile {type}"></div>

<style>
    .tile {
        width: 32px;
        height: 32px;
        border: 1px solid #44656b;
    }

    .grass {
        background-image: url('/tiles/grass.jpg');
        background-size: cover;
    }

    .house {
        background-image: 
            url('/tiles/house_1.png'),
            url('/tiles/grass.jpg'); 
        background-size: contain, cover;
        background-repeat: no-repeat, repeat;
        background-position: center, center;
    }
</style>