<script lang="ts">
  import { profileStore, type UserProfile } from '$lib/stores/profile';
  import { onMount } from 'svelte';

  let isOpen = $state(false);
  let profiles = $derived($profileStore.profiles);
  let activeId = $derived($profileStore.activeId);
  let activeProfile = $derived(profiles.find(p => p.id === activeId));

  function toggleDropdown() {
    isOpen = !isOpen;
  }

  function handleSelect(id: string) {
    profileStore.setActive(id);
    isOpen = false;
  }

  // Handle outside click
  function handleOutsideClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest('.profile-switcher')) {
      isOpen = false;
    }
  }

  onMount(() => {
    window.addEventListener('click', handleOutsideClick);
    profileStore.load(); // Load profiles on mount
    return () => window.removeEventListener('click', handleOutsideClick);
  });
</script>

<div class="profile-switcher">
  <button class="active-profile-btn" onclick={toggleDropdown} aria-expanded={isOpen}>
    <div class="profile-info">
      <span class="profile-avatar">{activeProfile?.avatar || '👤'}</span>
      <span class="profile-name truncate">{activeProfile?.name || 'Default User'}</span>
    </div>
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="chevron" class:rotated={isOpen}>
      <path d="M6 9l6 6 6-6"/>
    </svg>
  </button>

  {#if isOpen}
    <div class="profile-dropdown">
      <div class="dropdown-label">Switch Profile</div>
      <div class="profiles-list">
        {#each profiles as p}
          <button 
            class="profile-item" 
            class:active={p.id === activeId}
            onclick={() => handleSelect(p.id)}
          >
            <span class="item-avatar">{p.avatar}</span>
            <span class="item-name truncate">{p.name}</span>
            {#if p.id === activeId}
              <span class="current-tag">active</span>
            {/if}
          </button>
        {/each}
      </div>
      <div class="dropdown-footer">
        <button class="add-profile-btn">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <path d="M12 5v14M5 12h14"/>
          </svg>
          Create New Profile
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .profile-switcher {
    position: relative;
    padding: 10px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .active-profile-btn {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 8px 10px;
    background: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: var(--border-radius-sm);
    cursor: pointer;
    transition: all 0.2s;
  }

  .active-profile-btn:hover {
    background: var(--bg-elevated);
    border-color: var(--accent-primary);
  }

  .profile-info {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    min-width: 0;
  }

  .profile-avatar {
    font-size: 1.1rem;
    flex-shrink: 0;
  }

  .profile-name {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-primary);
  }

  .chevron {
    color: var(--text-tertiary);
    transition: transform 0.2s;
  }

  .chevron.rotated {
    transform: rotate(180deg);
  }

  .profile-dropdown {
    position: absolute;
    top: calc(100% - 2px);
    left: 10px;
    right: 10px;
    background: var(--bg-app);
    border: 1px solid var(--border-default);
    border-radius: 0 0 var(--border-radius-sm) var(--border-radius-sm);
    box-shadow: 0 12px 24px rgba(0,0,0,0.4);
    z-index: 50;
    overflow: hidden;
    animation: slideDown 0.2s ease;
  }

  @keyframes slideDown {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .dropdown-label {
    padding: 10px 12px 6px;
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    color: var(--text-tertiary);
    letter-spacing: 0.05em;
  }

  .profiles-list {
    padding: 4px;
  }

  .profile-item {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 8px 10px;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s;
    text-align: left;
  }

  .profile-item:hover {
    background: var(--bg-elevated);
    color: var(--text-primary);
  }

  .profile-item.active {
    background: var(--accent-dim);
    border: 1px solid rgba(32, 217, 210, 0.2);
    color: var(--accent-primary);
  }

  .item-avatar { font-size: 1.1rem; }
  .item-name { flex: 1; font-weight: 500; font-size: var(--font-size-sm); }
  
  .current-tag {
    font-size: 9px;
    font-weight: 700;
    text-transform: uppercase;
    background: var(--accent-primary);
    color: #000;
    padding: 1px 4px;
    border-radius: 3px;
    letter-spacing: 0.02em;
  }

  .dropdown-footer {
    padding: 6px;
    border-top: 1px solid var(--border-subtle);
  }

  .add-profile-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px;
    background: transparent;
    border: 1px dashed var(--border-default);
    border-radius: 4px;
    color: var(--text-tertiary);
    font-size: var(--font-size-xs);
    cursor: pointer;
    transition: all 0.2s;
  }

  .add-profile-btn:hover {
    background: var(--bg-elevated);
    border-color: var(--text-tertiary);
    color: var(--text-secondary);
  }
</style>
