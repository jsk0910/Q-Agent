<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, tick } from 'svelte';
  import { activeCitation } from '$lib/stores/ui';
  import { projectStore } from '$lib/stores/projectStore';

  let { open = false, standalone = false, onclose } = $props();

  let sources = $state<any[]>([]);
  let uploadText = $state("");
  let isUploading = $state(false);
  let listContainer = $state<HTMLDivElement>();

  // React to activeCitation changes to scroll and highlight
  $effect(() => {
    if ($activeCitation && sources.length > 0 && listContainer && (open || standalone)) {
      const match = $activeCitation.match(/(\d+)/);
      if (match) {
        const idx = parseInt(match[1]) - 1;
        if (idx >= 0 && idx < sources.length) {
          tick().then(() => {
            const card = listContainer?.querySelector(`[data-index="${idx}"]`);
            if (card) {
              card.scrollIntoView({ behavior: 'smooth', block: 'center' });
              card.classList.add('highlight-pulse');
              setTimeout(() => card.classList.remove('highlight-pulse'), 2000);
            }
          });
        }
      }
      activeCitation.set(null); // Reset after handling
    }
  });

  async function fetchSources() {
    try {
      sources = await invoke('get_sources', { projectId: $projectStore.activeProjectId || null });
    } catch (e) {
      console.error(e);
    }
  }

  async function handleAddSource() {
    if (!uploadText.trim()) return;
    isUploading = true;
    try {
      await invoke('add_source', { 
        text: uploadText, 
        format: 'txt', 
        projectId: $projectStore.activeProjectId || null 
      });
      uploadText = "";
      await fetchSources();
    } catch (e) {
      console.error(e);
      alert("Failed to add source: " + e);
    } finally {
      isUploading = false;
    }
  }

  $effect(() => {
    // Re-fetch when project changes or when opened
    let currentProj = $projectStore.activeProjectId;
    if (open || standalone) {
      fetchSources();
    }
  });
</script>

