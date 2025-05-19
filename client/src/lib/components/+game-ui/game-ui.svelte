<script lang="ts" context="module">
  import type { BaseLand } from '$lib/api/land';
  import type { LandWithActions } from '$lib/api/land.svelte';
  // import LandHud from '$lib/components/land/hud/land-hud.svelte';
  import WidgetLandInfo from '$lib/components/+game-ui/widgets/land-info/widget-land-info.svelte';
  import WidgetMyLands from '$lib/components/+game-ui/widgets/my-lands/widget-my-lands.svelte';
  import WidgetSettings from '$lib/components/+game-ui/widgets/settings/widget-settings.svelte';
  import WidgetLauncher from '$lib/components/+game-ui/widgets/widget-launcher.svelte';
  import Draggable from '$lib/components/ui/draggable.svelte';
  import { widgetsStore } from '$lib/stores/widgets.store';
  import WidgetWallet from './widgets/wallet/widget-wallet.svelte';

  // Function to open buy land widget
  export function openBuyLandWidget(land: BaseLand) {
    widgetsStore.addWidget({
      id: `buy-land-${land.location.x}-${land.location.y}`,
      type: 'buy-land',
      position: { x: 300, y: 100 },
      dimensions: { width: 200, height: 50 },
      isMinimized: false,
      isOpen: true,
      data: { location: land.location },
    });
  }

  // Function to open land info widget
  export function openLandInfoWidget(land: LandWithActions) {
    console.log('openLandInfoWidget', land);

    widgetsStore.addWidget({
      id: `land-info-${land.location}`,
      type: 'land-info',
      position: { x: 300, y: 100 },
      dimensions: { width: 800, height: 600 },
      isMinimized: false,
      isOpen: true,
      data: { location: land.location },
    });
  }
</script>

<div
  class="z-50 absolute top-0 left-0 right-0 bottom-0"
  style="pointer-events: none;"
>
  <WidgetLauncher />

  {#each Object.entries($widgetsStore) as [id, widget]}
    {#if widget.isOpen}
      <Draggable
        {id}
        type={widget.type}
        initialPosition={widget.position}
        initialDimensions={widget.dimensions}
      >
        {@const type = widget.type}
        {#if type === 'wallet'}
          <WidgetWallet />
        {:else if type === 'land-hud'}
          <!-- <LandHud /> -->
        {:else if type === 'land-info' && widget.data}
          <WidgetLandInfo data={widget.data} />
        {:else if type === 'settings'}
          <WidgetSettings />
        {:else if type === 'my-lands'}
          <WidgetMyLands />
        {/if}
      </Draggable>
    {/if}
  {/each}
</div>
