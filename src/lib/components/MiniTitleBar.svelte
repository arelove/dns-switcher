<script lang="ts">
	import { onMount } from 'svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import type { UnlistenFn } from '@tauri-apps/api/event';

	const appWindow = getCurrentWindow();
	
	let windowWidth = $state(320);
	let showTitle = $state(true);
	let showControls = $state(true);

	onMount(() => {
		let unlisten: UnlistenFn;

		const setup = async () => {
			const updateSize = async () => {
				const size = await appWindow.innerSize();
				windowWidth = size.width;
				
				// Адаптивная логика
				showTitle = windowWidth > 180;      // Скрыть название при ширине <= 180px
				showControls = windowWidth > 100;   // Скрыть доп. кнопки при ширине <= 100px
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

	async function minimizeWindow() {
		await appWindow.hide();
	}

	async function maximizeWindow() {
		await appWindow.toggleMaximize();
	}

	async function closeWindow() {
		await appWindow.hide();
	}
</script>

<div data-tauri-drag-region class="mini-titlebar">
	<div class="mini-titlebar-title" data-tauri-drag-region>
		<div class="app-icon">🛡️</div>
		{#if showTitle}
			<span class="app-name">DNS</span>
		{/if}
	</div>

	<div class="mini-titlebar-controls">
		{#if showControls}
			<!-- Minimize Button -->
			<button
				class="mini-titlebar-button minimize-button"
				onclick={minimizeWindow}
				aria-label="Minimize"
				title="Minimize"
			>
				<svg viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
					<rect x="1" y="5.5" width="10" height="1" rx="0.5" fill="currentColor" />
				</svg>
			</button>

			<!-- Maximize Button -->
			<button
				class="mini-titlebar-button maximize-button"
				onclick={maximizeWindow}
				aria-label="Maximize"
				title="Maximize"
			>
				<svg viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
					<rect
						x="1.5"
						y="1.5"
						width="9"
						height="9"
						rx="1"
						stroke="currentColor"
						stroke-width="1"
					/>
				</svg>
			</button>
		{/if}

		<!-- Close Button - всегда видна -->
		<button
			class="mini-titlebar-button close-button"
			onclick={closeWindow}
			aria-label="Close"
			title="Close"
		>
			<svg viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
				<path
					d="M1.5 1.5L10.5 10.5M10.5 1.5L1.5 10.5"
					stroke="currentColor"
					stroke-width="1.5"
					stroke-linecap="round"
				/>
			</svg>
		</button>
	</div>
</div>

<style>
	.mini-titlebar {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: 28px;
		background: rgba(255, 255, 255, 0.85);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border-bottom: 1px solid rgba(0, 0, 0, 0.08);
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0 8px;
		z-index: 1000;
		user-select: none;
		-webkit-user-select: none;
		-webkit-app-region: drag;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	@media (prefers-color-scheme: dark) {
		.mini-titlebar {
			background: rgba(15, 23, 42, 0.85);
			border-bottom: 1px solid rgba(6, 182, 212, 0.1);
			box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
		}
	}

	.mini-titlebar-title {
		display: flex;
		align-items: center;
		gap: 6px;
		flex: 1;
		font-size: 11px;
		font-weight: 700;
		color: #667eea;
		text-transform: uppercase;
		letter-spacing: 0.3px;
		min-width: 0;
	}

	@media (prefers-color-scheme: dark) {
		.mini-titlebar-title {
			color: #06b6d4;
		}
	}

	.app-icon {
		width: 18px;
		height: 18px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 12px;
		flex-shrink: 0;
	}

	.app-name {
		font-weight: 700;
		letter-spacing: 0.5px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.mini-titlebar-controls {
		display: flex;
		align-items: center;
		gap: 2px;
		-webkit-app-region: no-drag;
	}

	.mini-titlebar-button {
		width: 24px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #718096;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
		border-radius: 4px;
		flex-shrink: 0;
	}

	.mini-titlebar-button svg {
		width: 10px;
		height: 10px;
		transition: all 0.2s;
	}

	.mini-titlebar-button:hover {
		background: rgba(102, 126, 234, 0.1);
		color: #667eea;
	}

	@media (prefers-color-scheme: dark) {
		.mini-titlebar-button {
			color: #94a3b8;
		}

		.mini-titlebar-button:hover {
			background: rgba(6, 182, 212, 0.15);
			color: #06b6d4;
		}
	}

	.minimize-button:hover svg {
		transform: scaleX(1.15);
	}

	.maximize-button:hover svg {
		transform: scale(1.15);
	}

	.close-button:hover {
		background: rgba(239, 68, 68, 0.9);
		color: white;
	}

	.close-button:hover svg {
		transform: scale(1.15);
	}

	@media (prefers-color-scheme: dark) {
		.close-button:hover {
			background: rgba(239, 68, 68, 0.9);
			box-shadow: 0 0 10px rgba(239, 68, 68, 0.3);
		}
	}

	.mini-titlebar-button:active {
		transform: scale(0.92);
	}

	@keyframes slideDown {
		from {
			transform: translateY(-100%);
			opacity: 0;
		}
		to {
			transform: translateY(0);
			opacity: 1;
		}
	}

	.mini-titlebar {
		animation: slideDown 0.25s ease-out;
	}
</style>