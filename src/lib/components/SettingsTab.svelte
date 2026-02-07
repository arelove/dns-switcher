<script lang="ts">
  import type { WindowsVersion, NetworkAdapter, DnsConfiguration } from '$lib/types';
  interface Props {
    windowsVersion: WindowsVersion | null;
    versionLoading: boolean;
    currentTheme: 'light' | 'dark' | 'auto';
    currentLocale: string;
    selectedAdapter: string | null;
    adapters: NetworkAdapter[];
    currentDns: DnsConfiguration | null;
    isLoading: boolean;
    onChangeTheme: (theme: 'light' | 'dark' | 'auto') => void;
    onChangeLanguage: (locale: string) => void;
    onRefreshAdapters: () => Promise<void>;
    onResetDns: () => Promise<void>;
    getCurrentDnsName: () => string;
  }
  let {
    windowsVersion,
    versionLoading,
    currentTheme,
    currentLocale,
    selectedAdapter,
    adapters,
    currentDns,
    isLoading,
    onChangeTheme,
    onChangeLanguage,
    onRefreshAdapters,
    onResetDns,
    getCurrentDnsName
  }: Props = $props();
  function getWindowsVersionText(): string {
    if (!windowsVersion) return 'Unknown';
    const { major, minor, build } = windowsVersion;
    if (major === 10 && minor === 0) {
      return build >= 22000 ? 'Windows 11' : 'Windows 10';
    }
    return `Windows ${major}.${minor}`;
  }
