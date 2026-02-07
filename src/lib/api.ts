import { invoke } from '@tauri-apps/api/core';
import type {
	NetworkAdapter,
	DnsConfiguration,
	DnsPreset,
	DnsTestResult,
	WindowsVersion,
} from './types';
import { getPresetColor } from './presetUI';

export async function getNetworkAdapters(): Promise<NetworkAdapter[]> {
	return await invoke('get_network_adapters');
}

export async function getCurrentDns(adapterName: string): Promise<DnsConfiguration> {
	return await invoke('get_current_dns', { adapterName });
}

export async function setDns(
	adapterName: string,
	ipv4Servers: string[],
	ipv6Servers: string[] = [],
	dohTemplate: string | null = null,
	dotHostname: string | null = null
): Promise<void> {
	return invoke('set_dns', {
		adapterName,
		ipv4Servers,
		ipv6Servers,
		dohTemplate,
		dotHostname,
	});
}

export async function resetDns(adapterName: string): Promise<void> {
	return await invoke('reset_dns', { adapterName });
}

export async function testDns(server: string): Promise<DnsTestResult> {
	return await invoke('test_dns', { server });
}

export async function getDnsPresets(): Promise<DnsPreset[]> {
	const presets = await invoke<DnsPreset[]>('get_dns_presets');
	return presets.map((preset) => ({
		...preset,
		color: getPresetColor(preset.id, preset.category === 'custom'),
	}));
}

export async function getPresetById(id: string): Promise<DnsPreset | null> {
	const preset = await invoke<DnsPreset | null>('get_preset_by_id', { id });
	if (preset) {
		return {
			...preset,
			color: getPresetColor(preset.id, preset.category === 'custom'),
		};
	}
	return null;
}

export async function flushDnsCache(): Promise<void> {
	return await invoke('flush_dns_cache');
}

export async function getWindowsVersion(): Promise<WindowsVersion> {
	return await invoke('get_windows_version');
}

export async function getCustomPresets(): Promise<DnsPreset[]> {
	const presets = await invoke<DnsPreset[]>('get_custom_presets');
	return presets.map((preset) => ({
		...preset,
		color: getPresetColor(preset.id, true),
	}));
}

export async function addCustomPreset(preset: Omit<DnsPreset, 'color'>): Promise<void> {
	return await invoke('add_custom_preset', { preset });
}

export async function deleteCustomPreset(id: string): Promise<void> {
	return await invoke('delete_custom_preset', { id });
}

export async function updateCustomPreset(preset: Omit<DnsPreset, 'color'>): Promise<void> {
	return await invoke('update_custom_preset', { preset });
}

// Mini window operations
export async function toggleMiniWindow(): Promise<void> {
	return invoke('toggle_mini_window');
}

export async function setMiniAlwaysOnTop(alwaysOnTop: boolean): Promise<void> {
	return invoke('set_mini_always_on_top', { alwaysOnTop });
}

export async function updateTrayTooltip(tooltip: string): Promise<void> {
	return invoke('update_tray_tooltip', { tooltip });
}

export async function updateTrayMenu(
	currentDns: string | null,
	quickPresetNames: string[]
): Promise<void> {
	return invoke('update_tray_menu', { 
		currentDns, 
		quickPresetNames 
	});
}

// Shared adapter state
export async function getSelectedAdapter(): Promise<string | null> {
	return invoke('get_selected_adapter');
}

export async function setSelectedAdapter(adapterName: string): Promise<void> {
	return invoke('set_selected_adapter', { adapterName });
}