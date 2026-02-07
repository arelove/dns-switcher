<script lang="ts">
  import type { DnsPresetWithId } from '../types';
  import { open } from '@tauri-apps/plugin-shell';
  
  interface Props {
    preset: DnsPresetWithId | null;
    isLoading: boolean;
    onClose: () => void;
    onApply: () => void;
  }
  
  let { preset, isLoading, onClose, onApply }: Props = $props();
  
  async function openWebsite(url: string) {
    try {
      await open(url);
    } catch (error) {
      console.error('Failed to open website:', error);
    }
  }
</script>

{#if preset}
  <div 
    class="modal-overlay" 
    role="button"
    tabindex="0"
    onclick={onClose}
    onkeydown={(e) => e.key === 'Escape' && onClose()}
  >
    <div 
      class="modal detail-modal" 
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <div class="modal-header">
        <div class="modal-title-section">
          <span class="modal-icon">{preset.icon}</span>
          <div>

          <div class="modal-title-section" style="gap: 1rem;">
            <h2>{preset.name}</h2>
            <!-- Additional Info -->
            {#if preset.website}
            <div class="detail-section">
                <button 
                class="website-link" 
                onclick={() => openWebsite(preset.website!)}
                type="button"
                >
                {preset.website} ↗
                </button>
            </div>
            {/if}
          </div>

            <p class="modal-subtitle">{preset.description}</p>
            
          </div>
           
        </div>
        <button class="close-btn" onclick={onClose}>✕</button>
      </div>
      
      <div class="modal-body detail-body">
       
        <!-- Badges -->
        <div class="detail-badges">
          {#if preset.supports_doh}
            <span class="badge doh large">🔒 DNS over HTTPS</span>
          {/if}
          {#if preset.supports_dot}
            <span class="badge dot large">🔐 DNS over TLS</span>
          {/if}
          {#if preset.servers_ipv6 && preset.servers_ipv6.length > 0}
            <span class="badge ipv6 large">🌍 IPv6 Support</span>
          {/if}
        </div>
        
        <!-- DNS Addresses -->
        <div class="detail-section">
          <h3 class="detail-section-title">📍 DNS Addresses</h3>
          
          {#if preset.servers_ipv4 && preset.servers_ipv4.length > 0}
            <div class="detail-subsection">
              <h4 class="detail-subsection-title">IPv4 Servers</h4>
              {#each preset.servers_ipv4 as server, index}
                <div class="detail-address">
                  <span class="address-label">{index === 0 ? 'Primary' : 'Secondary'}:</span>
                  <code class="address-value">{server}</code>
                  <button class="copy-btn" onclick={() => navigator.clipboard.writeText(server)} title="Copy to clipboard">📋</button>
                </div>
              {/each}
            </div>
          {/if}
          
          {#if preset.servers_ipv6 && preset.servers_ipv6.length > 0}
            <div class="detail-subsection">
              <h4 class="detail-subsection-title">IPv6 Servers</h4>
              {#each preset.servers_ipv6 as server, index}
                <div class="detail-address">
                  <span class="address-label">{index === 0 ? 'Primary' : 'Secondary'}:</span>
                  <code class="address-value ipv6">{server}</code>
                  <button class="copy-btn" onclick={() => navigator.clipboard.writeText(server)} title="Copy to clipboard">📋</button>
                </div>
              {/each}
            </div>
          {/if}
          
          {#if preset.supports_doh && preset.doh_template}
            <div class="detail-subsection">
              <h4 class="detail-subsection-title">DNS over HTTPS (DoH)</h4>
              <div class="detail-address">
                <code class="address-value full-width">{preset.doh_template}</code>
                <button class="copy-btn" onclick={() => preset.doh_template && navigator.clipboard.writeText(preset.doh_template)} title="Copy to clipboard">📋</button>
              </div>
            </div>
          {/if}
          
          {#if preset.supports_dot && preset.dot_hostname}
            <div class="detail-subsection">
              <h4 class="detail-subsection-title">DNS over TLS (DoT)</h4>
              <div class="detail-address">
                <code class="address-value full-width">{preset.dot_hostname}</code>
                <button class="copy-btn" onclick={() => preset.dot_hostname && navigator.clipboard.writeText(preset.dot_hostname)} title="Copy to clipboard">📋</button>
              </div>
            </div>
          {/if}
        </div>
        
        <!-- Setup Instructions -->
        <div class="detail-section">
          <h3 class="detail-section-title">🛠️ Setup Instructions</h3>
          
          <div class="instructions-tabs">
            <div class="instruction-tab">
              <h4>💻 Windows 10/11</h4>
              <ol class="instruction-list">
                <li>Open <strong>Settings</strong> → <strong>Network & Internet</strong></li>
                <li>Click on your connection (Wi-Fi or Ethernet)</li>
                <li>Click <strong>Edit</strong> next to DNS server assignment</li>
                <li>Choose <strong>Manual</strong> and enable IPv4</li>
                <li>Enter the DNS addresses above</li>
                <li>Click <strong>Save</strong></li>
              </ol>
            </div>
            
            <div class="instruction-tab">
              <h4>🍎 macOS</h4>
              <ol class="instruction-list">
                <li>Open <strong>System Preferences</strong> → <strong>Network</strong></li>
                <li>Select your connection and click <strong>Advanced</strong></li>
                <li>Go to the <strong>DNS</strong> tab</li>
                <li>Click <strong>+</strong> to add the DNS addresses</li>
                <li>Click <strong>OK</strong> and then <strong>Apply</strong></li>
              </ol>
            </div>
            
            <div class="instruction-tab">
              <h4>🐧 Linux</h4>
              <ol class="instruction-list">
                <li>Edit <code>/etc/resolv.conf</code> or use NetworkManager</li>
                <li>Add: <code>nameserver [DNS_ADDRESS]</code></li>
                <li>For persistent settings, use <code>/etc/systemd/resolved.conf</code></li>
                <li>Restart systemd-resolved: <code>sudo systemctl restart systemd-resolved</code></li>
              </ol>
            </div>
            
            <div class="instruction-tab">
              <h4>📱 Android</h4>
              <ol class="instruction-list">
                <li>Go to <strong>Settings</strong> → <strong>Network & Internet</strong></li>
                <li>Tap on <strong>Wi-Fi</strong> and long-press your network</li>
                <li>Tap <strong>Modify network</strong> → <strong>Advanced options</strong></li>
                <li>Change IP settings to <strong>Static</strong></li>
                <li>Enter DNS addresses in DNS 1 and DNS 2 fields</li>
                <li>Tap <strong>Save</strong></li>
              </ol>
            </div>
            
            <div class="instruction-tab">
              <h4>📱 iOS/iPadOS</h4>
              <ol class="instruction-list">
                <li>Go to <strong>Settings</strong> → <strong>Wi-Fi</strong></li>
                <li>Tap the <strong>ⓘ</strong> icon next to your network</li>
                <li>Tap <strong>Configure DNS</strong></li>
                <li>Select <strong>Manual</strong></li>
                <li>Tap <strong>Add Server</strong> and enter DNS addresses</li>
                <li>Tap <strong>Save</strong></li>
              </ol>
            </div>
            
            <div class="instruction-tab">
              <h4>🌐 Router</h4>
              <ol class="instruction-list">
                <li>Access your router's admin panel (usually 192.168.1.1 or 192.168.0.1)</li>
                <li>Find DNS settings (often under WAN or Internet settings)</li>
                <li>Enter the primary and secondary DNS addresses</li>
                <li>Save settings and restart the router if needed</li>
                <li><strong>Note:</strong> This will apply DNS to all devices on your network</li>
              </ol>
            </div>
          </div>
        </div>
        
        
      </div>
      
      <div class="modal-footer">
        <button class="btn secondary" onclick={onClose}>Close</button>
        <button class="btn primary" onclick={() => { onApply(); onClose(); }} disabled={isLoading}>
          Apply This DNS
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 1rem;
  }
  
  .modal {
    background: white;
    border-radius: 16px;
    max-width: 600px;
    width: 100%;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  }
  
  .modal.detail-modal {
    max-width: 800px;
  }
  
  @media (prefers-color-scheme: dark) {
    .modal {
      background: #2d3748;
    }
  }
  
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 2px solid #e2e8f0;
  }
  
  .modal-title-section {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .modal-icon {
    font-size: 2.5rem;
  }
  
  .modal-header h2 {
    font-size: 1.25rem;
    font-weight: 700;
    color: #2d3748;
    margin: 0;
  }
  
  .modal-subtitle {
    font-size: 0.875rem;
    color: #718096;
    margin: 0.25rem 0 0 0;
  }
  
  @media (prefers-color-scheme: dark) {
    .modal-header {
      border-bottom-color: rgba(255, 255, 255, 0.1);
    }
    .modal-header h2 {
      color: #f7fafc;
    }
    .modal-subtitle {
      color: #cbd5e0;
    }
  }
  
  .close-btn {
    width: 2rem;
    height: 2rem;
    border: none;
    border-radius: 8px;
    background: #e2e8f0;
    color: #4a5568;
    font-size: 1.25rem;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .close-btn:hover {
    background: #cbd5e0;
  }
  @media (prefers-color-scheme: dark) {
    .close-btn {
      background: rgba(255, 255, 255, 0.1);
      color: #cbd5e0;
    }
    .close-btn:hover {
      background: rgba(255, 255, 255, 0.2);
    }
  }
  
  .modal-body {
    padding: 1.5rem;
    overflow-y: auto;
    flex: 1;
  }
  
  .detail-body {
    display: grid;
    gap: 2rem;
  }
  
  .detail-badges {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
  }
  
  .badge {
    display: inline-flex;
    align-items: center;
    padding: 0.25rem 0.5rem;
    border-radius: 6px;
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.3px;
  }
  
  .badge.large {
    padding: 0.375rem 0.75rem;
    font-size: 0.8125rem;
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
  
  .detail-section {
    display: grid;
    gap: 1rem;
  }
  
  .detail-section-title {
    font-size: 1.125rem;
    font-weight: 700;
    color: #2d3748;
    margin: 0;
  }
  
  @media (prefers-color-scheme: dark) {
    .detail-section-title {
      color: #f7fafc;
    }
  }
  
  .detail-subsection {
    display: grid;
    gap: 0.75rem;
    padding: 1rem;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 8px;
  }
  
  @media (prefers-color-scheme: dark) {
    .detail-subsection {
      background: rgba(255, 255, 255, 0.02);
    }
  }
  
  .detail-subsection-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: #4a5568;
    margin: 0;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  
  @media (prefers-color-scheme: dark) {
    .detail-subsection-title {
      color: #cbd5e0;
    }
  }
  
  .detail-address {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  
  .address-label {
    font-size: 0.875rem;
    font-weight: 600;
    color: #4a5568;
    min-width: 5rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .address-label {
      color: #cbd5e0;
    }
  }
  
  .address-value {
    flex: 1;
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    color: #2d3748;
    background: rgba(102, 126, 234, 0.1);
    padding: 0.5rem 0.75rem;
    border-radius: 6px;
    word-break: break-all;
  }
  
  .address-value.full-width {
    width: 100%;
  }
  
  .address-value.ipv6 {
    font-size: 0.75rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .address-value {
      color: #f7fafc;
      background: rgba(6, 182, 212, 0.2);
    }
  }
  
  .copy-btn {
    padding: 0.5rem;
    border: 2px solid #e2e8f0;
    border-radius: 6px;
    background: white;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 1rem;
  }
  
  .copy-btn:hover {
    border-color: #667eea;
    background: rgba(102, 126, 234, 0.1);
  }
  
  @media (prefers-color-scheme: dark) {
    .copy-btn {
      background: rgba(30, 41, 59, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
    }
    .copy-btn:hover {
      border-color: #06b6d4;
      background: rgba(6, 182, 212, 0.2);
    }
  }
  
  .instructions-tabs {
    display: grid;
    gap: 1.5rem;
  }
  
  .instruction-tab {
    padding: 1.25rem;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 8px;
    border-left: 4px solid #667eea;
  }
  
  @media (prefers-color-scheme: dark) {
    .instruction-tab {
      background: rgba(255, 255, 255, 0.02);
      border-left-color: #06b6d4;
    }
  }
  
  .instruction-tab h4 {
    font-size: 1rem;
    font-weight: 700;
    color: #2d3748;
    margin: 0 0 1rem 0;
  }
  
  @media (prefers-color-scheme: dark) {
    .instruction-tab h4 {
      color: #f7fafc;
    }
  }
  
  .instruction-list {
    margin: 0;
    padding-left: 1.5rem;
    color: #4a5568;
  }
  
  .instruction-list li {
    margin-bottom: 0.5rem;
    line-height: 1.5;
  }
  
  .instruction-list code {
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    background: rgba(102, 126, 234, 0.1);
    padding: 0.125rem 0.375rem;
    border-radius: 3px;
  }
  
  @media (prefers-color-scheme: dark) {
    .instruction-list {
      color: #cbd5e0;
    }
    .instruction-list code {
      background: rgba(6, 182, 212, 0.2);
    }
  }
  
  .website-link {
    display: inline-flex;
    align-items: center;
    padding: 0.25rem 1rem;
    background: rgba(102, 126, 234, 0.1);
    border: 2px solid rgba(102, 126, 234, 0.2);
    border-radius: 8px;
    color: #667eea;
    font-weight: 600;
    font-size: 0.7375rem;
    cursor: pointer;
    transition: all 0.2s;
    width: fit-content;
  }
  
  .website-link:hover {
    background: rgba(102, 126, 234, 0.2);
    border-color: #667eea;
  }
  
  @media (prefers-color-scheme: dark) {
    .website-link {
      background: rgba(6, 182, 212, 0.2);
      border-color: rgba(6, 182, 212, 0.3);
      color: #06b6d4;
    }
    .website-link:hover {
      background: rgba(6, 182, 212, 0.3);
      border-color: #06b6d4;
    }
  }
  
  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1.5rem;
    border-top: 2px solid #e2e8f0;
  }
  @media (prefers-color-scheme: dark) {
    .modal-footer {
      border-top-color: rgba(255, 255, 255, 0.1);
    }
  }
  
  .btn {
    padding: 0.625rem 1.25rem;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s;
  }
  .btn.secondary {
    background: #e2e8f0;
    color: #4a5568;
  }
  .btn.secondary:hover {
    background: #cbd5e0;
  }
  .btn.primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }
  .btn.primary:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
  }
  .btn.primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  @media (prefers-color-scheme: dark) {
    .btn.secondary {
      background: rgba(255, 255, 255, 0.1);
      color: #cbd5e0;
    }
    .btn.secondary:hover {
      background: rgba(255, 255, 255, 0.2);
    }
    .btn.primary {
      background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
    }
  }
</style>