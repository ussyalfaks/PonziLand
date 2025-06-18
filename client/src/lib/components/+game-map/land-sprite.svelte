<!--
	-	Example of using animations as a static sprite atlas
	- each frame is named and used as a different tree randomly
	- to achieve this playmode is "PAUSE" and autoUpdate={false}
	- the instanced sprite has to be updated once when initialized
		and then, each time the atlas changes
	- uses <Instance/> component instead of hook to set positions and frames
 -->
<script lang="ts">
  import {
    InstancedSprite,
    buildSpritesheet,
    type SpritesheetMetadata,
  } from '@threlte/extras';
  import { onMount } from 'svelte';

  let { billboarding = true } = $props();

  const buildingAtlasMeta = [
    {
      url: '/tokens/+global/buildings.png',
      type: 'rowColumn',
      width: 12,
      height: 21,
      animations: [{ name: 'lords_1', frameRange: [0, 0] }],
    },
  ] as const satisfies SpritesheetMetadata;

  const buildingAtlas =
    buildSpritesheet.from<typeof buildingAtlasMeta>(buildingAtlasMeta);

  let sprite: any = $state();

  // Generate grid positions for 64x64 sprites
  const gridSize = 64;
  const spacing = 4; // Distance between sprites

  const gridPositions: [number, number, number][] = Array.from(
    { length: gridSize * gridSize },
    (_, i) => {
      const x = i % gridSize;
      const z = Math.floor(i / gridSize);
      const posX = (x - gridSize / 2) * spacing;
      const posZ = (z - gridSize / 2) * spacing;
      return [posX, 1, posZ];
    },
  );

  onMount(() => {
    console.log('Sprite initialized');
    console.log(gridPositions);
  });
</script>

{#await buildingAtlas.spritesheet then spritesheet}
  <InstancedSprite
    count={gridSize * gridSize + 1}
    autoUpdate={false}
    playmode={'PAUSE'}
    {billboarding}
    {spritesheet}
    bind:ref={sprite}
  >
    {#snippet children({ Instance }: { Instance: any })}
      {#each gridPositions as position, i}
        <Instance animationName={'lords_1'} {position} scale={[3, 3]} id={i} />
      {/each}
    {/snippet}
  </InstancedSprite>
{/await}
