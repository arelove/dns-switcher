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
    show: boolean;
    isEdit: boolean;
    editingList: CustomDnsList | null;
    dnsPresets: DnsPresetWithId[];
    customPresets: DnsPresetWithId[];
    onClose: () => void;
    onSave: (list: Omit<CustomDnsList, 'id' | 'createdAt'>) => void;
    getPresetCategory: (preset: DnsPresetWithId) => string;
  }
  
  let {
    show,
    isEdit,
    editingList,
    dnsPresets,
    customPresets,
    onClose,
    onSave,
    getPresetCategory
  }: Props = $props();
  
  let formName = $state('');
  let formDescription = $state('');
  let formIcon = $state('📋');
  let selectedPresetIds = $state<Set<string>>(new Set());
  let listSearchQuery = $state('');
  let listFilterCategory = $state<string>('all');
  
  const availableIcons = ['📋', '⭐', '🎯', '🔥', '💎', '🚀', '⚡', '🎮', '🏆', '🎪', '🎨', '🎭', '🎬', '📱', '💻', '🖥️', '⌨️', '🖱️', '🎧', '🎤'];
  
  // Reset form when modal opens
  $effect(() => {
    if (show) {
      if (isEdit && editingList) {
        formName = editingList.name;
        formDescription = editingList.description;
        formIcon = editingList.icon;
        selectedPresetIds = new Set(editingList.presets);
      } else {
        formName = '';
        formDescription = '';
        formIcon = '📋';
        selectedPresetIds = new Set();
      }
      listSearchQuery = '';
      listFilterCategory = 'all';
    }
  });
  
  function togglePresetSelection(presetId: string) {
    const newSet = new Set(selectedPresetIds);
    if (newSet.has(presetId)) {
      newSet.delete(presetId);
    } else {
      newSet.add(presetId);
    }
    selectedPresetIds = newSet;
  }
  
  function selectAll() {
    const allPresets = [...dnsPresets, ...customPresets];
    selectedPresetIds = new Set(allPresets.map(p => p.id));
  }
  
  function deselectAll() {
    selectedPresetIds = new Set();
  }
  
  const modalFilteredPresets = $derived(() => {
    let filtered = [...dnsPresets, ...customPresets];
    if (listSearchQuery) {
      filtered = filtered.filter(p =>
        p.name.toLowerCase().includes(listSearchQuery.toLowerCase()) ||
        p.description.toLowerCase().includes(listSearchQuery.toLowerCase())
      );
    }
    if (listFilterCategory !== 'all') {
      filtered = filtered.filter(p => getPresetCategory(p) === listFilterCategory);
    }
    return filtered;
  });
  
  const modalCategories = $derived(() => {
    const cats = new Set<string>();
    [...dnsPresets, ...customPresets].forEach(p => cats.add(getPresetCategory(p)));
    return ['all', ...Array.from(cats).sort()];
  });
  
  function handleSave() {
    if (!formName.trim() || selectedPresetIds.size === 0) return;
    
    onSave({
      name: formName.trim(),
      description: formDescription.trim(),
      icon: formIcon,
      presets: Array.from(selectedPresetIds)
    });
  }
</script>

