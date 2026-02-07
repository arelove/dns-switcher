<script lang="ts">
  import { onMount } from 'svelte';
  import { emit } from '@tauri-apps/api/event';
  import { 
    getCurrentDns, 
    setDns, 
    getNetworkAdapters,
    resetDns as apiResetDns,
    getWindowsVersion,
    getDnsPresets,
    getCustomPresets,
    addCustomPreset,
    deleteCustomPreset,
    setSelectedAdapter,
    updateTrayMenu,
    updateTrayTooltip,
  } from '$lib/api';
  import type { WindowsVersion, NetworkAdapter, DnsConfiguration, DnsPresetWithUI, DnsPreset } from '$lib/types';
  import { notificationStore, selectedAdapter} from '$lib/stores';
  import Notification from '$lib/components/Notification.svelte';
  import TitleBar from '$lib/components/TitleBar.svelte';
  import { locale } from '$lib/stores';
  import type { DnsPresetWithId } from '$lib/components/types';
  
  // Import tab components
  import MainTab from '$lib/components/MainTab/MainTab.svelte';
  import ServicesTab from '$lib/components/serviceTab/ServicesTab.svelte';
  import CustomTab from '$lib/components/CustomTab.svelte';
  import SettingsTab from '$lib/components/SettingsTab.svelte';

  // State
  let adapters: NetworkAdapter[] = $state([]);
  let currentDns: DnsConfiguration | null = $state(null);
  let isLoading = $state(false);
  let selectedPreset: string | null = $state(null);
  let customPrimary = $state('');
  let customSecondary = $state('');
  let customDohTemplate = $state('');
  let activeTab: 'main' | 'services' | 'custom' | 'settings' = $state('main');

  let windowsVersion: WindowsVersion | null = $state(null);
  let versionLoading = $state(false);
  
  // Quick Presets State
  let quickPresets: DnsPresetWithId[] = $state([]);
  
  // Custom Services State
  let customPresets: DnsPresetWithId[] = $state([]);
  let newCustomName = $state('');
  let newCustomPrimary = $state('');
  let newCustomSecondary = $state('');
  let newCustomDohTemplate = $state('');
  
  // Settings State
  let currentLocale = $state('en');
  let currentTheme: 'light' | 'dark' | 'auto' = $state('auto');

  let dnsPresets: DnsPresetWithUI[] = $state([]);

  // All available services
  const allServices = $derived([...dnsPresets, ...customPresets]);

  onMount(async () => {
    loadLocale();
    await loadPresetsFromBackend(); 
    await loadCustomPresetsFromBackend();
    loadAdapters();
    loadQuickPresets();
    loadThemePreference();
    await loadWindowsVersion();
  });

  async function loadPresetsFromBackend() {
    try {
      dnsPresets = await getDnsPresets();
      // console.log('Loaded presets:', dnsPresets);
    } catch (error) {
      console.error('Failed to load DNS presets:', error);
      notificationStore.show('Failed to load DNS presets', 'error');
    }
  }

  async function loadCustomPresetsFromBackend() {
    try {
      customPresets = await getCustomPresets();
      // console.log('Loaded custom presets:', customPresets);
    } catch (error) {
      console.error('Failed to load custom presets:', error);
      notificationStore.show('Failed to load custom presets', 'error');
    }
  }

  async function loadWindowsVersion() {
    versionLoading = true;
    try {
      windowsVersion = await getWindowsVersion();
      // console.log('Windows version detected:', windowsVersion);
      
      if (windowsVersion && !windowsVersion.supports_doh) {
        notificationStore.show(
          '⚠️ DNS over HTTPS (DoH) unavailable on this Windows version. Requires Windows 11 (build 22000+).',
          'warning'
        );
      }
    } catch (error) {
      console.error('Failed to get Windows version:', error);
      notificationStore.show('Error checking Windows version', 'error');
    } finally {
      versionLoading = false;
    }
  }

  function loadLocale() {
    locale.subscribe(value => {
      currentLocale = value;
    });
  }

  async function loadAdapters() {
    try {
      isLoading = true;
      const result = await getNetworkAdapters();
      console.log('[DEBUG] Adapters loaded:', result);
      adapters = result;
      
      console.log('[DEBUG] Current selectedAdapter:', $selectedAdapter);
      
      if (adapters.length > 0 && !$selectedAdapter) {
        const activeAdapter = adapters.find((a) => a.is_connected) ?? adapters[0];
        
        if (activeAdapter) {
          // ✅ Обновляем и локальный store и backend state
          selectedAdapter.set(activeAdapter.name);
          await setSelectedAdapter(activeAdapter.name);
        }
      }
      
      if ($selectedAdapter) {
        await loadCurrentDns();
      }
    } catch (error) {
      console.error('[ERROR] loadAdapters failed:', error);
      notificationStore.show('Failed to load network adapters', 'error');
    } finally {
      isLoading = false;
    }
  }

  async function loadCurrentDns() {
    if (!$selectedAdapter) return;
    
    try {
      isLoading = true;
      const dns = await getCurrentDns($selectedAdapter);
      currentDns = dns;
      
      // ✅ Обновляем tray tooltip И menu при изменении DNS
      const tooltip = dns.primary ? `DNS: ${dns.primary}` : 'DNS: Auto (DHCP)';
      await updateTrayTooltip(tooltip);
      
      // Обновляем tray menu с текущими quick presets
      const presetNames = quickPresets.map(p => p.name);
      await updateTrayMenu(dns.primary || null, presetNames);
      
    } catch (error) {
      notificationStore.show('Failed to load current DNS', 'error');
      console.error(error);
    } finally {
      isLoading = false;
    }
  }

  async function applyPreset(preset: DnsPreset) {
    if (!$selectedAdapter) {  // ← $selectedAdapter
      notificationStore.show('No adapter selected', 'error');
      return;
    }
    
    try {
      isLoading = true;
      selectedPreset = preset.id;
      
      const ipv4Servers = preset.servers_ipv4 || [];
      const ipv6Servers = preset.servers_ipv6 || [];
      
      await setDns(
        $selectedAdapter,  // ← $selectedAdapter
        ipv4Servers,
        ipv6Servers,
        preset.doh_template || null,
        preset.dot_hostname || null
      );
      
      await loadCurrentDns();
      notificationStore.show(`Applied ${preset.name} DNS`, 'success');
    } catch (error) {
      console.error('Error applying preset:', error);
      notificationStore.show(`Failed to apply DNS: ${error}`, 'error');
    } finally {
      isLoading = false;
      selectedPreset = null;
    }
  }

  async function applyCustomDns() {
    if (!$selectedAdapter || !customPrimary) return;
    
    try {
      isLoading = true;
      const servers = customSecondary 
        ? [customPrimary, customSecondary]
        : [customPrimary];
      
      await setDns(
        $selectedAdapter,
        servers,
        [],
        customDohTemplate || null,
        null
      );
      
      await loadCurrentDns();
      notificationStore.show('Custom DNS applied', 'success');
      customPrimary = '';
      customSecondary = '';
      customDohTemplate = '';
    } catch (error) {
      notificationStore.show('Failed to apply custom DNS', 'error');
      console.error(error);
    } finally {
      isLoading = false;
    }
  }

  async function resetDns() {
    if (!$selectedAdapter) return;
    
    try {
      isLoading = true;
      await apiResetDns($selectedAdapter);
      await loadCurrentDns();
      notificationStore.show('DNS reset to automatic', 'success');
    } catch (error) {
      notificationStore.show('Failed to reset DNS', 'error');
      console.error(error);
    } finally {
      isLoading = false;
    }
  }

  function loadQuickPresets() {
    try {
      const saved = localStorage.getItem('quickPresets');
      if (saved) {
        const parsed = JSON.parse(saved);
        
        // Миграция старого формата в новый
        quickPresets = parsed.map((preset: any) => {
          if (preset.servers_ipv4) {
            return preset;
          }
          
          return {
            ...preset,
            servers_ipv4: [preset.primary, preset.secondary].filter(Boolean),
            servers_ipv6: [],
            primary: undefined,
            secondary: undefined
          };
        });
        
        saveQuickPresets(quickPresets);
        
        // ✅ Сразу обновляем tray menu после загрузки
        const presetNames = quickPresets.map(p => p.name);
        updateTrayMenu(
          currentDns?.primary || null,
          presetNames
        ).catch(err => console.error('Failed to update tray menu:', err));
      }
    } catch (error) {
      console.error('Failed to load quick presets:', error);
    }
  }


  function saveQuickPresets(presets: DnsPresetWithId[]) {
    try {
      localStorage.setItem('quickPresets', JSON.stringify(presets));
      
      // ✅ Обновляем tray menu
      const presetNames = presets.map(p => p.name);
      updateTrayMenu(
        currentDns?.primary || null,
        presetNames
      ).catch(err => console.error('Failed to update tray menu:', err));
      
      // ✅ Уведомляем mini window
      emit('quick-presets-changed').catch(err => 
        console.error('Failed to emit quick-presets-changed:', err)
      );
      
    } catch (error) {
      console.error('Failed to save quick presets:', error);
    }
  }

  function addToQuickPresets(preset: DnsPresetWithId) {
    if (quickPresets.some(p => p.id === preset.id)) {
      quickPresets = quickPresets.filter(p => p.id !== preset.id);
      notificationStore.show('Removed from quick presets', 'info');
    } else {
      if (quickPresets.length >= 4) {
        notificationStore.show('Maximum 4 quick presets allowed', 'error');
        return;
      }
      quickPresets = [...quickPresets, preset];
      notificationStore.show('Added to quick presets', 'success');
    }
    saveQuickPresets(quickPresets);
  }

  function removeFromQuickPresets(presetId: string) {
    quickPresets = quickPresets.filter(p => p.id !== presetId);
    saveQuickPresets(quickPresets);
    notificationStore.show('Removed from quick presets', 'info');
  }

  async function addCustomService() {
    if (!newCustomName || !newCustomPrimary) {
      notificationStore.show('Name and primary DNS are required', 'error');
      return;
    }

    const newService: Omit<DnsPreset, 'color'> = {
      id: `custom-${Date.now()}`,
      name: newCustomName,
      description: 'Custom DNS service',
      category: 'custom',
      servers_ipv4: [newCustomPrimary, newCustomSecondary].filter(s => s),
      servers_ipv6: [],
      doh_template: newCustomDohTemplate || null,
      dot_hostname: null,
      supports_doh: !!newCustomDohTemplate,
      supports_dot: false,
      icon: '⚙️'
    };

    try {
      await addCustomPreset(newService);
      await loadCustomPresetsFromBackend(); // Перезагружаем
      
      // Очищаем поля
      newCustomName = '';
      newCustomPrimary = '';
      newCustomSecondary = '';
      newCustomDohTemplate = '';
      
      notificationStore.show('Custom service added', 'success');
      activeTab = 'services';
    } catch (error) {
      console.error('Failed to add custom service:', error);
      notificationStore.show('Failed to add custom service', 'error');
    }
  }

  async function deleteCustomService(presetId: string) {
    try {
      await deleteCustomPreset(presetId);
      await loadCustomPresetsFromBackend(); // Перезагружаем
      
      // Удаляем из quick presets если есть
      quickPresets = quickPresets.filter(p => p.id !== presetId);
      saveQuickPresets(quickPresets);
      
      notificationStore.show('Custom service deleted', 'info');
    } catch (error) {
      console.error('Failed to delete custom service:', error);
      notificationStore.show('Failed to delete custom service', 'error');
    }
  }

  function changeLanguage(newLocale: string) {
    locale.set(newLocale);
    currentLocale = newLocale;
    notificationStore.show('Language changed', 'success');
  }

  function loadThemePreference() {
    const saved = localStorage.getItem('theme');
    if (saved === 'light' || saved === 'dark' || saved === 'auto') {
      currentTheme = saved;
    }
  }

  function changeTheme(theme: 'light' | 'dark' | 'auto') {
    currentTheme = theme;
    localStorage.setItem('theme', theme);
    
    if (theme === 'auto') {
      document.documentElement.removeAttribute('data-theme');
    } else {
      document.documentElement.setAttribute('data-theme', theme);
    }
    
    notificationStore.show(`Theme changed to ${theme}`, 'success');
  }

  function fillExample(primary: string, secondary: string, dohTemplate?: string) {
    customPrimary = primary;
    customSecondary = secondary;
    if (windowsVersion?.supports_doh && dohTemplate) {
      customDohTemplate = dohTemplate;
    }
  }

  function getCurrentDnsName(): string {
    if (!currentDns) return 'Loading...';
    if (currentDns.is_dhcp) return 'Automatic (DHCP)';
    
    const matchedPreset = allServices.find(
      p => p.servers_ipv4?.[0] === currentDns?.primary // ✅ Безопасный доступ
    );
    
    return matchedPreset ? matchedPreset.name : 'Custom DNS';
  }

  function getWindowsVersionText(): string {
    if (!windowsVersion) return '';
    
    const { major, minor, build } = windowsVersion;
    
    if (major === 10 && minor === 0) {
      if (build >= 22000) {
        return `Win 11 (Build ${build})`;
      } else {
        return `Win 10 (Build ${build})`;
      }
    }
    
    return `Win ${major}.${minor} (Build ${build})`;
  }
