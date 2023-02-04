<script lang="ts">
    import Terms from "./Terms.svelte";
    import Keyword from "./Keyword.svelte";
</script>

<div class="flex flex-col p-2">
    <div class="mb-4">
        <Terms />
    </div>
    <div>
        <Keyword />
    </div>
</div>
<hr class="py-2 px-4">
<div class="p-2 h-5/6">
    <button class="bg-red-500 text-gray-200 font-medium py-2 px-4 rounded-lg hover:bg-red-600 disabled:opacity-50 disabled:hover:bg-red-500 w-full">Deploy!</button>
</div>