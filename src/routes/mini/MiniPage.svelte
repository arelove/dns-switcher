<script lang="ts">
	import { onMount } from 'svelte';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { 
		getCurrentDns, 
		getDnsPresets, 
		setDns, 
		updateTrayTooltip,
		updateTrayMenu,
		getSelectedAdapter,
	} from '$lib/api';
	import { selectedAdapter } from '$lib/stores';
	import type { DnsConfiguration, DnsPreset } from '$lib/types';

	let currentDns = $state<DnsConfiguration | null>(null);
	let quickPresets = $state<DnsPreset[]>([]);
	let allPresets = $state<DnsPreset[]>([]);
	let loading = $state(false);
	let selectedPresetId = $state<string | null>(null);
	let unlisten: UnlistenFn | null = null;
	let adapterChangeUnlisten: UnlistenFn | null = null;
	let presetIndexUnlisten: UnlistenFn | null = null;
	
	// Window size tracking
	let windowWidth = $state(320);
	let windowHeight = $state(480);
	let isCompactMode = $state(false);
	let gridColumns = $state(2); // ✅ Новая переменная
	let gridRows = $state(2);

	const appWindow = getCurrentWindow();

	// Track window size changes
	onMount(() => {
		const updateSize = async () => {
			const size = await appWindow.innerSize();
			windowWidth = size.width;
			windowHeight = size.height;
			
			// Компактный режим при малых размерах
			isCompactMode = windowHeight <= 190;
			
			if (isCompactMode) {
				// ✅ Умный расчет сетки на основе размеров окна
				const minButtonSize = 50; // Минимальный размер кнопки
				const padding = 16; // Отступы (8px * 2)
				const gap = 8; // Промежуток между кнопками
				
				// Рассчитываем сколько колонок влезет по ширине
				const availableWidth = windowWidth - padding;
				const maxCols = Math.floor((availableWidth + gap) / (minButtonSize + gap));
				gridColumns = Math.max(1, Math.min(maxCols, 4)); // От 1 до 4 колонок
				
				// Рассчитываем сколько рядов влезет по высоте
				const availableHeight = windowHeight - padding;
				const maxRows = Math.floor((availableHeight + gap) / (minButtonSize + gap));
				gridRows = Math.max(1, Math.min(maxRows, 2)); // От 1 до 4 рядов
				
				console.log(`📐 Grid: ${gridColumns}×${gridRows} (${windowWidth}×${windowHeight})`);
			}
		};

		updateSize();

		const unlisten = appWindow.onResized(() => {
			updateSize();
		});

		return () => {
			unlisten.then(fn => fn());
		};
	});

	// Load quick presets from localStorage
	function loadQuickPresets() {
		if (typeof window === 'undefined') return;
		
		try {
			const saved = localStorage.getItem('quickPresets');
			if (saved) {
				const savedPresets = JSON.parse(saved);
				quickPresets = savedPresets
					.map((saved: DnsPreset) => 
						allPresets.find(p => p.id === saved.id)
					)
					.filter((p: DnsPreset | undefined): p is DnsPreset => p !== undefined);
				
				// console.log('⚡ Loaded quick presets from localStorage:', quickPresets);
				
				const presetNames = quickPresets.map(p => p.name);
				updateTrayMenu(
					currentDns?.primary || null,
					presetNames
				).catch(err => console.error('Failed to update tray menu:', err));
			}
		} catch (error) {
			console.error('Failed to load quick presets:', error);
		}
	}

	async function loadData() {
		if (!$selectedAdapter) {
			console.log('⚠️ No adapter selected');
			return;
		}

		loading = true;
		// console.log('🔄 Loading mini window data for adapter:', $selectedAdapter);
		
		try {
			const [dns, presets] = await Promise.all([
				getCurrentDns($selectedAdapter),
				getDnsPresets(),
			]);

			// console.log('📡 Current DNS:', dns);
			// console.log('📋 All presets:', presets);

			currentDns = dns;
			allPresets = presets;
			
			loadQuickPresets();

			if (dns.primary) {
				const matchedPreset = quickPresets.find(
					(p) => p.servers_ipv4[0] === dns.primary
				);
				selectedPresetId = matchedPreset ? matchedPreset.id : null;
				// console.log('🎯 Selected preset ID:', selectedPresetId);
			} else {
				selectedPresetId = null;
			}

			const tooltip = dns.primary ? `DNS: ${dns.primary}` : 'DNS: Auto (DHCP)';
			await updateTrayTooltip(tooltip);
			// console.log('💬 Updated tray tooltip:', tooltip);
		} catch (error) {
			console.error('❌ Failed to load data:', error);
		} finally {
			loading = false;
		}
	}

	async function applyPreset(preset: DnsPreset) {
		if (!$selectedAdapter || loading) return;

		console.log('🔧 Applying preset:', preset.name);
		loading = true;
		
		try {
			await setDns(
				$selectedAdapter,
				preset.servers_ipv4,
				preset.servers_ipv6 || [],
				preset.doh_template || null,
				preset.dot_hostname || null
			);

			selectedPresetId = preset.id;
			console.log('✅ Preset applied successfully');
			
			await loadData();
		} catch (error) {
			console.error('❌ Failed to apply preset:', error);
			alert(`Failed to apply preset: ${error}`);
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		// console.log('🚀 Mini window mounted');
		
		const initAdapter = async () => {
			try {
				const adapter = await getSelectedAdapter();
				if (adapter) {
					// console.log('📍 Got selected adapter from backend:', adapter);
					selectedAdapter.set(adapter);
					await loadData();
				} else {
					console.log('⚠️ No adapter selected yet');
				}
			} catch (error) {
				console.error('❌ Failed to get selected adapter:', error);
			}
		};

		initAdapter();

		const setupAdapterListener = async () => {
			adapterChangeUnlisten = await listen<string>('adapter-changed', async (event) => {
				const newAdapter = event.payload;
				console.log('📢 Adapter changed event received:', newAdapter);
				selectedAdapter.set(newAdapter);
				await loadData();
			});
		};

		setupAdapterListener();

		const setupPresetListener = async () => {
			presetIndexUnlisten = await listen<number>('apply-preset-from-tray', async (event) => {
				const presetIndex = event.payload;
				console.log('📢 Received tray event for preset index:', presetIndex);
				
				const preset = quickPresets[presetIndex];
				if (preset) {
					console.log('✅ Found matching preset, applying:', preset.name);
					await applyPreset(preset);
				} else {
					console.log('⚠️ Invalid preset index:', presetIndex);
				}
			});
		};

		setupPresetListener();

		const setupQuickPresetsListener = async () => {
			unlisten = await listen('quick-presets-changed', () => {
				console.log('📢 Quick presets changed, reloading...');
				loadQuickPresets();
			});
		};

		setupQuickPresetsListener();

		return () => {
			if (unlisten) unlisten();
			if (adapterChangeUnlisten) adapterChangeUnlisten();
			if (presetIndexUnlisten) presetIndexUnlisten();
		};
	});
