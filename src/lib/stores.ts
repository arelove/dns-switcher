// src/lib/stores.ts
import { writable, derived } from 'svelte/store';
import type { Writable } from 'svelte/store';
import type { DnsConfiguration, DnsPreset, NetworkAdapter } from './types';

// ────────────────────────────────────────────────
// Types
export type Theme = 'light' | 'dark' | 'auto' | 'system';
export type NotificationType = 'info' | 'success' | 'error' | 'warning';

export interface NotificationState {
  show: boolean;
  message: string;
  type: NotificationType;
  duration?: number; // in ms
}

// ────────────────────────────────────────────────
// Theme
const initialTheme = getInitialTheme();
export const theme: Writable<Theme> = writable(initialTheme);

if (typeof window !== 'undefined') {
  theme.subscribe((value) => {
    localStorage.setItem('theme', value);
    // Handle 'auto' / 'system' variant
    if (value === 'auto' || value === 'system') {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      document.documentElement.classList.toggle('dark', prefersDark);
    } else {
      document.documentElement.classList.toggle('dark', value === 'dark');
    }
  });

  // Listen to system preference changes when in 'auto' mode
  window
    .matchMedia('(prefers-color-scheme: dark)')
    .addEventListener('change', (e) => {
      theme.subscribe((current) => {
        if (current === 'auto' || current === 'system') {
          document.documentElement.classList.toggle('dark', e.matches);
        }
      })();
    });
}

function getInitialTheme(): Theme {
  if (typeof window === 'undefined') return 'dark';
  const saved = localStorage.getItem('theme') as Theme | null;
  return saved && ['light', 'dark', 'auto', 'system'].includes(saved)
    ? saved
    : 'dark';
}

// ────────────────────────────────────────────────
// Locale
const initialLocale = typeof window !== 'undefined'
  ? (localStorage.getItem('locale') as string) || 'en'
  : 'en';

export const locale = writable<string>(initialLocale);

if (typeof window !== 'undefined') {
  locale.subscribe((value) => {
    localStorage.setItem('locale', value);
  });
}

// ────────────────────────────────────────────────
// Network / DNS related stores

// ✅ Shared selected adapter (used by both main and mini windows)
export const selectedAdapter = writable<string | null>(null);

export const adapters = writable<NetworkAdapter[]>([]);
export const currentDns = writable<DnsConfiguration | null>(null);
export const dnsPresets = writable<DnsPreset[]>([]);
export const isLoading = writable<boolean>(false);

// Custom DNS input fields
export const customPrimary = writable<string>('');
export const customSecondary = writable<string>('');
export const customDohTemplate = writable<string>('');

// Quick testing / debug
export const testResults = writable<Record<string, unknown>>({});

// ────────────────────────────────────────────────
// Notification
export const notification = writable<NotificationState>({
  show: false,
  message: '',
  type: 'info',
  duration: 5000,
});

let notificationTimeout: number | undefined;

export const notificationStore = {
  show: (message: string, type: NotificationType = 'info', duration = 5000) => {
    // Clear previous timeout
    if (notificationTimeout) {
      clearTimeout(notificationTimeout);
    }

    notification.set({ show: true, message, type, duration });

    // Auto-hide after duration
    notificationTimeout = window.setTimeout(() => {
      notification.update((n) => (n.show ? { ...n, show: false } : n));
    }, duration);
  },

  hide: () => {
    if (notificationTimeout) {
      clearTimeout(notificationTimeout);
    }
    notification.update((n) => ({ ...n, show: false }));
  },

  // Convenience methods
  success: (msg: string, duration?: number) => notificationStore.show(msg, 'success', duration),
  error: (msg: string, duration?: number) => notificationStore.show(msg, 'error', duration),
  info: (msg: string, duration?: number) => notificationStore.show(msg, 'info', duration),
  warning: (msg: string, duration?: number) => notificationStore.show(msg, 'warning', duration),
};

// ────────────────────────────────────────────────
// Derived stores

// Has any adapter selected
export const hasSelectedAdapter = derived(
  selectedAdapter, 
  ($adapter) => !!$adapter
);