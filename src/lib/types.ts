export interface NetworkAdapter {
  name: string;
  description: string;
  is_connected: boolean;
  current_dns: DnsConfiguration;
}

export interface DnsConfiguration {
  primary: string | null;
  secondary: string | null;
  primary_ipv6: string | null;
  secondary_ipv6: string | null;
  doh_enabled: boolean;
  doh_template: string | null;
  dot_hostname: string | null;
  is_dhcp: boolean;
  original_primary: string | null;
  original_secondary: string | null;
  original_primary_ipv6: string | null;
  original_secondary_ipv6: string | null;
}

export interface DnsPreset {
  id: string;
  name: string;
  description: string;
  category: 'public' | 'security' | 'privacy' | 'gaming' | 'family' | 'custom' | 'adblock';
  servers_ipv4: string[];
  servers_ipv6: string[];
  doh_template: string | null;
  dot_hostname: string | null;
  supports_doh: boolean;
  supports_dot: boolean;
  icon: string;
  color: string;
  website?: string | null;
}

// Расширенный тип для UI (с цветом)
export interface DnsPresetWithUI extends DnsPreset {
  color: string;
}

export interface DnsTestResult {
  server: string;
  latency_ms: number | null;
  is_available: boolean;
  error?: string;
}

export interface WindowsVersion {
  major: number;
  minor: number;
  build: number;
  supports_doh: boolean;
}

export interface AppState {
  adapters: NetworkAdapter[];
  selectedAdapter: string | null;
  currentDns: DnsConfiguration | null;
  presets: DnsPreset[];
  customPresets: DnsPreset[];
  loading: boolean;
  windowsVersion: WindowsVersion | null;
}

export type NotificationType = 'success' | 'error' | 'info' | 'warning';

export interface Notification {
  id: string;
  type: NotificationType;
  message: string;
  duration?: number;
}

// Custom DNS Lists
export interface CustomDnsList {
	id: string;
	name: string;
	description: string;
	icon: string;
	presets: string[]; // Array of preset IDs
	createdAt: number;
	updatedAt?: number;
}

// Category metadata
export interface CategoryInfo {
	id: string;
	name: string;
	icon: string;
	description: string;
	color: string;
}

export const DNS_CATEGORIES: Record<string, CategoryInfo> = {
	public: {
		id: 'public',
		name: 'Public DNS',
		icon: '🌐',
		description: 'General-purpose public DNS services',
		color: '#667eea'
	},
	privacy: {
		id: 'privacy',
		name: 'Privacy-Focused',
		icon: '🔒',
		description: 'DNS with enhanced privacy protection',
		color: '#3b82f6'
	},
	security: {
		id: 'security',
		name: 'Security',
		icon: '🛡️',
		description: 'DNS with malware and phishing protection',
		color: '#10b981'
	},
	adblock: {
		id: 'adblock',
		name: 'Ad Blocking',
		icon: '🚫',
		description: 'DNS with built-in ad and tracker blocking',
		color: '#ef4444'
	},
	family: {
		id: 'family',
		name: 'Family Protection',
		icon: '👨‍👩‍👧‍👦',
		description: 'Family-friendly DNS with content filtering',
		color: '#f59e0b'
	},
	gaming: {
		id: 'gaming',
		name: 'Gaming',
		icon: '🎮',
		description: 'Optimized DNS for gaming and low latency',
		color: '#8b5cf6'
	}
};