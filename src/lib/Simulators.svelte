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

  async function getSimulators() {
    const simulators = await simulatorsSchema.parseAsync(
      await invoke('get_simulators'),
    );
    return simulators;
  }
</script>

<div>
  <p><AppleLogo /> Simulators</p>
  {#await simulatorsPromise}
    Loading simulators...
  {:then simulators}
    <p>{JSON.stringify(simulators)}</p>
  {:catch someError}
    System error: {someError.message}.
  {/await}
</div>
