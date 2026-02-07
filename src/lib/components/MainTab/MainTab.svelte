<script lang="ts">
  import type { DnsConfiguration, DnsPreset } from '$lib/types';
  import CurrentDnsCard from './CurrentDnsCard.svelte';
  import QuickPresetsSection from './QuickPresetsSection.svelte';
  
  interface Props {
    currentDns: DnsConfiguration | null;
    quickPresets: DnsPreset[];
    isLoading: boolean;
    selectedPreset: string | null;
    allServices: DnsPreset[];
    onResetDns: () => Promise<void>;
    onApplyPreset: (preset: DnsPreset) => Promise<void>;
    onRemoveFromQuick: (presetId: string) => void;
  }
  
  let {
    currentDns,
    quickPresets,
    isLoading,
    selectedPreset,
    allServices,
    onResetDns,
    onApplyPreset,
    onRemoveFromQuick
  }: Props = $props();
</script>

<div class="main-tab">
  <CurrentDnsCard 
    {currentDns}
    {isLoading}
    {allServices}
    {onResetDns}
  />
  
  <QuickPresetsSection
    {quickPresets}
    {isLoading}
    {selectedPreset}
    {onApplyPreset}
    {onRemoveFromQuick}
  />
</div>

<style>
  .main-tab {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
  }
  
  @media (max-width: 768px) {
    .main-tab {
      grid-template-columns: 1fr;
    }
  }
</style>