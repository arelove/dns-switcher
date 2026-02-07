<script lang="ts">
  import type { DnsPresetWithId } from '../types';
  
  interface Props {
    preset: DnsPresetWithId;
    category: string;
    isQuickPreset: boolean;
    isLoading: boolean;
    onApply: () => Promise<void>;
    onToggleQuick: () => void;
    onDelete?: () => void;
    onShowDetail: () => void;
  }
  
  let {
    preset,
    category,
    isQuickPreset,
    isLoading,
    onApply,
    onToggleQuick,
    onDelete,
    onShowDetail
  }: Props = $props();
</script>

<div 
  class="service-card" 
  data-category={category}
  onclick={onShowDetail}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === 'Enter' && onShowDetail()}
>
  <div class="service-header">
    <span class="service-icon">{preset.icon}</span>
    <div class="service-info">
      <h4 class="service-name">{preset.name}</h4>
      <p class="service-desc">{preset.description}</p>
      {#if preset.supports_doh || preset.supports_dot || (preset.servers_ipv6 && preset.servers_ipv6.length > 0)}
        <div class="service-badges">
          {#if preset.supports_doh}<span class="badge doh">DoH</span>{/if}
          {#if preset.supports_dot}<span class="badge dot">DoT</span>{/if}
          {#if preset.servers_ipv6 && preset.servers_ipv6.length > 0}<span class="badge ipv6">IPv6</span>{/if}
        </div>
      {/if}
    </div>
  </div>
  
  <div class="service-dns">
    {#if preset.servers_ipv4 && preset.servers_ipv4.length > 0}
      <div class="dns-section">
        <div class="dns-section-label">
          <span class="dns-icon">🌐</span>
          <span>IPv4</span>
        </div>
        {#each preset.servers_ipv4 as server, index}
          <div class="dns-item">
            <span class="dns-item-label">IP{index + 1}:</span>
            <code class="dns-item-value">{server}</code>
          </div>
        {/each}
      </div>
    {/if}
    
    {#if preset.servers_ipv6 && preset.servers_ipv6.length > 0}
      <div class="dns-section">
        <div class="dns-section-label">
          <span class="dns-icon">🌍</span>
          <span>IPv6</span>
        </div>
        {#each preset.servers_ipv6 as server, index}
          <div class="dns-item">
            <span class="dns-item-label">IP{index + 1}:</span>
            <code class="dns-item-value ipv6">{server}</code>
          </div>
        {/each}
      </div>
    {/if}
  </div>
  
  <div
    class="service-actions"
    role="group"
    aria-label="Service actions"
    tabindex="-1"
  >
    <button class="apply-service-btn" onclick={onApply} disabled={isLoading}>
      Apply
    </button>
    <button
      class="quick-toggle-btn"
      class:active={isQuickPreset}
      onclick={onToggleQuick}
      title={isQuickPreset ? 'Remove from quick presets' : 'Add to quick presets'}
    >
      ⚡
    </button>
    {#if onDelete}
      <button
        class="delete-service-btn"
        onclick={onDelete}
        title="Delete custom service"
      >
        🗑️
      </button>
    {/if}
  </div>
</div>

<style>
  .service-card {
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border: 2px solid #e2e8f0;
    border-radius: 10px;
    padding: 1rem;
    transition: all 0.3s ease;
    cursor: pointer;
  }
  
  .service-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
  
  .service-card[data-category="security"] {
    border-left: 3px solid #10b981;
  }
  .service-card[data-category="privacy"] {
    border-left: 3px solid #3b82f6;
  }
  .service-card[data-category="adblock"] {
    border-left: 3px solid #ef4444;
  }
  .service-card[data-category="gaming"] {
    border-left: 3px solid #8b5cf6;
  }
  .service-card[data-category="family"] {
    border-left: 3px solid #f59e0b;
  }
  
  @media (prefers-color-scheme: dark) {
    .service-card {
      background: linear-gradient(135deg, rgba(45, 55, 72, 0.8) 0%, rgba(30, 41, 59, 0.8) 100%);
      border-color: rgba(255, 255, 255, 0.1);
    }
  }
  
  .service-header {
    display: flex;
    align-items: flex-start;
    gap: 0.625rem;
    margin-bottom: 0.75rem;
  }
  
  .service-icon {
    font-size: 1.75rem;
  }
  
  .service-info {
    flex: 1;
  }
  
  .service-name {
    font-size: 1rem;
    font-weight: 700;
    color: #2d3748;
    margin: 0 0 0.25rem 0;
  }
  
  @media (prefers-color-scheme: dark) {
    .service-name {
      color: #f7fafc;
    }
  }
  
  .service-desc {
    font-size: 0.8125rem;
    color: #718096;
    margin: 0 0 0.5rem 0;
    min-height: 2.25rem;
    line-height: 1.125rem;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  
  @media (prefers-color-scheme: dark) {
    .service-desc {
      color: #cbd5e0;
    }
  }
  
  .service-badges {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
  }
  
  .badge {
    display: inline-flex;
    align-items: center;
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
    font-size: 0.625rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.3px;
  }
  
  .badge.doh {
    background: rgba(16, 185, 129, 0.15);
    color: #059669;
  }
  .badge.dot {
    background: rgba(59, 130, 246, 0.15);
    color: #2563eb;
  }
  .badge.ipv6 {
    background: rgba(168, 85, 247, 0.15);
    color: #7c3aed;
  }
  
  @media (prefers-color-scheme: dark) {
    .badge.doh {
      background: rgba(16, 185, 129, 0.2);
      color: #34d399;
    }
    .badge.dot {
      background: rgba(59, 130, 246, 0.2);
      color: #60a5fa;
    }
    .badge.ipv6 {
      background: rgba(168, 85, 247, 0.2);
      color: #a78bfa;
    }
  }
  
  .service-dns {
    display: grid;
    gap: 0.625rem;
    margin-bottom: 0.75rem;
    padding: 0.625rem;
    background: rgba(255, 255, 255, 0.5);
    border-radius: 6px;
  }
  
  @media (prefers-color-scheme: dark) {
    .service-dns {
      background: rgba(0, 0, 0, 0.2);
    }
  }
  
  .dns-section {
    display: grid;
    gap: 0.375rem;
  }
  
  .dns-section:not(:last-child) {
    padding-bottom: 0.625rem;
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
  }
  
  @media (prefers-color-scheme: dark) {
    .dns-section:not(:last-child) {
      border-bottom-color: rgba(255, 255, 255, 0.05);
    }
  }
  
  .dns-section-label {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.6875rem;
    font-weight: 700;
    color: #4a5568;
    text-transform: uppercase;
    letter-spacing: 0.3px;
  }
  
  @media (prefers-color-scheme: dark) {
    .dns-section-label {
      color: #cbd5e0;
    }
  }
  
  .dns-icon {
    font-size: 0.75rem;
  }
  
  .dns-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
  }
  
  .dns-item-label {
    font-size: 0.6875rem;
    font-weight: 600;
    color: #4a5568;
    white-space: nowrap;
    min-width: 2rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .dns-item-label {
      color: #cbd5e0;
    }
  }
  
  .dns-item-value {
    font-family: monospace;
    font-size: 0.6875rem;
    color: #2d3748;
    background: rgba(102, 126, 234, 0.1);
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
    word-break: break-all;
    flex: 1;
    text-align: right;
  }
  
  .dns-item-value.ipv6 {
    font-size: 0.6rem;
    background: rgba(59, 130, 246, 0.1);
  }
  
  @media (prefers-color-scheme: dark) {
    .dns-item-value {
      color: #f7fafc;
      background: rgba(6, 182, 212, 0.2);
    }
    .dns-item-value.ipv6 {
      background: rgba(59, 130, 246, 0.2);
    }
  }
  
  .service-actions {
    display: flex;
    gap: 0.5rem;
  }
  
  .apply-service-btn {
    flex: 1;
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: 6px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    font-weight: 600;
    font-size: 0.8125rem;
    cursor: pointer;
    transition: all 0.3s ease;
  }
  
  .apply-service-btn:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);
  }
  
  .apply-service-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  @media (prefers-color-scheme: dark) {
    .apply-service-btn {
      background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
    }
  }
  
  .quick-toggle-btn,
  .delete-service-btn {
    width: 2.25rem;
    height: 2.25rem;
    border: 2px solid #e2e8f0;
    border-radius: 6px;
    background: white;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.875rem;
  }
  
  .quick-toggle-btn:hover {
    border-color: #667eea;
    background: rgba(102, 126, 234, 0.1);
  }
  
  .quick-toggle-btn.active {
    background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
    border-color: #f59e0b;
  }
  
  .delete-service-btn:hover {
    border-color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
  }
  
  @media (prefers-color-scheme: dark) {
    .quick-toggle-btn,
    .delete-service-btn {
      background: rgba(45, 55, 72, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
    }
    .quick-toggle-btn:hover {
      border-color: #06b6d4;
      background: rgba(6, 182, 212, 0.2);
    }
    .quick-toggle-btn.active {
      background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
    }
    .delete-service-btn:hover {
      border-color: #ef4444;
      background: rgba(239, 68, 68, 0.2);
    }
  }
</style>