</script>

{#if isCompactMode}
	<!-- Compact Mode: Icons only with drag region -->
	<div 
		class="compact-container" 
		data-tauri-drag-region
		style="
			--grid-columns: {gridColumns};
			--grid-rows: {gridRows};
		"
	>
		{#each quickPresets.slice(0, gridColumns * gridRows) as preset (preset.id)}
			<button
				class="compact-preset {selectedPresetId === preset.id ? 'selected' : ''}"
				onclick={() => applyPreset(preset)}
				disabled={loading || !$selectedAdapter}
				title={preset.name}
				style="background: {preset.color}"
			>
				<span class="compact-icon">{preset.icon}</span>
			</button>
		{/each}
	</div>
{:else}
	<!-- Normal Mode: Full UI -->
	<div class="mini-page">
		<section class="mini-section">
			<h3 class="mini-section-title">Current DNS</h3>
			<div class="current-dns-card">
				{#if loading && !currentDns}
					<div class="loading-state">
						<div class="spinner-small"></div>
					</div>
				{:else if currentDns}
					<div class="dns-status">
						<div class="dns-indicator {currentDns.primary ? 'active' : 'inactive'}"></div>
						<span class="dns-status-text">
							{currentDns.primary ? 'Custom' : 'Auto'}
						</span>
					</div>
					{#if currentDns.primary}
						<div class="dns-servers">
							<div class="dns-server">
								<span class="dns-label">Primary</span>
								<span class="dns-value">{currentDns.primary}</span>
							</div>
							{#if currentDns.secondary}
								<div class="dns-server">
									<span class="dns-label">Secondary</span>
									<span class="dns-value">{currentDns.secondary}</span>
								</div>
							{/if}
						</div>
					{:else}
						<p class="dns-dhcp">Using DHCP</p>
					{/if}
					{#if currentDns.doh_template}
						<div class="doh-badge">
							<svg
								width="12"
								height="12"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
							>
								<path
									d="M12 2L2 7v10c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V7l-10-5z"
								/>
								<path d="M9 12l2 2 4-4" />
							</svg>
							<span>DoH Active</span>
						</div>
					{/if}
				{:else}
					<p class="no-data">Select an adapter in main window</p>
				{/if}
			</div>
		</section>

		<section class="mini-section">
			<h3 class="mini-section-title">Quick Presets ({quickPresets.length}/4)</h3>
			<div class="presets-grid">
				{#if loading && quickPresets.length === 0}
					<div class="loading-state">
						<div class="spinner-small"></div>
					</div>
				{:else if quickPresets.length === 0}
					<div class="empty-state">
						<div class="empty-icon">⭐</div>
						<p class="empty-text">No quick presets</p>
						<p class="empty-hint">Add from main window</p>
					</div>
				{:else}
					{#each quickPresets as preset (preset.id)}
						<button
							class="preset-card {selectedPresetId === preset.id ? 'selected' : ''}"
							onclick={() => applyPreset(preset)}
							disabled={loading || !$selectedAdapter}
						>
							<div class="preset-icon" style="background: {preset.color}">
								<span style="font-size: 20px;">{preset.icon}</span>
							</div>
							<div class="preset-info">
								<h4 class="preset-name">{preset.name}</h4>
								<p class="preset-dns">{preset.servers_ipv4[0]}</p>
							</div>
							{#if preset.supports_doh}
								<div class="preset-badge" title="DNS over HTTPS">
									<svg
										width="10"
										height="10"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
									>
										<path
											d="M12 2L2 7v10c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V7l-10-5z"
										/>
									</svg>
								</div>
							{/if}
						</button>
					{/each}
				{/if}
			</div>
		</section>
	</div>
{/if}
<style>
	/* Прозрачный фон для компактного режима */
	:global(body) {
		background: transparent !important;
	}

	:global(html) {
		background: transparent !important;
	}

	/* Compact Mode Styles */
	.compact-container {
		width: 100%;
		height: 100%;
		padding: 4px;
		position: relative;
		overflow: hidden;
		box-sizing: border-box;
		background: transparent;
		display: grid;
		grid-template-columns: repeat(var(--grid-columns, 2), 1fr); /* ✅ Динамические колонки */
		grid-template-rows: repeat(var(--grid-rows, 2), 1fr); /* ✅ Динамические ряды */
		gap: 8px;
	}

	.compact-preset {
		width: 100%;
		height: 100%;
		min-width: 40px;
		min-height: 30px;
		border: none;
		border-radius: 16px; /* ✅ Более круглые углы */
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1); /* ✅ Плавная анимация */
		opacity: 0.9;
		box-shadow: 
			0 4px 12px rgba(0, 0, 0, 0.15),
			inset 0 1px 0 rgba(255, 255, 255, 0.2); /* ✅ Внутренняя подсветка */
		position: relative;
		z-index: 10;
		backdrop-filter: blur(10px); /* ✅ Размытие фона */
		border: 2px solid rgba(255, 255, 255, 0.1); /* ✅ Тонкая граница */
	}

	/* Эффект свечения при наведении */
	.compact-preset::before {
		content: '';
		position: absolute;
		inset: -2px;
		border-radius: 16px;
		background: linear-gradient(135deg, rgba(255, 255, 255, 0.3), rgba(255, 255, 255, 0));
		opacity: 0;
		transition: opacity 0.3s ease;
		z-index: -1;
	}

	.compact-preset:hover:not(:disabled) {
		opacity: 1;
		transform: translateY(-2px) scale(1.05); /* ✅ Поднятие и увеличение */
		box-shadow: 
			0 8px 24px rgba(0, 0, 0, 0.25),
			0 0 0 1px rgba(255, 255, 255, 0.2),
			inset 0 1px 0 rgba(255, 255, 255, 0.3);
	}

	.compact-preset:hover:not(:disabled)::before {
		opacity: 1;
	}

	.compact-preset.selected {
		opacity: 1;
		transform: scale(0.95); /* ✅ Небольшое сжатие для выбранного */
		box-shadow: 
			0 0 0 3px rgba(255, 255, 255, 0.8) inset, /* ✅ Белая внутренняя граница */
			0 8px 24px rgba(0, 0, 0, 0.3),
			0 0 20px rgba(255, 255, 255, 0.3); /* ✅ Внешнее свечение */
		border-color: rgba(255, 255, 255, 0.5);
	}

	.compact-preset:active:not(:disabled) {
		transform: translateY(0) scale(0.98);
		transition-duration: 0.1s;
	}

	.compact-preset:disabled {
		opacity: 0.3;
		cursor: not-allowed;
		filter: grayscale(0.5); /* ✅ Обесцвечивание неактивных */
	}

	.compact-icon {
		font-size: 20px; /* ✅ Чуть больше иконки */
		line-height: 1;
		filter: drop-shadow(0 2px 6px rgba(0, 0, 0, 0.3));
		pointer-events: none;
		transition: transform 0.3s ease;
	}

	.compact-preset:hover:not(:disabled) .compact-icon {
		transform: scale(1.1) rotate(5deg); /* ✅ Небольшое вращение при наведении */
	}

	.compact-preset.selected .compact-icon {
		transform: scale(1.15);
		filter: drop-shadow(0 3px 8px rgba(0, 0, 0, 0.4));
	}

	/* Пульсация для выбранного пресета */
	@keyframes selected-pulse {
		0%, 100% {
			box-shadow: 
				0 0 0 3px rgba(255, 255, 255, 0.8) inset,
				0 8px 24px rgba(0, 0, 0, 0.3),
				0 0 20px rgba(255, 255, 255, 0.3);
		}
		50% {
			box-shadow: 
				0 0 0 3px rgba(255, 255, 255, 0.8) inset,
				0 8px 24px rgba(0, 0, 0, 0.3),
				0 0 30px rgba(255, 255, 255, 0.5);
		}
	}

	.compact-preset.selected {
		animation: selected-pulse 2s ease-in-out infinite;
	}

	/* Normal Mode Styles - оставляем без изменений */
	.mini-page {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.mini-section {
		display: flex;
		flex-direction: column;
		padding-top: 4px;
		gap: 8px;
	}

	.mini-section-title {
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: #718096;
		margin: 0;
	}

	.current-dns-card {
		background: rgba(255, 255, 255, 0.5);
		border: 1px solid rgba(0, 0, 0, 0.1);
		border-radius: 10px;
		padding: 12px;
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	@media (prefers-color-scheme: dark) {
		.current-dns-card {
			background: rgba(30, 41, 59, 0.5);
			border-color: rgba(255, 255, 255, 0.1);
		}
	}

	.dns-status {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.dns-indicator {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #cbd5e0;
	}

	.dns-indicator.active {
		background: #48bb78;
		box-shadow: 0 0 0 3px rgba(72, 187, 120, 0.2);
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.6;
		}
	}

	.dns-status-text {
		font-size: 11px;
		font-weight: 600;
		color: #4a5568;
	}

	@media (prefers-color-scheme: dark) {
		.dns-status-text {
			color: #cbd5e0;
		}
	}

	.dns-servers {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.dns-server {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.dns-label {
		font-size: 9px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.3px;
		color: #a0aec0;
	}

	.dns-value {
		font-size: 11px;
		font-family: 'Courier New', monospace;
		color: #2d3748;
		word-break: break-all;
	}

	@media (prefers-color-scheme: dark) {
		.dns-value {
			color: #e2e8f0;
		}
	}

	.dns-dhcp,
	.no-data {
		font-size: 11px;
		color: #a0aec0;
		margin: 0;
	}

	.doh-badge {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 6px 10px;
		background: rgba(72, 187, 120, 0.1);
		border-radius: 6px;
		font-size: 10px;
		font-weight: 600;
		color: #48bb78;
	}

	.presets-grid {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.empty-state {
		text-align: center;
		padding: 20px;
	}

	.empty-icon {
		font-size: 2rem;
		opacity: 0.3;
		margin-bottom: 8px;
	}

	.empty-text {
		color: #a0aec0;
		font-size: 11px;
		margin: 0 0 4px 0;
		font-weight: 600;
	}

	.empty-hint {
		color: #a0aec0;
		font-size: 9px;
		margin: 0;
	}

	.preset-card {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px;
		background: rgba(255, 255, 255, 0.5);
		border: 1px solid rgba(0, 0, 0, 0.1);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s ease;
		position: relative;
	}

	.preset-card:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.8);
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.preset-card.selected {
		background: rgba(102, 126, 234, 0.15);
		border-color: #667eea;
		box-shadow: 0 0 0 2px rgba(102, 126, 234, 0.3);
	}

	.preset-card:active:not(:disabled) {
		transform: translateY(0);
	}

	.preset-card:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	@media (prefers-color-scheme: dark) {
		.preset-card {
			background: rgba(30, 41, 59, 0.5);
			border-color: rgba(255, 255, 255, 0.1);
		}

		.preset-card:hover:not(:disabled) {
			background: rgba(30, 41, 59, 0.8);
		}

		.preset-card.selected {
			background: rgba(6, 182, 212, 0.2);
			border-color: #06b6d4;
			box-shadow: 0 0 0 2px rgba(6, 182, 212, 0.3);
		}
	}

	.preset-icon {
		width: 32px;
		height: 32px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		flex-shrink: 0;
	}

	.preset-info {
		flex: 1;
		min-width: 0;
		text-align: left;
	}

	.preset-name {
		font-size: 12px;
		font-weight: 600;
		color: #2d3748;
		margin: 0 0 2px 0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	@media (prefers-color-scheme: dark) {
		.preset-name {
			color: #f7fafc;
		}
	}

	.preset-dns {
		font-size: 9px;
		font-family: 'Courier New', monospace;
		color: #718096;
		margin: 0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.preset-badge {
		width: 16px;
		height: 16px;
		background: rgba(72, 187, 120, 0.2);
		border-radius: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #48bb78;
		flex-shrink: 0;
	}

	.loading-state {
		display: flex;
		justify-content: center;
		padding: 20px;
	}

	.spinner-small {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(102, 126, 234, 0.2);
		border-top-color: #667eea;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@media (prefers-color-scheme: dark) {
		.spinner-small {
			border-color: rgba(6, 182, 212, 0.2);
			border-top-color: #06b6d4;
		}
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>