{#if standalone}
  <div class="panel-body standalone">
    <!-- Add Source Area -->
    <section class="setting-section">
      <h3 class="section-title">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
          <polyline points="17 8 12 3 7 8"></polyline>
          <line x1="12" y1="3" x2="12" y2="15"></line>
        </svg>
        Add New Source
      </h3>
      <textarea
        class="text-input textarea"
        bind:value={uploadText}
        placeholder="Paste document text or content to vectorize..."
        rows="4"
      ></textarea>
      <button class="action-btn" onclick={handleAddSource} disabled={isUploading}>
        {isUploading ? 'Chunking & Embedding...' : 'Add to Knowledge Base'}
      </button>
    </section>

    <!-- Source List -->
    <section class="setting-section flex-grow">
      <h3 class="section-title">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
          <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
        </svg>
        Your Sources ({sources.length})
      </h3>
      <div class="sources-list" bind:this={listContainer}>
        {#each sources as src, i}
          <div class="source-card" data-index={i}>
            <div class="src-title">{src.title}</div>
            <div class="src-summary">{src.summary}</div>
          </div>
        {:else}
          <div class="empty-state">No sources added yet.</div>
        {/each}
      </div>
    </section>
  </div>
{:else if open}
  <div class="overlay" onclick={onclose} role="presentation" />
  <div class="panel" role="dialog" aria-label="Knowledge Sources">
    <div class="panel-header">
      <h2>Notebook Sources</h2>
      <button class="close-btn" onclick={onclose} aria-label="Close">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <path d="M18 6L6 18M6 6l12 12"/>
        </svg>
      </button>
    </div>

    <div class="panel-body">
      <!-- Add Source Area -->
      <section class="setting-section">
        <h3 class="section-title">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
            <polyline points="17 8 12 3 7 8"></polyline>
            <line x1="12" y1="3" x2="12" y2="15"></line>
          </svg>
          Add New Source
        </h3>
        <textarea
          id="upload-text-nested"
          class="text-input textarea"
          bind:value={uploadText}
          placeholder="Paste document text or content to vectorize..."
          rows="4"
        ></textarea>
        <button class="action-btn" onclick={handleAddSource} disabled={isUploading}>
          {isUploading ? 'Chunking & Embedding...' : 'Add to Knowledge Base'}
        </button>
      </section>

      <!-- Source List -->
      <section class="setting-section flex-grow">
        <h3 class="section-title">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
            <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
          </svg>
          Your Sources ({sources.length})
        </h3>
        <div class="sources-list" bind:this={listContainer}>
          {#each sources as src, i}
            <div class="source-card" data-index={i}>
              <div class="src-title">{src.title}</div>
              <div class="src-summary">{src.summary}</div>
            </div>
          {:else}
            <div class="empty-state">No sources added yet.</div>
          {/each}
        </div>
      </section>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed; inset: 0;
    background: rgba(0,0,0,0.55);
    backdrop-filter: blur(4px);
    z-index: 100;
    animation: fadeIn 150ms ease;
  }

  .panel {
    position: fixed;
    right: 0; top: 0; bottom: 0;
    width: 380px;
    max-width: 95vw;
    background: var(--bg-surface);
    border-left: 1px solid var(--border-default);
    z-index: 101;
    display: flex;
    flex-direction: column;
    animation: slideIn 200ms cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: -8px 0 40px rgba(0,0,0,0.5);
  }

  @keyframes slideIn {
    from { transform: translateX(100%); opacity: 0; }
    to   { transform: translateX(0);    opacity: 1; }
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to   { opacity: 1; }
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 18px 20px;
    border-bottom: 1px solid var(--border-subtle);
  }

  h2 {
    font-size: var(--font-size-md);
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .close-btn {
    display: flex; align-items: center; justify-content: center;
    width: 30px; height: 30px;
    border: none; background: var(--bg-elevated);
    color: var(--text-secondary);
    border-radius: var(--border-radius-sm);
    cursor: pointer;
    transition: background var(--transition-fast);
  }

  .close-btn:hover { background: var(--bg-overlay); }

  .panel-body {
    flex: 1;
    overflow-y: auto;
    padding: 16px 20px 24px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .setting-section {
    background: var(--bg-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--border-radius-md);
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .flex-grow { flex: 1; }

  .section-title {
    display: flex; align-items: center; gap: 7px;
    font-size: var(--font-size-xs); font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.08em;
    color: var(--text-tertiary); margin-bottom: 2px;
  }

  .text-input {
    width: 100%; padding: 8px 10px;
    background: var(--bg-app); border: 1px solid var(--border-default);
    color: var(--text-primary); border-radius: var(--border-radius-sm);
    font-size: var(--font-size-sm); font-family: var(--font-sans);
    outline: none; transition: border-color var(--transition-fast);
  }
  .text-input:focus {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 2px var(--accent-dim);
  }
  .textarea { resize: vertical; min-height: 80px; line-height: 1.5; }

  .action-btn {
    width: 100%; padding: 10px;
    background: var(--accent-primary); color: white;
    border: none; border-radius: var(--border-radius-sm);
    font-weight: 600; cursor: pointer; transition: opacity 0.2s;
  }
  .action-btn:disabled { opacity: 0.6; cursor: not-allowed; }
  .action-btn:not(:disabled):hover { opacity: 0.9; }

  .sources-list {
    display: flex; flex-direction: column; gap: 8px;
    overflow-y: auto;
  }

  .source-card {
    background: var(--bg-app);
    padding: 12px;
    border-radius: var(--border-radius-sm);
    border: 1px solid var(--border-subtle);
    transition: box-shadow 0.3s ease, border-color 0.3s ease, background-color 0.3s ease;
  }

  :global(.source-card.highlight-pulse) {
    border-color: var(--accent-primary);
    background-color: rgba(32, 217, 210, 0.1);
    box-shadow: 0 0 15px rgba(32, 217, 210, 0.4);
  }

  .src-title {
    font-weight: 600; font-size: var(--font-size-sm);
    color: var(--text-primary); margin-bottom: 4px;
  }
  .src-summary {
    font-size: var(--font-size-xs); color: var(--text-secondary);
    line-height: 1.4;
  }
  .empty-state {
    font-size: var(--font-size-sm); color: var(--text-tertiary);
    text-align: center; padding: 20px 0;
  }
</style>
