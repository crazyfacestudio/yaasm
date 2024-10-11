<script lang="ts">
	import { goto } from '$app/navigation';
	import { commands, type ServerUser, type ServerConfig } from '$lib/bindings';
	import { untrack } from 'svelte';

	let errorMsg = $state('');
	let servers = $state<ServerConfig[]>([]);

	async function deleteServer(server_id: string | null) {
		if (server_id !== null) {
			const res = await commands.removeServer(server_id);
			let sRes = await commands.getServers();
			if (sRes.status === 'ok') {
				console.log(sRes.data);
				servers = sRes.data;
			}
		}
	}

	// async function greet() {
	// 	let sRes = await commands.getServers();
	// 	if (sRes.status === 'ok') {
	// 		console.log(sRes.data);
	// 		servers = sRes.data;
	// 	}

	// 	errorMsg = '';

	// 	// const res = await commands.getServers();

	// 	commands
	// 		.addServer({
	// 			id: null,
	// 			server_type: { type: 'ASA' },
	// 			name: 'test',
	// 			host: '31.214.196.6',
	// 			query_port: 1234,
	// 			rcon_port: 11600,
	// 			rcon_password: 'test'
	// 		})
	// 		.then(async (addRes) => {
	// 			if (addRes.status === 'error') {
	// 				errorMsg = 'Error: ' + addRes.error;
	// 				return;
	// 			}

	// 			try {
	// 				const res = await commands.getServers();
	// 				console.error(res);
	// 				if (res.status === 'ok') {
	// 					servers = res.data;
	// 				} else {
	// 					errorMsg = 'Error: ' + res.error;
	// 					console.error(res.error);
	// 				}
	// 			} catch (error) {
	// 				errorMsg = error as string;
	// 				console.error(error);
	// 			}
	// 		});
	// }

	$effect(() => {
		untrack(() => {
			errorMsg = '';
			commands.getServers().then((sRes) => {
				if (sRes.status === 'ok') {
					console.log(sRes.data);
					servers = sRes.data;
				} else {
					errorMsg = 'Error: ' + sRes.error;
					console.error(sRes.error);
				}
			});
		});
	});
</script>

<div class="container mx-auto p-4">
	<h1>Your Servers</h1>
	<div class="relative overflow-x-auto">
		<div class="pb-4">
			<label for="table-search" class="sr-only">Search</label>
			<div class="relative mt-1">
				<div
					class="absolute inset-y-0 rtl:inset-r-0 start-0 flex items-center ps-3 pointer-events-none"
				>
					<svg
						class="w-4 h-4 text-gray-500 dark:text-gray-400"
						aria-hidden="true"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 20 20"
					>
						<path
							stroke="currentColor"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"
						/>
					</svg>
				</div>
				<input
					type="text"
					id="table-search"
					class="block pt-2 ps-10 text-sm border rounded-lg w-80 bg-yaasm-400 border-yaasm-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500"
					placeholder="Filter servers"
				/>
			</div>
		</div>
		<table class="w-full text-sm text-left rtl:text-right shadow-md sm:rounded-lg">
			<thead class="text-xs text-gray-400 uppercase bg-yaasm-600">
				<tr>
					<th scope="col" class="px-6 py-3"> Server</th>
					<th scope="col" class="px-6 py-3"> Type </th>
					<th scope="col" class="px-6 py-3"> Host </th>
					<th scope="col" class="px-6 py-3"> RCON Port </th>
					<th scope="col" class="px-6 py-3"> Query Port </th>
					<th scope="col" class="px-6 py-3"> Action </th>
				</tr>
			</thead>
			<tbody>
				{#each servers as server}
					<tr class=" bg-yaasm-500 hover:bg-yaasm-400">
						<th
							scope="row"
							class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
						>
							{server.name}
						</th>
						<td class="px-6 py-4">{server.server_type.type}</td>
						<td class="px-6 py-4">{server.host}</td>
						<td class="px-6 py-4">{server.rcon_port}</td>
						<td class="px-6 py-4">{server.query_port}</td>
						<td class="px-6 py-4">
							<div class="inline-flex rounded-md shadow-sm" role="group">
								<button
									type="button"
									class="btn-primary py-1.5 px-3"
									onclick={() => goto(`/servers/${server.id}`)}
									aria-label="Edit"
								>
									<i class="fa-solid fa-pen-to-square"></i>
								</button>

								<button
									type="button"
									class="btn-alert py-1.5 px-3"
									onclick={() => deleteServer(server.id)}
									aria-label="Delete"
								>
									<i class="fa-solid fa-trash"></i>
								</button>
							</div>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
