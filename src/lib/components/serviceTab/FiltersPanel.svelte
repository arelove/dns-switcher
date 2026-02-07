<script lang="ts">
  interface Props {
    searchQuery: string;
    selectedCategory: 'all' | 'public' | 'security' | 'privacy' | 'adblock' | 'family' | 'gaming' | 'custom';
    showOnlyDoH: boolean;
    showOnlyDoT: boolean;
    showOnlyIPv6: boolean;
    categoryStats: {
      all: number;
      public: number;
      security: number;
      privacy: number;
      adblock: number;
      gaming: number;
      family: number;
      custom: number;
    };
    onSearchChange: (value: string) => void;
    onCategoryChange: (category: 'all' | 'public' | 'security' | 'privacy' | 'adblock' | 'family' | 'gaming' | 'custom') => void;
    onDoHToggle: (value: boolean) => void;
    onDoTToggle: (value: boolean) => void;
    onIPv6Toggle: (value: boolean) => void;
    onResetFilters: () => void;
  }
  
  let {
    searchQuery,
    selectedCategory,
    showOnlyDoH,
    showOnlyDoT,
    showOnlyIPv6,
    categoryStats,
    onSearchChange,
    onCategoryChange,
    onDoHToggle,
    onDoTToggle,
    onIPv6Toggle,
    onResetFilters
  }: Props = $props();
  
  let showCategories = $state(false);
  let showFeatures = $state(false);
  
  const hasActiveFilters = $derived(
    searchQuery || selectedCategory !== 'all' || showOnlyDoH || showOnlyDoT || showOnlyIPv6
  );
  
  const activeFiltersCount = $derived(
    (selectedCategory !== 'all' ? 1 : 0) +
    (showOnlyDoH ? 1 : 0) +
    (showOnlyDoT ? 1 : 0) +
    (showOnlyIPv6 ? 1 : 0)
  );
</script>

