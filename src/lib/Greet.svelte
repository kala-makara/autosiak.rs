<script>
    import { invoke } from "@tauri-apps/api/tauri";

    let name = "";
    let greet_msg = "";

    async function greet() {
        greet_msg = await invoke('greet', { name });
    }
</script>

<div>
    <input id="greet-input" placeholder="Put your name here..." bind:value="{ name }" />
    <button on:click="{ greet }">Greet</button>
    <p>{ greet_msg }</p>
</div>