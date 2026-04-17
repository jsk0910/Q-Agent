<script lang="ts">
  import { ALL_MODELS, type ModelMetadata } from '$lib/data/models';
  import { profileStore } from '$lib/stores/profile';
  import { modeStore } from '$lib/stores/mode';
  import { invoke } from '@tauri-apps/api/core';

  let { onclose, installedModels, onrequestInstall } = $props<{
    onclose: () => void;
    installedModels: string[];
    onrequestInstall: (model: string) => void;
  }>();

  let searchQuery = $state('');
  
  let filteredModels = $derived(
    ALL_MODELS.filter(m => 
      m.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
      m.id.toLowerCase().includes(searchQuery.toLowerCase()) ||
      m.tags.some(t => t.toLowerCase().includes(searchQuery.toLowerCase()))
    )
  );

  function handleSelect(modelId: string) {
    if (installedModels.includes(modelId)) {
      profileStore.setModelForMode($modeStore, modelId);
      onclose();
    } else {
      onrequestInstall(modelId);
    }
  }
</script>

<div class="modal-overlay" onclick={onclose}>
  <div class="modal-content" onclick={e => e.stopPropagation()}>
    <div class="modal-header">
      <div class="header-left">
        <span class="header-icon">🌐</span>
        <h3>Explore AI Models</h3>
      </div>
      <button class="close-btn" onclick={onclose}>&times;</button>
    </div>

    <div class="search-bar">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/>
      </svg>
      <input 
        type="text" 
        placeholder="Search for models by name, provider, or capability..." 
        bind:value={searchQuery}
        autofocus
      />
    </div>

    <div class="model-grid">
      {#each filteredModels as model}
        <div class="model-card" class:installed={installedModels.includes(model.id)}>
          <div class="card-header">
            <div class="model-main">
              <h4>{model.name}</h4>
              <span class="provider">{model.provider}</span>
            </div>
            {#if installedModels.includes(model.id)}
              <span class="badge installed">Installed</span>
            {:else}
              <span class="badge available">Available</span>
            {/if}
          </div>

          <p class="description">{model.description}</p>
          
          <div class="meta-row">
            <span class="meta-item">📦 {model.parameters}</span>
            <span class="meta-item">🛠️ {model.useCase}</span>
          </div>

          <div class="tags">
            {#each model.tags as tag}
              <span class="tag">{tag}</span>
            {/each}
          </div>

          <div class="card-actions">
            <button 
              class="action-btn" 
              class:primary={!installedModels.includes(model.id)}
              onclick={() => handleSelect(model.id)}
            >
              {installedModels.includes(model.id) ? 'Select Model' : 'Install Model'}
            </button>
          </div>
        </div>
      {/each}

      {#if filteredModels.length === 0}
        <div class="empty-state">
          No models found matching "{searchQuery}"
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0, 0, 0, 0.8);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2100;
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }

  .modal-content {
    background: var(--bg-app);
    border: 1px solid var(--border-default);
    border-radius: var(--border-radius-lg);
    width: 90vw;
    max-width: 900px;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 32px 64px rgba(0,0,0,0.8);
    overflow: hidden;
    animation: scaleUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes scaleUp { from { transform: scale(0.96); opacity: 0; } to { transform: scale(1); opacity: 1; } }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 24px 32px;
    background: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .header-icon { font-size: 1.5rem; filter: drop-shadow(0 0 8px var(--accent-glow)); }

  h3 { margin: 0; font-size: 1.25rem; font-weight: 700; color: var(--text-primary); }

  .close-btn {
    background: none;
    border: none;
    font-size: 2rem;
    color: var(--text-tertiary);
    cursor: pointer;
    line-height: 1;
    transition: color 0.2s;
  }

  .close-btn:hover { color: var(--text-primary); }

  .search-bar {
    padding: 20px 32px;
    display: flex;
    align-items: center;
    gap: 12px;
    background: var(--bg-app);
    border-bottom: 1px solid var(--border-subtle);
  }

  .search-bar input {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-size: 1rem;
    outline: none;
  }

  .model-grid {
    padding: 32px;
    overflow-y: auto;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 20px;
    background: var(--bg-surface-dim);
  }

  .model-card {
    background: var(--bg-app);
    border: 1px solid var(--border-default);
    border-radius: var(--border-radius-md);
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 14px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .model-card:hover {
    border-color: var(--accent-primary);
    transform: translateY(-4px);
    box-shadow: 0 12px 24px rgba(0,0,0,0.4);
  }

  .model-card.installed {
    border-left: 4px solid var(--accent-primary);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .model-main h4 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .provider {
    font-size: 0.75rem;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .badge {
    font-size: 10px;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 99px;
    text-transform: uppercase;
  }

  .badge.installed { background: var(--accent-dim); color: var(--accent-primary); border: 1px solid var(--accent-primary); }
  .badge.available { background: var(--bg-elevated); color: var(--text-tertiary); border: 1px solid var(--border-default); }

  .description {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    line-height: 1.5;
    margin: 0;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .meta-row {
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-size: 0.75rem;
    color: var(--text-tertiary);
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .tag {
    font-size: 10px;
    background: var(--bg-elevated);
    color: var(--text-tertiary);
    padding: 2px 8px;
    border-radius: 4px;
  }

  .card-actions {
    margin-top: auto;
    padding-top: 8px;
  }

  .action-btn {
    width: 100%;
    padding: 8px;
    border-radius: 6px;
    border: 1px solid var(--border-default);
    background: var(--bg-elevated);
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .action-btn:hover {
    background: var(--bg-overlay);
    border-color: var(--text-tertiary);
  }

  .action-btn.primary {
    background: var(--accent-primary);
    color: #000;
    border: none;
  }

  .action-btn.primary:hover {
    filter: brightness(1.1);
    box-shadow: 0 0 15px var(--accent-glow);
  }

  .empty-state {
    grid-column: 1 / -1;
    text-align: center;
    padding: 60px;
    color: var(--text-tertiary);
    font-style: italic;
  }
</style>
