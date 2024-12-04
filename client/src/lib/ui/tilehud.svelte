<script lang="ts">
    import { tileHUD } from '$lib/stores';
</script>
{#if $tileHUD}
    <div class="fixed top-0 right-0 bg-white p-4 rounded shadow-lg z-50">
        <h1 class="text-lg font-bold mb-2">Tile HUD</h1>
            <div class="space-y-2">
                <p>Location: ({Math.floor($tileHUD.location % 64) + 1}, {Math.floor($tileHUD.location / 64) + 1})</p>
                <p>Owner: {$tileHUD.owner ?? 'Unclaimed'}</p>
            </div>
    </div>
{/if}
