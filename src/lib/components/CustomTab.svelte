<script lang="ts">
  import type { WindowsVersion } from '$lib/types';
  
  interface Props {
    newCustomName: string;
    newCustomPrimary: string;
    newCustomSecondary: string;
    newCustomDohTemplate: string;
    customPrimary: string;
    customSecondary: string;
    customDohTemplate: string;
    windowsVersion: WindowsVersion | null;
    isLoading: boolean;
    onAddCustomService: () => void;
    onApplyCustomDns: () => Promise<void>;
    onFillExample: (primary: string, secondary: string, dohTemplate?: string) => void;
    onUpdateNewCustomName: (value: string) => void;
    onUpdateNewCustomPrimary: (value: string) => void;
    onUpdateNewCustomSecondary: (value: string) => void;
    onUpdateNewCustomDohTemplate: (value: string) => void;
    onUpdateCustomPrimary: (value: string) => void;
    onUpdateCustomSecondary: (value: string) => void;
    onUpdateCustomDohTemplate: (value: string) => void;
  }
  
  let {
    newCustomName,
    newCustomPrimary,
    newCustomSecondary,
    newCustomDohTemplate,
    customPrimary,
    customSecondary,
    customDohTemplate,
    windowsVersion,
    isLoading,
    onAddCustomService,
    onApplyCustomDns,
    onFillExample,
    onUpdateNewCustomName,
    onUpdateNewCustomPrimary,
    onUpdateNewCustomSecondary,
    onUpdateNewCustomDohTemplate,
    onUpdateCustomPrimary,
    onUpdateCustomSecondary,
    onUpdateCustomDohTemplate
  }: Props = $props();
</script>

