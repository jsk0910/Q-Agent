<script lang="ts">
  interface Props {
    id: string;
    thinking?: string;
    showThinking: boolean;
    expandedThinking: Set<string>;
    onToggle: (id: string) => void;
    harnessStatus?: string;
    harnessPlan?: any;
    harnessResults?: string[];
  }

  const {
    id,
    thinking,
    showThinking,
    expandedThinking,
    onToggle,
    harnessStatus,
    harnessPlan,
    harnessResults
  } = $props<Props>();

  let showResults = $state(false);
</script>

{#if (thinking || harnessStatus || harnessPlan) && showThinking}
  <div class="thinking-container">
    <div class="thinking-header-row">
      <button
        class="thinking-toggle"
        onclick={() => onToggle(id)}
        aria-expanded={expandedThinking.has(id)}
      >
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/>
        </svg>
        <span>
          {#if harnessStatus}
            {harnessStatus}
          {:else if thinking}
            Thinking process
          {:else}
            Processing task
          {/if}
        </span>
        <svg
          class="chevron"
          class:rotated={expandedThinking.has(id)}
          width="12" height="12" viewBox="0 0 24 24"
          fill="none" stroke="currentColor" stroke-width="2.5"
        >
          <path d="M6 9l6 6 6-6"/>
        </svg>
      </button>

      {#if harnessResults && harnessResults.length > 0}
        <button 
          class="results-view-btn" 
          onclick={() => { showResults = !showResults; if (!expandedThinking.has(id)) onToggle(id); }}
        >
          {showResults ? 'Hide Results' : 'View Step Results'}
        </button>
      {/if}
    </div>

    {#if expandedThinking.has(id)}
      <div class="thinking-content">
        {#if showResults && harnessResults}
          <div class="results-grid">
            {#each harnessResults as res, i}
              <div class="result-box">
                <header class="result-header">Result {i + 1}</header>
                <div class="result-body">{res}</div>
              </div>
            {/each}
          </div>
        {/if}

        {#if harnessPlan && harnessPlan.steps}
          <div class="plan-card">
            <h4 class="plan-title">Execution Plan</h4>
            <ul class="step-list">
              {#each harnessPlan.steps as step, i}
                <li class="step-item" class:active={harnessStatus?.includes(step.title)}>
                  <span class="step-num">{i + 1}</span>
                  <div class="step-info">
                    <span class="step-label">{step.title}</span>
                    <p class="step-desc">{step.task}</p>
                  </div>
                </li>
              {/each}
            </ul>
          </div>
        {/if}

        {#if thinking}
          <div class="thinking-inner">
            {thinking}
          </div>
        {/if}
      </div>
    {/if}
  </div>
{/if}

<style>
  .thinking-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin: 8px 0;
  }

  .thinking-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
  }

  .results-view-btn {
    padding: 6px 12px;
    background: var(--accent-dim);
    border: 1px solid var(--accent-subtle);
    color: var(--accent-primary);
    border-radius: var(--border-radius-sm);
    font-size: var(--font-size-xs);
    font-weight: 700;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .results-view-btn:hover {
    background: var(--accent-primary);
    color: white;
  }

  .thinking-toggle {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    padding: 6px 14px;
    background: var(--bg-overlay);
    border: 1px solid var(--border-subtle);
    color: var(--text-secondary);
    border-radius: var(--border-radius-md);
    font-size: var(--font-size-xs);
    font-weight: 700;
    cursor: pointer;
    font-family: var(--font-sans);
    transition: all var(--transition-fast);
    align-self: flex-start;
    box-shadow: var(--shadow-sm);
  }

  .thinking-toggle:hover {
    background: var(--bg-surface);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .chevron { transition: transform var(--transition-base); }
  .chevron.rotated { transform: rotate(180deg); }

  .results-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 12px;
    margin-bottom: 8px;
  }

  .result-box {
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: var(--border-radius-sm);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    box-shadow: var(--shadow-sm);
  }

  .result-header {
    background: var(--bg-overlay);
    padding: 6px 10px;
    font-size: 10px;
    font-weight: 800;
    text-transform: uppercase;
    color: var(--accent-primary);
    border-bottom: 1px solid var(--border-subtle);
  }

  .result-body {
    padding: 10px;
    font-size: var(--font-size-xs);
    color: var(--text-secondary);
    max-height: 120px;
    overflow-y: auto;
    line-height: 1.5;
  }

  .thinking-content {
    display: flex;
    flex-direction: column;
    gap: 12px;
    animation: fadeUp 200ms ease;
  }

  .plan-card {
    background: var(--bg-overlay);
    border: 1px solid var(--border-subtle);
    border-radius: var(--border-radius-md);
    padding: 14px;
    border-left: 4px solid var(--accent-primary);
  }

  .plan-title {
    font-size: var(--font-size-xs);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-tertiary);
    margin-bottom: 12px;
    font-weight: 800;
  }

  .step-list {
    list-style: none;
    padding: 0; margin: 0;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .step-item {
    display: flex;
    gap: 12px;
    opacity: 0.6;
    transition: opacity 0.3s ease;
  }

  .step-item.active {
    opacity: 1;
    color: var(--accent-primary);
  }

  .step-num {
    display: flex; align-items: center; justify-content: center;
    width: 20px; height: 20px;
    background: var(--accent-dim);
    border-radius: 50%;
    font-size: 10px;
    font-weight: 800;
    flex-shrink: 0;
  }

  .step-item.active .step-num {
    background: var(--accent-primary);
    color: white;
    box-shadow: 0 0 10px var(--accent-glow);
  }

  .step-info { display: flex; flex-direction: column; gap: 2px; }
  .step-label { font-size: var(--font-size-sm); font-weight: 700; }
  .step-desc { font-size: var(--font-size-xs); color: var(--text-tertiary); line-height: 1.4; }

  .thinking-inner {
    background: var(--bg-subtle);
    border: 1px solid var(--border-subtle);
    border-radius: var(--border-radius-md);
    padding: 12px 14px;
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    line-height: 1.7;
    white-space: pre-wrap;
    font-family: var(--font-mono);
    max-height: 300px;
    overflow-y: auto;
  }

  @keyframes fadeUp {
    from { opacity: 0; transform: translateY(8px); }
    to   { opacity: 1; transform: translateY(0); }
  }
</style>
