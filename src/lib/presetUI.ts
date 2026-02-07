import type { DnsPresetWithUI } from "./types";

// UI-специфичные данные для пресетов
export const presetUIConfig: Record<string, { color: string }> = {
  'xbox-dns': { color: '#107c10' },
  'google': { color: '#4285f4' },
  'cloudflare': { color: '#f6821f' },
  'cloudflare-malware': { color: '#ff6b35' },
  'cloudflare-family': { color: '#6366f1' },
  'quad9': { color: '#00897b' },
  'opendns': { color: '#ff8c00' },
  'adguard': { color: '#66b574' },
  'adguard-family': { color: '#4ade80' },
  'yandex-basic': { color: '#ff0000' },
  'yandex-safe': { color: '#dc2626' },
  'yandex-family': { color: '#f97316' },
  'comodo': { color: '#e91e63' },
  'level3': { color: '#2196f3' },
};

// Дефолтный цвет для кастомных пресетов
export const defaultCustomColor = '#9333ea';

// Получить цвет пресета
export function getPresetColor(id: string, isCustom: boolean = false): string {
  if (isCustom) return defaultCustomColor;
  return presetUIConfig[id]?.color || '#6b7280';
}


// src/lib/presetUI.ts
export const categoryLabels: Record<string, string> = {
  public: 'Public DNS',
  security: 'Security',
  privacy: 'Privacy',
  gaming: 'Gaming',
  family: 'Family Safe',
  adblock: 'Ad Blocking',
  custom: 'Custom'
};

export function getPresetsByCategory(presets: DnsPresetWithUI[], category: string) {
  return presets.filter(p => p.category === category);
}