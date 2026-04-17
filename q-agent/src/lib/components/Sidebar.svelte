<script lang="ts">
  import { chatStore, settings } from '$lib/stores/ui';
  import { modeStore, MODES } from '$lib/stores/mode';
  import ProfileSwitcher from './ProfileSwitcher.svelte';
  import ProjectSwitcher from './ProjectSwitcher.svelte';

  let { onSettingsOpen } = $props();

  let conversations = $derived($chatStore.conversations);
  let pinned = $derived(conversations.filter(c => c.pinned));
  let recent = $derived(conversations.filter(c => !c.pinned));
  let collapsed = $derived($settings.sidebarCollapsed);

  function toggleSidebar() {
    settings.update(s => ({ ...s, sidebarCollapsed: !s.sidebarCollapsed }));
  }

  function createNew() {
    chatStore.newConversation();
  }

  function formatTime(date: Date) {
    const d = new Date(date);
    const now = new Date();
    const diffMs = now.getTime() - d.getTime();
    const diffDays = Math.floor(diffMs / 86400000);
    if (diffDays === 0) return 'Today';
    if (diffDays === 1) return 'Yesterday';
    if (diffDays < 7) return `${diffDays}d ago`;
    return d.toLocaleDateString('ko-KR', { month: 'short', day: 'numeric' });
  }

  function handleKeyDelete(e: KeyboardEvent, id: string) {
    if (e.key === 'Delete' || e.key === 'Backspace') {
      e.preventDefault();
      chatStore.deleteConversation(id);
    }
  }
</script>

