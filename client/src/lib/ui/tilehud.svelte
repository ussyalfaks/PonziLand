<script lang="ts">
    import { tileHUD } from '$lib/stores';
    import { mousePosCoords } from '$lib/stores';
</script>

<!-- Permanent mouse coordinates display -->
<div class="fixed top-0 right-0 bg-white p-2 rounded shadow-lg z-50">
    <p>Mouse: {$mousePosCoords ? `${$mousePosCoords.x}, ${$mousePosCoords.y}` : ''}</p>
</div>

<!-- Tile HUD with close button -->
{#if $tileHUD}
    <div class="fixed bottom-0 right-0 bg-white p-4 rounded shadow-lg z-50">
        <button 
            class="absolute top-4 right-4 text-gray-500 hover:text-gray-700"
            on:click={() => $tileHUD = null}
        >
            ✕
        </button>
        <h1 class="text-lg font-bold mb-2">Tile HUD</h1>
        <div class="space-y-2">
            <p>Location: ({Math.floor($tileHUD.location % 64) + 1}, {Math.floor($tileHUD.location / 64) + 1})</p>
            <p>Owner: {$tileHUD.owner ?? 'Unclaimed'}</p>
        </div>
    </div>
{/if}
