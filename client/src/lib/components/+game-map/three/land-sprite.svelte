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

  // BUILDING
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
        { name: 'pimp_1', frameRange: [72, 72] },
        { name: 'pimp_2', frameRange: [84, 84] },
        { name: 'pimp_3', frameRange: [96, 96] },
        { name: 'ekubo_1', frameRange: [108, 108] },
        { name: 'ekubo_2', frameRange: [120, 120] },
        { name: 'ekubo_3', frameRange: [132, 132] },
        { name: 'sisters_1', frameRange: [144, 144] },
        { name: 'sisters_2', frameRange: [156, 156] },
        { name: 'sisters_3', frameRange: [168, 168] },
        { name: 'slinky_1', frameRange: [180, 180] },
        { name: 'slinky_2', frameRange: [192, 192] },
        { name: 'slinky_3', frameRange: [204, 204] },
        { name: 'btc_1', frameRange: [216, 216] },
        { name: 'btc_2', frameRange: [228, 228] },
        { name: 'btc_3', frameRange: [240, 240] },

        { name: 'dope_1', frameRange: [3, 3] },
        { name: 'dope_2', frameRange: [15, 15] },
        { name: 'dope_3', frameRange: [27, 27] },
        { name: 'strk_1', frameRange: [39, 39] },
        { name: 'strk_2', frameRange: [51, 51] },
        { name: 'strk_3', frameRange: [63, 63] },
        { name: 'eth_1', frameRange: [75, 75] },
        { name: 'eth_2', frameRange: [87, 87] },
        { name: 'eth_3', frameRange: [99, 99] },
        { name: 'brother_1', frameRange: [111, 111] },
        { name: 'brother_2', frameRange: [123, 123] },
        { name: 'brother_3', frameRange: [135, 135] },
        { name: 'circus_1', frameRange: [147, 147] },
        // { name: 'circus_2', frameRange: [159, 159] },
        // { name: 'circus_3', frameRange: [171, 171] },
        { name: 'nums_1', frameRange: [183, 183] },
        { name: 'nums_2', frameRange: [195, 195] },
        { name: 'nums_3', frameRange: [207, 207] },
        { name: 'flip_1', frameRange: [219, 219] },
        { name: 'flip_2', frameRange: [231, 231] },
        { name: 'flip_3', frameRange: [243, 243] },
        { name: 'wnt_1', frameRange: [6, 6] },
        { name: 'wnt_2', frameRange: [18, 18] },
        { name: 'wnt_3', frameRange: [30, 30] },
        { name: 'qq_1', frameRange: [42, 42] },
        { name: 'qq_2', frameRange: [54, 54] },
        { name: 'qq_3', frameRange: [66, 66] },
        { name: 'evreai_1', frameRange: [9, 9] },
        { name: 'evreai_2', frameRange: [21, 21] },
        { name: 'evreai_3', frameRange: [33, 33] },
      ],
    },
  ] as const satisfies SpritesheetMetadata;
  const buildingAtlas =
    buildSpritesheet.from<typeof buildingAtlasMeta>(buildingAtlasMeta);
  let buildingSprite: any = $state();
  let buildingAnimations = buildingAtlasMeta[0].animations.map(
    (anim) => anim.name,
  );
  const randomBuildingAnimations = Array.from(
    { length: gridSize * gridSize },
    () =>
      buildingAnimations[Math.floor(Math.random() * buildingAnimations.length)],
  );
  $effect(() => {
    if (buildingSprite) {
      buildingSprite.update();
    }
  });

  // LORDS
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
  const lords3animationAtlas =
    buildSpritesheet.from<typeof lordsSpriteMeta>(lordsSpriteMeta);
  let lords3Sprite: any = $state();
  $effect(() => {
    if (lords3Sprite) {
      lords3Sprite.update();
    }
  });

  // BIOME
  const biomeAtlasMeta = [
    {
      url: '/tokens/+global/biomes.png',
      type: 'rowColumn',
      width: 4,
      height: 5,
      animations: [
        { name: 'biome_0', frameRange: [0, 0] },
        // { name: 'auction', frameRange: [1, 1] },
        { name: 'biome_2', frameRange: [2, 2] },
        { name: 'dope', frameRange: [3, 3] },
        { name: 'ekubo', frameRange: [4, 4] },
        { name: 'eth', frameRange: [5, 5] },
        { name: 'slinky', frameRange: [6, 6] },
        { name: 'lords', frameRange: [7, 7] },
        { name: 'biome_8', frameRange: [8, 8] },
        { name: 'biome_9', frameRange: [9, 9] },
        { name: 'biome_10', frameRange: [10, 10] },
        { name: 'biome_11', frameRange: [11, 11] },
        { name: 'biome_12', frameRange: [12, 12] },
        { name: 'biome_13', frameRange: [13, 13] },
        { name: 'biome_14', frameRange: [14, 14] },
        { name: 'biome_15', frameRange: [15, 15] },
        { name: 'biome_16', frameRange: [16, 16] },
        { name: 'biome_17', frameRange: [17, 17] },
      ],
    },
  ] as const satisfies SpritesheetMetadata;
  const biomeAtlas =
    buildSpritesheet.from<typeof biomeAtlasMeta>(biomeAtlasMeta);
  let biomeAnimations = biomeAtlasMeta[0].animations.map((anim) => anim.name);
  const randomBiomeAnimations = Array.from(
    { length: gridSize * gridSize },
    () => biomeAnimations[Math.floor(Math.random() * biomeAnimations.length)],
  );
  let biomeSprite: any = $state();
  $effect(() => {
    if (biomeSprite) {
      biomeSprite.update();
    }
  });

  // ROAD
  const roadAtlasMeta = [
    {
      url: '/land-display/road.png',
      type: 'rowColumn',
      width: 1,
      height: 1,
      animations: [{ name: 'default', frameRange: [0, 0] }],
    },
  ] as const satisfies SpritesheetMetadata;
  const roadAtlas = buildSpritesheet.from<typeof roadAtlasMeta>(roadAtlasMeta);
  let roadSprite: any = $state();
  $effect(() => {
    if (roadSprite) {
      roadSprite.update();
    }
  });
