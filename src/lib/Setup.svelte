<script lang="ts">
	import Terms from './Terms.svelte';
	import Keyword from './Keyword.svelte';
	import { login_status } from './stores';
	import { invoke } from '@tauri-apps/api/tauri';
    import { asStr } from './util';

	async function deploy() {
		await invoke('worker')
			.then((val) => {
				console.log(`deploy succes: ${asStr(val)}`);
			})
			.catch((error) => {
				console.log(`deploy error: ${error}`);
			});
	}
</script>

<div class="flex flex-col p-2">
	<div class="mb-4">
		<Terms />
	</div>
	<div>
		<Keyword />
	</div>
</div>
<hr class="py-2 px-4" />
<div class="p-2">
	<button
		class="bg-red-500 text-gray-200 font-medium py-2 px-4 rounded-lg hover:bg-red-600 disabled:opacity-50 disabled:hover:bg-red-500 w-full"
		disabled={!$login_status}
		on:click={deploy}>Deploy!</button
	>
</div>