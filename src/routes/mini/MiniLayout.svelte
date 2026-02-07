<script lang="ts">
	import { onMount } from 'svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import type { UnlistenFn } from '@tauri-apps/api/event';
	import MiniTitleBar from '$lib/components/MiniTitleBar.svelte';

	let { children } = $props();

	const appWindow = getCurrentWindow();
	
	let windowHeight = $state(480);
	let showTitleBar = $state(true);

	onMount(() => {
		let unlisten: UnlistenFn;

		const setup = async () => {
			try {
				await appWindow.setAlwaysOnTop(true);
			} catch (error) {
				console.error('Failed to set always on top:', error);
			}

			const updateSize = async () => {
				const size = await appWindow.innerSize();
				windowHeight = size.height;
				showTitleBar = windowHeight > 200;
			};

			await updateSize();

			unlisten = await appWindow.onResized(() => {
				updateSize();
			});
		};

		setup();

		return () => {
			if (unlisten) unlisten();
		};
	});
</script>

<div class="mini-layout" data-tauri-drag-region>
	{#if showTitleBar}
		<MiniTitleBar />
	{/if}

	<div class="mini-content" class:no-titlebar={!showTitleBar} class:with-titlebar={showTitleBar}>
		{@render children()}
	</div>
</div>

<style>
	.mini-layout {
		width: 100%;
		height: 100vh;
		background: rgba(255, 255, 255, 0.95);
		backdrop-filter: blur(20px);
		border-radius: 12px;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
		cursor: move;
	}

	@media (prefers-color-scheme: dark) {
		.mini-layout {
			background: rgba(15, 23, 42, 0.95);
		}
	}

	.mini-content {
		flex: 1;
		overflow-y: hidden;
		overflow-x: hidden;
	}

	.mini-content.with-titlebar {
		padding: 0 12px 12px;
		margin-top: 28px;
	}

	.mini-content.no-titlebar {
		padding: 0;
	}

	.mini-content::-webkit-scrollbar {
		width: 6px;
	}

	.mini-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.mini-content::-webkit-scrollbar-thumb {
		background: rgba(0, 0, 0, 0.2);
		border-radius: 3px;
	}

	@media (prefers-color-scheme: dark) {
		.mini-content::-webkit-scrollbar-thumb {
			background: rgba(255, 255, 255, 0.2);
		}
	}
</style>