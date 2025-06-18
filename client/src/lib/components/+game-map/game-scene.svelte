<script lang="ts">
  import { T, useThrelte } from '@threlte/core';
  import { CSM, Sky, useTexture } from '@threlte/extras';
  import { NearestFilter, RepeatWrapping } from 'three';
  import LandSprite from './land-sprite.svelte';
  import { DEG2RAD } from 'three/src/math/MathUtils.js';
  export let billboarding = false;
  export let fps = 30;
  const grass = useTexture('/extra/pixel-grass.png', {
    transform: (texture) => {
      texture.wrapS = texture.wrapT = RepeatWrapping;
      texture.repeat.set(500, 500);
      texture.minFilter = NearestFilter;
      texture.magFilter = NearestFilter;
      texture.needsUpdate = true;
      return texture;
    },
  });
  const { renderer } = useThrelte();
  renderer.setPixelRatio(1);
</script>

<slot />
<CSM
  args={{
    mode: 'logarithmic',
  }}
  lightDirection={[-1, -1, -1]}
  lightIntensity={5}
>
  <LandSprite {billboarding} />
  {#if $grass}
    <T.Mesh rotation.x={-DEG2RAD * 90} receiveShadow>
      <T.CircleGeometry args={[300]} />
      <T.MeshLambertMaterial map={$grass} />
    </T.Mesh>
  {/if}
</CSM>
<Sky elevation={13.35} />
<T.AmbientLight intensity={1} />
