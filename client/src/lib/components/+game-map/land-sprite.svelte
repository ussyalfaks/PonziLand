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

  let { billboarding = true } = $props();

  const buildingAtlasMeta = [
    {
      url: '/tokens/+global/buildings.png',
      type: 'rowColumn',
      width: 12,
      height: 21,
      animations: [
        { name: 'lords_1', frameRange: [0, 0] },
        { name: 'lords_2', frameRange: [12, 12] },
        { name: 'lords_3', frameRange: [24, 24] },
        { name: 'pal_1', frameRange: [36, 36] },
        { name: 'pal_2', frameRange: [48, 48] },
        { name: 'pal_3', frameRange: [60, 60] },
      ],
    },
  ] as const satisfies SpritesheetMetadata;

  const biomeAtlasMeta = [
    {
      url: '/tokens/+global/biomes.png',
      type: 'rowColumn',
      width: 4,
      height: 5,
      animations: [
        { name: 'biome_1', frameRange: [0, 0] },
        { name: 'biome_2', frameRange: [2, 2] },
      ],
    },
  ] as const satisfies SpritesheetMetadata;

  const buildingAtlas =
    buildSpritesheet.from<typeof buildingAtlasMeta>(buildingAtlasMeta);
  const biomeAtlas =
    buildSpritesheet.from<typeof biomeAtlasMeta>(biomeAtlasMeta);

  let buildingSprite: any = $state();
  let biomeSprite: any = $state();

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

  let buildingAnimations = buildingAtlasMeta[0].animations.map(
    (anim) => anim.name,
  );
  const randomBuildingAnimations = Array.from(
    { length: gridSize * gridSize },
    () =>
      buildingAnimations[Math.floor(Math.random() * buildingAnimations.length)],
  );

  let biomeAnimations = biomeAtlasMeta[0].animations.map((anim) => anim.name);
  const randomBiomeAnimations = Array.from(
    { length: gridSize * gridSize },
    () => biomeAnimations[Math.floor(Math.random() * biomeAnimations.length)],
  );

  $effect(() => {
    if (buildingSprite) {
      buildingSprite.update();
    }
    if (biomeSprite) {
      biomeSprite.update();
    }
  });
</script>

{#await Promise.all( [buildingAtlas.spritesheet, biomeAtlas.spritesheet], ) then [buildingSpritesheet, biomeSpritesheet]}
  <!-- Biome sprites (background layer) -->
  <InstancedSprite
    count={gridSize * gridSize}
    autoUpdate={false}
    playmode={'PAUSE'}
    {billboarding}
    spritesheet={biomeSpritesheet}
    bind:ref={biomeSprite}
  >
    {#snippet children({ Instance }: { Instance: any })}
      {#each gridPositions as position, i}
        <Instance
          animationName={randomBiomeAnimations[i]}
          position={[position[0], position[1] - 0.1, position[2]]}
          scale={[3, 3]}
          id={i}
        />
      {/each}
    {/snippet}
  </InstancedSprite>

  <!-- Building sprites (foreground layer) -->
  <InstancedSprite
    count={gridSize * gridSize}
    autoUpdate={false}
    playmode={'PAUSE'}
    {billboarding}
    spritesheet={buildingSpritesheet}
    bind:ref={buildingSprite}
  >
    {#snippet children({ Instance }: { Instance: any })}
      {#each gridPositions as position, i}
        <Instance
          animationName={randomBuildingAnimations[i]}
          {position}
          scale={[3, 3]}
          id={i}
        />
      {/each}
    {/snippet}
  </InstancedSprite>
{/await}
