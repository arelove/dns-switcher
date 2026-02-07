<script lang="ts">
  import type { DnsConfiguration } from '$lib/types';
  
  interface Props {
    currentDns: DnsConfiguration;
  }
  
  let { currentDns }: Props = $props();
</script>

<div class="dns-details">
  <!-- IPv4 Section -->
  {#if !currentDns.is_dhcp && currentDns.primary}
    <div class="detail-section">
      <div class="detail-header">
        <span class="detail-icon">🌐</span>
        <span class="detail-title">IPv4</span>
      </div>
      <div class="detail-content">
        <div class="detail-item">
          <span class="detail-label">IP1:</span>
          <code class="detail-value">{currentDns.primary}</code>
        </div>
        {#if currentDns.secondary}
          <div class="detail-item">
            <span class="detail-label">IP2:</span>
            <code class="detail-value">{currentDns.secondary}</code>
          </div>
        {/if}
      </div>
    </div>
  {/if}
  
  <!-- IPv6 Section -->
  {#if !currentDns.is_dhcp && (currentDns.primary_ipv6 || currentDns.secondary_ipv6)}
    <div class="detail-section">
      <div class="detail-header">
        <span class="detail-icon">🌍</span>
        <span class="detail-title">IPv6</span>
      </div>
      <div class="detail-content">
        {#if currentDns.primary_ipv6}
          <div class="detail-item">
            <span class="detail-label">IP1:</span>
            <code class="detail-value ipv6">{currentDns.primary_ipv6}</code>
          </div>
        {/if}
        {#if currentDns.secondary_ipv6}
          <div class="detail-item">
            <span class="detail-label">IP2:</span>
            <code class="detail-value ipv6">{currentDns.secondary_ipv6}</code>
          </div>
        {/if}
      </div>
    </div>
  {/if}
  
  <!-- Security Features -->
 <!-- Status Bar -->
  <div class="status-bar">
    <div class="status-item">
      <span class="status-icon">{currentDns.is_dhcp ? '🔄' : '📌'}</span>
      <span class="status-label">{currentDns.is_dhcp ? 'Auto (DHCP)' : 'Static'}</span>
    </div>
    {#if !currentDns.is_dhcp}
      <div class="status-item" class:active={currentDns.doh_enabled}>
        <span class="status-icon">{currentDns.doh_enabled ? '🔒' : '🔓'}</span>
        <span class="status-label">DoH</span>
      </div>
    {/if}
  </div>
  
</div>

<style>
  .dns-details {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .detail-section {
    background: rgba(255, 255, 255, 0.5);
    border-radius: 6px;
    padding: 0.625rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .detail-section {
      background: rgba(0, 0, 0, 0.2);
    }
  }
  
  .detail-header {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    margin-bottom: 0.375rem;
    font-size: 0.6875rem;
    font-weight: 700;
    color: #4a5568;
    text-transform: uppercase;
    letter-spacing: 0.3px;
  }
  
  @media (prefers-color-scheme: dark) {
    .detail-header {
      color: #cbd5e0;
    }
  }
  
  .detail-icon {
    font-size: 0.75rem;
  }
  
  .detail-content {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }
  
  .detail-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
  }
  
  .detail-label {
    font-size: 0.6875rem;
    font-weight: 600;
    color: #718096;
    min-width: 2rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .detail-label {
      color: #cbd5e0;
    }
  }
  
  .detail-value {
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
  
  .detail-value.ipv6 {
    font-size: 0.625rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .detail-value {
      color: #f7fafc;
      background: rgba(6, 182, 212, 0.2);
    }
  }
  
  .status-bar {
  display: flex;
  gap: 0.5rem;
  padding: 0.5rem;
  background: rgba(255, 255, 255, 0.5);
  border-radius: 6px;
}

@media (prefers-color-scheme: dark) {
  .status-bar {
    background: rgba(0, 0, 0, 0.2);
  }
}

.status-item {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.25rem 0.5rem;
  background: rgba(0, 0, 0, 0.05);
  border-radius: 4px;
  font-size: 0.75rem;
  font-weight: 600;
  color: #718096;
}

.status-item.active {
  background: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

@media (prefers-color-scheme: dark) {
  .status-item {
    background: rgba(255, 255, 255, 0.05);
    color: #cbd5e0;
  }
  .status-item.active {
    background: rgba(16, 185, 129, 0.2);
    color: #34d399;
  }
}

.status-icon {
  font-size: 0.875rem;
  line-height: 1;
}

.status-label {
  font-size: 0.6875rem;
}
</style>