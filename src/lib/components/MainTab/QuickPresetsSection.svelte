<script lang="ts">
  import type { DnsPreset } from '$lib/types';
  import QuickPresetCard from './QuickPresetCard.svelte';
  
  interface Props {
    quickPresets: DnsPreset[];
    isLoading: boolean;
    selectedPreset: string | null;
    onApplyPreset: (preset: DnsPreset) => Promise<void>;
    onRemoveFromQuick: (presetId: string) => void;
  }
  
  let {
    quickPresets,
    isLoading,
    selectedPreset,
    onApplyPreset,
    onRemoveFromQuick
  }: Props = $props();
</script>

<div class="quick-presets-section">
  <div class="section-header">
    <h2 class="section-title">⚡ Quick Access</h2>
    <span class="preset-count">{quickPresets.length}/4</span>
  </div>
  
  {#if quickPresets.length === 0}
    <div class="empty-state">
      <div class="empty-icon">⭐</div>
      <p class="empty-text">Add favorites from Services tab</p>
    </div>
  {:else}
    <div class="quick-presets-grid">
      {#each quickPresets as preset}
        <QuickPresetCard
          {preset}
          {isLoading}
          isSelected={selectedPreset === preset.id}
          {onApplyPreset}
          {onRemoveFromQuick}
        />
      {/each}
    </div>
    
    {#if quickPresets.length < 4}
      <div class="add-hint">
        💡 {4 - quickPresets.length} more slot{4 - quickPresets.length > 1 ? 's' : ''} available
      </div>
    {/if}
  {/if}
</div>

<style>
  .quick-presets-section {
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .quick-presets-section {
      background: linear-gradient(135deg, rgba(45, 55, 72, 0.8) 0%, rgba(30, 41, 59, 0.8) 100%);
      border-color: rgba(255, 255, 255, 0.1);
    }
  }
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .section-title {
    font-size: 1rem;
    font-weight: 700;
    color: #2d3748;
    margin: 0;
  }
  
  @media (prefers-color-scheme: dark) {
    .section-title {
      color: #f7fafc;
    }
  }
  
  .preset-count {
    background: rgba(102, 126, 234, 0.1);
    color: #667eea;
    padding: 0.25rem 0.625rem;
    border-radius: 8px;
    font-weight: 700;
    font-size: 0.75rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .preset-count {
      background: rgba(6, 182, 212, 0.2);
      color: #06b6d4;
    }
  }
  
  .empty-state {
    text-align: center;
    padding: 2rem 1rem;
  }
  
  .empty-icon {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
    opacity: 0.4;
  }
  
  .empty-text {
    font-size: 0.875rem;
    color: #718096;
    margin: 0;
  }
  
  @media (prefers-color-scheme: dark) {
    .empty-text {
      color: #cbd5e0;
    }
  }
  
  .quick-presets-grid {
    display: grid;
    gap: 0.75rem;
  }
  
  .add-hint {
    text-align: center;
    font-size: 0.75rem;
    color: #718096;
    padding: 0.375rem;
    background: rgba(102, 126, 234, 0.05);
    border-radius: 6px;
  }
  
  @media (prefers-color-scheme: dark) {
    .add-hint {
      color: #cbd5e0;
      background: rgba(6, 182, 212, 0.1);
    }
  }
</style>