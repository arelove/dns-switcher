<script lang="ts">
  import type { DnsPresetWithId } from '../types';
  import FiltersPanel from './FiltersPanel.svelte';
  import ServiceCard from './ServiceCard.svelte';
  import CustomListsSection from './CustomListsSection.svelte';
  import DetailModal from './DetailModal.svelte';
  import ListFormModal from './ListFormModal.svelte';
  
  interface CustomDnsList {
    id: string;
    name: string;
    description: string;
    icon: string;
    presets: string[];
    createdAt: number;
  }
  
  interface Props {
    dnsPresets: DnsPresetWithId[];
    customPresets: DnsPresetWithId[];
    quickPresets: DnsPresetWithId[];
    isLoading: boolean;
    onApplyPreset: (preset: DnsPresetWithId) => Promise<void>;
    onToggleQuickPreset: (preset: DnsPresetWithId) => void;
    onDeleteCustomService: (presetId: string) => void;
  }
  
  let {
    dnsPresets,
    customPresets,
    quickPresets,
    isLoading,
    onApplyPreset,
    onToggleQuickPreset,
    onDeleteCustomService
  }: Props = $props();
  
  // Tab state
  let activeSubTab = $state<'presets' | 'lists'>('presets');
  
  // Filter states
  let searchQuery = $state('');
  let selectedCategory = $state<'all' | 'public' | 'security' | 'privacy' | 'adblock' | 'family' | 'gaming' | 'custom'>('all');
  let showOnlyDoH = $state(false);
  let showOnlyDoT = $state(false);
  let showOnlyIPv6 = $state(false);
  
  // Pagination state
  let currentPage = $state(1);
  let itemsPerPage = $state(3);
  const itemsPerPageOptions = [3, 6, 12, 24, 48];
  
  // Custom Lists state
  let customLists = $state<CustomDnsList[]>([]);
  let showCreateModal = $state(false);
  let showEditModal = $state(false);
  let editingList: CustomDnsList | null = $state(null);
  
  // Detail Modal state
  let showDetailModal = $state(false);
  let detailPreset: DnsPresetWithId | null = $state(null);
  
  // Load custom lists from localStorage
  $effect(() => {
    const stored = localStorage.getItem('customDnsLists');
    if (stored) {
      try {
        customLists = JSON.parse(stored);
      } catch (e) {
        console.error('Failed to load custom lists:', e);
      }
    }
  });
  
  // Save custom lists to localStorage
  function saveCustomLists() {
    localStorage.setItem('customDnsLists', JSON.stringify(customLists));
  }
  
  // Get category for a preset
  function getPresetCategory(preset: DnsPresetWithId): string {
    return preset.category || 'public';
  }
  
  // Filter presets
  const filteredDnsPresets = $derived(
    dnsPresets.filter(preset => {
      if (searchQuery && !preset.name.toLowerCase().includes(searchQuery.toLowerCase()) &&
          !preset.description.toLowerCase().includes(searchQuery.toLowerCase())) {
        return false;
      }
      if (selectedCategory !== 'all') {
        const presetCategory = getPresetCategory(preset);
        if (presetCategory !== selectedCategory) {
          return false;
        }
      }
      if (showOnlyDoH && !preset.supports_doh) return false;
      if (showOnlyDoT && !preset.supports_dot) return false;
      if (showOnlyIPv6 && (!preset.servers_ipv6 || preset.servers_ipv6.length === 0)) return false;
      return true;
    })
  );
  
  const filteredCustomPresets = $derived(
    customPresets.filter(preset => {
      if (searchQuery && !preset.name.toLowerCase().includes(searchQuery.toLowerCase()) &&
          !preset.description.toLowerCase().includes(searchQuery.toLowerCase())) {
        return false;
      }
      if (selectedCategory !== 'all' && selectedCategory !== 'custom') {
        return false;
      }
      if (showOnlyDoH && !preset.supports_doh) return false;
      if (showOnlyDoT && !preset.supports_dot) return false;
      if (showOnlyIPv6 && (!preset.servers_ipv6 || preset.servers_ipv6.length === 0)) return false;
      return true;
    })
  );
  
  // Combined filtered presets
  const allFilteredPresets = $derived([...filteredDnsPresets, ...filteredCustomPresets]);
  
  // Pagination calculations
  const totalPages = $derived(Math.ceil(allFilteredPresets.length / itemsPerPage));
  const paginatedPresets = $derived(
    allFilteredPresets.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)
  );
  
  // Reset to page 1 when filters change
  $effect(() => {
    searchQuery;
    selectedCategory;
    showOnlyDoH;
    showOnlyDoT;
    showOnlyIPv6;
    itemsPerPage;
    currentPage = 1;
  });
  
  // Get category stats
  const categoryStats = $derived({
    all: dnsPresets.length + customPresets.length,
    public: dnsPresets.filter(p => getPresetCategory(p) === 'public').length,
    security: dnsPresets.filter(p => getPresetCategory(p) === 'security').length,
    privacy: dnsPresets.filter(p => getPresetCategory(p) === 'privacy').length,
    adblock: dnsPresets.filter(p => getPresetCategory(p) === 'adblock').length,
    gaming: dnsPresets.filter(p => getPresetCategory(p) === 'gaming').length,
    family: dnsPresets.filter(p => getPresetCategory(p) === 'family').length,
    custom: customPresets.length,
  });
  
  // Functions
  function goToPage(page: number) {
    if (page >= 1 && page <= totalPages) {
      currentPage = page;
      document.querySelector('.services-categories')?.scrollIntoView({ behavior: 'smooth' });
    }
  }
  
  function changeItemsPerPage(newValue: number) {
    itemsPerPage = newValue;
  }
  
  function resetFilters() {
    searchQuery = '';
    selectedCategory = 'all';
    showOnlyDoH = false;
    showOnlyDoT = false;
    showOnlyIPv6 = false;
  }
  
  function openDetailModal(preset: DnsPresetWithId) {
    detailPreset = preset;
    showDetailModal = true;
  }
  
  function closeDetailModal() {
    showDetailModal = false;
    detailPreset = null;
  }
  
  function openCreateModal() {
    showCreateModal = true;
  }
  
  function openEditModal(list: CustomDnsList) {
    editingList = list;
    showEditModal = true;
  }
  
  function closeListFormModal() {
    showCreateModal = false;
    showEditModal = false;
    editingList = null;
  }
  
  function createList(listData: Omit<CustomDnsList, 'id' | 'createdAt'>) {
    const newList: CustomDnsList = {
      id: `list-${Date.now()}`,
      ...listData,
      createdAt: Date.now(),
    };
    customLists = [...customLists, newList];
    saveCustomLists();
    showCreateModal = false;
  }
  
  function updateList(listData: Omit<CustomDnsList, 'id' | 'createdAt'>) {
    if (!editingList) return;
    
    customLists = customLists.map(list =>
      list.id === editingList!.id
        ? { ...list, ...listData }
        : list
    );
    saveCustomLists();
    showEditModal = false;
    editingList = null;
  }
  
  function deleteList(listId: string) {
    if (confirm('Are you sure you want to delete this list?')) {
      customLists = customLists.filter(list => list.id !== listId);
      saveCustomLists();
    }
  }
