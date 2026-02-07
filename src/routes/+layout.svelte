<script lang="ts">
	import { onMount } from 'svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import Notification from '$lib/components/Notification.svelte';
	import { notification } from '$lib/stores';
	
	// Add children prop
	let { children } = $props();

	let isMiniWindow = $state(false);

	onMount(async () => {
		const appWindow = getCurrentWindow();
		const label = await appWindow.label;
		isMiniWindow = label === 'mini';
	});
</script>

<div class="app-layout">
	{#if !isMiniWindow}
		<TitleBar />
	{/if}

	<div class="content-wrapper" class:mini-window={isMiniWindow}>
		{@render children()}
	</div>

	{#if $notification}
		<Notification />
	{/if}
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		overflow: hidden;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Segoe UI Variable', Roboto, Oxygen,
			Ubuntu, Cantarell, sans-serif;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
	}

	.app-layout {
		display: flex;
		flex-direction: column;
		height: 100vh;
		width: 100vw;
		overflow: hidden;
	}

	.content-wrapper {
		flex: 1;
		overflow-y: auto;
		overflow-x: hidden;
		position: relative;
	}

	/* Для мини-окна убираем стандартные отступы */
	.content-wrapper.mini-window {
		overflow: hidden;
	}

	/* Modern styled scrollbar */
	.content-wrapper:not(.mini-window)::-webkit-scrollbar {
		width: 10px;
	}

	.content-wrapper:not(.mini-window)::-webkit-scrollbar-track {
		background: rgba(0, 0, 0, 0.05);
		border-radius: 10px;
		margin: 4px;
	}

	.content-wrapper:not(.mini-window)::-webkit-scrollbar-thumb {
		background: rgba(0, 0, 0, 0.2);
		border-radius: 10px;
		border: 2px solid transparent;
		background-clip: content-box;
	}

	.content-wrapper:not(.mini-window)::-webkit-scrollbar-thumb:hover {
		background: rgba(0, 0, 0, 0.35);
		background-clip: content-box;
	}

	.content-wrapper:not(.mini-window)::-webkit-scrollbar-thumb:active {
		background: rgba(0, 0, 0, 0.5);
		background-clip: content-box;
	}

	@media (prefers-color-scheme: dark) {
		.content-wrapper:not(.mini-window)::-webkit-scrollbar-track {
			background: rgba(255, 255, 255, 0.05);
		}

		.content-wrapper:not(.mini-window)::-webkit-scrollbar-thumb {
			background: rgba(255, 255, 255, 0.2);
			background-clip: content-box;
		}

		.content-wrapper:not(.mini-window)::-webkit-scrollbar-thumb:hover {
			background: rgba(255, 255, 255, 0.3);
			background-clip: content-box;
		}

		.content-wrapper:not(.mini-window)::-webkit-scrollbar-thumb:active {
			background: rgba(255, 255, 255, 0.4);
			background-clip: content-box;
		}
	}

	/* Firefox scrollbar */
	.content-wrapper:not(.mini-window) {
		scrollbar-width: thin;
		scrollbar-color: rgba(0, 0, 0, 0.2) rgba(0, 0, 0, 0.05);
	}

	@media (prefers-color-scheme: dark) {
		.content-wrapper:not(.mini-window) {
			scrollbar-color: rgba(255, 255, 255, 0.2) rgba(255, 255, 255, 0.05);
		}
	}
</style>