{#if show}
  <div 
    class="modal-overlay" 
    role="button"
    tabindex="0"
    onclick={onClose}
    onkeydown={(e) => e.key === 'Escape' && onClose()}
  >
    <div 
      class="modal" 
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <div class="modal-header">
        <h2>{isEdit ? 'Edit DNS List' : 'Create Custom DNS List'}</h2>
        <button class="close-btn" onclick={onClose}>✕</button>
      </div>
      
      <div class="modal-body">
        <div class="form-group">
          <label for="list-name">List Name *</label>
          <input 
            id="list-name" 
            type="text" 
            class="form-input" 
            placeholder="e.g., Gaming DNS Collection" 
            bind:value={formName} 
          />
        </div>
        
        <div class="form-group">
          <label for="list-desc">Description</label>
          <textarea 
            id="list-desc" 
            class="form-textarea" 
            placeholder="Optional description..." 
            bind:value={formDescription} 
            rows="2"
          ></textarea>
        </div>
        
        <div class="form-group">
          <span class="form-label">Icon</span>
          <div class="icon-selector">
            {#each availableIcons as icon}
              <button 
                class="icon-option" 
                class:selected={formIcon === icon} 
                onclick={() => formIcon = icon}
              >
                {icon}
              </button>
            {/each}
          </div>
        </div>
        
        <div class="form-group">
          <div class="presets-header">
            <span class="form-label">Select DNS Presets * ({selectedPresetIds.size} selected)</span>
            <div class="select-actions">
              <button class="text-btn" onclick={selectAll}>Select All</button>
              <button class="text-btn" onclick={deselectAll}>Deselect All</button>
            </div>
          </div>
          
          <div class="search-filter">
            <input 
              type="text" 
              class="search-input-modal" 
              placeholder="Search presets..." 
              bind:value={listSearchQuery} 
            />
            <select class="category-filter" bind:value={listFilterCategory}>
              {#each modalCategories() as category}
                <option value={category}>
                  {category === 'all' ? 'All Categories' : category.charAt(0).toUpperCase() + category.slice(1)}
                </option>
              {/each}
            </select>
          </div>
          
          <div class="presets-list">
            {#each modalFilteredPresets() as preset}
              <label class="preset-checkbox-item">
                <input 
                  type="checkbox" 
                  checked={selectedPresetIds.has(preset.id)} 
                  onchange={() => togglePresetSelection(preset.id)} 
                />
                <div class="preset-checkbox-content">
                  <span class="preset-checkbox-icon">{preset.icon}</span>
                  <div class="preset-checkbox-info">
                    <div class="preset-checkbox-name">{preset.name}</div>
                    <div class="preset-checkbox-desc">{preset.description}</div>
                  </div>
                </div>
              </label>
            {/each}
          </div>
        </div>
      </div>
      
      <div class="modal-footer">
        <button class="btn secondary" onclick={onClose}>Cancel</button>
        <button 
          class="btn primary" 
          onclick={handleSave} 
          disabled={!formName.trim() || selectedPresetIds.size === 0}
        >
          {isEdit ? 'Save Changes' : 'Create List'}
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
  
  .modal-header h2 {
    font-size: 1.25rem;
    font-weight: 700;
    color: #2d3748;
    margin: 0;
  }
  
  @media (prefers-color-scheme: dark) {
    .modal-header {
      border-bottom-color: rgba(255, 255, 255, 0.1);
    }
    .modal-header h2 {
      color: #f7fafc;
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
  
  .form-group {
    margin-bottom: 1.25rem;
  }
  
  .form-group label {
    display: block;
    font-size: 0.875rem;
    font-weight: 600;
    color: #4a5568;
    margin-bottom: 0.5rem;
  }
  @media (prefers-color-scheme: dark) {
    .form-group label {
      color: #cbd5e0;
    }
  }
  
  .form-input,
  .form-textarea {
    width: 100%;
    padding: 0.75rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    font-size: 0.9375rem;
    background: white;
    transition: all 0.2s;
  }
  .form-input:focus,
  .form-textarea:focus {
    outline: none;
    border-color: #667eea;
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }
  @media (prefers-color-scheme: dark) {
    .form-input,
    .form-textarea {
      background: rgba(30, 41, 59, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
      color: #f7fafc;
    }
    .form-input:focus,
    .form-textarea:focus {
      border-color: #06b6d4;
      box-shadow: 0 0 0 3px rgba(6, 182, 212, 0.1);
    }
  }
  
  .form-label {
    display: block;
    font-size: 0.875rem;
    font-weight: 600;
    color: #4a5568;
    margin-bottom: 0.5rem;
  }
  @media (prefers-color-scheme: dark) {
    .form-label {
      color: #cbd5e0;
    }
  }
  
  .icon-selector {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  
  .icon-option {
    width: 2.5rem;
    height: 2.5rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    background: white;
    font-size: 1.25rem;
    cursor: pointer;
    transition: all 0.2s;
  }
  .icon-option:hover {
    border-color: #667eea;
    background: rgba(102, 126, 234, 0.05);
  }
  .icon-option.selected {
    border-color: #667eea;
    background: rgba(102, 126, 234, 0.15);
  }
  @media (prefers-color-scheme: dark) {
    .icon-option {
      background: rgba(30, 41, 59, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
    }
    .icon-option:hover {
      border-color: #06b6d4;
      background: rgba(6, 182, 212, 0.1);
    }
    .icon-option.selected {
      border-color: #06b6d4;
      background: rgba(6, 182, 212, 0.2);
    }
  }
  
  .presets-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }
  
  .select-actions {
    display: flex;
    gap: 0.75rem;
  }
  
  .text-btn {
    background: none;
    border: none;
    color: #667eea;
    font-size: 0.8125rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }
  .text-btn:hover {
    color: #764ba2;
  }
  @media (prefers-color-scheme: dark) {
    .text-btn {
      color: #06b6d4;
    }
    .text-btn:hover {
      color: #3b82f6;
    }
  }
  
  .search-filter {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
  }
  
  .search-input-modal {
    padding: 0.5rem 0.75rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    font-size: 0.875rem;
    background: white;
  }
  
  .category-filter {
    padding: 0.5rem 0.75rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    font-size: 0.875rem;
    background: white;
    cursor: pointer;
  }
  @media (prefers-color-scheme: dark) {
    .search-input-modal,
    .category-filter {
      background: rgba(30, 41, 59, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
      color: #f7fafc;
    }
  }
  
  .presets-list {
    max-height: 300px;
    overflow-y: auto;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    padding: 0.5rem;
    display: grid;
    gap: 0.5rem;
  }
  @media (prefers-color-scheme: dark) {
    .presets-list {
      border-color: rgba(255, 255, 255, 0.1);
    }
  }
  
  .preset-checkbox-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .preset-checkbox-item:hover {
    background: rgba(102, 126, 234, 0.05);
  }
  @media (prefers-color-scheme: dark) {
    .preset-checkbox-item:hover {
      background: rgba(6, 182, 212, 0.1);
    }
  }
  
  .preset-checkbox-item input[type="checkbox"] {
    width: 1.125rem;
    height: 1.125rem;
    cursor: pointer;
    accent-color: #667eea;
  }
  @media (prefers-color-scheme: dark) {
    .preset-checkbox-item input[type="checkbox"] {
      accent-color: #06b6d4;
    }
  }
  
  .preset-checkbox-content {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    flex: 1;
  }
  
  .preset-checkbox-icon {
    font-size: 1.25rem;
  }
  
  .preset-checkbox-info {
    flex: 1;
  }
  
  .preset-checkbox-name {
    font-size: 0.875rem;
    font-weight: 600;
    color: #2d3748;
  }
  .preset-checkbox-desc {
    font-size: 0.75rem;
    color: #718096;
  }
  @media (prefers-color-scheme: dark) {
    .preset-checkbox-name {
      color: #f7fafc;
    }
    .preset-checkbox-desc {
      color: #cbd5e0;
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