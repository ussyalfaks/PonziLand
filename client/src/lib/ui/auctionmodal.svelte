<script lang="ts">
    import type { TileInfo } from '$lib/interfaces';
    let { onCancel, onBuy } = $props();


    function handleBuyClick() {
        onBuy();
    }

    function handleCancelClick() {
        onCancel();
    }
</script>

<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg p-6 max-w-md w-full">
        <h2 class="text-2xl font-bold mb-4">Auction Modal</h2>
        
        <div class="flex justify-end space-x-4 mt-6">
            <button 
                class="px-4 py-2 border rounded-md hover:bg-gray-100 transition-colors"
                onclick={handleCancelClick}
            >
                Cancel
            </button>
            <button 
                class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors"
                onclick={handleBuyClick}
            >
                Buy
            </button>
        </div>
    </div>
</div>