<script lang="ts">
    import Sidebar from './sidebar.svelte';
    import TileHUD from './tilehud.svelte';
    import Modal from './buymodal.svelte';
    import type { TileInfo, BuyData } from '$lib/interfaces';

    let showModal = $state<boolean>(false);
    let modalData = $state<TileInfo | null>(null);

    function handleTileBuy(info: TileInfo) {
        modalData = info;
        showModal = true;
    }

    function handleCancel(): void {
        showModal = false;
    }

    function handleBuy(data: BuyData): void {
        console.log("Buying land with data:", data);
        // TODO: call buyTile function + front end sugar
        showModal = false;
    }
</script>

<div class="z-50 absolute top-0 left-0">
    <Sidebar />
    <TileHUD onBuyTile={handleTileBuy} />
    {#if showModal && modalData}
        <Modal
            onCancel={handleCancel}
            onBuy={handleBuy}
            data={modalData}
        />
    {/if}
</div>