<aside class="sidebar" class:collapsed>
  <!-- Header -->
  <div class="sidebar-header">
    {#if !collapsed}
      <div class="brand">
        <span class="brand-icon">⬡</span>
        <span class="brand-name">Q-Agent</span>
      </div>
    {/if}
    <button
      class="icon-btn collapse-btn"
      id="sidebar-toggle"
      title={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
      onclick={toggleSidebar}
      aria-label="Toggle sidebar"
    >
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        {#if collapsed}
          <path d="M9 18l6-6-6-6"/>
        {:else}
          <path d="M15 18l-6-6 6-6"/>
        {/if}
      </svg>
    </button>
  </div>

  {#if !collapsed}
    <ProfileSwitcher />
    <ProjectSwitcher />
  {/if}

  <!-- Mode Selector -->
  {#if !collapsed}
    <div class="mode-tabs">
      {#each MODES as mode}
        <button 
          class="mode-tab" 
          class:active={$modeStore === mode.id}
          onclick={() => modeStore.set(mode.id)}
          title={mode.description}
        >
          <span class="mode-icon">{mode.icon}</span>
          <span class="mode-label">{mode.label}</span>
        </button>
      {/each}
    </div>
  {/if}

  <!-- New Chat Button -->
  <div class="sidebar-actions">
    <button class="new-chat-btn" id="new-chat-btn" onclick={createNew} title="New Chat">
      <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <path d="M12 5v14M5 12h14"/>
      </svg>
      {#if !collapsed}<span>New Chat</span>{/if}
    </button>
  </div>

  {#if !collapsed}
    <!-- Conversation List -->
    <div class="conv-list">
      {#if pinned.length > 0}
        <div class="conv-group">
          <div class="conv-group-label">
            <svg width="11" height="11" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
            </svg>
            Pinned
          </div>
          {#each pinned as conv (conv.id)}
            <button
              class="conv-item"
              class:active={$chatStore.activeId === conv.id}
              onclick={() => chatStore.setActive(conv.id)}
              onkeydown={e => handleKeyDelete(e, conv.id)}
            >
              <span class="conv-icon">📌</span>
              <span class="conv-title truncate">{conv.title}</span>
            </button>
          {/each}
        </div>
      {/if}

      {#if recent.length > 0}
        <div class="conv-group">
          <div class="conv-group-label">Recent</div>
          {#each recent.slice(0, 30) as conv (conv.id)}
            <button
              class="conv-item"
              class:active={$chatStore.activeId === conv.id}
              onclick={() => chatStore.setActive(conv.id)}
              onkeydown={e => handleKeyDelete(e, conv.id)}
            >
              <span class="conv-dot"></span>
              <span class="conv-title truncate">{conv.title}</span>
              <span class="conv-time">{formatTime(conv.updatedAt)}</span>
            </button>
          {/each}
        </div>
      {/if}

      {#if conversations.length === 0}
        <div class="conv-empty">
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.3">
            <path d="M21 15a2 2 0 01-2 2H7l-4 4V5a2 2 0 012-2h14a2 2 0 012 2z"/>
          </svg>
          <p>No conversations yet</p>
        </div>
      {/if}
    </div>
  {/if}

  <!-- Bottom Nav -->
  <div class="sidebar-footer">
    {#if !collapsed}
      <button class="footer-btn" id="sidebar-settings-btn" onclick={onSettingsOpen}>
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.07 4.93l-1.41 1.41M4.93 4.93l1.41 1.41M12 2v2M12 20v2M2 12h2M20 12h2M19.07 19.07l-1.41-1.41M4.93 19.07l1.41-1.41"/>
        </svg>
        Settings
      </button>
      <button class="footer-btn" id="sidebar-clear-btn" onclick={() => chatStore.clear()}>
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="3 6 5 6 21 6"/>
          <path d="M19 6l-1 14H6L5 6"/>
          <path d="M10 11v6M14 11v6"/>
        </svg>
        Clear All
      </button>
    {:else}
      <button class="icon-btn" id="sidebar-settings-icon-btn" title="Settings" onclick={onSettingsOpen}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.07 4.93l-1.41 1.41M4.93 4.93l1.41 1.41M12 2v2M12 20v2M2 12h2M20 12h2"/>
        </svg>
      </button>
    {/if}
  </div>
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    width: var(--sidebar-width);
    min-width: var(--sidebar-width);
    height: 100vh;
    background: var(--bg-sidebar);
    border-right: 1px solid var(--border-subtle);
    transition: width var(--transition-slow), min-width var(--transition-slow);
    overflow: hidden;
    position: relative;
    z-index: 20;
  }

  .sidebar.collapsed {
    width: 56px;
    min-width: 56px;
  }

  /* Header */
  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 12px 10px;
    border-bottom: 1px solid var(--border-subtle);
    gap: 8px;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
    overflow: hidden;
    flex: 1;
  }

  .brand-icon {
    font-size: 1.2rem;
    color: var(--accent-primary);
    flex-shrink: 0;
    filter: drop-shadow(0 0 6px var(--accent-glow));
  }

  .brand-name {
    font-weight: 700;
    font-size: 0.95rem;
    color: var(--text-primary);
    letter-spacing: -0.01em;
    white-space: nowrap;
  }

  .brand-badge {
    font-size: 0.6rem;
    font-weight: 700;
    background: var(--accent-dim);
    color: var(--accent-primary);
    padding: 1px 5px;
    border-radius: 99px;
    border: 1px solid var(--accent-primary);
    letter-spacing: 0.05em;
    flex-shrink: 0;
  }

  .collapse-btn {
    flex-shrink: 0;
  }

  /* New Chat */
  .sidebar-actions {
    padding: 10px 10px 6px;
  }

  .new-chat-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 10px;
    background: var(--accent-dim);
    color: var(--accent-primary);
    border: 1px solid rgba(32, 217, 210, 0.2);
    border-radius: var(--border-radius-sm);
    font-size: var(--font-size-sm);
    font-weight: 600;
    cursor: pointer;
    transition: background var(--transition-fast), box-shadow var(--transition-fast);
    white-space: nowrap;
    font-family: var(--font-sans);
  }

  .new-chat-btn:hover {
    background: rgba(32, 217, 210, 0.18);
    box-shadow: 0 0 12px var(--accent-glow);
  }

  /* Mode Selector */
  .mode-tabs {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 4px;
    padding: 10px;
    background: var(--bg-surface);
    margin: 8px 10px;
    border-radius: var(--border-radius-sm);
    border: 1px solid var(--border-subtle);
  }

  .mode-tab {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 4px;
    padding: 8px 4px;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all 0.2s;
  }

  .mode-tab:hover {
    background: var(--bg-elevated);
    color: var(--text-secondary);
  }

  .mode-tab.active {
    background: var(--bg-app);
    color: var(--accent-primary);
    box-shadow: 0 2px 8px rgba(0,0,0,0.2);
    border: 1px solid var(--border-subtle);
  }

  .mode-icon { font-size: 1.1rem; }
  .mode-label { font-size: 10px; font-weight: 700; text-transform: uppercase; }

  /* Conversation List */
  .conv-list {
    flex: 1;
    overflow-y: auto;
    padding: 6px 6px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .conv-group {
    margin-bottom: 8px;
  }

  .conv-group-label {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: var(--font-size-xs);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--text-tertiary);
    padding: 4px 8px 6px;
  }

  .conv-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 7px 8px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    border-radius: var(--border-radius-sm);
    cursor: pointer;
    text-align: left;
    font-size: var(--font-size-sm);
    font-family: var(--font-sans);
    transition: background var(--transition-fast), color var(--transition-fast);
    position: relative;
  }

  .conv-item:hover {
    background: var(--bg-elevated);
    color: var(--text-primary);
  }

  .conv-item.active {
    background: var(--bg-overlay);
    color: var(--text-primary);
  }

  .conv-item.active::before {
    content: '';
    position: absolute;
    left: 0;
    top: 20%;
    height: 60%;
    width: 2px;
    border-radius: 99px;
    background: var(--accent-primary);
  }

  .conv-icon { font-size: 0.85rem; flex-shrink: 0; }

  .conv-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: var(--text-tertiary);
    flex-shrink: 0;
  }

  .conv-title {
    flex: 1;
    min-width: 0;
    line-height: 1.3;
  }

  .conv-time {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
    flex-shrink: 0;
  }

  .conv-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    padding: 40px 20px;
    color: var(--text-tertiary);
    font-size: var(--font-size-sm);
    text-align: center;
  }

  /* Footer */
  .sidebar-footer {
    padding: 10px;
    border-top: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .footer-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 7px 8px;
    border: none;
    background: transparent;
    color: var(--text-tertiary);
    border-radius: var(--border-radius-sm);
    cursor: pointer;
    font-size: var(--font-size-sm);
    font-family: var(--font-sans);
    transition: background var(--transition-fast), color var(--transition-fast);
    text-align: left;
  }

  .footer-btn:hover {
    background: var(--bg-elevated);
    color: var(--text-secondary);
  }

  /* Icon button base */
  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    background: transparent;
    color: var(--text-tertiary);
    border-radius: var(--border-radius-sm);
    cursor: pointer;
    transition: background var(--transition-fast), color var(--transition-fast);
    flex-shrink: 0;
  }

  .icon-btn:hover {
    background: var(--bg-elevated);
    color: var(--text-secondary);
  }
</style>
