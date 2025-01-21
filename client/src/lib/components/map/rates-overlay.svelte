<script lang="ts">
  import type { YieldInfo } from '$lib/interfaces';
  import { type SelectedLandType } from '$lib/stores/stores.svelte';

  let { land } = $props<{ land: SelectedLandType }>();

  let yieldInfo: YieldInfo[] | undefined;

  $effect(() => {
    console.log('land from rates', land);
    if (land) {
      land
        .getYieldInfo()
        .then((res: YieldInfo[] | undefined) => {
          yieldInfo = res;
          console.log('yield info response:', res);
        })
        .catch((error: any) => {
          console.error('Error fetching yield info:', error);
        });
    }
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
