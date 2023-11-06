<script lang="ts">
  import Simulators from './lib/Simulators.svelte';
</script>

<div class="flex w-full flex-row items-center justify-center">
  <div
    class="border-b-base-100 h-0 w-0 border-b-8 border-l-8 border-r-8 border-solid border-l-transparent border-r-transparent"
  ></div>
</div>
<main class="bg-base-100 px-6 py-3">
  <div class="flex flex-row items-center space-x-2 pb-8">
    <img src="/icon-large.png" class="h-5" alt="Deeplinkr logo" />
    <h1 class="text-2xl font-bold">Deeplinkr</h1>
  </div>
  <Simulators />
</main>
