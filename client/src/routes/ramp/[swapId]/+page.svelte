<script lang="ts">
  import { Hash } from 'lucide-svelte';
  import type { PageData, RouteParams } from './$types';
  import ExecuteTransfer from './ExecuteTransfer.svelte';
  import Success from './Success.svelte';
  import { sessionState } from '@sv-use/core';
  import Wait from './Wait.svelte';
  import { Card } from '$lib/components/ui/card';

  let { data }: { data: PageData } = $props();

  let hasDoneTransaction = sessionState(
    'hasDoneTransaction-' + data.swap.id,
    '',
  );

  let txLink = $derived.by(() => {
    return data.swap.txExplorerTemplate?.replace(
      '{0}',
      hasDoneTransaction.current,
    );
  });

  $inspect(txLink);

  let actionRequired = $derived(data.deposits?.length ?? 0 > 0);
</script>

<div class="flex flex-col items-center justify-center w-screen h-screen">
  <Card class="text-white w-1/3">
    {#if actionRequired && !hasDoneTransaction.current}
      <ExecuteTransfer
        action={data.deposits![0]}
        ondone={(tx) => (hasDoneTransaction.current = tx)}
      />
    {:else if data.swap.status === 'completed'}
      <Success />
    {:else}
      <Wait {txLink} />
    {/if}
  </Card>
</div>
