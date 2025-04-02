<script>
  import { Card, CardContent } from '$lib/components/ui/card';
  import LandHudAuction from './land-hud-auction.svelte';
  import { tutorialProgression } from '../../stores';
</script>

{#if $tutorialProgression > 3}
  <Card class="fixed bottom-0 right-0 z-50 w-96 bg-ponzi">
    {#if $tutorialProgression === 5}
      <LandHudAuction />
      <!-- {:else if $selectedLandMeta.type === 'grass'}
      <LandHudEmpty />
    {:else if isOwner}
      <LandHudOwned />
    {:else}
      <LandHudOther /> -->
    {/if}
  </Card>
{/if}
