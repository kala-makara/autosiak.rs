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
    <div class="w-64 bg-gray-800 p-4">
        <p class="text-center text-3xl font-thin text-yellow-400">Auto<span class="font-medium">SIAK</span></p>
        <p class="pb-10 
                  text-center
                  font-mono
                  text-base
                  font-thin
                  tracking-tight
                  text-gray-400">by <span class="cursor-pointer
                                                   transition
                                                   ease-in-out
                                                   hover:text-yellow-400
                                                   hover:font-black
                                                   duration-250" on:click={ _ => { invoke('credit') }} aria-hidden=true>deadManAlive</span></p>
        <form>
            <div class="mb-8">
                <input
                    class="w-full border border-slate-600 p-2 rounded-lg bg-gray-600 text-gray-200"
                    type="text"
                    id="username"
                    name="username"
                    placeholder="username"
                    autocomplete="off"
                    bind:value={ username }
                >
            </div>
            <div class="mb-8">
                <input
                    class="w-full border border-slate-600 p-2 rounded-lg bg-gray-600 text-gray-200"
                    type="password"
                    id="password"
                    name="password"
                    placeholder="password"
                    autocomplete="off"
                    bind:value={ password }
                >
            </div>
            <div class="pb-10 text-center">
                <button
                    class="bg-blue-500 text-gray-200 font-medium py-2 px-4 rounded-lg hover:bg-blue-600"
                    on:click={ login }
                >
                    Log In
                </button>
            </div>
        </form>
        <p class="text-center text-lg font-medium text-gray-200 py-10">Login status:</p>
        <p class="text-center text-sm font-normal text-gray-200">{ status }</p>
    </div>
    <div class="w-full bg-slate-600 p-4">
    </div>
</div>