</script>

<TitleBar/>
<div class="app-container">
  
  {#if isLoading || versionLoading}
    <div class="loading-overlay">
      <div class="loading-spinner">
        <div class="spinner"></div>
        <span class="loading-text">
          {#if versionLoading}Checking Windows version...{:else}Loading...{/if}
        </span>
      </div>
    </div>
  {/if}

  <!-- Header -->
  <div class="header">
    <div class="header-content">
      <div class="title-section">
        <h1 class="app-title">🌐 DNS Switcher</h1>
        <p class="app-subtitle">Fast & Modern DNS Manager</p>
      </div>

      <!-- Windows Version Badge -->
      {#if windowsVersion}
        <div class="windows-badge">
          <span class="version-text">{getWindowsVersionText()}</span>
          {#if windowsVersion.supports_doh}
            <span class="doh-badge doh-yes">DoH ✓</span>
          {:else}
            <span class="doh-badge doh-no">DoH ✗</span>
          {/if}
        </div>
      {/if}
      
      <div class="adapter-selector">
        <label for="adapter">Network Adapter:</label>
        <select 
          id="adapter"
          bind:value={$selectedAdapter}
          onchange={loadCurrentDns}
          disabled={isLoading}
        >
          {#each adapters as adapter}
            <option value={adapter.name}>
              {adapter.name} {adapter.is_connected ? '●' : '○'}
            </option>
          {/each}
        </select>
      </div>
    </div>
  </div>

  <!-- Tabs Navigation -->
  <div class="tabs-container">
    <button 
      class="tab-btn" 
      class:active={activeTab === 'main'}
      onclick={() => activeTab = 'main'}
    >
      🏠 Main
    </button>
    <button 
      class="tab-btn" 
      class:active={activeTab === 'services'}
      onclick={() => activeTab = 'services'}
    >
      📋 Services
    </button>
    <button 
      class="tab-btn" 
      class:active={activeTab === 'custom'}
      onclick={() => activeTab = 'custom'}
    >
      ➕ Custom
    </button>
    <button 
      class="tab-btn" 
      class:active={activeTab === 'settings'}
      onclick={() => activeTab = 'settings'}
    >
      ⚙️ Settings
    </button>
  </div>

  <!-- Tab Content -->
  <div class="tab-content">
    
    <!-- Main Tab -->
    {#if activeTab === 'main'}
      <MainTab
        {currentDns}
        {quickPresets}
        {isLoading}
        {selectedPreset}
        {allServices}
        onResetDns={resetDns}
        onApplyPreset={applyPreset}
        onRemoveFromQuick={removeFromQuickPresets}
      />
    {/if}

    <!-- Services Tab -->
    {#if activeTab === 'services'}
      <ServicesTab
        dnsPresets={dnsPresets as DnsPresetWithId[]}
        {customPresets}
        {quickPresets}
        {isLoading}
        onApplyPreset={applyPreset}
        onToggleQuickPreset={addToQuickPresets}
        onDeleteCustomService={deleteCustomService}
      />
    {/if}

    <!-- Custom Tab -->
    {#if activeTab === 'custom'}
      <CustomTab
        {newCustomName}
        {newCustomPrimary}
        {newCustomSecondary}
        {newCustomDohTemplate}
        {customPrimary}
        {customSecondary}
        {customDohTemplate}
        {windowsVersion}
        {isLoading}
        onAddCustomService={addCustomService}
        onApplyCustomDns={applyCustomDns}
        onFillExample={fillExample}
        onUpdateNewCustomName={(v) => newCustomName = v}
        onUpdateNewCustomPrimary={(v) => newCustomPrimary = v}
        onUpdateNewCustomSecondary={(v) => newCustomSecondary = v}
        onUpdateNewCustomDohTemplate={(v) => newCustomDohTemplate = v}
        onUpdateCustomPrimary={(v) => customPrimary = v}
        onUpdateCustomSecondary={(v) => customSecondary = v}
        onUpdateCustomDohTemplate={(v) => customDohTemplate = v}
      />
    {/if}

    <!-- Settings Tab -->
    {#if activeTab === 'settings'}
      <SettingsTab
        {windowsVersion}
        {versionLoading}
        {currentTheme}
        {currentLocale}
        selectedAdapter={$selectedAdapter}
        {adapters}
        {currentDns}
        {isLoading}
        onChangeTheme={changeTheme}
        onChangeLanguage={changeLanguage}
        onRefreshAdapters={loadAdapters}
        onResetDns={resetDns}
        {getCurrentDnsName}
      />
    {/if}
    
  </div>
  
  <Notification />
</div>

<style>
  /* Убираем scrollbar для body в основном окне */
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    min-height: 100vh;
    overflow: auto; /* ✅ Разрешаем scroll для основного окна */
  }

  @media (prefers-color-scheme: dark) {
    :global(body) {
      background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
    }
  }

  .app-container {
    margin-top: 2rem;
    padding: 1rem 1rem 0.6rem 1rem;
  }

  /* Header */
  .header {
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    border-radius: 16px;
    padding: 1rem; 
    margin-bottom: 1rem; 
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1); 
  }

  @media (prefers-color-scheme: dark) {
    .header {
      background: rgba(30, 41, 59, 0.95);
    }
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .title-section {
    flex: 1;
    min-width: 200px;
  }

  .app-title {
    font-size: 1.5rem;
    font-weight: 800;
    margin: 0;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  @media (prefers-color-scheme: dark) {
    .app-title {
      background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
      -webkit-background-clip: text;
      background-clip: text;
      -webkit-text-fill-color: transparent;
    }
  }

.app-subtitle {
    margin: 0.125rem 0 0 0; 
    color: #718096;
    font-size: 0.75rem; 
  }

  @media (prefers-color-scheme: dark) {
    .app-subtitle {
      color: #cbd5e0;
    }
  }

  /* Windows Version Badge */
.windows-badge {
    display: flex;
    gap: 0.5rem; /* ✅ 0.75rem -> 0.5rem */
    align-items: center;
    font-size: 0.75rem; /* ✅ 0.875rem -> 0.75rem */
    padding: 0.375rem 0.75rem; /* ✅ 0.5rem 1rem -> 0.375rem 0.75rem */
    background: rgba(255, 255, 255, 0.7);
    border-radius: 8px; /* ✅ 12px -> 8px */
    border: 1px solid rgba(0, 0, 0, 0.1);
  }

  @media (prefers-color-scheme: dark) {
    .windows-badge {
      background: rgba(30, 41, 59, 0.7);
      border-color: rgba(255, 255, 255, 0.1);
    }
  }

  .version-text {
    font-weight: 600;
    color: #2d3748;
  }

  @media (prefers-color-scheme: dark) {
    .version-text {
      color: #f7fafc;
    }
  }

.doh-badge {
    padding: 0.125rem 0.5rem; 
    border-radius: 999px;
    font-weight: 600;
    font-size: 0.625rem;
  }

  .doh-yes { 
    background: #d1fae5; 
    color: #065f46; 
  }
  
  .doh-no { 
    background: #fee2e2; 
    color: #991b1b; 
  }

  @media (prefers-color-scheme: dark) {
    .doh-yes { 
      background: #064e3b; 
      color: #6ee7b7; 
    }
    
    .doh-no { 
      background: #7f1d1d; 
      color: #fca5a5; 
    }
  }

.adapter-selector {
    display: flex;
    align-items: center;
    gap: 0.5rem; 
  }

  .adapter-selector label {
    font-weight: 600;
    color: #2d3748;
    font-size: 0.75rem;
    white-space: nowrap;
  }

  @media (prefers-color-scheme: dark) {
    .adapter-selector label {
      color: #f7fafc;
    }
  }

  .adapter-selector select {
    padding: 0.375rem 0.75rem;
    border: 2px solid #e2e8f0;
    border-radius: 6px;
    background: white;
    color: #2d3748;
    font-size: 0.75rem;
    cursor: pointer;
    transition: all 0.2s;
    min-width: 180px;
  }

  .adapter-selector select:focus {
    outline: none;
    border-color: #667eea;
  }

  @media (prefers-color-scheme: dark) {
    .adapter-selector select {
      background: rgba(45, 55, 72, 0.8);
      border-color: rgba(255, 255, 255, 0.2);
      color: #f7fafc;
    }

    .adapter-selector select:focus {
      border-color: #06b6d4;
    }
  }

  /* Tabs */
  .tabs-container {
    display: flex;
    gap: 0.375rem; /* ✅ 0.5rem -> 0.375rem */
    background: rgba(255, 255, 255, 0.95);
    
    border-radius: 12px; /* ✅ 16px -> 12px */
    padding: 0.375rem; /* ✅ 0.5rem -> 0.375rem */
    margin-bottom: 1rem; /* ✅ 1.5rem -> 1rem */
  }

  @media (prefers-color-scheme: dark) {
    .tabs-container {
      background: rgba(30, 41, 59, 0.95);
    }
  }

  .tab-btn {
      flex: 1;
      padding: 0.5rem 0.75rem;
      border: none;
      border-radius: 8px;
      background: transparent;
      color: #718096;
      font-weight: 600;
      font-size: 0.75rem;
      cursor: pointer;
      transition: all 0.3s ease;
  }

  .tab-btn:hover {
    background: rgba(102, 126, 234, 0.1);
    color: #667eea;
  }

  .tab-btn.active {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
  }

  @media (prefers-color-scheme: dark) {
    .tab-btn {
      color: #cbd5e0;
    }

    .tab-btn:hover {
      background: rgba(6, 182, 212, 0.1);
    }

    .tab-btn.active {
      background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
    }
  }

  /* Tab Content */
  .tab-content {
      background: rgba(255, 255, 255, 0.95);
      backdrop-filter: blur(10px);
      border-radius: 16px;
      padding: 1rem 1.5rem 1rem 1.5rem;
      box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1); /* ✅ Меньше тень */
      min-height: 400px;
  }

  @media (prefers-color-scheme: dark) {
    .tab-content {
      background: rgba(30, 41, 59, 0.95);
    }
  }

  /* Loading Overlay */
  .loading-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .loading-spinner {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 2rem 3rem;
    background: rgba(255, 255, 255, 0.95);
    border-radius: 20px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  }

  @media (prefers-color-scheme: dark) {
    .loading-spinner {
      background: rgba(30, 41, 59, 0.95);
    }
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #e2e8f0;
    border-top-color: #667eea;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @media (prefers-color-scheme: dark) {
    .spinner {
      border-color: rgba(255, 255, 255, 0.1);
      border-top-color: #06b6d4;
    }
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .loading-text {
    font-weight: 600;
    color: #2d3748;
  }

  @media (prefers-color-scheme: dark) {
    .loading-text {
      color: #f7fafc;
    }
  }

  @media (max-width: 768px) {
    .app-container {
      padding: 0.75rem;
    }

    .header-content {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.75rem; 
    }

    .app-title {
      font-size: 1.125rem; 
    }

    .windows-badge {
      width: 100%;
      justify-content: center;
    }

    .adapter-selector {
      width: 100%;
      flex-direction: column;
      align-items: flex-start;
    }

    .adapter-selector select {
      width: 100%;
    }

    .tabs-container {
      overflow-x: auto;
      flex-wrap: nowrap;
    }

    .tab-btn {
      white-space: nowrap;
    }

    .tab-content {
      padding: 1rem; 
    }
  }
</style>