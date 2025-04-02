<script lang="ts">
  import { uiStore } from '$lib/stores/ui.store.svelte';
  import AuctionModal from '$lib/components/auction/auction-modal.svelte';
  import BuyModal from '$lib/components/buy/buy-modal.svelte';
  import LandHud from './land/hud/land-hud.svelte';
  import LandInfoModal from '$lib/components/land/land-info-modal.svelte';
  import Toolbar from '$lib/components/toolbar/toolbar.svelte';
  import WalletLookup from './wallet.svelte';
  import TutoProgression from './tuto-progression.svelte';
</script>

<div class="z-50 absolute top-0 left-0">
  <!-- Wallet Lookup -->
  <!-- <WalletLookup /> -->

  <!-- Leaderboard-->
  <!-- <Leaderboard /> -->

  <!-- Tile HUD -->

  <div class="absolute">
    <LandHud />
  </div>

  <!-- <Toolbar /> -->

  <!-- Modals -->
  <!-- {#if uiStore.showModal}
    {#if uiStore.modalType == 'buy'}
      <BuyModal />
    {:else if uiStore.modalType == 'bid'}
      <AuctionModal />
    {:else if uiStore.modalType == 'land-info'}
      <LandInfoModal />
    {/if}
  {/if}
   -->
  <TutoProgression />
</div>
