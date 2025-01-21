<script lang="ts">
  import type { YieldInfo } from '$lib/interfaces';
  import { type SelectedLandType } from '$lib/stores/stores.svelte';

  let { land } = $props<{ land: SelectedLandType }>();

  $inspect('land from rates-overlay', land);

  let yieldInfo: YieldInfo[] | undefined;

  $effect(() => {
    land.getYieldInfo().then((res: YieldInfo[]) => {
      yieldInfo = res;
      $inspect('yeildInfo', yieldInfo);
    });
  });
</script>

<!-- <div
  class="absolute inset-0 grid grid-cols-3 grid-rows-3"
  style="transform: translate(-33.33%, -33.33%); width: 300%; height: 300%;"
>
  {#each Array(9) as _, i}
    <div class="border-2 border-blue-400/50 bg-blue-400/10"></div>
  {/each}
</div> -->
