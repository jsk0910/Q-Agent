<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';

  interface PullProgress {
    status: string;
    digest?: string;
    total?: number;
    completed?: number;
    percentage?: number;
  }

  let { model, onclose, onsuccess } = $props<{
    model: string;
    onclose: () => void;
    onsuccess: (model: string) => void;
  }>();

  let progress = $state<PullProgress>({ status: 'Starting...' });
  let error = $state<string | null>(null);
  let unlisten: UnlistenFn | undefined;

  async function startPull() {
    try {
      unlisten = await listen<PullProgress>('pull_progress', (event) => {
        progress = event.payload;
        if (progress.status === 'success') {
          setTimeout(() => {
            onsuccess(model);
            onclose();
          }, 1000);
        }
      });

      await invoke('pull_model', { model });
    } catch (e: any) {
      error = e.toString();
    }
  }

  onMount(() => {
    startPull();
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });

  function formatBytes(bytes?: number) {
    if (!bytes) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }
</script>

<div class="modal-overlay" onclick={onclose}>
  <div class="modal-content" onclick={e => e.stopPropagation()}>
    <div class="modal-header">
      <div class="title-wrap">
        <span class="download-icon">⬇️</span>
        <h3>Installing {model}</h3>
      </div>
      {#if !progress.percentage || progress.percentage < 100}
        <button class="close-btn" onclick={onclose}>&times;</button>
      {/if}
    </div>

    <div class="modal-body">
      {#if error}
        <div class="error-box">
          <span class="error-msg">⚠️ {error}</span>
          <button class="retry-btn" onclick={() => { error = null; startPull(); }}>Retry</button>
        </div>
      {:else}
        <div class="status-text">{progress.status}</div>
        
        {#if progress.percentage !== undefined}
          <div class="progress-container">
            <div class="progress-bar">
              <div class="progress-fill" style="width: {progress.percentage}%"></div>
            </div>
            <div class="progress-meta">
              <span>{progress.percentage.toFixed(1)}%</span>
              {#if progress.completed && progress.total}
                <span>{formatBytes(progress.completed)} / {formatBytes(progress.total)}</span>
              {/if}
            </div>
          </div>
        {:else}
          <div class="indented-loader">
            <div class="loader-bar"></div>
          </div>
        {/if}

        <p class="hint">Please don't close the application until the download is complete.</p>
      {/if}
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
    animation: fadeIn 0.2s ease;
  }

  @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }

  .modal-content {
    background: var(--bg-app);
    border: 1px solid var(--border-default);
    border-radius: var(--border-radius-lg);
    width: 100%;
    max-width: 440px;
    box-shadow: 0 24px 64px rgba(0,0,0,0.6);
    overflow: hidden;
    animation: scaleUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes scaleUp { from { transform: scale(0.95); } to { transform: scale(1); } }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 20px 24px;
    background: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
  }

  .title-wrap {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .download-icon {
    font-size: 1.5rem;
    filter: drop-shadow(0 0 8px var(--accent-glow));
  }

  h3 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: var(--text-tertiary);
    cursor: pointer;
    line-height: 1;
  }

  .modal-body {
    padding: 24px;
  }

  .status-text {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    margin-bottom: 12px;
    font-weight: 500;
  }

  .progress-container {
    margin: 16px 0;
  }

  .progress-bar {
    height: 8px;
    background: var(--bg-elevated);
    border-radius: 99px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--accent-primary), #60f5f0);
    box-shadow: 0 0 10px var(--accent-glow);
    transition: width 0.3s ease;
  }

  .progress-meta {
    display: flex;
    justify-content: space-between;
    margin-top: 8px;
    font-size: 12px;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
  }

  .indented-loader {
    height: 8px;
    background: var(--bg-elevated);
    border-radius: 99px;
    overflow: hidden;
    margin: 16px 0;
    position: relative;
  }

  .loader-bar {
    position: absolute;
    top: 0; left: 0; bottom: 0;
    width: 30%;
    background: var(--accent-primary);
    box-shadow: 0 0 10px var(--accent-glow);
    animation: loaderMove 1.5s infinite ease-in-out;
  }

  @keyframes loaderMove {
    0% { left: -30%; }
    100% { left: 100%; }
  }

  .error-box {
    background: rgba(255, 100, 100, 0.1);
    border: 1px solid rgba(255, 100, 100, 0.2);
    border-radius: var(--border-radius-sm);
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .error-msg {
    color: #ff6b6b;
    font-size: var(--font-size-sm);
  }

  .retry-btn {
    align-self: flex-end;
    background: var(--bg-elevated);
    border: 1px solid var(--border-default);
    color: var(--text-primary);
    padding: 6px 16px;
    border-radius: 4px;
    cursor: pointer;
    font-size: var(--font-size-xs);
  }

  .hint {
    font-size: 12px;
    color: var(--text-tertiary);
    text-align: center;
    margin-top: 24px;
  }
</style>
