<script lang="ts">
    import type { Tab } from "./types";

	export let items: Tab[] = [];
	export let active_tab_value = 1;

	const handleClick: (tab_value: number) => () => void = (tab_value) => () =>
		(active_tab_value = tab_value);
</script>

<ul class="flex flex-wrap w-full text-center text-sm font-medium text-gray-400" role="tablist">
    {#each items as item}
        <li class="mr-2" role="presentation">
            <button
                on:click={handleClick(item.value)}
                class="{active_tab_value === item.value ? 'active-tab' : 'inactive-tab'}"
            >
            {item.label}
            </button>
        </li>
    {/each}
</ul>
{#each items as item}
    {#if active_tab_value == item.value}
        <div class="p-3 border-2 border-gray-800 rounded-b-lg h-5/6 rounded-tr-lg">
            <svelte:component this={item.component} />
        </div>
    {/if}
{/each}

<style lang="postcss">
	.active-tab {
        @apply inline-block rounded-t-lg p-4 bg-gray-800 text-yellow-400;
    }
    .inactive-tab {
        @apply inline-block rounded-t-lg p-4 bg-gray-900 hover:bg-gray-700 hover:text-gray-300;
    }
</style>
