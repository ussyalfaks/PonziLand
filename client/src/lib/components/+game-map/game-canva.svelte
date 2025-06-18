<script lang="ts">
  import { Canvas, T } from '@threlte/core';
  import Scene from './game-scene.svelte';
  import { OrbitControls } from '@threlte/extras';
  import { DEG2RAD } from 'three/src/math/MathUtils.js';
  let billboarding = true;
  let fps = 30;
</script>

<div>
  <Canvas>
    <T.PerspectiveCamera makeDefault position.z={0} position.y={50}>
      <OrbitControls
        autoRotate
        autoRotateSpeed={0.5}
        enableRotate={false}
        enableZoom={true}
        enablePan={true}
        panSpeed={1.0}
        screenSpacePanning={true}
      />
    </T.PerspectiveCamera>
    <Scene {billboarding} {fps} />
  </Canvas>
</div>

<style>
  div {
    height: 100%;
  }
</style>
