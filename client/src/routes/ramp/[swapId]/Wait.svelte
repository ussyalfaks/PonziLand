<script>
  const { txLink } = $props();
</script>

<div class="text-center">
  <p class="text-lg font-bold">Swap is in progress...</p>
  <p>
    You can check the status of your transaction <a
      href={txLink}
      target="_blank">here</a
    >.
  </p>
  <p>Please wait for the swap to complete.</p>
</div>
