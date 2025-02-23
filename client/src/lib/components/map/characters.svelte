<script lang="ts">
  import { onDestroy } from 'svelte';
  import type { Tile } from '$lib/api/tile-store.svelte';

  let {
    tiles,
  }: {
    tiles: Tile[][];
  } = $props();

  const hexToInt = (hexLocation: string) => parseInt(hexLocation, 16);

  const interval = setInterval(() => {
    const activeLands = tiles.flat().filter((tile) => tile?.type !== 'grass');
    const shuffledLands = [...activeLands].sort(() => Math.random() - 0.5);
    const selected = shuffledLands
      .slice(0, 2)
      .map((tile) => hexToInt(tile.location));
    animateCharacter(selected);
  }, 10000);

  onDestroy(() => clearInterval(interval));

  let character: HTMLDivElement;

  function waitForTransitionEnd(
    element: HTMLElement,
    propertyName: string,
  ): Promise<void> {
    return new Promise((resolve) => {
      const handler = (event: TransitionEvent) => {
        if (event.propertyName === propertyName) {
          element.removeEventListener('transitionend', handler);
          resolve();
        }
      };
      element.addEventListener('transitionend', handler);
    });
  }

  async function animateCharacter(buildingPair: number[]) {
    const [from, to] = buildingPair;

    const fromX = (from % 64) * 32;
    const fromY = Math.floor(from / 64) * 32;
    const toX = (to % 64) * 32;
    const toY = Math.floor(to / 64) * 32;

    if (!character) {
      character = document.querySelector('.character') as HTMLDivElement;
    }

    if (character) {
      character.style.opacity = '1';
      character.style.transition = 'none';
      character.style.transform = `translate(${fromX}px, ${fromY}px)`;

      character.getBoundingClientRect();

      const dx = toX - fromX;
      const dy = toY - fromY;

      const speed = 10;
      const durationX = (Math.abs(dx) / speed) * 1000;
      const durationY = (Math.abs(dy) / speed) * 1000;

      if (dx !== 0 && dy !== 0) {
        character.style.transition = `transform ${durationX}ms linear`;
        requestAnimationFrame(() => {
          character.style.transform = `translate(${toX}px, ${fromY}px)`;
        });
        await waitForTransitionEnd(character, 'transform');

        character.style.transition = `transform ${durationY}ms linear`;
        requestAnimationFrame(() => {
          character.style.transform = `translate(${toX}px, ${toY}px)`;
        });
        await waitForTransitionEnd(character, 'transform');
      } else if (dx !== 0) {
        character.style.transition = `transform ${durationX}ms linear`;
        requestAnimationFrame(() => {
          character.style.transform = `translate(${toX}px, ${fromY}px)`;
        });
        await waitForTransitionEnd(character, 'transform');
      } else if (dy !== 0) {
        character.style.transition = `transform ${durationY}ms linear`;
        requestAnimationFrame(() => {
          character.style.transform = `translate(${fromX}px, ${toY}px)`;
        });
        await waitForTransitionEnd(character, 'transform');
      }

      character.style.opacity = '0';
    }
  }
</script>

<div
  class="absolute z-40 pointer-events-none h-[2048px] w-[2048px] map-container"
>
  <div
    bind:this={character}
    class="character h-2 w-2 bg-red-600 absolute transition-transform ease-linear"
    style="opacity: 0;"
  ></div>
</div>
