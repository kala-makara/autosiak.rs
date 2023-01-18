<!-- left sidebar component -->

<script>
    import { invoke } from "@tauri-apps/api/tauri";

    let username = "";
    let password = "";
    let status = "status shown here..."

    async function login() {
        status = await invoke('login', {username, password});
        username = "";
        password = "";
    }
</script>

<div class="flex min-h-screen">
    <div class="w-1/4 bg-white p-4">
        <p class="text-center text=xl font-medium">AutoSIAK</p>
        <p class="text-center text-sm font-normal">by deadManAlive</p>
        <form>
            <div class="mb-4">
                <input
                    class="w-full border border-gray-400 p-2 rounded-lg"
                    type="text"
                    id="username"
                    name="username"
                    placeholder="username"
                    bind:value={ username }
                >
            </div>
            <div class="mb-4">
                <input
                    class="w-full border border-gray-400 p-2 rounded-lg"
                    type="password"
                    id="password"
                    name="password"
                    placeholder="password"
                    bind:value={ password }
                >
            </div>
            <div class="text-center">
                <button
                    class="bg-blue-500 text-white py-2 px-4 rounded-lg hover:bg-blue-600"
                    on:click={ login }
                >
                    Submit
                </button>
            </div>
        </form>
        <p class="text-center text-lg font-medium">Login status:</p>
        <p class="text-center text-sm font-normal">{ status }</p>
    </div>
    <div class="w-3/4 bg-gray-100 p-4">
    </div>
</div>