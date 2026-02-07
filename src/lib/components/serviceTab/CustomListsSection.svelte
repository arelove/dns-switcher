<script lang="ts">
  import type { DnsPresetWithId } from '../types';
  
  interface CustomDnsList {
    id: string;
    name: string;
    description: string;
    icon: string;
    presets: string[];
    createdAt: number;
  }
  
  interface Props {
    customLists: CustomDnsList[];
    dnsPresets: DnsPresetWithId[];
    customPresets: DnsPresetWithId[];
    isLoading: boolean;
    onCreateList: () => void;
    onEditList: (list: CustomDnsList) => void;
    onDeleteList: (listId: string) => void;
    onApplyPreset: (preset: DnsPresetWithId) => Promise<void>;
  }
  
  let {
    customLists,
    dnsPresets,
    customPresets,
    isLoading,
    onCreateList,
    onEditList,
    onDeleteList,
    onApplyPreset
  }: Props = $props();
  
  let selectedListId = $state<string | null>(null);
  
  function getListPresets(list: CustomDnsList): DnsPresetWithId[] {
    const allPresets = [...dnsPresets, ...customPresets];
    return list.presets
      .map(id => allPresets.find(p => p.id === id))
      .filter((p): p is DnsPresetWithId => p !== undefined);
  }
</script>

