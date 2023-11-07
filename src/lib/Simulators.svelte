<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { z } from 'zod';
  import AppleLogo from './assets/apple.svelte';

  const simulatorsSchema = z
    .object({
      devices: z.record(
        z.array(
          z.object({
            name: z.string(),
            udid: z.string(),
            state: z.string(),
            is_available: z.boolean(),
          }),
        ),
      ),
    })
    .transform((result) => {
      let simulators: {
        name: string;
        udid: string;
        state: string;
        is_available: boolean;
      }[] = [];
      Object.values(result.devices).forEach((list) => {
        simulators = [...simulators, ...list];
      });
      return simulators.filter((simulator) => simulator.is_available);
    });

  let simulatorsPromise = getSimulators();
  export let selectedSimulator: string | undefined;

  async function getSimulators() {
    const simulators = await simulatorsSchema.parseAsync(
      await invoke('get_simulators'),
    );
    return simulators;
  }
</script>

<div class="flex w-full flex-col">
  <div class="text-primary-content flex flex-row items-center space-x-2">
    <AppleLogo width={18} height={18} />
    <p class="pt-0.5 text-lg">Simulators</p>
  </div>
  {#await simulatorsPromise}
    <div class="flex w-full flex-row justify-center py-4">
      <span class="loading loading-spinner loading-md"></span>
    </div>
  {:then simulators}
    <select
      bind:value={selectedSimulator}
      class="select select-bordered select-sm my-2 w-full"
    >
      <option disabled selected value="">Select a simulator</option>
      {#each simulators as { name, udid, state }}
        <option value={`${udid},${state}`}>{name}</option>
      {/each}
    </select>
    <div class="flex w-full flex-col space-y-2 py-2"></div>
  {:catch someError}
    System error: {someError.message}.
  {/await}
</div>
