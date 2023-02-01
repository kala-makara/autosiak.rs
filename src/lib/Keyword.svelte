<script lang="ts">
    let query_list: string[] = [];
    let new_query = '';

    function add() {
        query_list = [...query_list, new_query.trim()];
        new_query = '';
    }

    function remove(query: string) {
        query_list = query_list.filter((value, index, array) => {return value !== query;});
    }

    function clear() {
        query_list = new Array<string>();;
    }
</script>

<fieldset>
    <legend class="sr-only">Query</legend>

    <div class="flex flex-row items-start">
		<h1 class="mr-8 font-semibold text-gray-200">Query:</h1>
        <div class="flex flex-wrap">
            <form>
                <div class="flex rounded-full bg-slate-400 p-1 w-60 items-center mr-2">
                    <input class="w-3/4 rounded-l-full rounded-r-xl px-2 mr-1" placeholder="Insert queries here..." bind:value={new_query}>
                    <button class="w-16 bg-blue-500 hover:bg-blue-600 rounded-r-full rounded-l-xl font-bold text-gray-200 disabled:opacity-50" on:click={add} disabled={new_query === ''}>+</button>
                </div>
            </form>
            <!-- TODO: or not: the wraps act funny here -->
            {#each query_list as query}
                <div class="flex rounded-full mr-2 bg-slate-200 px-2 items-center py-1 mb-2">
                    <p class="mr-2">{query}</p>
                    <button class="rounded-full bg-red-500 hover:bg-red-600 w-8 text-gray-200 font-bold" on:click={() => {remove(query)}}> &#215;</button>
                </div>
            {/each}
            <button class="flex rounded-full mr-2 bg-red-500 hover:bg-red-600 disabled:hover:bg-red-500 text-gray-200 font-semibold px-2 items-center py-1 mb-2 disabled:opacity-50" disabled={query_list.length === 0} on:click={clear}>Clear</button>
        </div>
    </div>
</fieldset>