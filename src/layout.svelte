<script lang="ts">
	import { onMount } from 'svelte';
	
	let { children } = $props();
	
	onMount(() => {
		// Применяем сохранённую тему
		const savedTheme = localStorage.getItem('theme') || 'dark';
		if (savedTheme === 'dark') {
			document.documentElement.classList.add('dark');
		}
	});
</script>

<svelte:head>
	<title>DNS Changer</title>
</svelte:head>

{@render children()}