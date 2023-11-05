<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  let simulatorsPromise = getSimulators();

  async function getSimulators(){
    return invoke("get_simulators")
  }
</script>

<div>
  <p>Simulators</p>
  {#await simulatorsPromise}
		Loading simulators...
	{:then simulators}
    <p>{JSON.stringify(simulators)}</p>
	{:catch someError}
		System error: {someError.message}.
	{/await}
</div>