<div class="filters-panel">
  <!-- Search with dropdown toggle -->
  <div class="search-section">
    <div class="search-input-wrapper">
      <span class="search-icon">🔍</span>
      <input
        type="text"
        class="search-input"
        placeholder="Search services..."
        value={searchQuery}
        oninput={(e) => onSearchChange(e.currentTarget.value)}
      />
      {#if searchQuery}
        <button class="clear-search" onclick={() => onSearchChange('')}>✕</button>
      {/if}
      <button 
        class="toggle-filters" 
        class:expanded={showCategories || showFeatures}
        onclick={() => {
          showCategories = !showCategories;
          showFeatures = !showFeatures;
        }}
        title="Toggle filters"
      >
        <svg class="filter-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" />
        </svg>
        {#if activeFiltersCount > 0}
          <span class="filter-badge">{activeFiltersCount}</span>
        {/if}
      </button>
    </div>
    
    {#if hasActiveFilters}
      <button class="reset-filters-btn" onclick={onResetFilters}>
        🔄 Reset
      </button>
    {/if}
  </div>

  <!-- Category Filters (collapsible) -->
  {#if showCategories}
    <div class="filter-section animate-in">
      <div class="filter-label">📂 Category</div>
      <div class="category-buttons">
        <button class="category-btn" class:active={selectedCategory === 'all'} onclick={() => onCategoryChange('all')}>
          All ({categoryStats.all})
        </button>
        <button class="category-btn" class:active={selectedCategory === 'public'} onclick={() => onCategoryChange('public')}>
          🌐 Public ({categoryStats.public})
        </button>
        <button class="category-btn" class:active={selectedCategory === 'security'} onclick={() => onCategoryChange('security')}>
          🛡️ Security ({categoryStats.security})
        </button>
        <button class="category-btn" class:active={selectedCategory === 'privacy'} onclick={() => onCategoryChange('privacy')}>
          🔒 Privacy ({categoryStats.privacy})
        </button>
        <button class="category-btn" class:active={selectedCategory === 'adblock'} onclick={() => onCategoryChange('adblock')}>
          🚫 AdBlock ({categoryStats.adblock})
        </button>
        <button class="category-btn" class:active={selectedCategory === 'gaming'} onclick={() => onCategoryChange('gaming')}>
          🎮 Gaming ({categoryStats.gaming})
        </button>
        <button class="category-btn" class:active={selectedCategory === 'family'} onclick={() => onCategoryChange('family')}>
          👨‍👩‍👧‍👦 Family ({categoryStats.family})
        </button>
        <button class="category-btn" class:active={selectedCategory === 'custom'} onclick={() => onCategoryChange('custom')}>
          ⚙️ Custom ({categoryStats.custom})
        </button>
      </div>
    </div>
  {/if}

  <!-- Feature Filters (collapsible) -->
  {#if showFeatures}
    <div class="filter-section animate-in">
      <div class="filter-label">✨ Features</div>
      <div class="feature-toggles">
        <label class="toggle-label">
          <input type="checkbox" class="toggle-checkbox" checked={showOnlyDoH} onchange={(e) => onDoHToggle(e.currentTarget.checked)} />
          <span class="toggle-text">🔒 DoH</span>
        </label>
        <label class="toggle-label">
          <input type="checkbox" class="toggle-checkbox" checked={showOnlyDoT} onchange={(e) => onDoTToggle(e.currentTarget.checked)} />
          <span class="toggle-text">🔐 DoT</span>
        </label>
        <label class="toggle-label">
          <input type="checkbox" class="toggle-checkbox" checked={showOnlyIPv6} onchange={(e) => onIPv6Toggle(e.currentTarget.checked)} />
          <span class="toggle-text">🌍 IPv6</span>
        </label>
      </div>
    </div>
  {/if}
</div>

<style>
  .filters-panel {
    background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
    border: 2px solid #e2e8f0;
    border-radius: 12px;
    padding: 0.75rem;
    display: grid;
    gap: 0.75rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .filters-panel {
      background: linear-gradient(135deg, rgba(45, 55, 72, 0.8) 0%, rgba(30, 41, 59, 0.8) 100%);
      border-color: rgba(255, 255, 255, 0.1);
    }
  }
  
  .search-section {
    display: grid;
    gap: 0.5rem;
  }
  
  .search-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }
  
  .search-icon {
    position: absolute;
    left: 0.875rem;
    font-size: 1rem;
    pointer-events: none;
    z-index: 1;
  }
  
  .search-input {
    width: 100%;
    padding: 0.625rem 3.5rem 0.625rem 2.5rem;
    border: 2px solid #e2e8f0;
    border-radius: 10px;
    font-size: 0.875rem;
    background: white;
    transition: all 0.2s;
  }
  
  .search-input:focus {
    outline: none;
    border-color: #667eea;
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }
  
  @media (prefers-color-scheme: dark) {
    .search-input {
      background: rgba(30, 41, 59, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
      color: #f7fafc;
    }
    .search-input:focus {
      border-color: #06b6d4;
      box-shadow: 0 0 0 3px rgba(6, 182, 212, 0.1);
    }
  }
  
  .clear-search {
    position: absolute;
    right: 3rem;
    background: rgba(0, 0, 0, 0.05);
    border: none;
    border-radius: 6px;
    width: 1.375rem;
    height: 1.375rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    font-size: 0.6875rem;
    color: #64748b;
    transition: all 0.2s;
    z-index: 1;
  }
  
  .clear-search:hover {
    background: rgba(0, 0, 0, 0.1);
    color: #1e293b;
  }
  
  @media (prefers-color-scheme: dark) {
    .clear-search {
      background: rgba(255, 255, 255, 0.05);
      color: #94a3b8;
    }
    .clear-search:hover {
      background: rgba(255, 255, 255, 0.1);
      color: #f1f5f9;
    }
  }
  
  .toggle-filters {
    position: absolute;
    right: 0.5rem;
    background: white;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: #64748b;
    transition: all 0.2s;
    z-index: 1;
  }
  
  .toggle-filters:hover {
    border-color: #667eea;
    color: #667eea;
    background: rgba(102, 126, 234, 0.05);
  }
  
  .toggle-filters.expanded {
    border-color: #667eea;
    color: #667eea;
    background: rgba(102, 126, 234, 0.1);
  }
  
  @media (prefers-color-scheme: dark) {
    .toggle-filters {
      background: rgba(30, 41, 59, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
      color: #94a3b8;
    }
    .toggle-filters:hover {
      border-color: #06b6d4;
      color: #06b6d4;
      background: rgba(6, 182, 212, 0.1);
    }
    .toggle-filters.expanded {
      border-color: #06b6d4;
      color: #06b6d4;
      background: rgba(6, 182, 212, 0.15);
    }
  }
  
  .filter-icon {
    width: 1rem;
    height: 1rem;
    transition: transform 0.3s;
  }
  
  .toggle-filters.expanded .filter-icon {
    transform: rotate(180deg);
  }
  
  .filter-badge {
    position: absolute;
    top: -4px;
    right: -4px;
    background: #ef4444;
    color: white;
    border-radius: 50%;
    width: 1.125rem;
    height: 1.125rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.5625rem;
    font-weight: 700;
    border: 2px solid white;
  }
  
  @media (prefers-color-scheme: dark) {
    .filter-badge {
      border-color: rgba(30, 41, 59, 0.8);
    }
  }
  
  .reset-filters-btn {
    padding: 0.375rem 0.75rem;
    border: 2px solid #ef4444;
    border-radius: 8px;
    background: white;
    color: #ef4444;
    font-weight: 600;
    font-size: 0.75rem;
    cursor: pointer;
    transition: all 0.2s;
    justify-self: start;
  }
  
  .reset-filters-btn:hover {
    background: #ef4444;
    color: white;
    transform: translateY(-1px);
  }
  
  @media (prefers-color-scheme: dark) {
    .reset-filters-btn {
      background: rgba(30, 41, 59, 0.8);
    }
  }
  
  .filter-section {
    display: grid;
    gap: 0.625rem;
    padding-top: 0.5rem;
    border-top: 1px solid #e2e8f0;
  }
  
  @media (prefers-color-scheme: dark) {
    .filter-section {
      border-top-color: rgba(255, 255, 255, 0.1);
    }
  }
  
  .animate-in {
    animation: slideDown 0.3s ease-out;
  }
  
  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
  
  .filter-label {
    font-size: 0.8125rem;
    font-weight: 600;
    color: #4a5568;
  }
  
  @media (prefers-color-scheme: dark) {
    .filter-label {
      color: #cbd5e0;
    }
  }
  
  .category-buttons {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
  }
  
  .category-btn {
    padding: 0.375rem 0.75rem;
    border: 2px solid #e2e8f0;
    border-radius: 6px;
    background: white;
    font-size: 0.75rem;
    font-weight: 500;
    color: #4a5568;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .category-btn:hover {
    border-color: #667eea;
    background: rgba(102, 126, 234, 0.05);
  }
  
  .category-btn.active {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-color: #667eea;
    color: white;
  }
  
  @media (prefers-color-scheme: dark) {
    .category-btn {
      background: rgba(30, 41, 59, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
      color: #cbd5e0;
    }
    .category-btn:hover {
      border-color: #06b6d4;
      background: rgba(6, 182, 212, 0.1);
    }
    .category-btn.active {
      background: linear-gradient(135deg, #06b6d4 0%, #3b82f6 100%);
      border-color: #06b6d4;
    }
  }
  
  .feature-toggles {
    display: flex;
    flex-wrap: wrap;
    gap: 0.875rem;
  }
  
  .toggle-label {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    cursor: pointer;
    user-select: none;
  }
  
  .toggle-checkbox {
    width: 1rem;
    height: 1rem;
    cursor: pointer;
    accent-color: #667eea;
  }
  
  @media (prefers-color-scheme: dark) {
    .toggle-checkbox {
      accent-color: #06b6d4;
    }
  }
  
  .toggle-text {
    font-size: 0.8125rem;
    font-weight: 500;
    color: #4a5568;
  }
  
  @media (prefers-color-scheme: dark) {
    .toggle-text {
      color: #cbd5e0;
    }
  }
</style>