<div class="custom-tab">
  <div class="two-column-layout">
    <!-- Add Custom DNS Service -->
    <div class="section-block">
      <h2 class="section-title">➕ Add Custom DNS Service</h2>
      
      <div class="custom-form">
        <div class="form-group">
          <label for="custom-name">Service Name</label>
          <input
            id="custom-name"
            type="text"
            class="form-input"
            value={newCustomName}
            oninput={(e) => onUpdateNewCustomName(e.currentTarget.value)}
            placeholder="e.g., My DNS Server"
          />
        </div>
        
        <div class="form-group">
          <label for="custom-primary">Primary DNS *</label>
          <input
            id="custom-primary"
            type="text"
            class="form-input"
            value={newCustomPrimary}
            oninput={(e) => onUpdateNewCustomPrimary(e.currentTarget.value)}
            placeholder="e.g., 8.8.8.8"
          />
        </div>
        
        <div class="form-group">
          <label for="custom-secondary">Secondary DNS</label>
          <input
            id="custom-secondary"
            type="text"
            class="form-input"
            value={newCustomSecondary}
            oninput={(e) => onUpdateNewCustomSecondary(e.currentTarget.value)}
            placeholder="e.g., 8.8.4.4"
          />
        </div>
        
        {#if windowsVersion?.supports_doh}
          <div class="form-group">
            <label for="custom-doh">DoH Template 🔒</label>
            <input
              id="custom-doh"
              type="text"
              class="form-input"
              value={newCustomDohTemplate}
              oninput={(e) => onUpdateNewCustomDohTemplate(e.currentTarget.value)}
              placeholder="e.g., https://dns.google/dns-query"
            />
          </div>
        {/if}
        
        <button
          class="add-custom-btn"
          onclick={onAddCustomService}
          disabled={!newCustomName || !newCustomPrimary}
        >
          ➕ Add Service
        </button>
      </div>
    </div>

    <!-- Or Quick Apply -->
    <div class="section-block">
      <h2 class="section-title">🚀 Or Quick Apply</h2>
      
      <div class="quick-custom-section">
        <div class="form-group">
          <label for="quick-primary">Primary DNS</label>
          <input
            id="quick-primary"
            type="text"
            class="form-input"
            value={customPrimary}
            oninput={(e) => onUpdateCustomPrimary(e.currentTarget.value)}
            placeholder="e.g., 8.8.8.8"
          />
        </div>
        
        <div class="form-group">
          <label for="quick-secondary">Secondary DNS</label>
          <input
            id="quick-secondary"
            type="text"
            class="form-input"
            value={customSecondary}
            oninput={(e) => onUpdateCustomSecondary(e.currentTarget.value)}
            placeholder="e.g., 8.8.4.4"
          />
        </div>
        
        {#if windowsVersion?.supports_doh}
          <div class="form-group">
            <label for="quick-doh">DoH Template 🔒</label>
            <input
              id="quick-doh"
              type="text"
              class="form-input"
              value={customDohTemplate}
              oninput={(e) => onUpdateCustomDohTemplate(e.currentTarget.value)}
              placeholder="e.g., https://dns.google/dns-query"
            />
          </div>
        {/if}
        
        <button
          class="apply-custom-btn"
          onclick={onApplyCustomDns}
          disabled={!customPrimary || isLoading}
        >
          🚀 Apply Now
        </button>
      </div>
    </div>
  </div>

  <!-- Examples Section -->
  <div class="examples-section">
    <h3 class="subsection-title">Examples</h3>
    <div class="examples-grid">
      <button 
        class="example-card"
        onclick={() => onFillExample('8.8.8.8', '8.8.4.4', 'https://dns.google/dns-query')}
      >
        <span class="example-name">Google</span>
        <span class="example-value">8.8.8.8{windowsVersion?.supports_doh ? ' 🔒' : ''}</span>
      </button>
      <button 
        class="example-card"
        onclick={() => onFillExample('1.1.1.1', '1.0.0.1', 'https://cloudflare-dns.com/dns-query')}
      >
        <span class="example-name">Cloudflare</span>
        <span class="example-value">1.1.1.1{windowsVersion?.supports_doh ? ' 🔒' : ''}</span>
      </button>
      <button 
        class="example-card"
        onclick={() => onFillExample('9.9.9.9', '149.112.112.112', 'https://dns.quad9.net/dns-query')}
      >
        <span class="example-name">Quad9</span>
        <span class="example-value">9.9.9.9{windowsVersion?.supports_doh ? ' 🔒' : ''}</span>
      </button>
    </div>
  </div>
</div>

<style>
  .custom-tab {
    display: grid;
    gap: 1.25rem;
  }

  .two-column-layout {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.25rem;
  }

  .section-block {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .section-title {
    font-size: 1.125rem;
    font-weight: 700;
    color: #2d3748;
    margin: 0;
  }

  @media (prefers-color-scheme: dark) {
    .section-title {
      color: #f7fafc;
    }
  }

  .custom-form,
  .quick-custom-section {
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  @media (prefers-color-scheme: dark) {
    .custom-form,
    .quick-custom-section {
      background: linear-gradient(135deg, rgba(45, 55, 72, 0.8) 0%, rgba(30, 41, 59, 0.8) 100%);
      border-color: rgba(255, 255, 255, 0.1);
    }
  }

  .subsection-title {
    font-size: 0.9375rem;
    font-weight: 600;
    color: #2d3748;
    margin: 0 0 0.75rem 0;
  }

  @media (prefers-color-scheme: dark) {
    .subsection-title {
      color: #f7fafc;
    }
  }

  .form-group {
    display: flex;
    flex-direction: column;
  }

  .form-group label {
    font-weight: 600;
    color: #4a5568;
    margin-bottom: 0.375rem;
    font-size: 0.8125rem;
  }

  @media (prefers-color-scheme: dark) {
    .form-group label {
      color: #cbd5e0;
    }
  }

  .form-input {
    width: 90%;
    min-width: 280px;
    padding: 0.5rem 0.625rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    background: white;
    color: #2d3748;
    font-size: 0.8125rem;
    font-family: 'Courier New', monospace;
    transition: all 0.2s;
  }

  .form-input:focus {
    outline: none;
    border-color: #667eea;
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }

  @media (prefers-color-scheme: dark) {
    .form-input {
      background: rgba(45, 55, 72, 0.8);
      border-color: rgba(255, 255, 255, 0.2);
      color: #f7fafc;
    }
    .form-input:focus {
      border-color: #06b6d4;
      box-shadow: 0 0 0 3px rgba(6, 182, 212, 0.2);
    }
  }

  .add-custom-btn,
  .apply-custom-btn {
    width: 100%;
    max-width: 280px;
    padding: 0.625rem;
    border: none;
    border-radius: 10px;
    background: linear-gradient(135deg, #9333ea 0%, #7e22ce 100%);
    color: white;
    font-weight: 600;
    font-size: 0.9375rem;
    cursor: pointer;
    transition: all 0.3s ease;
    margin-top: 0.25rem;
  }

  .add-custom-btn:hover:not(:disabled),
  .apply-custom-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 12px rgba(147, 51, 234, 0.3);
  }

  .add-custom-btn:disabled,
  .apply-custom-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .apply-custom-btn {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  }

  @media (prefers-color-scheme: dark) {
    .apply-custom-btn {
      background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
    }
  }

  .examples-section {
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    padding: 1rem;
  }

  @media (prefers-color-scheme: dark) {
    .examples-section {
      background: linear-gradient(135deg, rgba(45, 55, 72, 0.8) 0%, rgba(30, 41, 59, 0.8) 100%);
      border-color: rgba(255, 255, 255, 0.1);
    }
  }

  .examples-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 0.625rem;
  }

  .example-card {
    padding: 0.625rem;
    background: white;
    border: 2px solid #e2e8f0;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.3s ease;
    text-align: left;
    display: flex;
    flex-direction: column;
    gap: 0.1875rem;
  }

  .example-card:hover {
    transform: translateY(-2px);
    border-color: #667eea;
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.2);
  }

  @media (prefers-color-scheme: dark) {
    .example-card {
      background: rgba(45, 55, 72, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
    }
    .example-card:hover {
      border-color: #06b6d4;
      box-shadow: 0 4px 12px rgba(6, 182, 212, 0.2);
    }
  }

  .example-name {
    font-weight: 600;
    color: #2d3748;
    font-size: 0.875rem;
  }

  @media (prefers-color-scheme: dark) {
    .example-name {
      color: #f7fafc;
    }
  }

  .example-value {
    font-size: 0.6875rem;
    font-family: 'Courier New', monospace;
    color: #718096;
  }

  @media (prefers-color-scheme: dark) {
    .example-value {
      color: #cbd5e0;
    }
  }
</style>