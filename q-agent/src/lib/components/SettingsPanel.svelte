<script lang="ts">
  import { settings } from '$lib/stores/ui';
  import type { Theme, AccentColor, FontSize } from '$lib/stores/ui';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let { 
    open = $bindable(false), 
    standalone = false,
    onclose 
  } = $props();

  let availableModels = $state<string[]>(['qwen3.5:4b']);

  const themes: { value: Theme; label: string; icon: string }[] = [
    { value: 'dark',  label: 'Dark',  icon: '🌙' },
    { value: 'light', label: 'Light', icon: '☀️' },
    { value: 'oled',  label: 'OLED',  icon: '⚫' },
  ];

  const accents: { value: AccentColor; label: string; color: string }[] = [
    { value: 'teal',   label: 'Teal',   color: '#20d9d2' },
    { value: 'violet', label: 'Violet', color: '#a78bfa' },
    { value: 'orange', label: 'Orange', color: '#fb923c' },
    { value: 'green',  label: 'Green',  color: '#4ade80' },
    { value: 'pink',   label: 'Pink',   color: '#f472b6' },
  ];

  const fontSizes: { value: FontSize; label: string }[] = [
    { value: 'sm', label: 'Small' },
    { value: 'md', label: 'Medium' },
    { value: 'lg', label: 'Large' },
  ];

  function setTheme(t: Theme) {
    settings.update(s => ({ ...s, theme: t }));
  }
  function setAccent(a: AccentColor) {
    settings.update(s => ({ ...s, accent: a }));
  }
  function setFontSize(f: FontSize) {
    settings.update(s => ({ ...s, fontSize: f }));
  }
  function toggleSendOnEnter() {
    settings.update(s => ({ ...s, sendOnEnter: !s.sendOnEnter }));
  }
  function toggleShowThinking() {
    settings.update(s => ({ ...s, showThinking: !s.showThinking }));
  }
  function toggleStream() {
    settings.update(s => ({ ...s, streamTokens: !s.streamTokens }));
  }
  function toggleEvalLoop() {
    settings.update(s => ({ ...s, enableEvalLoop: !s.enableEvalLoop }));
  }
  function toggleAutoPlanning() {
    settings.update(s => ({ ...s, autoPlanning: !s.autoPlanning }));
  }

  onMount(async () => {
    try {
      const models = await invoke<string[]>('list_models');
      if (models && models.length > 0) {
        availableModels = models;
      }
    } catch (err) {
      console.warn("Failed to list models:", err);
    }
  });
</script>

