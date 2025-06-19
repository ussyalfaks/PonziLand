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

  // LandTile class to hold token metadata
  class LandTile {
    position: [number, number, number];
    tokenName: string;
    level: number;

    constructor(
      position: [number, number, number],
      tokenName: string,
      level: number,
    ) {
      this.position = position;
      this.tokenName = tokenName;
      this.level = level;
    }

    // Derive building animation name from token name and level
    getBuildingAnimationName(): string {
      return `${this.tokenName}_${this.level}`;
    }

    // Derive biome animation name from token name
    getBiomeAnimationName(): string {
      return this.tokenName;
    }
  }

  // Generate grid positions and create LandTile instances
  const gridSize = 64;
  const spacing = 1; // Distance between sprites

  // Available token names (based on your biome animations)
  const tokenNames = [
    'lords',
    'pal',
    'pimp',
    'ekubo',
    'sisters',
    'slinky',
    'btc',
    'dope',
    'strk',
    'eth',
    'brother',
    'circus',
    'nums',
    'flip',
    'wnt',
    'qq',
    'evreai',
  ];

  // Create land tiles with random token data
  // const landTiles: LandTile[] = Array.from(
  //   { length: gridSize * gridSize },
  //   (_, i) => {
  //     const x = i % gridSize;
  //     const z = Math.floor(i / gridSize);
  //     const posX = (x - gridSize / 2) * spacing;
  //     const posZ = (z - gridSize / 2) * spacing;
  //     const position: [number, number, number] = [posX, 1, posZ];

  //     // Random token name and level
  //     const tokenName =
  //       tokenNames[Math.floor(Math.random() * tokenNames.length)];
  //     const level = Math.floor(Math.random() * 3) + 1; // levels 1, 2, 3

  //     return new LandTile(position, tokenName, level);
  //   },
  // );

  const landTiles: LandTile[] = Array.from(
    { length: gridSize * gridSize },
    (_, i) => {
      const x = i % gridSize;
      const z = Math.floor(i / gridSize);
      const posX = (x - gridSize / 2) * spacing;
      const posZ = (z - gridSize / 2) * spacing;
      const position: [number, number, number] = [posX, 1, posZ];

      // Iterate over token names and levels in order
      const tokenName = tokenNames[i % tokenNames.length];
      const level = (i % 3) + 1; // levels: 1, 2, 3, 1, 2, 3...

      return new LandTile(position, tokenName, level);
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
        { name: 'brother_1', frameRange: [75, 75] },
        { name: 'brother_2', frameRange: [87, 87] },
        { name: 'brother_3', frameRange: [99, 99] },
        { name: 'eth_1', frameRange: [111, 111] },
        { name: 'eth_2', frameRange: [123, 123] },
        { name: 'eth_3', frameRange: [135, 135] },
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
        { name: 'empty', frameRange: [1, 1] },
      ],
    },
  ] as const satisfies SpritesheetMetadata;
  const buildingAtlas =
    buildSpritesheet.from<typeof buildingAtlasMeta>(buildingAtlasMeta);
  let buildingSprite: any = $state();

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
        { name: 'slinky', frameRange: [0, 0] },
        { name: 'auction', frameRange: [1, 1] },
        { name: 'pimp', frameRange: [2, 2] },
        { name: 'dope', frameRange: [3, 3] },
        { name: 'ekubo', frameRange: [4, 4] },
        { name: 'eth', frameRange: [5, 5] },
        { name: 'flip', frameRange: [6, 6] },
        { name: 'lords', frameRange: [7, 7] },
        { name: 'nums', frameRange: [8, 8] },
        { name: 'pal', frameRange: [9, 9] },
        { name: 'circus', frameRange: [10, 10] },
        { name: 'sisters', frameRange: [11, 11] },
        { name: 'btc', frameRange: [12, 12] },
        { name: 'brother', frameRange: [13, 13] },
        { name: 'strk', frameRange: [14, 14] },
        { name: 'wnt', frameRange: [15, 15] },
        { name: 'qq', frameRange: [16, 16] },
        { name: 'evreai', frameRange: [17, 17] },
      ],
    },
  ] as const satisfies SpritesheetMetadata;
  const biomeAtlas =
    buildSpritesheet.from<typeof biomeAtlasMeta>(biomeAtlasMeta);
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

  // Helper function to get a fallback animation name if the derived one doesn't exist
  function getBuildingAnimationOrFallback(
    tile: LandTile,
    availableAnimations: string[],
  ): string {
    const derivedName = tile.getBuildingAnimationName();
    if (availableAnimations.includes(derivedName)) {
      return derivedName;
    }
    // Fallback to a random available animation
    return 'empty';
  }

  function getBiomeAnimationOrFallback(
    tile: LandTile,
    availableAnimations: string[],
  ): string {
    const derivedName = tile.getBiomeAnimationName();
    if (availableAnimations.includes(derivedName)) {
      return derivedName;
    }
    // Fallback to a random available animation
    return 'auction';
  }

  // Get available animation names
  const buildingAnimations = buildingAtlasMeta[0].animations.map(
    (anim) => anim.name,
  );
  const biomeAnimations = biomeAtlasMeta[0].animations.map((anim) => anim.name);
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
      {#each landTiles as tile, i}
        <Instance
          animationName={'default'}
          position={[
            tile.position[0],
            tile.position[1] - 0.01,
            tile.position[2],
          ]}
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
      {#each landTiles as tile, i}
        <BiomeInstance
          animationName={getBiomeAnimationOrFallback(tile, biomeAnimations)}
          position={[
            tile.position[0],
            tile.position[1] - 0.01,
            tile.position[2],
          ]}
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
      {#each landTiles as tile, i}
        <Instance
          animationName={getBuildingAnimationOrFallback(
            tile,
            buildingAnimations,
          )}
          position={tile.position}
          id={i}
          offset={100 * i}
          on:click={() => {
            console.log('Clicked tile:', tile);
            console.log('Token:', tile.tokenName, 'Level:', tile.level);
          }}
        />
      {/each}
    {/snippet}
  </InstancedSprite>
{/await}
