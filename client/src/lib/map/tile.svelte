<script lang="ts">    

    import { tileHUD } from '$lib/stores';

    let { type, location, owner } = $props<{
        type: string;
        location: number;
        owner: string | null;
    }>()

    function handleClick() {
        console.log('clicked');
        $tileHUD = {
            location: location,
            owner: owner
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