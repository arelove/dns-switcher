<script>
	import MiniLayout from './MiniLayout.svelte';

	let { children } = $props();
</script>

<MiniLayout>
	{@render children()}
</MiniLayout>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		overflow: hidden !important; /* ✅ Добавьте !important */
	}

	/* Дополнительно скрываем scrollbar полностью */
	:global(body)::-webkit-scrollbar {
		display: none;
		width: 0;
		height: 0;
	}

	:global(body) {
		-ms-overflow-style: none;  /* IE and Edge */
		scrollbar-width: none;  /* Firefox */
	}

	:global(html) {
		overflow: hidden !important;
	}
</style>