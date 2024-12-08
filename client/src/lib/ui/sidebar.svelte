<script lang="ts">
    import { playerLands } from '$lib/api/land';
    let open = $state(false);
</script>

{#if open}
    <div class="fixed top-0 left-0 h-screen w-64 bg-white shadow-lg z-50">
        <div class="flex justify-between items-center px-2 border-b">
            <h1>Sidebar</h1>
            <button class="p-2" aria-label="Close sidebar" onclick={() => (open = false)}>
                ×
            </button>
        </div>
        
        <div class="p-4 overflow-y-auto max-h-[calc(100vh-48px)]">
            <h2 class="text-lg font-bold mb-4">Your Lands</h2>
            {#each $playerLands as land}
                <div class="mb-4 p-3 border rounded shadow-sm">
                    <p class="font-medium">Location: {land.location}</p>
                    <p>Block date bought: {land.block_date_bought}</p>
                    <p>Sell Price: {land.sell_price}</p>
                    {#if land.token_used}
                        <p>Token: {land.token_used}</p>
                    {/if}
                    {#if land.pool_key}
                        <p>Pool: {land.pool_key}</p>
                    {/if}
                </div>
            {/each}
        </div>
    </div>
{:else}
    <button class="p-2 bg-white rounded" aria-label="Open sidebar" onclick={() => (open = true)}>
        <svg width="24" height="24" viewBox="0 0 24 24">
            <line x1="4" y1="6" x2="20" y2="6" stroke="black" stroke-width="2"/>
            <line x1="4" y1="12" x2="20" y2="12" stroke="black" stroke-width="2"/>
            <line x1="4" y1="18" x2="20" y2="18" stroke="black" stroke-width="2"/>
        </svg>
    </button>
{/if}
