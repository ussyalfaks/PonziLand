<script lang="ts">
  import { onMount } from 'svelte';

  let snowflakes = Array.from({ length: 200 }, (_, i) => ({
    id: i,
    x: Math.random() * 100 + 'vw',
    offset: Math.random() * 20 - 10 + 'vw',
    yoyoTime: Math.random() * 0.5 + 0.3,
    yoyoY: Math.random() * 100 + 'vh',
    scale: Math.random(),
    fallDuration: Math.random() * 20 + 10 + 's',
    fallDelay: Math.random() * -30 + 's',
    opacity: Math.random(),
  }));

  onMount(() => {
    const styleSheet = document.styleSheets[0];
    snowflakes.forEach((snow) => {
      const keyframes = `
          @keyframes fall-${snow.id} {
            ${Math.round(snow.yoyoTime * 100)}% {
              transform: translate(calc(${snow.x} + ${snow.offset}), ${snow.yoyoY}) scale(${snow.scale});
            }
            to {
              transform: translate(calc(${snow.x} + (${snow.offset} / 2)), 100vh) scale(${snow.scale});
            }
          }
        `;
      styleSheet.insertRule(keyframes, styleSheet.cssRules.length);
    });
  });
</script>

<div class="absolute z-30 h-full w-full overflow-hidden pointer-events-none">
  {#each snowflakes as snow}
    <div
      class="snow"
      style="
        opacity: {snow.opacity};
        transform: translate({snow.x}, -10px) scale({snow.scale});
        animation: fall-{snow.id} {snow.fallDuration} {snow.fallDelay} linear infinite;
      "
    ></div>
  {/each}
</div>

<style>
  .snow {
    position: absolute;
    width: 10px;
    height: 10px;
    background: white;
    border-radius: 50%;
  }
</style>
