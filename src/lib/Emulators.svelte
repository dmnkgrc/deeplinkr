<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { z } from 'zod';
  import AndroidLogo from './assets/android.svelte';

  const emulatorsSchema = z.array(z.string());

  let emulatorsPromise = getEmulators();
  export let selectedEmulator: string | undefined;

  async function getEmulators() {
    const emulators = await emulatorsSchema.parseAsync(
      await invoke('get_emulators'),
    );
    return emulators;
  }
</script>

<div class="flex w-full flex-col">
  <div class="text-primary-content flex flex-row items-center space-x-2">
    <AndroidLogo width={18} height={18} />
    <p class="pt-0.5 text-lg">Emulators</p>
  </div>
  {#await emulatorsPromise}
    <div class="flex w-full flex-row justify-center py-4">
      <span class="loading loading-spinner loading-md"></span>
    </div>
  {:then emulators}
    <select
      bind:value={selectedEmulator}
      class="select select-bordered select-sm my-2 w-full"
    >
      <option disabled selected value="">Select an emulator</option>
      {#each emulators as emulator}
        <option value={emulator}>{emulator}</option>
      {/each}
    </select>
    <div class="flex w-full flex-col space-y-2 py-2"></div>
  {:catch someError}
    System error: {someError.message}.
  {/await}
</div>