</script>
<div class="settings-tab">
  <h2 class="page-title">⚙️ Settings</h2>
  
  <!-- System, Theme & Language Grid -->
  <div class="top-grid">
    <div class="settings-section">
      <h3 class="section-title">🪟 System</h3>
      {#if versionLoading}
        <div class="loading-text">Checking...</div>
      {:else if windowsVersion}
        <div class="info-compact">
          <div class="compact-item">
            <span class="label">{getWindowsVersionText()}</span>
            <span class="value mono">{windowsVersion.build}</span>
          </div>
          <div class="compact-item">
            <span class="label">DoH</span>
            <span class="value" class:success={windowsVersion.supports_doh} class:error={!windowsVersion.supports_doh}>
              {windowsVersion.supports_doh ? '✓ Yes' : '✗ No'}
            </span>
          </div>
        </div>
      {/if}
    </div>
    
    <div class="settings-section">
      <h3 class="section-title">🎨 Appearance</h3>
      <div class="button-group">
        <button class="btn" class:active={currentTheme === 'light'} onclick={() => onChangeTheme('light')}>☀️</button>
        <button class="btn" class:active={currentTheme === 'dark'} onclick={() => onChangeTheme('dark')}>🌙</button>
        <button class="btn" class:active={currentTheme === 'auto'} onclick={() => onChangeTheme('auto')}>🔄</button>
      </div>
    </div>
    
    <div class="settings-section">
      <h3 class="section-title">🌍 Language</h3>
      <div class="button-group">
        <button class="btn" class:active={currentLocale === 'en'} onclick={() => onChangeLanguage('en')}>🇬🇧 EN</button>
        <button class="btn" class:active={currentLocale === 'ru'} onclick={() => onChangeLanguage('ru')}>🇷🇺 RU</button>
      </div>
    </div>
  </div>
  
  <!-- Network Info -->
  <div class="settings-section">
    <h3 class="section-title">📡 Network</h3>
    <div class="info-list">
      <div class="info-row">
        <span>Adapter</span>
        <span>{selectedAdapter || 'None'}</span>
      </div>
      <div class="info-row">
        <span>Total</span>
        <span>{adapters.length}</span>
      </div>
      <div class="info-row">
        <span>DNS</span>
        <span>{getCurrentDnsName()}</span>
      </div>
      {#if currentDns?.primary}
        <div class="info-row">
          <span>Primary</span>
          <span class="mono">{currentDns.primary}</span>
        </div>
      {/if}
      {#if currentDns?.doh_enabled}
        <div class="info-row">
          <span>DoH</span>
          <span class="success">🔒 Enabled</span>
        </div>
      {/if}
    </div>
  </div>
  <!-- Actions -->
  <div class="settings-section">
    <h3 class="section-title">🔧 Actions</h3>
    <div class="action-buttons">
      <button class="action-btn refresh" onclick={onRefreshAdapters} disabled={isLoading}>
        🔄 Refresh
      </button>
      <button class="action-btn reset" onclick={onResetDns} disabled={isLoading || currentDns?.is_dhcp}>
        ♻️ Reset DNS
      </button>
    </div>
  </div>
  <!-- About -->
  <div class="settings-section">
    <h3 class="section-title">ℹ️ About</h3>
    <div class="info-list compact">
      <div class="info-row">
        <span>DNS Switcher</span>
        <span>v1.0.0</span>
      </div>
    </div>
  </div>
</div>
<style>
  .settings-tab {
    display: grid;
    gap: 1rem;
  }
  
  .top-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
  }
  
  .page-title {
    font-size: 1.125rem;
    font-weight: 700;
    color: #2d3748;
    margin: 0 0 0.5rem 0;
  }
  @media (prefers-color-scheme: dark) {
    .page-title {
      color: #f7fafc;
    }
  }
  .settings-section {
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    padding: 1rem;
  }
  @media (prefers-color-scheme: dark) {
    .settings-section {
      background: linear-gradient(135deg, rgba(45, 55, 72, 0.8) 0%, rgba(30, 41, 59, 0.8) 100%);
      border-color: rgba(255, 255, 255, 0.1);
    }
  }
  .section-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: #2d3748;
    margin: 0 0 0.75rem 0;
  }
  @media (prefers-color-scheme: dark) {
    .section-title {
      color: #f7fafc;
    }
  }
  
  .info-compact {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .compact-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    background: rgba(255, 255, 255, 0.6);
    border-radius: 6px;
  }
  
  @media (prefers-color-scheme: dark) {
    .compact-item {
      background: rgba(0, 0, 0, 0.2);
    }
  }
  
  .compact-item .label {
    font-size: 0.75rem;
    color: #718096;
    font-weight: 600;
  }
  
  .compact-item .value {
    font-size: 0.8125rem;
    font-weight: 600;
    color: #2d3748;
  }
  
  .compact-item .value.mono {
    font-family: monospace;
  }
  
  .compact-item .value.success {
    color: #10b981;
  }
  
  .compact-item .value.error {
    color: #ef4444;
  }
  
  @media (prefers-color-scheme: dark) {
    .compact-item .label {
      color: #cbd5e0;
    }
    .compact-item .value {
      color: #f7fafc;
    }
  }
  
  .button-group {
    display: flex;
    gap: 0.5rem;
  }
  .btn {
    flex: 1;
    padding: 0.625rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    background: white;
    color: #2d3748;
    font-weight: 600;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s;
  }
  .btn:hover {
    border-color: #667eea;
    background: rgba(102, 126, 234, 0.1);
  }
  .btn.active {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-color: #667eea;
    color: white;
  }
  @media (prefers-color-scheme: dark) {
    .btn {
      background: rgba(45, 55, 72, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
      color: #f7fafc;
    }
    .btn:hover {
      border-color: #06b6d4;
      background: rgba(6, 182, 212, 0.2);
    }
    .btn.active {
      background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
      border-color: #06b6d4;
    }
  }
  .info-list {
    display: grid;
    gap: 0.5rem;
  }
  .info-list.compact {
    gap: 0;
  }
  .info-row {
    display: flex;
    justify-content: space-between;
    padding: 0.625rem;
    background: rgba(255, 255, 255, 0.5);
    border-radius: 6px;
    font-size: 0.8125rem;
  }
  .info-row span:first-child {
    color: #718096;
    font-weight: 600;
  }
  .info-row span:last-child {
    color: #2d3748;
  }
  .info-row .mono {
    font-family: monospace;
    background: rgba(102, 126, 234, 0.1);
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
  }
  .info-row .success {
    color: #10b981;
    font-weight: 600;
  }
  @media (prefers-color-scheme: dark) {
    .info-row {
      background: rgba(0, 0, 0, 0.2);
    }
    .info-row span:first-child {
      color: #cbd5e0;
    }
    .info-row span:last-child {
      color: #f7fafc;
    }
    .info-row .mono {
      background: rgba(6, 182, 212, 0.2);
    }
  }
  .action-buttons {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }
  .action-btn {
    padding: 0.75rem;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    font-size: 0.8125rem;
    cursor: pointer;
    transition: all 0.2s;
    color: white;
  }
  .action-btn.refresh {
    background: linear-gradient(135deg, #06b6d4 0%, #0891b2 100%);
  }
  .action-btn.reset {
    background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  }
  .action-btn:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  }
  .action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .loading-text {
    text-align: center;
    padding: 1rem;
    color: #718096;
    font-size: 0.875rem;
  }
  @media (prefers-color-scheme: dark) {
    .loading-text {
      color: #cbd5e0;
    }
  }
</style>