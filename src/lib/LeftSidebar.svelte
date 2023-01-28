<!-- left sidebar component -->
<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let username = '';
	let password = '';
	let status = '...';

    let puname = '';
    let is_logged_in = false;

	async function login() {
		let login_result: string = await invoke('login', { username, password });
        console.log(login_result);

        is_logged_in = false;

		if (login_result === 'Error') {
			status = 'Connection error!';
		} else if (login_result === 'Empty') {
			status = 'Server error!';
		} else if (login_result === 'Failed') {
            status = 'Login Failed!';
        } else {
			puname = username;
            is_logged_in = true
		}

		username = '';
		password = '';
	}

    async function logout() {
        await invoke('logout');
        status = '...';
        puname = '';
        is_logged_in = false;
    }

	function defer() {
		alert(
			'Defer mode skips credentials checking (username and password) and straight to deployment. This  will make fewer API call, which means smaller risk to get connection errors. MAKE SURE YOU USE CORRECT USERNAME AND PASSWORD OR PROGRAM WILL PANIC!\nProceed?'
		);
        status = "DEFER MODE"
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
		class="pb-10 
              text-center
              font-mono
              text-base
              font-thin
              tracking-tight
              text-gray-400 select-none"
	>
		by <span
			class="cursor-pointer
                                               transition
                                               ease-in-out
                                               hover:text-yellow-400
                                               hover:font-black
                                               duration-250"
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
    <div class="pt-2 pb-6 text-center">
        <button
            data-tooltip-target="tooltip-default"
            class="bg-red-500 text-gray-200 font-medium py-2 px-4 rounded-lg hover:bg-red-600 disabled:opacity-50 disabled:hover:bg-red-500"
            disabled='{is_cred_empty(username, password)}'
            on:click={defer}
        >
            Defer
        </button>
    </div>
	<div class="select-none">
		<p class="text-center text-lg font-medium text-gray-200 py-10">Login status:</p>
		<p class="text-center text-sm font-normal text-gray-200">{is_logged_in?puname:status}</p>
	</div>
    <div class="pt-6 text-center">
        <button
            data-tooltip-target="tooltip-default"
            class="bg-red-500 text-gray-200 font-medium py-2 px-4 rounded-lg hover:bg-red-600 disabled:opacity-50 disabled:hover:bg-red-500"
            disabled='{!is_logged_in}'
            on:click={logout}
        >
            Log Out
        </button>
    </div>
</div>
