<script lang="ts">
  import type { DnsPreset } from '$lib/types';
  
  interface Props {
    preset: DnsPreset;
    isLoading: boolean;
    isSelected: boolean;
    onApplyPreset: (preset: DnsPreset) => Promise<void>;
    onRemoveFromQuick: (presetId: string) => void;
  }
  
  let {
    preset,
    isLoading,
    isSelected,
    onApplyPreset,
    onRemoveFromQuick
  }: Props = $props();
</script>

<div class="quick-preset-card" style="border-left: 3px solid {preset.color}">
  <button
    class="preset-card-btn"
    onclick={() => onApplyPreset(preset)}
    disabled={isLoading || isSelected}
  >
    <div class="preset-header">
      <span class="preset-icon">{preset.icon}</span>
      <span class="preset-name">{preset.name}</span>
    </div>
    
    <div class="preset-details">
      <div class="dns-row">
        <span class="dns-label">IP1:</span>
        <code class="dns-value">{preset.servers_ipv4?.[0] || 'N/A'}</code>
      </div>
      {#if preset.servers_ipv4?.[1]}
        <div class="dns-row">
          <span class="dns-label">IP2:</span>
          <code class="dns-value">{preset.servers_ipv4[1]}</code>
        </div>
      {/if}
    </div>
    
    {#if preset.supports_doh || preset.supports_dot || preset.servers_ipv6?.length}
      <div class="preset-features">
        {#if preset.supports_doh}<span class="tag doh">DoH</span>{/if}
        {#if preset.supports_dot}<span class="tag dot">DoT</span>{/if}
        {#if preset.servers_ipv6?.length}<span class="tag ipv6">IPv6</span>{/if}
      </div>
    {/if}
  </button>
  
  <button
    class="remove-btn"
    onclick={() => onRemoveFromQuick(preset.id)}
    title="Remove from quick presets"
  >
    ✕
  </button>
</div>

<style>
  .quick-preset-card {
    position: relative;
    background: white;
    border-radius: 10px;
    overflow: hidden;
    transition: all 0.2s ease;
  }
  
  .quick-preset-card:hover {
    transform: translateX(3px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }
  
  @media (prefers-color-scheme: dark) {
    .quick-preset-card {
      background: rgba(45, 55, 72, 0.6);
    }
  }
  
  .preset-card-btn {
    width: 100%;
    padding: 0.75rem;
    border: none;
    background: transparent;
    cursor: pointer;
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .preset-card-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .preset-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  
  .preset-icon {
    font-size: 1.25rem;
    line-height: 1;
  }
  
  .preset-name {
    font-weight: 700;
    color: #2d3748;
    font-size: 0.875rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .preset-name {
      color: #f7fafc;
    }
  }
  
  .preset-details {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  
  .dns-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  
  .dns-label {
    font-size: 0.6875rem;
    font-weight: 600;
    color: #718096;
    min-width: 2rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .dns-label {
      color: #cbd5e0;
    }
  }
  
  .dns-value {
    font-family: monospace;
    font-size: 0.6875rem;
    color: #2d3748;
    background: rgba(102, 126, 234, 0.1);
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
    flex: 1;
  }
  
  @media (prefers-color-scheme: dark) {
    .dns-value {
      color: #f7fafc;
      background: rgba(6, 182, 212, 0.2);
    }
  }
  
  .preset-features {
    display: flex;
    gap: 0.375rem;
  }
  
  .tag {
    font-size: 0.625rem;
    font-weight: 600;
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
  }
  
  .tag.doh {
    background: rgba(16, 185, 129, 0.1);
    color: #059669;
  }
  
  .tag.dot {
    background: rgba(59, 130, 246, 0.1);
    color: #2563eb;
  }
  
  .tag.ipv6 {
    background: rgba(139, 92, 246, 0.1);
    color: #7c3aed;
  }
  
  @media (prefers-color-scheme: dark) {
    .tag.doh {
      background: rgba(16, 185, 129, 0.2);
      color: #10b981;
    }
    .tag.dot {
      background: rgba(59, 130, 246, 0.2);
      color: #60a5fa;
    }
    .tag.ipv6 {
      background: rgba(139, 92, 246, 0.2);
      color: #a78bfa;
    }
  }
  
  .remove-btn {
    position: absolute;
    top: 0.375rem;
    right: 0.375rem;
    width: 1.25rem;
    height: 1.25rem;
    border: none;
    border-radius: 50%;
    background: #ef4444;
    color: white;
    font-size: 0.625rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
    opacity: 0;
  }
  
  .quick-preset-card:hover .remove-btn {
    opacity: 1;
  }
  
  .remove-btn:hover {
    background: #dc2626;
    transform: scale(1.1);
  }
</style>