{#if standalone}
  <div class="panel-body standalone">
    <!-- Appearance -->
    <section class="setting-section">
      <h3 class="section-title">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="5"/><path d="M12 2v3M12 19v3M4.22 4.22l2.12 2.12M17.66 17.66l2.12 2.12M2 12h3M19 12h3M4.22 19.78l2.12-2.12M17.66 6.34l2.12-2.12"/>
        </svg>
        Appearance
      </h3>

      <!-- Theme -->
      <div class="setting-row">
        <label class="setting-label">Theme</label>
        <div class="toggle-group">
          {#each themes as t}
            <button
              class="toggle-btn"
              class:active={$settings.theme === t.value}
              onclick={() => setTheme(t.value)}
            >
              {t.icon} {t.label}
            </button>
          {/each}
        </div>
      </div>

      <!-- Accent Color -->
      <div class="setting-row">
        <label class="setting-label">Accent Color</label>
        <div class="color-swatches">
          {#each accents as a}
            <button
              class="swatch"
              class:active={$settings.accent === a.value}
              style="--c: {a.color}"
              title={a.label}
              onclick={() => setAccent(a.value)}
              aria-label={a.label}
            />
          {/each}
        </div>
      </div>

      <!-- Font Size -->
      <div class="setting-row">
        <label class="setting-label">Font Size</label>
        <div class="toggle-group">
          {#each fontSizes as f}
            <button
              class="toggle-btn"
              class:active={$settings.fontSize === f.value}
              onclick={() => setFontSize(f.value)}
            >
              {f.label}
            </button>
          {/each}
        </div>
      </div>
    </section>

    <!-- Model -->
    <section class="setting-section">
      <h3 class="section-title">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="2" y="3" width="20" height="14" rx="2"/><path d="M8 21h8M12 17v4"/>
        </svg>
        Model
      </h3>

      <div class="setting-row vertical">
        <label class="setting-label" for="model-input">Model Name</label>
        <input
          id="model-input"
          class="text-input"
          list="model-list"
          bind:value={$settings.modelName}
          placeholder="e.g. qwen3.5:4b"
        />
        <datalist id="model-list">
          {#each availableModels as model}
            <option value={model} />
          {/each}
        </datalist>
      </div>

      <div class="setting-row">
        <label class="setting-label" for="temp-slider">
          Temperature <span class="val-badge">{$settings.temperature.toFixed(1)}</span>
        </label>
        <input
          id="temp-slider"
          type="range" min="0" max="2" step="0.1"
          bind:value={$settings.temperature}
          class="slider"
        />
      </div>
    </section>

    <!-- Behavior -->
    <section class="setting-section">
      <h3 class="section-title">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
        </svg>
        Behavior
      </h3>

      <div class="setting-row">
        <label class="setting-label">Enter to Send</label>
        <button
          class="toggle-switch"
          class:on={$settings.sendOnEnter}
          onclick={toggleSendOnEnter}
          role="switch"
          aria-checked={$settings.sendOnEnter}
        >
          <span class="thumb"></span>
        </button>
      </div>

      <div class="setting-row">
        <label class="setting-label">Show Thinking</label>
        <button
          class="toggle-switch"
          class:on={$settings.showThinking}
          onclick={toggleShowThinking}
          role="switch"
          aria-checked={$settings.showThinking}
        >
          <span class="thumb"></span>
        </button>
      </div>

      <div class="setting-row">
        <label class="setting-label">Stream Tokens</label>
        <button
          class="toggle-switch"
          class:on={$settings.streamTokens}
          onclick={toggleStream}
          role="switch"
          aria-checked={$settings.streamTokens}
        >
          <span class="thumb"></span>
        </button>
      </div>

      <div class="setting-row">
        <div class="setting-label-group">
          <label class="setting-label">🧪 Eval Loop</label>
          <span class="setting-hint">Verify answers with a critic pass</span>
        </div>
        <button
          class="toggle-switch"
          class:on={$settings.enableEvalLoop}
          onclick={toggleEvalLoop}
          role="switch"
          aria-checked={$settings.enableEvalLoop}
        >
          <span class="thumb"></span>
        </button>
      </div>

      <div class="setting-row">
        <div class="setting-label-group">
          <label class="setting-label">🧩 Auto Planning</label>
          <span class="setting-hint">Step-by-step for SLM (≤ 10B)</span>
        </div>
        <button
          class="toggle-switch"
          class:on={$settings.autoPlanning}
          onclick={toggleAutoPlanning}
          role="switch"
          aria-checked={$settings.autoPlanning}
        >
          <span class="thumb"></span>
        </button>
      </div>

      <div class="setting-row">
        <div class="setting-label-group">
          <label class="setting-label">
            Summarize At <span class="val-badge">{($settings.summarizationThreshold * 100).toFixed(0)}%</span>
          </label>
          <span class="setting-hint">Compress history when context is full</span>
        </div>
        <input
          type="range" min="0.1" max="0.9" step="0.05"
          bind:value={$settings.summarizationThreshold}
          class="slider mini"
        />
      </div>
    </section>

    <!-- System Prompt -->
    <section class="setting-section">
      <h3 class="section-title">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 15a2 2 0 01-2 2H7l-4 4V5a2 2 0 012-2h14a2 2 0 012 2z"/>
        </svg>
        System Prompt
      </h3>
      <textarea
        id="system-prompt-input"
        class="text-input textarea"
        bind:value={$settings.systemPrompt}
        placeholder="Enter a system prompt..."
        rows="4"
      ></textarea>
    </section>

    <button class="reset-btn" id="settings-reset-btn" onclick={settings.reset}>
      Reset to Defaults
    </button>
  </div>
{:else if open}
  <div class="overlay" onclick={close} role="presentation" />
  <div class="panel" role="dialog" aria-label="Settings">
    <!-- Header -->
    <div class="panel-header">
      <h2>Settings</h2>
      <button class="close-btn" id="settings-close-btn" onclick={close} aria-label="Close settings">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <path d="M18 6L6 18M6 6l12 12"/>
        </svg>
      </button>
    </div>

    <div class="panel-body">

      <!-- Appearance -->
      <section class="setting-section">
        <h3 class="section-title">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="5"/><path d="M12 2v3M12 19v3M4.22 4.22l2.12 2.12M17.66 17.66l2.12 2.12M2 12h3M19 12h3M4.22 19.78l2.12-2.12M17.66 6.34l2.12-2.12"/>
          </svg>
          Appearance
        </h3>

        <!-- Theme -->
        <div class="setting-row">
          <label class="setting-label">Theme</label>
          <div class="toggle-group">
            {#each themes as t}
              <button
                class="toggle-btn"
                class:active={$settings.theme === t.value}
                onclick={() => setTheme(t.value)}
              >
                {t.icon} {t.label}
              </button>
            {/each}
          </div>
        </div>

        <!-- Accent Color -->
        <div class="setting-row">
          <label class="setting-label">Accent Color</label>
          <div class="color-swatches">
            {#each accents as a}
              <button
                class="swatch"
                class:active={$settings.accent === a.value}
                style="--c: {a.color}"
                title={a.label}
                onclick={() => setAccent(a.value)}
                aria-label={a.label}
              />
            {/each}
          </div>
        </div>

        <!-- Font Size -->
        <div class="setting-row">
          <label class="setting-label">Font Size</label>
          <div class="toggle-group">
            {#each fontSizes as f}
              <button
                class="toggle-btn"
                class:active={$settings.fontSize === f.value}
                onclick={() => setFontSize(f.value)}
              >
                {f.label}
              </button>
            {/each}
          </div>
        </div>
      </section>

      <!-- Model -->
      <section class="setting-section">
        <h3 class="section-title">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="2" y="3" width="20" height="14" rx="2"/><path d="M8 21h8M12 17v4"/>
          </svg>
          Model
        </h3>

        <div class="setting-row vertical">
          <label class="setting-label" for="model-input">Model Name</label>
          <input
            id="model-input-nested"
            class="text-input"
            list="model-list-nested"
            bind:value={$settings.modelName}
            placeholder="e.g. qwen3.5:4b"
          />
          <datalist id="model-list-nested">
            {#each availableModels as model}
              <option value={model} />
            {/each}
          </datalist>
        </div>

        <div class="setting-row">
          <label class="setting-label" for="temp-slider-nested">
            Temperature <span class="val-badge">{$settings.temperature.toFixed(1)}</span>
          </label>
          <input
            id="temp-slider-nested"
            type="range" min="0" max="2" step="0.1"
            bind:value={$settings.temperature}
            class="slider"
          />
        </div>
      </section>

      <!-- Behavior -->
      <section class="setting-section">
        <h3 class="section-title">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
          </svg>
          Behavior
        </h3>

        <div class="setting-row">
          <label class="setting-label">Enter to Send</label>
          <button
            class="toggle-switch"
            class:on={$settings.sendOnEnter}
            onclick={toggleSendOnEnter}
            role="switch"
            aria-checked={$settings.sendOnEnter}
          >
            <span class="thumb"></span>
          </button>
        </div>

        <div class="setting-row">
          <label class="setting-label">Show Thinking</label>
          <button
            class="toggle-switch"
            class:on={$settings.showThinking}
            onclick={toggleShowThinking}
            role="switch"
            aria-checked={$settings.showThinking}
          >
            <span class="thumb"></span>
          </button>
        </div>

        <div class="setting-row">
          <label class="setting-label">Stream Tokens</label>
          <button
            class="toggle-switch"
            class:on={$settings.streamTokens}
            onclick={toggleStream}
            role="switch"
            aria-checked={$settings.streamTokens}
          >
            <span class="thumb"></span>
          </button>
        </div>

        <div class="setting-row">
          <div class="setting-label-group">
            <label class="setting-label">🧪 Eval Loop</label>
            <span class="setting-hint">Verify answers with a critic pass</span>
          </div>
          <button
            class="toggle-switch"
            class:on={$settings.enableEvalLoop}
            onclick={toggleEvalLoop}
            role="switch"
            aria-checked={$settings.enableEvalLoop}
          >
            <span class="thumb"></span>
          </button>
        </div>

        <div class="setting-row">
          <div class="setting-label-group">
            <label class="setting-label">🧩 Auto Planning</label>
            <span class="setting-hint">Step-by-step for SLM (≤ 10B)</span>
          </div>
          <button
            class="toggle-switch"
            class:on={$settings.autoPlanning}
            onclick={toggleAutoPlanning}
            role="switch"
            aria-checked={$settings.autoPlanning}
          >
            <span class="thumb"></span>
          </button>
        </div>

        <div class="setting-row">
          <div class="setting-label-group">
            <label class="setting-label">
              Summarize At <span class="val-badge">{($settings.summarizationThreshold * 100).toFixed(0)}%</span>
            </label>
            <span class="setting-hint">Compress history when context is full</span>
          </div>
          <input
            type="range" min="0.1" max="0.9" step="0.05"
            bind:value={$settings.summarizationThreshold}
            class="slider mini"
          />
        </div>
      </section>

      <!-- System Prompt -->
      <section class="setting-section">
        <h3 class="section-title">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15a2 2 0 01-2 2H7l-4 4V5a2 2 0 012-2h14a2 2 0 012 2z"/>
          </svg>
          System Prompt
        </h3>
        <textarea
          id="system-prompt-input-nested"
          class="text-input textarea"
          bind:value={$settings.systemPrompt}
          placeholder="Enter a system prompt..."
          rows="4"
        ></textarea>
      </section>

      <button class="reset-btn" id="settings-reset-btn-nested" onclick={settings.reset}>
        Reset to Defaults
      </button>
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
    gap: 4px;
  }

  .setting-section {
    background: var(--bg-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--border-radius-md);
    padding: 14px;
    margin-bottom: 10px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .section-title {
    display: flex;
    align-items: center;
    gap: 7px;
    font-size: var(--font-size-xs);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--text-tertiary);
    margin-bottom: 2px;
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .setting-row.vertical {
    flex-direction: column;
    align-items: flex-start;
  }

  .setting-label {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .setting-label-group {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .setting-hint {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
  }

  .val-badge {
    background: var(--accent-dim);
    color: var(--accent-primary);
    padding: 1px 6px;
    border-radius: 99px;
    font-size: var(--font-size-xs);
    font-weight: 700;
  }

  /* Toggle group (theme/fontsize) */
  .toggle-group {
    display: flex;
    gap: 4px;
    background: var(--bg-overlay);
    padding: 3px;
    border-radius: var(--border-radius-sm);
  }

  .toggle-btn {
    padding: 4px 10px;
    font-size: var(--font-size-xs);
    font-weight: 600;
    border: none;
    background: transparent;
    color: var(--text-tertiary);
    border-radius: 5px;
    cursor: pointer;
    transition: background var(--transition-fast), color var(--transition-fast);
    font-family: var(--font-sans);
  }

  .toggle-btn.active {
    background: var(--bg-surface);
    color: var(--text-primary);
    box-shadow: var(--shadow-sm);
  }

  /* Color swatches */
  .color-swatches {
    display: flex;
    gap: 8px;
  }

  .swatch {
    width: 22px; height: 22px;
    border-radius: 50%;
    background: var(--c);
    border: 2px solid transparent;
    cursor: pointer;
    transition: transform var(--transition-fast), border-color var(--transition-fast);
    box-shadow: 0 0 0 2px var(--bg-elevated);
  }

  .swatch:hover { transform: scale(1.15); }

  .swatch.active {
    border-color: var(--c);
    box-shadow: 0 0 0 3px var(--bg-elevated), 0 0 8px var(--c);
    transform: scale(1.1);
  }

  /* Toggle switch */
  .toggle-switch {
    width: 38px; height: 22px;
    border-radius: 99px;
    border: none;
    background: var(--bg-overlay);
    cursor: pointer;
    position: relative;
    transition: background var(--transition-base);
    flex-shrink: 0;
  }

  .toggle-switch.on { background: var(--accent-primary); }

  .thumb {
    position: absolute;
    top: 3px; left: 3px;
    width: 16px; height: 16px;
    border-radius: 50%;
    background: white;
    transition: left var(--transition-base);
    box-shadow: 0 1px 4px rgba(0,0,0,0.4);
  }

  .toggle-switch.on .thumb { left: 19px; }

  /* Text input */
  .text-input {
    width: 100%;
    padding: 8px 10px;
    background: var(--bg-app);
    border: 1px solid var(--border-default);
    color: var(--text-primary);
    border-radius: var(--border-radius-sm);
    font-size: var(--font-size-sm);
    font-family: var(--font-sans);
    outline: none;
    transition: border-color var(--transition-fast);
  }

  .text-input:focus {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 2px var(--accent-dim);
  }

  .textarea { resize: vertical; min-height: 80px; line-height: 1.5; }

  /* Slider */
  .slider {
    -webkit-appearance: none;
    flex: 1;
    height: 4px;
    background: var(--bg-overlay);
    border-radius: 99px;
    outline: none;
    cursor: pointer;
  }

  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 14px; height: 14px;
    border-radius: 50%;
    background: var(--accent-primary);
    box-shadow: 0 0 6px var(--accent-glow);
  }

  .slider.mini {
    width: 100px;
    flex: none;
  }

  /* Reset */
  .reset-btn {
    margin-top: 6px;
    padding: 8px;
    width: 100%;
    background: transparent;
    border: 1px solid var(--border-default);
    color: var(--text-tertiary);
    border-radius: var(--border-radius-sm);
    font-size: var(--font-size-sm);
    font-family: var(--font-sans);
    cursor: pointer;
    transition: border-color var(--transition-fast), color var(--transition-fast);
  }

  .reset-btn:hover {
    border-color: var(--error);
    color: var(--error);
  }
</style>