</script>

{#await Promise.all( [buildingAtlas.spritesheet, biomeAtlas.spritesheet, lords3animationAtlas.spritesheet, roadAtlas.spritesheet], ) then [buildingSpritesheet, biomeSpritesheet, lords3Spritesheet, roadSpritesheet]}
  <!-- Road sprites-->
  <InstancedSprite
    count={gridSize * gridSize}
    autoUpdate={false}
    playmode={'PAUSE'}
    {billboarding}
    spritesheet={roadSpritesheet}
    bind:ref={roadSprite}
  >
    {#snippet children({ Instance }: { Instance: any })}
      {#each gridPositions as position, i}
        <Instance
          animationName={'default'}
          position={[position[0], position[1] - 0.01, position[2]]}
          id={i}
        />
      {/each}
    {/snippet}
  </InstancedSprite>

  <!-- Biome sprites (background layer) -->
  <InstancedSprite
    count={gridSize * gridSize}
    autoUpdate={false}
    playmode={'PAUSE'}
    {billboarding}
    spritesheet={biomeSpritesheet}
    bind:ref={biomeSprite}
  >
    {#snippet children({ Instance: BiomeInstance }: { Instance: any })}
      {#each gridPositions as position, i}
        <BiomeInstance
          animationName={randomBiomeAnimations[i]}
          position={[position[0], position[1] - 0.01, position[2]]}
          id={i}
        />
      {/each}
    {/snippet}
  </InstancedSprite>

  <!-- Building sprites (foreground layer) -->
  <InstancedSprite
    count={gridSize * gridSize}
    {billboarding}
    spritesheet={buildingSpritesheet}
    fps={10}
    bind:ref={buildingSprite}
  >
    {#snippet children({ Instance }: { Instance: any })}
      {#each gridPositions as position, i}
        <Instance
          animationName={randomBuildingAnimations[i]}
          {position}
          id={i}
          offset={100 * i}
          on:click={() => {
            console.log('clik');
          }}
        />
      {/each}
    {/snippet}
  </InstancedSprite>
{/await}
