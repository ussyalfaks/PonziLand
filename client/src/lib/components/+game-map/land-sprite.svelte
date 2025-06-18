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
      animations: [{ name: 'lords_1', frameRange: [0, 0] }],
    },
  ] as const satisfies SpritesheetMetadata;
  const buildingAtlas =
    buildSpritesheet.from<typeof buildingAtlasMeta>(buildingAtlasMeta);

  let sprite = $state();
</script>

{#await buildingAtlas.spritesheet then spritesheet}
  <InstancedSprite
    count={1}
    autoUpdate={false}
    playmode={'PAUSE'}
    {billboarding}
    {spritesheet}
    bind:ref={sprite}
  >
    {#snippet children({ Instance }: { Instance: any })}
      <!-- Set and freeze a random frame from the spritesheet -->
      <Instance animationName={'lords_1'} position={[0, 1, 0]} scale={[3, 3]} />
    {/snippet}
  </InstancedSprite>
{/await}
