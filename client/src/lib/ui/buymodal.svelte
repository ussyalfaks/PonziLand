<script lang="ts">
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

    let { onCancel, onBuy, data } = $props();

    let selectedTokens = $state<Token[]>([]);
    let manualTokenAddress = $state('');
    let manualLPAddress = $state('');
    let stakeAmount = $state('');
    let sellPrice = $state('');
    let showManualInput = $state(false);
    
    let availableTokens = $state<Token[]>([
        { name: 'ETH', address: '0x...', lpAddress: '0x...' },
        { name: 'USDC', address: '0x...', lpAddress: '0x...' },
        { name: 'USDT', address: '0x...', lpAddress: '0x...' }
    ]);

    function handleTokenSelect(event: Event) {
        const selectElement = event.target as HTMLSelectElement;
        selectedTokens = [...selectElement.selectedOptions].map(option => availableTokens[+option.value]);
    }

    function addManualToken() {
        if (manualTokenAddress && manualLPAddress) {
            selectedTokens = [
                ...selectedTokens,
                {
                    name: 'Custom Token',
                    address: manualTokenAddress,
                    lpAddress: manualLPAddress
                }
            ];
            manualTokenAddress = '';
            manualLPAddress = '';
            showManualInput = false;
        }
    }

    function handleBuyClick() {
        if (selectedTokens.length && stakeAmount && sellPrice) {
            onBuy({
                tokens: selectedTokens,
                stakeAmount,
                sellPrice
            });
        }
    }

    function handleCancelClick() {
        onCancel();
    }
</script>

<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg p-6 w-96 max-w-full">
        <h2 class="text-2xl font-bold mb-4">Buy Land</h2>
        
        <div class="mb-4">
            <label class="block text-sm font-medium mb-1">Select Tokens</label>
            <select 
                multiple 
                class="w-full border rounded-md p-2 h-32"
                on:change={handleTokenSelect}
            >
                {#each availableTokens as token, i}
                    <option value={i}>{token.name}</option>
                {/each}
            </select>
        </div>
        
        <button 
            class="mb-4 text-blue-600 hover:text-blue-800"
            on:click={() => showManualInput = !showManualInput}
        >
            + Add Token Manually
        </button>
        
        {#if showManualInput}
            <div class="mb-4 space-y-2">
                <input
                    type="text"
                    bind:value={manualTokenAddress}
                    placeholder="Token Address"
                    class="w-full border rounded-md p-2"
                />
                <input
                    type="text"
                    bind:value={manualLPAddress}
                    placeholder="LP Address"
                    class="w-full border rounded-md p-2"
                />
                <button 
                    class="bg-blue-500 text-white px-4 py-2 rounded-md hover:bg-blue-600"
                    on:click={addManualToken}
                >
                    Add Token
                </button>
            </div>
        {/if}
        
        <div class="mb-4">
            <label class="block text-sm font-medium mb-1">Stake Amount</label>
            <input
                type="number"
                bind:value={stakeAmount}
                placeholder="Enter stake amount"
                class="w-full border rounded-md p-2"
            />
        </div>
        
        <div class="mb-6">
            <label class="block text-sm font-medium mb-1">Sell Price</label>
            <input
                type="number"
                bind:value={sellPrice}
                placeholder="Enter sell price"
                class="w-full border rounded-md p-2"
            />
        </div>
        
        <div class="flex justify-end space-x-4">
            <button 
                class="px-4 py-2 border rounded-md hover:bg-gray-100"
                on:click={handleCancelClick}
            >
                Cancel
            </button>
            <button 
                class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600"
                on:click={handleBuyClick}
            >
                Buy
            </button>
        </div>
    </div>
</div>
