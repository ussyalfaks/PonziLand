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

  const lordsSpriteMeta = [
    {
      url: '/tokens/+global/buildings.png',
      type: 'rowColumn',
      width: 12,
      height: 21,
      animations: [
        { name: '1', frameRange: [0, 0] },
        { name: '2', frameRange: [12, 12] },
        { name: '3', frameRange: [24, 24] },
      ],
    },
    {
      url: '/tokens/eLORDS/3-animated.png',
      type: 'rowColumn',
      width: 5,
      height: 2,
      animations: [{ name: '3_animated', frameRange: [0, 9] }],
    },
  ] as const satisfies SpritesheetMetadata;

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
  const lords3animationAtlas =
    buildSpritesheet.from<typeof lordsSpriteMeta>(lordsSpriteMeta);

  let buildingSprite: any = $state();
  let biomeSprite: any = $state();
  let lords3Sprite: any = $state();

  // Generate grid positions for 64x64 sprites
  const gridSize = 64;
  const spacing = 1; // Distance between sprites

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
    if (lords3Sprite) {
      lords3Sprite.offset.setAt(1, 500);
      lords3Sprite.update();
    }
  });
</script>

{#await Promise.all( [buildingAtlas.spritesheet, biomeAtlas.spritesheet, lords3animationAtlas.spritesheet], ) then [buildingSpritesheet, biomeSpritesheet, lords3Spritesheet]}
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
          id={i}
        />
      {/each}
    {/snippet}
  </InstancedSprite>

  <!-- Building sprites (foreground layer) -->
  <!-- <InstancedSprite
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
  </InstancedSprite> -->

  <!-- Building sprites (foreground layer) -->
  <InstancedSprite
    count={gridSize * gridSize}
    {billboarding}
    spritesheet={lords3Spritesheet}
    fps={10}
    bind:ref={lords3Sprite}
  >
    {#snippet children({ Instance }: { Instance: any })}
      {#each gridPositions as position, i}
        <Instance animationName={'1'} {position} id={i} offset={100 * i} />
      {/each}
    {/snippet}
  </InstancedSprite>
{/await}
