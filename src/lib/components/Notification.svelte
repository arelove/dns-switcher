<script lang="ts">
	import { notification } from '$lib/stores';
	import { onDestroy } from 'svelte';

	let timeout: ReturnType<typeof setTimeout>;

	$: if ($notification.show) {
		clearTimeout(timeout);
		timeout = setTimeout(() => {
			notification.set({ show: false, message: '', type: 'info' });
		}, 4000);
	}

	function close() {
		notification.set({ show: false, message: '', type: 'info' });
	}

	onDestroy(() => {
		if (timeout) clearTimeout(timeout);
	});
</script>

{#if $notification.show}
	<div class="notification-container">
		<div class="notification notification-{$notification.type}">
			<div class="notification-content">
				<div class="notification-icon">
					{#if $notification.type === 'success'}
						<svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
						</svg>
					{:else if $notification.type === 'error'}
						<svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
						</svg>
					{:else}
						<svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
						</svg>
					{/if}
				</div>

				<div class="notification-body">
					<p class="notification-message">{$notification.message}</p>
				</div>

				<button class="notification-close" onclick={close} aria-label="Close">
					<svg class="icon-sm" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>
		</div>
	</div>
{/if}
<style>
	.notification-container {
		position: fixed;
		bottom: 1rem;
		right: 1rem;
		z-index: 9999;
	}

	.notification {
		background: rgba(255, 255, 255, 0.98);
		backdrop-filter: blur(10px);
		border-radius: 12px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
		padding: 1rem;
		border-left: 4px solid;
		max-width: 400px;
		min-width: 320px;
		animation: slide-in-from-right 0.3s ease-out;
	}

	@media (prefers-color-scheme: dark) {
		.notification {
			background: rgba(30, 41, 59, 0.98);
			box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
		}
	}

	.notification-success {
		border-left-color: #10b981;
	}

	.notification-error {
		border-left-color: #ef4444;
	}

	.notification-info {
		border-left-color: #667eea;
	}

	@media (prefers-color-scheme: dark) {
		.notification-info {
			border-left-color: #06b6d4;
		}
	}

	.notification-content {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
	}

	.notification-icon {
		flex-shrink: 0;
	}

	.notification-success .notification-icon {
		color: #10b981;
	}

	@media (prefers-color-scheme: dark) {
		.notification-success .notification-icon {
			color: #6ee7b7;
		}
	}

	.notification-error .notification-icon {
		color: #ef4444;
	}

	@media (prefers-color-scheme: dark) {
		.notification-error .notification-icon {
			color: #fca5a5;
		}
	}

	.notification-info .notification-icon {
		color: #667eea;
	}

	@media (prefers-color-scheme: dark) {
		.notification-info .notification-icon {
			color: #06b6d4;
		}
	}

	.notification-body {
		flex: 1;
	}

	.notification-message {
		font-size: 0.875rem;
		color: #2d3748;
		font-weight: 500;
		margin: 0;
	}

	@media (prefers-color-scheme: dark) {
		.notification-message {
			color: #f7fafc;
		}
	}

	.notification-close {
		flex-shrink: 0;
		color: #718096;
		background: transparent;
		border: none;
		cursor: pointer;
		padding: 0;
		transition: color 0.2s;
	}

	.notification-close:hover {
		color: #2d3748;
	}

	@media (prefers-color-scheme: dark) {
		.notification-close {
			color: #cbd5e0;
		}

		.notification-close:hover {
			color: #f7fafc;
		}
	}

	.icon {
		width: 1.25rem;
		height: 1.25rem;
	}

	.icon-sm {
		width: 1rem;
		height: 1rem;
	}

	@keyframes slide-in-from-right {
		from {
			transform: translateY(100%);
			opacity: 0;
		}
		to {
			transform: translateY(0);
			opacity: 1;
		}
	}
</style>