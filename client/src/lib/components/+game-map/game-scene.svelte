<script lang="ts">
  import { T, useThrelte } from '@threlte/core';
  import { CSM, interactivity, Sky, useTexture } from '@threlte/extras';
  import { NearestFilter, RepeatWrapping } from 'three';
  import LandSprite from './three/land-sprite.svelte';
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
<LandSprite {billboarding} />
