<script lang="ts">
	// import { setContext, untrack } from 'svelte';
	import { initFlowbite } from 'flowbite';
	import Sidebar from '$lib/Sidebar.svelte';

	const { children } = $props();

	$effect(() => {
		initFlowbite();
	});
</script>

<Sidebar />
<div class="p-4 sm:ml-64">
	{@render children()}
</div>