</script>

<div class="services-tab">
  <div class="services-header">
    <h2 class="section-title">📋 DNS Services & Lists</h2>
    
    <!-- Sub-tabs -->
    <div class="sub-tabs">
      <button
        class="sub-tab-btn"
        class:active={activeSubTab === 'presets'}
        onclick={() => activeSubTab = 'presets'}
      >
        🌐 All Presets
      </button>
      <button
        class="sub-tab-btn"
        class:active={activeSubTab === 'lists'}
        onclick={() => activeSubTab = 'lists'}
      >
        📚 My Lists ({customLists.length})
      </button>
    </div>
  </div>

  <!-- PRESETS TAB -->
  {#if activeSubTab === 'presets'}
    <div class="results-header">
      <div class="results-count">
        {allFilteredPresets.length} services found
      </div>
      <div class="items-per-page">
        <span class="items-label">Show:</span>
        {#each itemsPerPageOptions as option}
          <button
            class="items-option"
            class:active={itemsPerPage === option}
            onclick={() => changeItemsPerPage(option)}
          >
            {option}
          </button>
        {/each}
      </div>
    </div>

    <FiltersPanel
      {searchQuery}
      {selectedCategory}
      {showOnlyDoH}
      {showOnlyDoT}
      {showOnlyIPv6}
      {categoryStats}
      onSearchChange={(value) => searchQuery = value}
      onCategoryChange={(cat) => selectedCategory = cat}
      onDoHToggle={(value) => showOnlyDoH = value}
      onDoTToggle={(value) => showOnlyDoT = value}
      onIPv6Toggle={(value) => showOnlyIPv6 = value}
      onResetFilters={resetFilters}
    />

    <!-- Services List -->
    <div class="services-categories">
      {#if paginatedPresets.length > 0}
        <div class="services-grid">
          {#each paginatedPresets as preset}
            <ServiceCard
              {preset}
              category={getPresetCategory(preset)}
              isQuickPreset={quickPresets.some(p => p.id === preset.id)}
              {isLoading}
              onApply={() => onApplyPreset(preset)}
              onToggleQuick={() => onToggleQuickPreset(preset)}
              onDelete={preset.id.startsWith('custom-') ? () => onDeleteCustomService(preset.id) : undefined}
              onShowDetail={() => openDetailModal(preset)}
            />
          {/each}
        </div>
        
        <!-- Pagination -->
        {#if totalPages > 1}
          <div class="pagination">
            <button 
              class="pagination-btn" 
              onclick={() => goToPage(currentPage - 1)}
              disabled={currentPage === 1}
            >
              ← Previous
            </button>
            
            <div class="pagination-pages">
              {#if currentPage > 3}
                <button class="pagination-page" onclick={() => goToPage(1)}>1</button>
                {#if currentPage > 4}
                  <span class="pagination-ellipsis">...</span>
                {/if}
              {/if}
              
              {#each Array.from({ length: totalPages }, (_, i) => i + 1).filter(p => Math.abs(p - currentPage) <= 2) as page}
                <button 
                  class="pagination-page" 
                  class:active={page === currentPage}
                  onclick={() => goToPage(page)}
                >
                  {page}
                </button>
              {/each}
              
              {#if currentPage < totalPages - 2}
                {#if currentPage < totalPages - 3}
                  <span class="pagination-ellipsis">...</span>
                {/if}
                <button class="pagination-page" onclick={() => goToPage(totalPages)}>{totalPages}</button>
              {/if}
            </div>
            
            <button 
              class="pagination-btn" 
              onclick={() => goToPage(currentPage + 1)}
              disabled={currentPage === totalPages}
            >
              Next →
            </button>
          </div>
        {/if}
      {:else}
        <div class="no-results">
          <div class="no-results-icon">🔍</div>
          <h3>No services found</h3>
          <p>Try adjusting your filters or search query</p>
          <button class="reset-filters-btn" onclick={resetFilters}>Reset All Filters</button>
        </div>
      {/if}
    </div>

  <!-- CUSTOM LISTS TAB -->
  {:else if activeSubTab === 'lists'}
    <CustomListsSection
      {customLists}
      {dnsPresets}
      {customPresets}
      {isLoading}
      onCreateList={openCreateModal}
      onEditList={openEditModal}
      onDeleteList={deleteList}
      {onApplyPreset}
    />
  {/if}
  
  <!-- Detail Modal -->
  <DetailModal
    preset={detailPreset}
    {isLoading}
    onClose={closeDetailModal}
    onApply={() => detailPreset && onApplyPreset(detailPreset)}
  />
  
  <!-- Create/Edit Modal -->
  <ListFormModal
    show={showCreateModal || showEditModal}
    isEdit={showEditModal}
    {editingList}
    {dnsPresets}
    {customPresets}
    onClose={closeListFormModal}
    onSave={showCreateModal ? createList : updateList}
    {getPresetCategory}
  />
</div>

<style>
  .services-tab {
    display: grid;
    gap: 0.5rem;
  }

.services-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 0.75rem;
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

.sub-tabs {
  display: flex;
  gap: 0.375rem;
  background: rgba(0, 0, 0, 0.05);
  padding: 0.25rem;
  border-radius: 8px;
}

@media (prefers-color-scheme: dark) {
  .sub-tabs {
    background: rgba(255, 255, 255, 0.05);
  }
}

.sub-tab-btn {
  padding: 0.175rem 0.725rem;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: #4a5568;
  font-weight: 600;
  font-size: 0.7125rem;
  cursor: pointer;
  transition: all 0.2s;
}

.sub-tab-btn:hover {
  background: rgba(0, 0, 0, 0.05);
}

.sub-tab-btn.active {
  background: white;
  color: #667eea;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
}

@media (prefers-color-scheme: dark) {
  .sub-tab-btn {
    color: #cbd5e0;
  }
  .sub-tab-btn:hover {
    background: rgba(255, 255, 255, 0.05);
  }
  .sub-tab-btn.active {
    background: rgba(6, 182, 212, 0.2);
    color: #06b6d4;
  }
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.results-count {
  font-size: 0.8125rem;
  color: #718096;
  font-weight: 500;
}

@media (prefers-color-scheme: dark) {
  .results-count {
    color: #cbd5e0;
  }
}

.items-per-page {
  display: flex;
  align-items: center;
  gap: 0.375rem;
}

.items-label {
  font-size: 0.8125rem;
  font-weight: 600;
  color: #4a5568;
}

@media (prefers-color-scheme: dark) {
  .items-label {
    color: #cbd5e0;
  }
}

.items-option {
  padding: 0.25rem 0.5rem;
  border: 2px solid #e2e8f0;
  border-radius: 6px;
  background: white;
  color: #4a5568;
  font-size: 0.75rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.items-option:hover {
  border-color: #667eea;
  background: rgba(102, 126, 234, 0.05);
}

.items-option.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-color: #667eea;
  color: white;
}

@media (prefers-color-scheme: dark) {
  .items-option {
    background: rgba(30, 41, 59, 0.8);
    border-color: rgba(255, 255, 255, 0.1);
    color: #cbd5e0;
  }
  .items-option:hover {
    border-color: #06b6d4;
    background: rgba(6, 182, 212, 0.1);
  }
  .items-option.active {
    background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
    border-color: #06b6d4;
  }
}

.services-categories {
  display: grid;
  gap: 0rem;
}

.services-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(222px, 2fr)); /* ✅ 300px -> 280px */
  gap: 0.875rem;
}
  
  .pagination {
    display: grid;
    grid-template-columns: auto 1fr auto; /* ✅ Фиксированная структура */
    align-items: center;
    margin-top: 0.5rem;
    padding: 0.5rem;
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border-radius: 12px;
  }
  @media (prefers-color-scheme: dark) {
    .pagination {
      background: linear-gradient(135deg, rgba(45, 55, 72, 0.8) 0%, rgba(30, 41, 59, 0.8) 100%);
    }
  }
  
  .pagination-btn {
    padding: 0.5rem 1rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    background: white;
    color: #4a5568;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap; /* ✅ Не переносить текст */
    min-width: 100px; /* ✅ Минимальная ширина для стабильности */
  }
  .pagination-btn:hover:not(:disabled) {
    border-color: #667eea;
    background: rgba(102, 126, 234, 0.05);
    transform: translateY(-1px);
  }
  .pagination-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  @media (prefers-color-scheme: dark) {
    .pagination-btn {
      background: rgba(30, 41, 59, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
      color: #cbd5e0;
    }
    .pagination-btn:hover:not(:disabled) {
      border-color: #06b6d4;
      background: rgba(6, 182, 212, 0.1);
    }
  }
    
  .pagination-pages {
    display: flex;
    gap: 0.25rem;
    align-items: center;
    justify-content: center; /* ✅ Центрируем номера страниц */
  }
  
  .pagination-page {
    padding: 0.15rem;
    min-width: 1.5rem;
    height: 1.5rem;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    background: white;
    color: #4a5568;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }
  .pagination-page:hover {
    border-color: #667eea;
    background: rgba(102, 126, 234, 0.05);
  }
  .pagination-page.active {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-color: #667eea;
    color: white;
  }
  @media (prefers-color-scheme: dark) {
    .pagination-page {
      background: rgba(30, 41, 59, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
      color: #cbd5e0;
    }
    .pagination-page:hover {
      border-color: #06b6d4;
      background: rgba(6, 182, 212, 0.1);
    }
    .pagination-page.active {
      background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
      border-color: #06b6d4;
    }
  }
  
  .pagination-ellipsis {
    color: #718096;
    font-weight: 600;
    padding: 0 0.25rem;
  }
  @media (prefers-color-scheme: dark) {
    .pagination-ellipsis {
      color: #cbd5e0;
    }
  }
  
  .no-results {
    text-align: center;
    padding: 4rem 2rem;
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border: 2px dashed #e2e8f0;
    border-radius: 16px;
  }
  .no-results-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
    opacity: 0.5;
  }
  .no-results h3 {
    font-size: 1.25rem;
    font-weight: 700;
    color: #2d3748;
    margin: 0 0 0.5rem 0;
  }
  .no-results p {
    font-size: 0.9375rem;
    color: #718096;
    margin: 0 0 1.5rem 0;
  }
  @media (prefers-color-scheme: dark) {
    .no-results {
      background: linear-gradient(135deg, rgba(45, 55, 72, 0.8) 0%, rgba(30, 41, 59, 0.8) 100%);
      border-color: rgba(255, 255, 255, 0.1);
    }
    .no-results h3 {
      color: #f7fafc;
    }
    .no-results p {
      color: #cbd5e0;
    }
  }
  
  .reset-filters-btn {
    padding: 0.625rem 1.25rem;
    border: 2px solid #ef4444;
    border-radius: 10px;
    background: white;
    color: #ef4444;
    font-weight: 600;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s;
  }
  .reset-filters-btn:hover {
    background: #ef4444;
    color: white;
  }
  @media (prefers-color-scheme: dark) {
    .reset-filters-btn {
      background: rgba(30, 41, 59, 0.8);
    }
  }
</style>