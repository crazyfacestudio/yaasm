<script lang="ts">
	import { initFlowbite } from 'flowbite';
	import '../app.postcss';
	import '@awesome.me/kit-ad58914989/icons/css/all.min.css';

	const { children } = $props();

	$effect(() => {
		initFlowbite();
	});
</script>

{@render children()}
