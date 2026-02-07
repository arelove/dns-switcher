import type { DnsPreset } from '$lib/types';

// Extended type for presets with ID and color
export type DnsPresetWithId = DnsPreset & {
  color?: string;
  primary?: string;
  secondary?: string;
};