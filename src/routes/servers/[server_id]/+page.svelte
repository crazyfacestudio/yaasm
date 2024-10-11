<script lang="ts">
	import { page } from '$app/stores';
	import { commands, type ServerUser, type ServerConfig } from '$lib/bindings';
	import { untrack } from 'svelte';

	let errorMsg = $state('');
	let players = $state<ServerUser[]>([]);
	let chat = $state<string[]>([]);

	async function getChat(server_id: string) {
		errorMsg = '';
		chat = [];

		let res = await commands.getChat(server_id);
		if (res.status === 'ok') {
			chat = res.data;
		} else {
			errorMsg = 'Error: ' + res.error;
			console.error(res.error);
		}
		errorMsg = '';
	}

	async function listUsers(server_id: string) {
		errorMsg = '';
		players = [];

		let res = await commands.listUsers(server_id);
		if (res.status === 'ok') {
			players = res.data;
		} else {
			errorMsg = 'Error: ' + res.error;
			console.error(res.error);
		}
		errorMsg = '';
	}

	$effect(() => {
		untrack(() => {
			listUsers($page.params.server_id);
			getChat($page.params.server_id);
		});
	});

	// $effect(() => {
	// 	commands.getServers().then((sRes) => {
	// 		if (sRes.status === 'ok') {
	// 			console.log(sRes.data);
	// 			servers = sRes.data;
	// 		}
	// 	});
	// });
</script>

<div class="container">
	<h1>
		Chat
		<button
			type="submit"
			aria-label="Refresh Chat"
			onclick={() => getChat($page.params.server_id)}
			class="text-white text-xl px-2 py-1.5 me-2"
			><i class="fa-solid fa-arrows-rotate fa-lg"></i></button
		>
	</h1>
	{#each chat as msg}
		<p>{msg}</p>
	{/each}
	<br />

	<h1>
		Players
		<button
			type="submit"
			aria-label="Refresh Users"
			onclick={() => listUsers($page.params.server_id)}
			class="text-white text-xl px-2 py-1.5 me-2"
			><i class="fa-solid fa-arrows-rotate fa-lg"></i></button
		>
	</h1>
	<p class="p-4">Error: {errorMsg}</p>

	<table class="w-full text-sm text-left rtl:text-right shadow-md sm:rounded-lg">
		<thead class="text-xs text-gray-400 uppercase bg-yaasm-600">
			<tr>
				<th scope="col" class="px-6 py-3">ID</th>
				<th scope="col" class="px-6 py-3">Player</th>
				<th scope="col" class="px-6 py-3">Action</th>
			</tr>
		</thead>
		<tbody>
			{#each players as player}
				<tr class=" bg-yaasm-500 hover:bg-yaasm-300">
					<td class="px-6 py-3">{player.id}</td>
					<td class="px-6 py-3">{player.name}</td>
					<td class="px-6 py-3"
						><button
							type="button"
							class="focus:outline-none text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900"
							>Kick</button
						>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
