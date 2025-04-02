<script lang="ts">
  import { tutorialProgression } from './stores';
</script>

<div
  class="fixed bottom-8 right-24 z-[9999] pointer-events-none flex flex-col items-center"
>
  <img
    src="/tutorial/Ponzi_Arrow.png"
    alt="Ponzi Arrow"
    class="w-[60px] h-auto scale-x-[-1]"
  />
  <span class="text-white text-ponzi font-bold mt-1 text-sm">Next</span>
</div>

<div
  class="fixed bottom-16 left-0 right-0 mx-auto w-fit z-[9999] pointer-events-none flex flex-col items-center"
>
  <span class="text-white text-ponzi font-bold text-2xl">
    Step {$tutorialProgression}/15
  </span>
</div>

<div
  class="fixed bottom-8 left-24 z-[9999] pointer-events-none flex flex-col items-center"
>
  <img
    src="/tutorial/Ponzi_Arrow.png"
    alt="Ponzi Arrow"
    class="w-[60px] h-auto"
  />
  <span class="text-white text-ponzi font-bold mt-1 text-sm">Previous</span>
</div>
