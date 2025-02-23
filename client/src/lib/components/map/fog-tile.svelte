<script lang="ts">
  let { left, top, width, height } = $props<{
    left: number;
    top: number;
    width: number;
    height: number;
  }>();
</script>

<div
  class="empty-zone"
  style="
      left: {left}px;
      top: {top}px;
      width: {width}px;
      height: {height}px;
    "
/>

<style>
  .empty-zone {
    position: absolute;
    background-color: rgba(255, 0, 0, 0.5);
    pointer-events: none;
    z-index: 1;
  }
</style>
