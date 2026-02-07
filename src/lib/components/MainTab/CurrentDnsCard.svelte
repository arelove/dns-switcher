<script lang="ts">
  import type { DnsConfiguration, DnsPreset } from '$lib/types';
  import DnsStatusSection from './DnsStatusSection.svelte';
  import DnsDetailsSection from './DnsDetailsSection.svelte';
  
  interface Props {
    currentDns: DnsConfiguration | null;
    isLoading: boolean;
    allServices: DnsPreset[];
    onResetDns: () => Promise<void>;
  }
  
  let {
    currentDns,
    isLoading,
    allServices,
    onResetDns
  }: Props = $props();
  
  function getCurrentDnsName(): string {
    if (!currentDns) return 'Loading...';
    if (currentDns.is_dhcp) return 'Auto (DHCP)';
    
    const matchedPreset = allServices.find(
      p => p.servers_ipv4?.[0] === currentDns?.primary
    );
    
    return matchedPreset ? matchedPreset.name : 'Custom';
  }
  
  function getCurrentPresetIcon(): string {
    if (!currentDns || currentDns.is_dhcp) return '🔄';
    
    const matchedPreset = allServices.find(
      p => p.servers_ipv4?.[0] === currentDns?.primary
    );
    
    return matchedPreset?.icon || '⚙️';
  }
  
  function getCurrentPresetColor(): string {
    if (!currentDns || currentDns.is_dhcp) return '#f59e0b';
    
    const matchedPreset = allServices.find(
      p => p.servers_ipv4?.[0] === currentDns?.primary
    );
    
    return matchedPreset?.color || '#6b7280';
  }
</script>

<div class="current-dns-card">
  <div class="card-header">
    <h2 class="section-title">📡 Current DNS</h2>
    <button 
      class="reset-btn"
      onclick={onResetDns}
      disabled={isLoading || currentDns?.is_dhcp}
      title="Reset to automatic DNS"
    >
      🔄
    </button>
  </div>
  
  <DnsStatusSection
    dnsName={getCurrentDnsName()}
    icon={getCurrentPresetIcon()}
    color={getCurrentPresetColor()}
  />
  
  {#if currentDns}
    <DnsDetailsSection {currentDns} />
  {/if}
</div>

<style>
  .current-dns-card {
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .current-dns-card {
      background: linear-gradient(135deg, rgba(45, 55, 72, 0.8) 0%, rgba(30, 41, 59, 0.8) 100%);
      border-color: rgba(255, 255, 255, 0.1);
    }
  }
  
  .card-header {
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
  
  .reset-btn {
    width: 2rem;
    height: 2rem;
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
  
  .reset-btn:hover:not(:disabled) {
    border-color: #f59e0b;
    background: rgba(245, 158, 11, 0.1);
    transform: rotate(180deg);
  }
  
  .reset-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  @media (prefers-color-scheme: dark) {
    .reset-btn {
      background: rgba(45, 55, 72, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
    }
  }
</style>