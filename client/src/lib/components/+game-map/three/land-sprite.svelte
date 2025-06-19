<script lang="ts">
  import { AuctionLand } from '$lib/api/land/auction_land';
  import { BuildingLand } from '$lib/api/land/building_land';
  import { landStore } from '$lib/stores/store.svelte';
  import {
    InstancedSprite,
    buildSpritesheet,
    interactivity,
    type SpritesheetMetadata,
  } from '@threlte/extras';
  import { onMount } from 'svelte';
  // Import Three.js
  import { OutlineSpriteMaterial } from '$lib/materials/OutlineSpriteMaterial'; // Adjust path
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
      tokenName = tokenName || 'empty'; // Ensure tokenName is not null/undefined
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
        { name: 'eLORDS_1', frameRange: [0, 0] },
        { name: 'eLORDS_2', frameRange: [12, 12] },
        { name: 'eLORDS_3', frameRange: [24, 24] },
        { name: 'nftLORDS_1', frameRange: [0, 0] },
        { name: 'nftLORDS_2', frameRange: [12, 12] },
        { name: 'nftLORDS_3', frameRange: [24, 24] },
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
        { name: 'nftSTRK_1', frameRange: [39, 39] },
        { name: 'nftSTRK_2', frameRange: [51, 51] },
        { name: 'nftSTRK_3', frameRange: [63, 63] },
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
        { name: 'eWNT_1', frameRange: [6, 6] },
        { name: 'eWNT_2', frameRange: [18, 18] },
        { name: 'eWNT_3', frameRange: [30, 30] },
        { name: 'qq_1', frameRange: [42, 42] },
        { name: 'qq_2', frameRange: [54, 54] },
        { name: 'qq_3', frameRange: [66, 66] },
        { name: 'eQQ_1', frameRange: [42, 42] },
        { name: 'eQQ_2', frameRange: [54, 54] },
        { name: 'eQQ_3', frameRange: [66, 66] },
        { name: 'evreai_1', frameRange: [9, 9] },
        { name: 'evreai_2', frameRange: [21, 21] },
        { name: 'evreai_3', frameRange: [33, 33] },
        { name: 'eSG_1', frameRange: [9, 9] },
        { name: 'eSG_2', frameRange: [21, 21] },
        { name: 'eSG_3', frameRange: [33, 33] },
        { name: 'empty', frameRange: [1, 1] },
      ],
    },
  ] as const satisfies SpritesheetMetadata;
  const buildingAtlas =
    buildSpritesheet.from<typeof buildingAtlasMeta>(buildingAtlasMeta);

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
        { name: 'eLORDS', frameRange: [7, 7] },
        { name: 'nftLORDS', frameRange: [7, 7] },
        { name: 'nums', frameRange: [8, 8] },
        { name: 'pal', frameRange: [9, 9] },
        { name: 'circus', frameRange: [10, 10] },
        { name: 'sisters', frameRange: [11, 11] },
        { name: 'btc', frameRange: [12, 12] },
        { name: 'brother', frameRange: [13, 13] },
        { name: 'strk', frameRange: [14, 14] },
        { name: 'nftSTRK', frameRange: [14, 14] },
        { name: 'wnt', frameRange: [15, 15] },
        { name: 'eWNT', frameRange: [15, 15] },
        { name: 'qq', frameRange: [16, 16] },
        { name: 'eQQ', frameRange: [16, 16] },
        { name: 'evreai', frameRange: [17, 17] },
        { name: 'eSG', frameRange: [17, 17] },
      ],
    },
    {
      url: '/land-display/empty.png',
      type: 'rowColumn',
      width: 4,
      height: 3,
      animations: [
        { name: 'empty', frameRange: [0, 0] },
        { name: 'empty_0', frameRange: [0, 0] },
        { name: 'empty_1', frameRange: [1, 1] },
        { name: 'empty_2', frameRange: [2, 2] },
        { name: 'empty_3', frameRange: [3, 3] },
        { name: 'empty_4', frameRange: [4, 4] },
        { name: 'empty_5', frameRange: [5, 5] },
        { name: 'empty_6', frameRange: [6, 6] },
        { name: 'empty_7', frameRange: [7, 7] },
        { name: 'empty_8', frameRange: [8, 8] },
        { name: 'empty_9', frameRange: [9, 9] },
        { name: 'empty_10', frameRange: [10, 10] },
        { name: 'auction_shadow', frameRange: [11, 11] },
      ],
    },
  ] as const satisfies SpritesheetMetadata;
  const biomeAtlas =
    buildSpritesheet.from<typeof biomeAtlasMeta>(biomeAtlasMeta);

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

  // Helper function to get a fallback animation name if the derived one doesn't exist
  function getBuildingAnimationOrFallback(
    tile: LandTile,
    availableAnimations: string[],
  ): string {
    const derivedName = tile.getBuildingAnimationName();
    if (availableAnimations.includes(derivedName)) {
      return derivedName;
    }
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

    return 'empty';
  }

  // Get available animation names
  const buildingAnimations = buildingAtlasMeta[0].animations.map(
    (anim) => anim.name,
  );
  const biomeAnimations = biomeAtlasMeta[0].animations.map((anim) => anim.name);

  let landTiles: any[] = $state([]);

  onMount(() => {
    // Initialize land tiles from store
    landStore.getAllLands().subscribe((tiles) => {
      landTiles = tiles.map((tile) => {
        let tokenSymbol = 'empty';

        if (BuildingLand.is(tile)) {
          tokenSymbol = tile?.token?.symbol ?? 'empty'; // Default to 'empty' instead of console.warn
        }

        if (AuctionLand.is(tile)) {
          tokenSymbol = 'auction';
        }

        return new LandTile(
          [tile.location.x, 1, tile.location.y],
          tokenSymbol,
          tile.level,
        );
      });
    });
  });

  $effect(() => {
    // Update sprite instances when landTiles or other relevant data changes
    if (buildingSprite) {
      buildingSprite.update();
    }
    if (biomeSprite) {
      biomeSprite.update();
    }
    if (roadSprite) {
      roadSprite.update();
    }
  });

  let roadSprite: any = $state();
  let biomeSprite: any = $state();
  let buildingSprite: any = $state();

  let getBiomeMaterial = (resolvedBiomeSpritesheet: any) => {
    return new OutlineSpriteMaterial(
      resolvedBiomeSpritesheet, // This is the correct spritesheet object
      2.0, // Outline width in pixels
      0xffff00, // Yellow color (0xRRGGBB)
    );
  };

  interactivity({
    filter: (hits, state) => {
      // Only return the first hit
      return hits.slice(0, 1);
    },
  });

  import { useInteractivity } from '@threlte/extras';
  const { pointer, pointerOverTarget } = useInteractivity();
  $inspect($pointer, $pointerOverTarget);
</script>

{#await Promise.all( [buildingAtlas.spritesheet, biomeAtlas.spritesheet, roadAtlas.spritesheet], ) then [buildingSpritesheet, resolvedBiomeSpritesheet, roadSpritesheet]}
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
            tile.position[1] - 0.02,
            tile.position[2],
          ]}
          id={i}
        />
      {/each}
    {/snippet}
  </InstancedSprite>

  <!-- Biome sprites (background layer) with custom outline shader -->
  <!-- Only render if biomeOutlineMaterial is ready -->
  <InstancedSprite
    count={gridSize * gridSize}
    {billboarding}
    spritesheet={resolvedBiomeSpritesheet}
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
        />
      {/each}
    {/snippet}
  </InstancedSprite>
{/await}
