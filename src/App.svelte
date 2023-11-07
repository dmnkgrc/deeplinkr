<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import Emulators from './lib/Emulators.svelte';
  import Simulators from './lib/Simulators.svelte';

  let selectedSimulator: string | undefined;
  let selectedEmulator: string | undefined;
  let deeplink: string | undefined;
  function isDisabled({
    selectedSimulator,
    selectedEmulator,
    deeplink,
    loading,
  }: {
    selectedSimulator?: string;
    selectedEmulator?: string;
    deeplink?: string;
    loading?: boolean;
  }) {
    return (!selectedSimulator && !selectedEmulator) || !deeplink || loading;
  }
  let loading = false;
  async function openDeepLink() {
    loading = true;
    const [udid, state] = selectedSimulator?.split(',') || [];
    await invoke('open_deeplink_in_simulator', {
      link: {
        deep_link: deeplink,
        udid,
        state,
      },
    }).catch(console.error);
    loading = false;
  }
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
  <div class="space-y-2 pb-4">
    <input
      type="text"
      bind:value={deeplink}
      placeholder="Paste your deeplink here"
      class="input input-bordered w-full"
    />
  </div>
  <Simulators bind:selectedSimulator />
  <Emulators bind:selectedEmulator />
  <button
    disabled={isDisabled({
      selectedSimulator,
      selectedEmulator,
      deeplink,
      loading,
    })}
    on:click={openDeepLink}
    class="btn btn-primary"
  >
    {#if loading}
      <span class="loading loading-spinner"></span>
    {/if}
    Open deeplink!
  </button>
</main>
