<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { toggleMiniWindow } from '$lib/api';

	let appWindow = getCurrentWindow();

	async function minimizeWindow() {
		await appWindow.minimize();
	}

	async function maximizeWindow() {
		await appWindow.toggleMaximize();
	}

	async function closeWindow() {
		// Hide to tray instead of closing
		await appWindow.hide();
	}

	async function handleToggleMini() {
		try {
			await toggleMiniWindow();
		} catch (error) {
			console.error('Failed to toggle mini window:', error);
		}
	}
</script>

<div data-tauri-drag-region class="titlebar">
	<div class="titlebar-title" data-tauri-drag-region>
		<div class="app-icon">🌐</div>
		<span class="app-name">DNS Switcher</span>
	</div>

	<div class="titlebar-controls">
		<!-- Mini Window Button -->
		<button
			class="titlebar-button mini-button"
			on:click={handleToggleMini}
			aria-label="Toggle Mini Window"
			title="Toggle Mini Window"
		>
			<svg viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
				<rect x="2" y="2" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.5" />
				<rect x="9" y="2" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.5" />
				<rect x="2" y="9" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.5" />
				<rect x="9" y="9" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.5" />
			</svg>
		</button>

		<!-- Standard Controls -->
		<button
			class="titlebar-button minimize-button"
			on:click={minimizeWindow}
			aria-label="Minimize"
			title="Minimize"
		>
			<svg viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
				<rect x="1" y="5.5" width="10" height="1" rx="0.5" fill="currentColor" />
			</svg>
		</button>

		<button
			class="titlebar-button maximize-button"
			on:click={maximizeWindow}
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

		<button
			class="titlebar-button close-button"
			on:click={closeWindow}
			aria-label="Close to Tray"
			title="Close to Tray"
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
	.titlebar {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: 40px;
		background: rgba(255, 255, 255, 0.8);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border-bottom: 1px solid rgba(255, 255, 255, 0.3);
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0 1rem;
		z-index: 1000;
		user-select: none;
		-webkit-user-select: none;
		-webkit-app-region: drag;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	@media (prefers-color-scheme: dark) {
		.titlebar {
			background: rgba(30, 41, 59, 0.8);
			border-bottom: 1px solid rgba(6, 182, 212, 0.15);
			box-shadow:
				0 1px 3px rgba(0, 0, 0, 0.3),
				0 0 20px rgba(6, 182, 212, 0.05);
		}
	}

	.titlebar-title {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		flex: 1;
		font-size: 0.875rem;
		font-weight: 600;
		color: #2d3748;
	}

	@media (prefers-color-scheme: dark) {
		.titlebar-title {
			color: #f7fafc;
		}
	}

	.app-icon {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 16px;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		border-radius: 6px;
		box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);
	}

	@media (prefers-color-scheme: dark) {
		.app-icon {
			background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
			box-shadow: 0 2px 8px rgba(6, 182, 212, 0.3);
		}
	}

	.app-name {
		font-weight: 600;
		letter-spacing: -0.01em;
	}

	.titlebar-controls {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		-webkit-app-region: no-drag;
	}

	.titlebar-button {
		width: 36px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #718096;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
		border-radius: 6px;
		position: relative;
	}

	.titlebar-button svg {
		width: 12px;
		height: 12px;
		transition: all 0.2s;
	}

	.mini-button {
		margin-right: 8px;
	}

	.mini-button svg {
		width: 16px;
		height: 16px;
	}

	.mini-button:hover {
		background: rgba(102, 126, 234, 0.15);
		color: #667eea;
	}

	@media (prefers-color-scheme: dark) {
		.mini-button:hover {
			background: rgba(6, 182, 212, 0.2);
			color: #06b6d4;
		}
	}

	.titlebar-button:hover {
		background: rgba(102, 126, 234, 0.1);
		color: #667eea;
	}

	@media (prefers-color-scheme: dark) {
		.titlebar-button {
			color: #94a3b8;
		}

		.titlebar-button:hover {
			background: rgba(6, 182, 212, 0.15);
			color: #06b6d4;
		}
	}

	.minimize-button:hover svg {
		transform: scaleX(1.1);
	}

	.maximize-button:hover svg {
		transform: scale(1.1);
	}

	.close-button:hover {
		background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
		color: white;
	}

	.close-button:hover svg {
		transform: scale(1.1);
	}

	@media (prefers-color-scheme: dark) {
		.close-button:hover {
			background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
			box-shadow: 0 0 15px rgba(239, 68, 68, 0.4);
		}
	}

	.titlebar-button:active {
		transform: scale(0.95);
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

	.titlebar {
		animation: slideDown 0.3s ease-out;
	}
</style>