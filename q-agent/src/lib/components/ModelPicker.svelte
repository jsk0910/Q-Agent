<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { modeStore } from '$lib/stores/mode';
  import { profileStore } from '$lib/stores/profile';
  import { effectiveSettings } from '$lib/stores/ui';

  let { onrequestInstall, onopenExplorer } = $props<{
    onrequestInstall: (model: string) => void;
    onopenExplorer: () => void;
  }>();

  let installedModels = $state<string[]>([]);
  let isOpen = $state(false);
  let isLoading = $state(true);

  let activeModeInfo = $derived(modeStore.getModeInfo($modeStore));
  let currentModel = $derived($effectiveSettings.modelName || 'qwen3.5:4b');

  async function refreshModels() {
    isLoading = true;
    try {
      installedModels = await invoke<string[]>('list_models');
    } catch (e) {
      console.error('Failed to list models', e);
    } finally {
      isLoading = false;
    }
  }

  function toggleDropdown() {
    isOpen = !isOpen;
    if (isOpen) refreshModels();
  }

  function selectModel(model: string) {
    if (installedModels.includes(model)) {
      profileStore.setModelForMode($modeStore, model);
      isOpen = false;
    } else {
      // Prompt for download
      onrequestInstall(model);
      isOpen = false;
    }
  }

  // Handle clicks outside
  function handleOutsideClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest('.model-picker-container')) {
      isOpen = false;
    }
  }

  onMount(() => {
    refreshModels();
    window.addEventListener('click', handleOutsideClick);
    return () => window.removeEventListener('click', handleOutsideClick);
  });
</script>

<div class="model-picker-container">
  <button 
    class="model-picker-trigger" 
    onclick={toggleDropdown}
    aria-haspopup="listbox"
    aria-expanded={isOpen}
  >
    <div class="trigger-content">
      <span class="mode-icon">{activeModeInfo.icon}</span>
      <span class="model-name truncate">{currentModel}</span>
    </div>
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="chevron" class:rotated={isOpen}>
      <path d="M6 9l6 6 6-6"/>
    </svg>
  </button>

  {#if isOpen}
    <div class="model-dropdown">
      <div class="dropdown-header">
        <span class="header-title">Select Model</span>
        <button class="refresh-btn" onclick={refreshModels} disabled={isLoading}>
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class:spin={isLoading}>
            <path d="M23 4v6h-6M1 20v-6h6M3.51 9a9 9 0 0114.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0020.49 15"/>
          </svg>
        </button>
      </div>

      <div class="dropdown-sections">
        <!-- Recommended models for current mode -->
        <div class="section">
          <div class="section-label">Recommended for {activeModeInfo.label}</div>
          {#each activeModeInfo.recommendedModels as model}
            <button 
              class="model-item" 
              class:active={currentModel === model}
              onclick={() => selectModel(model)}
            >
              <div class="model-info">
                <span class="name">{model}</span>
                {#if installedModels.includes(model)}
                  <span class="status installed">🟢</span>
                {:else}
                  <span class="status missing">⬇️</span>
                {/if}
              </div>
            </button>
          {/each}
        </div>

        <!-- Other installed models -->
        <div class="section">
          <div class="section-label">Other Installed</div>
          {#each installedModels.filter(m => !activeModeInfo.recommendedModels.includes(m)) as model}
            <button 
              class="model-item" 
              class:active={currentModel === model}
              onclick={() => selectModel(model)}
            >
              <div class="model-info">
                <span class="name">{model}</span>
                <span class="status installed">🟢</span>
              </div>
            </button>
          {/each}
        </div>

        <div class="dropdown-footer">
          <button class="more-btn" onclick={() => { isOpen = false; onopenExplorer(); }}>
            <span>Explore All Models</span>
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6M15 3h6v6M10 14L21 3"/>
            </svg>
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .model-picker-container {
    position: relative;
    display: inline-block;
  }

  .model-picker-trigger {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px;
    background: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: 99px;
    color: var(--text-primary);
    cursor: pointer;
    transition: all 0.2s ease;
    min-width: 140px;
    max-width: 220px;
  }

  .model-picker-trigger:hover {
    background: var(--bg-elevated);
    border-color: var(--accent-primary);
    box-shadow: 0 0 10px var(--accent-glow);
  }

  .trigger-content {
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
    min-width: 0;
  }

  .mode-icon {
    font-size: 1rem;
    flex-shrink: 0;
  }

  .model-name {
    font-size: var(--font-size-xs);
    font-weight: 600;
  }

  .chevron {
    transition: transform 0.2s ease;
    color: var(--text-tertiary);
  }

  .chevron.rotated {
    transform: rotate(180deg);
  }

  .model-dropdown {
    position: absolute;
    bottom: calc(100% + 8px);
    left: 0;
    width: 260px;
    background: var(--bg-app);
    border: 1px solid var(--border-default);
    border-radius: var(--border-radius-lg);
    box-shadow: 0 8px 32px rgba(0,0,0,0.4);
    z-index: 1000;
    animation: dropdownFadeIn 0.2s ease;
    overflow: hidden;
  }

  @keyframes dropdownFadeIn {
    from { opacity: 0; transform: translateY(8px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .dropdown-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 14px;
    border-bottom: 1px solid var(--border-subtle);
    background: var(--bg-surface);
  }

  .header-title {
    font-size: var(--font-size-xs);
    font-weight: 700;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .refresh-btn {
    background: none;
    border: none;
    color: var(--text-tertiary);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    transition: all 0.2s;
  }

  .refresh-btn:hover:not(:disabled) {
    background: var(--bg-elevated);
    color: var(--text-primary);
  }

  .dropdown-sections {
    max-height: 400px;
    overflow-y: auto;
    padding: 8px 0;
  }

  .section-label {
    padding: 8px 14px 4px;
    font-size: 10px;
    font-weight: 700;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.02em;
  }

  .model-item {
    display: block;
    width: 100%;
    padding: 10px 14px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    text-align: left;
    cursor: pointer;
    transition: all 0.2s;
  }

  .model-item:hover {
    background: var(--bg-elevated);
    color: var(--text-primary);
  }

  .model-item.active {
    background: var(--accent-dim);
    color: var(--accent-primary);
  }

  .model-info {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .name {
    font-size: var(--font-size-sm);
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .status {
    font-size: 0.75rem;
    opacity: 0.8;
  }

  .dropdown-footer {
    padding: 8px;
    border-top: 1px solid var(--border-subtle);
    background: var(--bg-surface);
  }

  .more-btn {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 8px;
    background: var(--bg-elevated);
    border: 1px solid var(--border-default);
    border-radius: 6px;
    color: var(--accent-primary);
    font-size: var(--font-size-xs);
    font-weight: 700;
    cursor: pointer;
    transition: all 0.2s;
  }

  .more-btn:hover {
    background: var(--bg-overlay);
    border-color: var(--accent-primary);
    box-shadow: 0 0 10px var(--accent-glow);
  }

  .spin {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
