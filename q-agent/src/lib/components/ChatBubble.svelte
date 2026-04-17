<script lang="ts">
  import ThinkingPanel from './ThinkingPanel.svelte';
  
  let { 
    msg, 
    isLast, 
    isLoading, 
    showThinking, 
    expandedThinking, 
    onToggleThinking,
    formatTimestamp,
    renderMd 
  } = $props();
</script>

<div class="message-wrap" class:user={msg.role === 'user'} class:asst={msg.role === 'assistant'}>
  {#if msg.role === 'user'}
    <div class="user-bubble">
      <div class="bubble-content">{msg.content}</div>
      <span class="msg-time">{formatTimestamp(msg.timestamp)}</span>
    </div>
  {:else}
    <div class="asst-block">
      <div class="asst-avatar">
        <span>⬡</span>
      </div>
      <div class="asst-body">
        
        <ThinkingPanel 
          thinking={msg.thinking} 
          id={msg.id}
          {showThinking}
          {expandedThinking}
          onToggle={onToggleThinking}
          harnessStatus={msg.harnessStatus}
          harnessPlan={msg.harnessPlan}
          harnessResults={msg.harnessResults}
        />

        {#if msg.content}
          {#await renderMd(msg.content) then html}
            <div class="markdown-body">{@html html}</div>
          {/await}
        {/if}

        {#if isLoading && isLast && msg.role === 'assistant'}
          <div class="typing-indicator">
            <span></span><span></span><span></span>
          </div>
        {/if}

        <div class="asst-footer">
          <span class="msg-time">{formatTimestamp(msg.timestamp)}</span>
          {#if msg.model}
            <span class="model-tag">{msg.model}</span>
          {/if}
          {#if msg.content}
            <button
              class="action-btn"
              title="Copy response"
              onclick={() => navigator.clipboard.writeText(msg.content)}
            >
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="9" y="9" width="13" height="13" rx="2"/>
                <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
              </svg>
            </button>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .message-wrap {
    display: flex;
    flex-direction: column;
    padding: 6px 0;
  }

  .user-bubble {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 4px;
    animation: fadeUp 200ms ease both;
  }

  .bubble-content {
    max-width: 75%;
    padding: 10px 15px;
    background: var(--bg-elevated);
    border: 1px solid var(--border-default);
    border-radius: var(--border-radius-lg) var(--border-radius-lg) 4px var(--border-radius-lg);
    color: var(--text-primary);
    font-size: var(--font-size-base);
    line-height: 1.6;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .asst-block {
    display: flex;
    gap: 12px;
    animation: fadeUp 200ms ease both;
    padding: 8px 0;
  }

  .asst-avatar {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background: var(--accent-dim);
    border: 1px solid rgba(32, 217, 210, 0.25);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.85rem;
    flex-shrink: 0;
    margin-top: 2px;
    color: var(--accent-primary);
  }

  .asst-body {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .typing-indicator {
    display: flex;
    gap: 5px;
    padding: 12px 4px;
    align-items: center;
  }

  .typing-indicator span {
    width: 6px; height: 6px;
    border-radius: 50%;
    background: var(--accent-primary);
    animation: bounce 1s ease-in-out infinite;
    opacity: 0.7;
  }

  .typing-indicator span:nth-child(2) { animation-delay: 0.15s; }
  .typing-indicator span:nth-child(3) { animation-delay: 0.3s; }

  @keyframes bounce {
    0%, 100% { transform: translateY(0); opacity: 0.4; }
    50%       { transform: translateY(-5px); opacity: 1; }
  }

  .asst-footer {
    display: flex;
    align-items: center;
    gap: 8px;
    padding-top: 4px;
  }

  .msg-time {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
  }

  .model-tag {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
    background: var(--bg-elevated);
    padding: 1px 6px;
    border-radius: 99px;
    border: 1px solid var(--border-subtle);
  }

  .action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px; height: 24px;
    border: none;
    background: transparent;
    color: var(--text-tertiary);
    border-radius: 4px;
    cursor: pointer;
    transition: background var(--transition-fast), color var(--transition-fast);
  }

  .action-btn:hover {
    background: var(--bg-elevated);
    color: var(--text-secondary);
  }

  @keyframes fadeUp {
    from { opacity: 0; transform: translateY(8px); }
    to   { opacity: 1; transform: translateY(0); }
  }
</style>
