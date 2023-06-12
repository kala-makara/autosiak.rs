<!-- left sidebar component -->
<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
    import { login_status } from './stores';

	let username = '';
	let password = '';
	let status = '';

    let waiting = '';
    let wdival: NodeJS.Timer | undefined;

    function wait_start() {
        wdival = setInterval(() => {
            waiting += '.';
            if (waiting.length > 3) {
                waiting = '';
            }
        }, 800);
    }

    function wait_stop() {
        clearInterval(wdival);
    }

    let cached_username = '';
    let cached_password = '';

    // let is_logged_in = false;
    let is_error = false;
    let is_waiting = false;

	async function login() {
        status = 'Waiting';
        $login_status = false;
        is_error = false;
        is_waiting = true;
        wait_start();

        await invoke('login', {username: username, password: password})
            .then((val) => {
                console.log("success");
                cached_username = username;
                cached_password = password;
                $login_status = true;
                status = '';
            })
            .catch((error) => {
                console.log(`error: ${error}`);
                cached_username = '';
                cached_password = '';
                status = `Wrong Password/Username!`
                is_error = true;
            })
            .finally(() => {
                wait_stop();
                is_waiting = false;
            });

		username = '';
		password = '';
	}

    async function logout() {
        await invoke('logout');
        status = '';
        cached_username = '';
        cached_password = '';
        $login_status = false;
    }

    function is_cred_empty(uname: string, pword: string): boolean {
        if (uname === "" || pword === "") {
            return true;
        }
        else {
            return false;
        }
    }
</script>

<div class="w-64 bg-gray-800 p-4">
	<p class="text-center text-3xl font-thin text-yellow-400 select-none">
		Auto<span class="font-medium">SIAK</span>
	</p>
	<p
		class="pb-10 text-center font-mono text-base font-thin tracking-tight text-gray-400 select-none"
	>
		by <span
			class="cursor-pointer transition ease-in-out hover:text-yellow-400 hover:font-black duration-250"
			on:click={(_) => {
				invoke('credit');
			}}
			aria-hidden="true">deadManAlive</span
		>
	</p>
	<form>
		<div class="mb-8">
			<input
				class="w-full border border-slate-600 p-2 rounded-lg bg-gray-600 text-gray-200 text-sm"
				type="text"
				id="username"
				name="username"
				placeholder="username"
				bind:value={username}
                required
			/>
		</div>
		<div class="mb-8">
			<input
				class="w-full border border-slate-600 p-2 rounded-lg bg-gray-600 text-gray-200 text-sm"
				type="password"
				id="password"
				name="password"
				placeholder="password"
				autocomplete="off"
				bind:value={password}
                required
			/>
		</div>
		<div class="pb-2 text-center">
			<button
				class="bg-blue-500 text-gray-200 font-medium py-2 px-4 rounded-lg hover:bg-blue-600 disabled:opacity-50 disabled:hover:bg-blue-500"
                disabled='{is_cred_empty(username, password)}'
				on:click={login}
			>
				Log In
			</button>
		</div>
	</form>
	<div class="select-none">
		<p class="text-center text-lg font-medium text-gray-200 py-10">Login status:</p>
        {#if $login_status}
		    <p class="text-center text-sm font-normal text-gray-200">{cached_username}</p>
        {:else if is_error}
            <p class="text-center text-sm font-semibold text-red-500">{status}</p>
        {:else if is_waiting}
		    <p class="text-center text-sm font-semibold text-yellow-100">{status + waiting}</p>
        {:else}
            <p class="text-center text-sm font-semibold text-yellow-100">...</p>
        {/if}
	</div>
    <div class="pt-6 text-center">
        <button
            data-tooltip-target="tooltip-default"
            class="bg-red-500 text-gray-200 font-medium py-2 px-4 rounded-lg hover:bg-red-600 disabled:opacity-50 disabled:hover:bg-red-500"
            disabled='{!$login_status}'
            on:click={logout}
        >
            Log Out
        </button>
    </div>
</div>
