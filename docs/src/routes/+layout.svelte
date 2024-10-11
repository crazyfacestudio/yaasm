<script lang="ts">
	// import { setContext, untrack } from 'svelte';
	// import { initFlowbite } from 'flowbite';
	import '../app.postcss';

	const { children } = $props();
</script>

{@render children()}
