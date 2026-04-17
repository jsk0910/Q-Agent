<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { projectStore, parseId } from '$lib/stores/projectStore';

  let projects = $derived($projectStore.projects);
  let activeId = $derived($projectStore.activeProjectId);
  let activeProject = $derived(projects.find(p => parseId(p.id) === activeId));

  let isOpen = $state(false);
  let isCreating = $state(false);
  
  let newName = $state('');
  let newDesc = $state('');

  onMount(() => {
    projectStore.loadProjects();
    const closeMenu = (e: MouseEvent) => {
      if (!(e.target as Element).closest('.project-switcher')) {
        isOpen = false;
        isCreating = false;
      }
    };
    window.addEventListener('click', closeMenu);
    return () => window.removeEventListener('click', closeMenu);
  });

  function toggle() {
    isOpen = !isOpen;
    if (!isOpen) isCreating = false;
  }

  function select(id: string | null) {
    projectStore.setActive(id);
    isOpen = false;
  }

  async function create() {
    if (!newName.trim()) return;
    try {
      await projectStore.createProject(newName.trim(), newDesc.trim());
      newName = '';
      newDesc = '';
      isCreating = false;
      isOpen = false;
    } catch(e) {
      alert("Failed to create project: " + JSON.stringify(e));
    }
  }
  
  function getId(p: any): string {
    return parseId(p.id);
  }
</script>

<div class="project-switcher">
  <button class="switcher-trigger" onclick={toggle}>
    <div class="trigger-info">
      <span class="switcher-label">WORKSPACE</span>
      <span class="active-name truncate">{activeProject ? activeProject.name : 'Personal (Global)'}</span>
    </div>
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="chevron" class:rotated={isOpen}>
      <path d="M6 9l6 6 6-6"/>
    </svg>
  </button>

  {#if isOpen}
    <div class="switcher-dropdown">
      {#if isCreating}
        <div class="create-form">
          <h4>New Project</h4>
          <input type="text" placeholder="Project Name" bind:value={newName} autofocus />
          <input type="text" placeholder="Description (Optional)" bind:value={newDesc} />
          <div class="form-actions">
            <button class="btn-cancel" onclick={() => isCreating = false}>Cancel</button>
            <button class="btn-submit" onclick={create}>Create</button>
          </div>
        </div>
      {:else}
        <button class="project-item" class:active={!activeId} onclick={() => select(null)}>
          <span class="proj-icon">🌍</span>
          <div class="proj-info">
            <span class="name">Personal (Global)</span>
            <span class="desc">Unisolated workspace</span>
          </div>
        </button>

        <div class="divider"></div>

        <div class="project-list">
          {#each projects as p}
            <button class="project-item" class:active={getId(p) === activeId} onclick={() => select(getId(p))}>
              <span class="proj-icon">📁</span>
              <div class="proj-info">
                <span class="name">{p.name}</span>
                {#if p.description}
                  <span class="desc truncate">{p.description}</span>
                {/if}
              </div>
            </button>
          {/each}
        </div>

        <div class="dropdown-footer">
          <button class="create-btn" onclick={() => isCreating = true}>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <path d="M12 5v14M5 12h14"/>
            </svg>
            Create New Project
          </button>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .project-switcher {
    position: relative;
    margin: 12px 10px;
  }

  .switcher-trigger {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 8px 12px;
    background: rgba(255,255,255,0.03);
    border: 1px solid var(--border-default);
    border-radius: var(--border-radius-md);
    cursor: pointer;
    text-align: left;
    transition: all var(--transition-fast);
  }

  .switcher-trigger:hover {
    background: var(--bg-elevated);
    border-color: var(--accent-primary);
  }

  .trigger-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
    min-width: 0;
  }

  .switcher-label {
    font-size: 9px;
    font-weight: 700;
    color: var(--text-tertiary);
    letter-spacing: 0.05em;
  }

  .active-name {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-primary);
  }

  .chevron {
    color: var(--text-tertiary);
    transition: transform 0.2s;
    flex-shrink: 0;
  }

  .chevron.rotated {
    transform: rotate(180deg);
  }

  .switcher-dropdown {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    width: 260px;
    background: var(--bg-surface);
    border: 1px solid var(--border-default);
    border-radius: var(--border-radius-lg);
    box-shadow: 0 4px 24px rgba(0,0,0,0.4);
    z-index: 100;
    padding: 6px 0;
    animation: slideDown 0.15s ease-out;
  }

  @keyframes slideDown {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .project-list {
    max-height: 250px;
    overflow-y: auto;
  }

  .project-item {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 10px 14px;
    background: transparent;
    border: none;
    text-align: left;
    cursor: pointer;
    transition: background 0.15s;
  }

  .project-item:hover {
    background: var(--bg-elevated);
  }

  .project-item.active {
    background: var(--bg-overlay);
  }

  .proj-icon {
    font-size: 1.1rem;
    flex-shrink: 0;
  }

  .proj-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .name {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-primary);
  }

  .desc {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
  }

  .divider {
    height: 1px;
    background: var(--border-subtle);
    margin: 4px 0;
  }

  .dropdown-footer {
    padding: 6px 10px;
    border-top: 1px solid var(--border-subtle);
    margin-top: 4px;
  }

  .create-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    width: 100%;
    padding: 8px;
    background: transparent;
    border: 1px dashed var(--border-default);
    border-radius: var(--border-radius-md);
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .create-btn:hover {
    background: var(--bg-elevated);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .create-form {
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .create-form h4 {
    margin: 0;
    font-size: var(--font-size-sm);
    color: var(--text-primary);
  }

  .create-form input {
    width: 100%;
    padding: 8px 10px;
    background: var(--bg-input);
    border: 1px solid var(--border-default);
    border-radius: 4px;
    color: var(--text-primary);
    font-size: var(--font-size-sm);
  }

  .create-form input:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .form-actions {
    display: flex;
    gap: 8px;
    margin-top: 4px;
  }

  .btn-submit, .btn-cancel {
    flex: 1;
    padding: 6px;
    border-radius: 4px;
    font-size: var(--font-size-xs);
    font-weight: 600;
    cursor: pointer;
    border: none;
  }

  .btn-submit {
    background: var(--accent-primary);
    color: #000;
  }

  .btn-cancel {
    background: var(--bg-elevated);
    color: var(--text-secondary);
  }
</style>
