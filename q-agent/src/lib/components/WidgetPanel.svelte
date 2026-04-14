<script lang="ts">
  import { settings } from '$lib/stores/ui';
  import SettingsPanel from './SettingsPanel.svelte';
  import KnowledgePanel from './KnowledgePanel.svelte';

  let { 
    open = false, 
    activeTab = 'sources',
    mode = 'integrated', // 'integrated' | 'overlay'
    onclose 
  } = $props();

  function setTab(tab: 'sources' | 'settings' | 'references') {
    activeTab = tab;
  }
</script>

{#if open}
  {#if mode === 'overlay'}
    <div class="overlay" onclick={onclose} role="presentation" />
  {/if}
  
  <div class="panel" class:overlay-mode={mode === 'overlay'} class:integrated-mode={mode === 'integrated'} role="dialog" aria-label="Widgets">
    <div class="panel-header">
      <div class="tabs">
        <button 
          class="tab-btn" 
          class:active={activeTab === 'sources'} 
          onclick={() => setTab('sources')}
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
            <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
          </svg>
          Sources
        </button>
        <button 
          class="tab-btn" 
          class:active={activeTab === 'settings'} 
          onclick={() => setTab('settings')}
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"></circle>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
          </svg>
          Settings
        </button>
        <button 
          class="tab-btn" 
          class:active={activeTab === 'references'} 
          disabled
          title="Coming soon"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
          </svg>
          Refs
        </button>
      </div>
      <button class="close-btn" onclick={onclose} aria-label="Close">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <path d="M18 6L6 18M6 6l12 12"/>
        </svg>
      </button>
    </div>

    <div class="panel-content">
      {#if activeTab === 'sources'}
        <KnowledgePanel standalone={true} />
      {:else if activeTab === 'settings'}
        <SettingsPanel standalone={true} />
      {:else if activeTab === 'references'}
        <div class="empty-state">
          <p>Reference management is coming soon.</p>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed; inset: 0;
    background: rgba(0,0,0,0.3);
    backdrop-filter: blur(2px);
    z-index: 100;
  }

  .panel {
    width: 400px;
    max-width: 90vw;
    background: var(--bg-surface);
    border-left: 1px solid var(--border-default);
    display: flex;
    flex-direction: column;
    box-shadow: -10px 0 50px rgba(0,0,0,0.4);
    z-index: 101;
  }

  .panel.overlay-mode {
    position: fixed;
    right: 0; top: 0; bottom: 0;
    animation: slideIn 250ms cubic-bezier(0.16, 1, 0.3, 1);
  }

  .panel.integrated-mode {
    position: relative;
    height: 100%;
    flex-shrink: 0;
    box-shadow: none;
    border-left: 1px solid var(--border-subtle);
  }

  @keyframes slideIn {
    from { transform: translateX(100%); }
    to   { transform: translateX(0); }
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-subtle);
    background: var(--bg-app);
  }

  .tabs {
    display: flex;
    gap: 4px;
    background: var(--bg-elevated);
    padding: 3px;
    border-radius: 8px;
  }

  .tab-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border: none;
    background: transparent;
    color: var(--text-tertiary);
    font-size: var(--font-size-xs);
    font-weight: 600;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .tab-btn:hover:not(:disabled) {
    color: var(--text-secondary);
    background: rgba(255,255,255,0.05);
  }

  .tab-btn.active {
    background: var(--bg-surface);
    color: var(--accent-primary);
    box-shadow: 0 2px 8px rgba(0,0,0,0.2);
  }

  .tab-btn:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .close-btn {
    width: 32px; height: 32px;
    display: flex; align-items: center; justify-content: center;
    background: transparent;
    border: none;
    color: var(--text-tertiary);
    cursor: pointer;
    border-radius: 50%;
  }

  .close-btn:hover {
    background: var(--bg-overlay);
    color: var(--text-primary);
  }

  .panel-content {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-tertiary);
    font-size: var(--font-size-sm);
  }
</style>
