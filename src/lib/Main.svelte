<script lang="ts">
    import type { Tab } from "./types";
    import Tabs from "./Tabs.svelte";
    import Setup from "./Setup.svelte";
    import Result from "./Result.svelte";

    import { query_store } from "./stores";
    import { terms_store } from "./stores";

    const items: Tab[] = [
        {
            label: "Setup",
            value: 1,
            component: Setup,
        }, 
        {
            label: "Result",
            value: 2,
            component: Result,
        }
    ];
</script>

<div class="w-full bg-slate-600 p-4">
    <Tabs {items} />
</div>