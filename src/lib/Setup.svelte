<script lang="ts">
	let checkboxes = Array<boolean>(8).fill(false);
	let odd = false;
	let even = false;
    export let terms: number[];

    $: {
        terms = new Array<number>();
        for (let i = 0; i < 8; i++) {
            if (checkboxes[i]) {
                terms.push(i + 1);
            }
        }
    }
    

	function handleCheckboxChange(e: Event, index: number) {
		checkboxes[index] = (e.target as HTMLInputElement).checked;
		if (index % 2 === 0) {
			even = checkboxes.every((checkbox, i) => (i % 2 === 0 ? checkbox : true));
		} else {
			odd = checkboxes.every((checkbox, i) => (i % 2 !== 0 ? checkbox : true));
		}
	}

	function handleOddChange(e: Event) {
		odd = (e.target as HTMLInputElement).checked;
		if (odd) {
			checkboxes = checkboxes.map((checkbox, i) => (i % 2 !== 0 ? true : checkbox));
		} else {
			checkboxes = checkboxes.map((checkbox, i) => (i % 2 !== 0 ? false : checkbox));
		}
	}

	function handleEvenChange(e: Event) {
		even = (e.target as HTMLInputElement).checked;
		if (even) {
			checkboxes = checkboxes.map((checkbox, i) => (i % 2 === 0 ? true : checkbox));
		} else {
			checkboxes = checkboxes.map((checkbox, i) => (i % 2 === 0 ? false : checkbox));
		}
	}
</script>

<fieldset>
	<legend class="sr-only">Terms</legend>

	<div class="flex flex-row items-center">
		<h1 class="mr-8 font-semibold text-gray-200">Terms:</h1>
		<div class="flex flex-row border-2 border-gray-200 rounded-lg mr-2 items-center p-1">
			<div class="items-center mr-1 ml-1">
				<input
					id="checkbox-odd"
					type="checkbox"
					value={[1, 3, 5, 7]}
					bind:checked={odd}
                    on:change={handleOddChange}
					class="rounded border-gray-300 bg-gray-100 text-yellow-400 focus:ring-2 focus:ring-blue-500"
				/>
				<label for="checkbox-1" class="ml-0 text-sm font-bold text-gray-200">Odd</label>
			</div>
			<div class="items-center mr-1 ml-1">
				<input
					id="checkbox-even"
					type="checkbox"
					value={[2, 4, 6, 8]}
					bind:checked={even}
                    on:change={handleEvenChange}
					class="rounded border-gray-300 bg-gray-100 text-yellow-400 focus:ring-2 focus:ring-blue-500"
				/>
				<label for="checkbox-1" class="ml-0 text-sm font-bold text-gray-200">Even</label>
			</div>
		</div>
		<div class="flex flex-row border-2 border-gray-400 rounded-lg mr-2 items-center p-1">
			{#each checkboxes as checkbox, idx}
				<div class="items-center mr-1 ml-1">
					<input
						id="checkbox-1"
						type="checkbox"
						bind:checked={checkbox}
						on:change={(e) => handleCheckboxChange(e, idx)}
						class="rounded border-gray-300 bg-gray-100 text-blue-400 focus:ring-2 focus:ring-blue-500"
					/>
					<label for="checkbox-1" class="ml-0 text-sm text-gray-200">{idx + 1}</label>
				</div>
			{/each}
		</div>
	</div>
</fieldset>
<h1 class="text-gray-200">{terms}</h1>
