<script lang="ts">
  import Particles, { particlesInit } from '@tsparticles/svelte';
  import { loadSlim } from '@tsparticles/slim';
  import type { Container } from '@tsparticles/engine';

  let particlesConfig = {
    particles: {
      color: {
        value: [
          '#d3d3d3',
          '#c0c0c0',
          '#a9a9a9',
          '#808080',
          '#696969',
          '#505050',
        ],
      },
      links: {
        enable: false,
      },
      move: {
        enable: true,
        direction: 'bottom' as const,
        speed: 0.5,
      },
      number: {
        value: 200,
      },
      size: {
        value: { min: 2, max: 5 },
        random: true,
      },
      opacity: {
        value: 0.5,
        random: true,
      },
    },
    interactivity: {
      events: {
        onClick: {
          enable: false,
        },
        onHover: {
          enable: false,
        },
      },
    },
  };

  let onParticlesLoaded = (event: CustomEvent<{ container: Container }>) => {
    const particlesContainer = event.detail.container;
  };

  void particlesInit(async (engine) => {
    await loadSlim(engine);
  });
</script>

<Particles
  id="tsparticles"
  class="absolute z-30 h-full w-full overflow-hidden pointer-events-none"
  options={particlesConfig}
  on:particlesLoaded={onParticlesLoaded}
/>

<style>
  #tsparticles {
    position: absolute;
    width: 100%;
    height: 100%;
    background-color: transparent;
  }
</style>