<div class="custom-lists-section">
  <div class="lists-header">
    <button class="create-list-btn" onclick={onCreateList}>
      ➕ Create New List
    </button>
  </div>

  {#if customLists.length === 0}
    <div class="empty-state">
      <div class="empty-icon">📚</div>
      <h3>No custom lists yet</h3>
      <p>Create a custom list to organize your favorite DNS presets</p>
      <button class="create-list-btn primary" onclick={onCreateList}>
        Create Your First List
      </button>
    </div>
  {:else}
    <div class="lists-grid">
      {#each customLists as list}
        <div class="list-card" class:selected={selectedListId === list.id}>
          <div class="list-header">
            <div class="list-title-section">
              <span class="list-icon">{list.icon}</span>
              <div class="list-info">
                <h3 class="list-name">{list.name}</h3>
                <p class="list-desc">{list.description || 'No description'}</p>
              </div>
            </div>
            <div class="list-actions">
              <button class="icon-btn edit" onclick={() => onEditList(list)} title="Edit list">✏️</button>
              <button class="icon-btn delete" onclick={() => onDeleteList(list.id)} title="Delete list">🗑️</button>
            </div>
          </div>

          <div class="list-stats">
            <span class="stat">📊 {list.presets.length} DNS presets</span>
            <span class="stat">📅 {new Date(list.createdAt).toLocaleDateString()}</span>
          </div>

          <div class="list-presets-preview">
            {#each getListPresets(list).slice(0, 3) as preset}
              <div class="preset-chip">
                <span>{preset.icon}</span>
                <span>{preset.name}</span>
              </div>
            {/each}
            {#if list.presets.length > 3}
              <div class="preset-chip more">+{list.presets.length - 3} more</div>
            {/if}
          </div>

          <button class="view-list-btn" onclick={() => selectedListId = selectedListId === list.id ? null : list.id}>
            {selectedListId === list.id ? '▲ Hide Presets' : '▼ View Presets'}
          </button>

          {#if selectedListId === list.id}
            <div class="list-presets-full">
              {#each getListPresets(list) as preset}
                <div class="preset-item">
                  <div class="preset-item-header">
                    <span class="preset-item-icon">{preset.icon}</span>
                    <div class="preset-item-info">
                      <h4>{preset.name}</h4>
                      <p>{preset.description}</p>
                    </div>
                  </div>
                  <button class="apply-preset-btn" onclick={() => onApplyPreset(preset)} disabled={isLoading}>Apply</button>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .custom-lists-section {
    display: grid;
    gap: 1.5rem;
  }
  
  .lists-header {
    display: flex;
    justify-content: flex-end;
  }
  
  .create-list-btn {
    padding: 0.625rem 1.25rem;
    border: none;
    border-radius: 10px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    font-weight: 600;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.3s ease;
  }
  .create-list-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
  }
  .create-list-btn.primary {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
  }
  @media (prefers-color-scheme: dark) {
    .create-list-btn {
      background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
    }
  }
  
  .empty-state {
    text-align: center;
    padding: 4rem 2rem;
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border: 2px dashed #e2e8f0;
    border-radius: 16px;
  }
  .empty-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
    opacity: 0.5;
  }
  .empty-state h3 {
    font-size: 1.25rem;
    font-weight: 700;
    color: #2d3748;
    margin: 0 0 0.5rem 0;
  }
  .empty-state p {
    font-size: 0.9375rem;
    color: #718096;
    margin: 0 0 1.5rem 0;
  }
  @media (prefers-color-scheme: dark) {
    .empty-state {
      background: linear-gradient(135deg, rgba(45, 55, 72, 0.8) 0%, rgba(30, 41, 59, 0.8) 100%);
      border-color: rgba(255, 255, 255, 0.1);
    }
    .empty-state h3 {
      color: #f7fafc;
    }
    .empty-state p {
      color: #cbd5e0;
    }
  }
  
  .lists-grid {
    display: grid;
    gap: 1.25rem;
  }
  
  .list-card {
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border: 2px solid #e2e8f0;
    border-radius: 16px;
    padding: 1.5rem;
    transition: all 0.3s ease;
  }
  .list-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  }
  .list-card.selected {
    border-color: #667eea;
  }
  @media (prefers-color-scheme: dark) {
    .list-card {
      background: linear-gradient(135deg, rgba(45, 55, 72, 0.8) 0%, rgba(30, 41, 59, 0.8) 100%);
      border-color: rgba(255, 255, 255, 0.1);
    }
    .list-card.selected {
      border-color: #06b6d4;
    }
  }
  
  .list-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1rem;
  }
  
  .list-title-section {
    display: flex;
    gap: 0.75rem;
    flex: 1;
  }
  
  .list-icon {
    font-size: 2rem;
  }
  
  .list-info {
    flex: 1;
  }
  
  .list-name {
    font-size: 1.125rem;
    font-weight: 700;
    color: #2d3748;
    margin: 0 0 0.25rem 0;
  }
  .list-desc {
    font-size: 0.875rem;
    color: #718096;
    margin: 0;
  }
  @media (prefers-color-scheme: dark) {
    .list-name {
      color: #f7fafc;
    }
    .list-desc {
      color: #cbd5e0;
    }
  }
  
  .list-actions {
    display: flex;
    gap: 0.5rem;
  }
  
  .icon-btn {
    width: 2rem;
    height: 2rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    background: white;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.875rem;
  }
  .icon-btn.edit:hover {
    border-color: #667eea;
    background: rgba(102, 126, 234, 0.1);
  }
  .icon-btn.delete:hover {
    border-color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
  }
  @media (prefers-color-scheme: dark) {
    .icon-btn {
      background: rgba(30, 41, 59, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
    }
    .icon-btn.edit:hover {
      border-color: #06b6d4;
      background: rgba(6, 182, 212, 0.2);
    }
  }
  
  .list-stats {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
    font-size: 0.8125rem;
    color: #718096;
  }
  @media (prefers-color-scheme: dark) {
    .list-stats {
      color: #cbd5e0;
    }
  }
  
  .list-presets-preview {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }
  
  .preset-chip {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.75rem;
    background: rgba(102, 126, 234, 0.1);
    border-radius: 8px;
    font-size: 0.8125rem;
    font-weight: 500;
    color: #4a5568;
  }
  .preset-chip.more {
    background: rgba(0, 0, 0, 0.05);
  }
  @media (prefers-color-scheme: dark) {
    .preset-chip {
      background: rgba(6, 182, 212, 0.2);
      color: #cbd5e0;
    }
    .preset-chip.more {
      background: rgba(255, 255, 255, 0.1);
    }
  }
  
  .view-list-btn {
    width: 100%;
    padding: 0.625rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    background: white;
    color: #4a5568;
    font-weight: 600;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s;
  }
  .view-list-btn:hover {
    border-color: #667eea;
    background: rgba(102, 126, 234, 0.05);
  }
  @media (prefers-color-scheme: dark) {
    .view-list-btn {
      background: rgba(30, 41, 59, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
      color: #cbd5e0;
    }
    .view-list-btn:hover {
      border-color: #06b6d4;
      background: rgba(6, 182, 212, 0.1);
    }
  }
  
  .list-presets-full {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 2px solid #e2e8f0;
    display: grid;
    gap: 0.75rem;
  }
  @media (prefers-color-scheme: dark) {
    .list-presets-full {
      border-top-color: rgba(255, 255, 255, 0.1);
    }
  }
  
  .preset-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem;
    background: rgba(255, 255, 255, 0.5);
    border-radius: 8px;
  }
  @media (prefers-color-scheme: dark) {
    .preset-item {
      background: rgba(0, 0, 0, 0.2);
    }
  }
  
  .preset-item-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex: 1;
  }
  
  .preset-item-icon {
    font-size: 1.5rem;
  }
  
  .preset-item-info h4 {
    font-size: 0.9375rem;
    font-weight: 600;
    color: #2d3748;
    margin: 0;
  }
  .preset-item-info p {
    font-size: 0.8125rem;
    color: #718096;
    margin: 0.125rem 0 0 0;
  }
  @media (prefers-color-scheme: dark) {
    .preset-item-info h4 {
      color: #f7fafc;
    }
    .preset-item-info p {
      color: #cbd5e0;
    }
  }
  
  .apply-preset-btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 8px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    font-weight: 600;
    font-size: 0.8125rem;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }
  .apply-preset-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
  }
  .apply-preset-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  @media (prefers-color-scheme: dark) {
    .apply-preset-btn {
      background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
    }
  }
</style>