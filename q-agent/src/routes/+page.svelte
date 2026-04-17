<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import { marked } from 'marked';
  import hljs from 'highlight.js';
  import 'highlight.js/styles/github-dark.css';
  import { onMount, tick } from 'svelte';
  import { settings, chatStore, activeConversation, effectiveSettings } from '$lib/stores/ui';
  import type { Message } from '$lib/stores/ui';
  import { modeStore } from '$lib/stores/mode';
  import { profileStore } from '$lib/stores/profile';
  import { projectStore } from '$lib/stores/projectStore';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import WidgetPanel from '$lib/components/WidgetPanel.svelte';
  import ModelPicker from '$lib/components/ModelPicker.svelte';
  import ModelInstallModal from '$lib/components/ModelInstallModal.svelte';
  import ModelExplorerModal from '$lib/components/ModelExplorerModal.svelte';
  import ChatBubble from '$lib/components/ChatBubble.svelte';

  // ─── Marked setup ────────────────────────────────────────────────────────
  const renderer = new marked.Renderer();

  renderer.code = ({ text, lang }) => {
    const language = lang && hljs.getLanguage(lang) ? lang : 'plaintext';
    const highlighted = hljs.highlight(text, {
      language,
    }).value;
    const langLabel = language !== 'plaintext' ? language : '';
    return `<div class="code-block">
      <div class="code-header">
        <span class="code-lang">${langLabel}</span>
        <button class="copy-btn" onclick="navigator.clipboard.writeText(this.closest('.code-block').querySelector('code').textContent)">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
          </svg>
          Copy
        </button>
      </div>
      <pre><code class="hljs language-${language}">${highlighted}</code></pre>
    </div>`;
  };

  marked.setOptions({ renderer });

  // ─── State ───────────────────────────────────────────────────────────────
  let prompt = $state('');
  let isLoading = $state(false);
  let widgetPanelOpen = $state(false);
  let widgetActiveTab = $state<'sources' | 'settings' | 'references'>('sources');
  let chatEndEl = $state<HTMLDivElement>();
  let textareaEl = $state<HTMLTextAreaElement>();
  let installModel = $state<string | null>(null);
  let exploreModalOpen = $state(false);
  let evalStatus = $state('');
  let currentSummary = $state<string | null>(null);

  let installedModels = $state<string[]>([]);

  let currentConv = $derived($activeConversation);
  let messages = $derived(currentConv?.messages ?? []);
  let layoutMode = $derived(widgetActiveTab === 'settings' ? 'overlay' : 'integrated');

  // Auto-scroll on new message
  $effect(() => {
    if (messages.length > 0 && chatEndEl) {
      chatEndEl.scrollIntoView({ behavior: 'smooth' });
    }
  });

  // Auto-resize textarea
  function autoResize() {
    if (!textareaEl) return;
    textareaEl.style.height = 'auto';
    const max = 200;
    textareaEl.style.height = Math.min(textareaEl.scrollHeight, max) + 'px';
  }

  // ─── Parse Qwen Thinking ─────────────────────────────────────────────────
  function parseResponse(raw: string): { thinking: string | null; answer: string } {
    const match = raw.match(/<think>([\s\S]*?)<\/think>/i);
    if (match) {
      return {
        thinking: match[1].trim(),
        answer: raw.replace(/<think>[\s\S]*?<\/think>/i, '').trim(),
      };
    }
    // Handle unclosed thinking block during streaming
    if (raw.includes("<think>")) {
       const thinkingContent = raw.replace(/<think>/i, '').trim();
       return { thinking: thinkingContent, answer: "" };
    }

    return { thinking: null, answer: raw };
  }

  /**
   * Replaces [Source N] with stylized interactive chips.
   */
  async function renderMd(text: string): Promise<string> {
    const rawHtml = (await marked.parse(text)) as string;
    
    // Replace [Source N] or [Web Source N] with a clickable span
    // Pattern: [Source 1], [Web Source 1], etc.
    const citePattern = /\[(Source\s+\d+|Web\s+Source\s+\d+)\]/g;
    return rawHtml.replace(citePattern, (match, p1) => {
      const label = p1;
      return `<button class="cite-chip" data-cite="${label}" onclick="this.dispatchEvent(new CustomEvent('cite-click', { bubbles: true, detail: '${label}' }))">${label}</button>`;
    });
  }

  function openWidgets(tab: 'sources' | 'settings' | 'references' = 'sources') {
    widgetActiveTab = tab;
    widgetPanelOpen = true;
  }

  // ─── Send Message ─────────────────────────────────────────────────────────
  async function send() {
    const text = prompt.trim();
    if (!text || isLoading) return;

    // Ensure active conversation
    let convId = $chatStore.activeId;
    if (!convId) convId = chatStore.newConversation();

    // Compile history for Rust (exclude the empty assistant placeholder we are about to add)
    // Send raw contents (re-attach <think> if needed, but normally Qwen handles just the answers fine.
    // For safety, if thinking exists, we can prepend it or just send content)
    const historyToSend = messages.map((m: Message) => ({
      role: m.role,
      content: m.thinking ? `<think>\n${m.thinking}\n</think>\n${m.content}` : m.content
    }));
    
    // Append the new user message
    historyToSend.push({ role: 'user', content: text });

    // User message (UI update)
    const userMsg: Message = {
      id: crypto.randomUUID(),
      role: 'user',
      content: text,
      timestamp: new Date(),
    };
    chatStore.addMessage(convId, userMsg);

    // Auto-title first message
    if (messages.length === 0) {
      chatStore.renameConversation(
        convId,
        text.length > 44 ? text.slice(0, 44) + '…' : text
      );
    }

    prompt = '';
    if (textareaEl) { textareaEl.style.height = 'auto'; }

    // Assistant placeholder
    const asstId = crypto.randomUUID();
    const asstPlaceholder: Message = {
      id: asstId,
      role: 'assistant',
      content: '',
      thinking: undefined,
      timestamp: new Date(),
      model: $effectiveSettings.modelName,
    };
    chatStore.addMessage(convId, asstPlaceholder);

    isLoading = true;
    await tick();

    // -- Agent Routing Step -- 
    chatStore.updateMessage(convId, asstId, { harnessStatus: "Agent is gathering sources..." });
    
    let contextStr = "";
    try {
      contextStr = await invoke('intercept_and_route', { 
        window,
        prompt: text, 
        projectId: $projectStore.activeProjectId || null 
      });
    } catch(e) {
      console.error("Routing error:", e);
    }
    
    chatStore.updateMessage(convId, asstId, { harnessStatus: "" });
    const finalSystemPrompt = $effectiveSettings.systemPrompt + (contextStr ? "\n\n" + contextStr : "");

    let unlistenToken: UnlistenFn | undefined;
    let unlistenHarnessStatus: UnlistenFn | undefined;
    let unlistenHarnessPlan: UnlistenFn | undefined;
    let unlistenHarnessResults: UnlistenFn | undefined;
    let unlistenHarnessSummary: UnlistenFn | undefined;

    let streamRaw = "";

    try {
      unlistenHarnessStatus = await listen<string>('harness_status', (event) => {
        chatStore.updateMessage(convId, asstId, { harnessStatus: event.payload });
      });
      unlistenHarnessPlan = await listen<any>('harness_plan_created', (event) => {
        chatStore.updateMessage(convId, asstId, { harnessPlan: event.payload });
      });
      unlistenHarnessResults = await listen<string[]>('harness_results_updated', (event) => {
        chatStore.updateMessage(convId, asstId, { harnessResults: event.payload });
      });
      unlistenHarnessSummary = await listen<string | null>('harness_summary_updated', (event) => {
        currentSummary = event.payload;
      });

      if ($settings.streamTokens) {
        unlistenToken = await listen<string>('chat_token', (event) => {
          streamRaw += event.payload;
          const { thinking, answer } = parseResponse(streamRaw);
          chatStore.updateMessage(convId, asstId, {
            content: answer,
            thinking: thinking ?? undefined,
          });
          // Auto-expand thinking when it starts
          if (thinking && !expandedThinking.has(asstId)) {
             toggleThinking(asstId);
          }
        });

        // We could listen to chat_finished to toggle isLoading, but invoke() will resolve anyway
      }

      const raw: string = await invoke('chat_with_ollama', {
        messages: historyToSend,
        model: $effectiveSettings.modelName,
        temperature: $settings.temperature,
        systemPrompt: finalSystemPrompt,
        stream: $settings.streamTokens,
        auto_plan: $settings.autoPlanning,
        summarize_threshold: $settings.summarizationThreshold,
        summary: currentSummary
      });

      // Cleanup listeners
      if (unlistenToken) unlistenToken();
      if (unlistenHarnessStatus) unlistenHarnessStatus();
      if (unlistenHarnessPlan) unlistenHarnessPlan();
      if (unlistenHarnessResults) unlistenHarnessResults();
      if (unlistenHarnessSummary) unlistenHarnessSummary();

      // Final reconciliation
      const finalRaw = $settings.streamTokens ? streamRaw : raw;
      const { thinking, answer } = parseResponse(finalRaw);
      chatStore.updateMessage(convId, asstId, {
        content: answer,
        thinking: thinking ?? undefined,
      });

      // ── Eval Loop (Critic Pass) ──────────────────────────────────────
      if ($settings.enableEvalLoop && answer.trim().length > 0) {
        evalStatus = 'Initializing verification...';
        chatStore.updateMessage(convId, asstId, {
          content: answer + `\n\n*🧪 Initializing verification...*`,
        });

        const unlistenEval = await listen<{iteration: number, max_iterations: number, status: string}>('eval_progress', (event) => {
          evalStatus = event.payload.status;
          chatStore.updateMessage(convId, asstId, {
            content: answer + `\n\n*🧪 ${evalStatus}*`,
          });
        });

        try {
          const verified: string = await invoke('evaluate_response', {
            draft: answer,
            prompt: text,
            model: $effectiveSettings.modelName,
          });
          chatStore.updateMessage(convId, asstId, { content: verified });
        } catch (evalErr) {
          console.warn('Eval loop failed, keeping original answer:', evalErr);
          chatStore.updateMessage(convId, asstId, { content: answer });
        } finally {
          unlistenEval();
          evalStatus = '';
        }
      }
      
    } catch (err) {
      if (unlistenToken) unlistenToken();
      
      chatStore.updateMessage(convId, asstId, {
        content: `⚠️ Error: ${err}`,
      });
    } finally {
      isLoading = false;
    }
  }

  async function stopAction() {
    try {
      await invoke('stop_generation');
      isLoading = false;
    } catch (e) {
      console.error('Stop failed:', e);
    }
  }

  // ─── Export Chat ────────────────────────────────────────────────────────
  async function exportChat(format: 'md' | 'html' = 'md') {
    if (!currentConv) return;
    
    let content = '';
    let mime = '';
    let ext = '';

    if (format === 'md') {
      content = `# ${currentConv.title}\n\n`;
      currentConv.messages.forEach((m: Message) => {
        content += `### ${m.role === 'user' ? 'User' : 'Assistant'}\n`;
        if (m.thinking && $settings.showThinking) {
          content += `<details><summary>Thinking process</summary>\n\n${m.thinking}\n</details>\n\n`;
        }
        content += `${m.content}\n\n---\n\n`;
      });
      mime = 'text/markdown';
      ext = 'md';
    } else {
      let htmlBody = `<h1>${currentConv.title}</h1>`;
      for (const m of currentConv.messages) {
        htmlBody += `<h3>${m.role === 'user' ? 'User' : 'Assistant'}</h3>`;
        if (m.thinking && $settings.showThinking) {
          htmlBody += `<details><summary>Thinking process</summary><pre style="white-space:pre-wrap;background:#f4f4f4;padding:1em;border-radius:4px;">${m.thinking}</pre></details>`;
        }
        const renderedObj = await marked.parse(m.content);
        htmlBody += `<div>${renderedObj}</div><hr/>`;
      }
      content = `<!DOCTYPE html><html><head><meta charset="utf-8"><title>${currentConv.title}</title><style>body{font-family:sans-serif;max-width:800px;margin:2em auto;line-height:1.6;color:#333;padding:0 20px;} h1{border-bottom:1px solid #ccc;padding-bottom:10px;} hr{border:0;border-top:1px solid #eee;margin:2em 0;}</style></head><body>${htmlBody}</body></html>`;
      mime = 'text/html';
      ext = 'html';
    }

    const blob = new Blob([content], { type: mime });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${currentConv.title.replace(/[^a-zA-Z0-9\s]/g, '_').trim()}.${ext}`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  // ─── Keyboard & Tokens ──────────────────────────────────────────────────
  function handleKeydown(e: KeyboardEvent) {
    if ($settings.sendOnEnter && e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  }

  function handleGlobalKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && isLoading) {
      e.preventDefault();
      stopAction();
      return;
    }
    if (e.ctrlKey || e.metaKey) {
      if (e.key === 'n') {
        e.preventDefault();
        chatStore.newConversation();
      }
      if (e.key === ',') {
        e.preventDefault();
        openWidgets('settings');
      }
    }
  }

  let totalTokens = $derived(messages.reduce((acc: number, m: Message) => 
    acc + Math.ceil(m.content.length / 4) + (m.thinking ? Math.ceil(m.thinking.length / 4) : 0)
  , 0));

  function formatTimestamp(date: Date) {
    return new Date(date).toLocaleTimeString('ko-KR', {
      hour: '2-digit', minute: '2-digit',
    });
  }

  onMount(() => {
    // Start with a fresh conversation
    if (!$chatStore.activeId) chatStore.newConversation();
    refreshInstalledModels();
  });

  async function refreshInstalledModels() {
    try {
      installedModels = await invoke<string[]>('list_models');
    } catch (e) {
      console.error('Failed to list models', e);
    }
  }

  // Thinking toggle per message
  let expandedThinking = $state(new Set<string>());

  function toggleThinking(id: string) {
    if (expandedThinking.has(id)) {
      expandedThinking.delete(id);
    } else {
      expandedThinking.add(id);
    }
    // In Svelte 5, Set mutation is generally reactive if wrapped in $state
    // but we can re-assign to be absolutely sure if needed, 
    // though Svelte 5 usually tracks the Set methods.
  }
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<div class="app-shell">
  <Sidebar onSettingsOpen={() => openWidgets('settings')} />
  <!-- WidgetPanel will now render at the end of the shell for integrated mode -->

  <!-- Main -->
  <main class="main">
    <!-- Top bar -->
    <header class="topbar">
      <div class="topbar-left">
        {#if messages.length > 0}
          <span class="token-chip" title="Estimated Tokens">
            {totalTokens} tokens
          </span>
        {/if}
        {#if currentConv}
          <span class="conv-title-small truncate">{currentConv.title}</span>
        {/if}
      </div>

      <div class="topbar-right">
        <button
          class="topbar-btn"
          id="new-chat-topbar-btn"
          title="New Chat"
          onclick={() => chatStore.newConversation()}
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <path d="M12 5v14M5 12h14"/>
          </svg>
          New Chat
        </button>
        <button
          class="topbar-icon-btn"
          id="topbar-export-btn"
          title="Export Conversation (MD)"
          onclick={() => exportChat('md')}
          disabled={!currentConv || messages.length === 0}
        >
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
            <polyline points="7 10 12 15 17 10"></polyline>
            <line x1="12" y1="15" x2="12" y2="3"></line>
          </svg>
        </button>
        <button
          class="topbar-icon-btn"
          title="Export Conversation (HTML)"
          onclick={() => exportChat('html')}
          disabled={!currentConv || messages.length === 0}
        >
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
            <polyline points="14 2 14 8 20 8"></polyline>
            <line x1="16" y1="13" x2="8" y2="13"></line>
            <line x1="16" y1="17" x2="8" y2="17"></line>
            <polyline points="10 9 9 9 8 9"></polyline>
          </svg>
        </button>
        <button
          class="topbar-icon-btn"
          id="topbar-knowledge-btn"
          title="Notebook Sources"
          onclick={() => openWidgets('sources')}
        >
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
            <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
          </svg>
        </button>
        <button
          class="topbar-icon-btn"
          id="topbar-settings-btn"
          title="Settings (Ctrl+,)"
          onclick={() => openWidgets('settings')}
        >
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.07 4.93l-1.41 1.41M4.93 4.93l1.41 1.41M12 2v2M12 20v2M2 12h2M20 12h2M19.07 19.07l-1.41-1.41M4.93 19.07l1.41-1.41"/>
          </svg>
        </button>
      </div>
    </header>

    <!-- Chat area -->
    <div class="chat-area" id="chat-area">
      {#if messages.length === 0}
        <!-- Welcome screen -->
        <div class="welcome">
          <div class="welcome-icon">⬡</div>
          <h1 class="welcome-title">How can I help you?</h1>
          <p class="welcome-sub">
            Powered by <strong>{$effectiveSettings.modelName}</strong> — Ask anything, research papers, write code.
          </p>
          <div class="quick-prompts">
            {#each [
              { icon: '📄', label: 'Summarize a paper', prompt: 'Summarize this research paper for me:' },
              { icon: '💻', label: 'Write code', prompt: 'Write a Rust function that:' },
              { icon: '🔍', label: 'Explain concept', prompt: 'Explain the concept of:' },
              { icon: '🐛', label: 'Debug error', prompt: 'Help me debug this error:\n```\n\n```' },
            ] as q}
              <button
                class="quick-btn"
                onclick={() => { prompt = q.prompt; textareaEl?.focus(); }}
              >
                <span class="quick-icon">{q.icon}</span>
                <span class="quick-label">{q.label}</span>
              </button>
            {/each}
          </div>
        </div>
      {:else}
        <div class="messages-list" oncite-click={(e) => {
          // Store the citation ID for highlighting
          const citationId = e.detail;
          widgetActiveTab = 'sources';
          widgetPanelOpen = true;
          // Trigger a custom store or event to scroll and highlight (we'll implement this store next)
          import('$lib/stores/ui').then(({ activeCitation }) => {
            activeCitation.set(citationId);
          });
        }}>
          {#each messages as msg, i (msg.id)}
            <ChatBubble 
              {msg}
              isLast={i === messages.length - 1}
              {isLoading}
              showThinking={$settings.showThinking}
              {expandedThinking}
              onToggleThinking={toggleThinking}
              {formatTimestamp}
              {renderMd}
            />
          {/each}
          <div bind:this={chatEndEl}></div>
        </div>
      {/if}
    </div>

    <!-- Input bar -->
    <div class="input-zone">
      <div class="input-bar" class:focused={false} id="input-bar">
        <div class="model-picker-wrapper">
          <ModelPicker 
            onrequestInstall={m => installModel = m} 
            onopenExplorer={() => exploreModalOpen = true}
          />
        </div>
        <textarea
          id="chat-input"
          bind:this={textareaEl}
          bind:value={prompt}
          oninput={autoResize}
          onkeydown={handleKeydown}
          placeholder="Ask anything… (Shift+Enter for newline)"
          rows="1"
          disabled={isLoading}
          aria-label="Chat input"
        ></textarea>
        <div class="input-actions">
          <span class="hint-text">
            {isLoading ? 'Esc to Stop' : ($settings.sendOnEnter ? '↵ Send' : 'Ctrl+↵ Send')}
          </span>
          {#if isLoading}
            <button
              class="stop-btn"
              id="stop-btn"
              onclick={stopAction}
              aria-label="Stop generation"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                <rect x="6" y="6" width="12" height="12" rx="2"/>
              </svg>
            </button>
          {:else}
            <button
              class="send-btn"
              id="send-btn"
              onclick={send}
              disabled={!prompt.trim()}
              aria-label="Send message"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                <path d="M22 2L11 13M22 2L15 22l-4-9-9-4 20-7z"/>
              </svg>
            </button>
          {/if}
        </div>
      </div>
      <p class="disclaimer">
        Q-Agent may produce inaccurate results. Always verify critical information.
      </p>
    </div>
  </main>

  <WidgetPanel 
    open={widgetPanelOpen} 
    activeTab={widgetActiveTab} 
    mode={layoutMode}
    onclose={() => widgetPanelOpen = false} 
  />

  {#if installModel}
    <ModelInstallModal 
      model={installModel} 
      onclose={() => installModel = null}
      onsuccess={(m) => {
        profileStore.setModelForMode($modeStore, m);
        installModel = null;
        refreshInstalledModels();
      }}
    />
  {/if}

  {#if exploreModalOpen}
    <ModelExplorerModal 
      {installedModels}
      onclose={() => exploreModalOpen = false}
      onrequestInstall={(m) => {
        exploreModalOpen = false;
        installModel = m;
      }}
    />
  {/if}
</div>

<style>
  /* ─── Shell layout ─────────────────────────────────────────────── */
  .app-shell {
    display: flex;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background: var(--bg-app);
  }

  .main {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    min-width: 0;
  }

  /* ─── Top bar ──────────────────────────────────────────────────── */
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: var(--topbar-height);
    padding: 0 16px;
    border-bottom: 1px solid var(--border-subtle);
    background: var(--bg-app);
    flex-shrink: 0;
    gap: 12px;
  }

  .topbar-left {
    display: flex;
    align-items: center;
    gap: 10px;
    overflow: hidden;
    flex: 1;
  }

  .model-chip {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--accent-primary);
    background: var(--accent-dim);
    padding: 3px 9px;
    border-radius: 99px;
    border: 1px solid rgba(32, 217, 210, 0.2);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .conv-title-small {
    font-size: var(--font-size-sm);
    color: var(--text-tertiary);
    max-width: 300px;
  }

  .token-chip {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
    background: var(--bg-overlay);
    padding: 3px 9px;
    border-radius: 99px;
    border: 1px solid var(--border-subtle);
  }

  .topbar-right {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .topbar-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: var(--bg-elevated);
    border: 1px solid var(--border-default);
    color: var(--text-secondary);
    border-radius: var(--border-radius-sm);
    font-size: var(--font-size-sm);
    font-weight: 500;
    cursor: pointer;
    font-family: var(--font-sans);
    transition: background var(--transition-fast), color var(--transition-fast);
  }

  .topbar-btn:hover {
    background: var(--bg-overlay);
    color: var(--text-primary);
  }

  .topbar-icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px; height: 32px;
    background: var(--bg-elevated);
    border: 1px solid var(--border-default);
    color: var(--text-secondary);
    border-radius: var(--border-radius-sm);
    cursor: pointer;
    transition: background var(--transition-fast);
  }

  .topbar-icon-btn:hover {
    background: var(--bg-overlay);
    color: var(--text-primary);
  }

  /* ─── Chat area ────────────────────────────────────────────────── */
  .chat-area {
    flex: 1;
    overflow-y: auto;
    padding: 0;
    display: flex;
    flex-direction: column;
  }

  /* ─── Welcome screen ─────────────────────────────────────────────── */
  .welcome {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 24px 40px;
    text-align: center;
    gap: 16px;
    animation: fadeUp 400ms ease both;
  }

  @keyframes fadeUp {
    from { opacity: 0; transform: translateY(16px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  .welcome-icon {
    font-size: 3rem;
    color: var(--accent-primary);
    filter: drop-shadow(0 0 16px var(--accent-glow));
    margin-bottom: 4px;
    animation: pulse 2.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { filter: drop-shadow(0 0 12px var(--accent-glow)); }
    50%       { filter: drop-shadow(0 0 24px var(--accent-glow)); }
  }

  .welcome-title {
    font-size: clamp(1.6rem, 4vw, 2.2rem);
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.03em;
    line-height: 1.2;
  }

  .welcome-sub {
    font-size: var(--font-size-sm);
    color: var(--text-tertiary);
    max-width: 420px;
    line-height: 1.6;
  }

  .welcome-sub strong { color: var(--accent-primary); }

  .quick-prompts {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    justify-content: center;
    max-width: 560px;
    margin-top: 8px;
  }

  .quick-btn {
    display: flex;
    align-items: center;
    gap: 7px;
    padding: 8px 14px;
    background: var(--bg-surface);
    border: 1px solid var(--border-default);
    color: var(--text-secondary);
    border-radius: var(--border-radius-lg);
    font-size: var(--font-size-sm);
    cursor: pointer;
    font-family: var(--font-sans);
    transition: all var(--transition-fast);
  }

  .quick-btn:hover {
    background: var(--bg-elevated);
    border-color: var(--accent-primary);
    color: var(--text-primary);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
  }

  .quick-icon { font-size: 1rem; }
  .quick-label { font-weight: 500; }

  /* ─── Message list ─────────────────────────────────────────────── */
  .messages-list {
    padding: 24px 0 16px;
    display: flex;
    flex-direction: column;
    gap: 2px;
    max-width: 800px;
    width: 100%;
    margin: 0 auto;
    padding-left: 24px;
    padding-right: 24px;
  }

  /* ─── Gathering Sources Pulse ─────────────────────────────────── */
  :global(.gathering-sources) {
    font-style: italic;
    color: var(--text-tertiary);
    animation: pulseText 2s infinite ease-in-out;
  }

  @keyframes pulseText {
    0%, 100% { opacity: 0.5; }
    50% { opacity: 1; }
  }

  /* ─── Markdown Styles ──────────────────────────────────────────── */
  :global(.markdown-body) {
    color: var(--text-primary);
    line-height: 1.8;
    font-size: var(--font-size-base);
    word-break: break-word;
  }

  :global(.markdown-body h1),
  :global(.markdown-body h2),
  :global(.markdown-body h3),
  :global(.markdown-body h4) {
    color: var(--text-primary);
    font-weight: 700;
    letter-spacing: -0.02em;
    margin: 1.2em 0 0.5em;
    line-height: 1.3;
  }

  :global(.markdown-body h1) { font-size: 1.4em; }
  :global(.markdown-body h2) { font-size: 1.2em; }
  :global(.markdown-body h3) { font-size: 1.05em; }

  :global(.markdown-body p) { margin: 0.7em 0; }
  :global(.markdown-body p:first-child) { margin-top: 0; }

  :global(.markdown-body a) {
    color: var(--accent-primary);
    text-decoration: none;
  }

  :global(.markdown-body a:hover) { text-decoration: underline; }

  :global(.markdown-body ul),
  :global(.markdown-body ol) {
    padding-left: 1.5em;
    margin: 0.6em 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  :global(.markdown-body li) { line-height: 1.7; }

  :global(.markdown-body blockquote) {
    border-left: 3px solid var(--accent-primary);
    margin: 1em 0;
    padding: 10px 16px;
    background: var(--accent-dim);
    border-radius: 0 var(--border-radius-sm) var(--border-radius-sm) 0;
    color: var(--text-secondary);
  }

  :global(.markdown-body hr) {
    border: none;
    border-top: 1px solid var(--border-subtle);
    margin: 1.5em 0;
  }

  /* Code block */
  :global(.code-block) {
    margin: 1em 0;
    border-radius: var(--border-radius-md);
    overflow: hidden;
    border: 1px solid var(--border-default);
    background: #0d1117;
  }

  :global(.code-header) {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 7px 14px;
    background: rgba(255,255,255,0.04);
    border-bottom: 1px solid var(--border-subtle);
  }

  :global(.code-lang) {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    font-family: var(--font-mono);
  }

  :global(.copy-btn) {
    display: flex;
    align-items: center;
    gap: 5px;
    background: transparent;
    border: 1px solid var(--border-subtle);
    color: var(--text-tertiary);
    font-size: var(--font-size-xs);
    padding: 3px 8px;
    border-radius: 4px;
    cursor: pointer;
    font-family: var(--font-sans);
    transition: background var(--transition-fast), color var(--transition-fast);
  }

  :global(.copy-btn:hover) {
    background: var(--bg-elevated);
    color: var(--text-secondary);
  }

  :global(.code-block pre) {
    margin: 0;
    padding: 14px 16px;
    overflow-x: auto;
    background: transparent;
  }

  :global(.code-block code) {
    font-family: var(--font-mono);
    font-size: 0.85em;
    line-height: 1.65;
  }

  /* Inline code */
  :global(.markdown-body p code),
  :global(.markdown-body li code) {
    background: var(--bg-overlay);
    color: var(--accent-primary);
    padding: 0.15em 0.4em;
    border-radius: 4px;
    font-family: var(--font-mono);
    font-size: 0.85em;
    border: 1px solid var(--border-subtle);
  }

  /* Tables */
  :global(.markdown-body table) {
    width: 100%;
    border-collapse: collapse;
    margin: 1em 0;
    font-size: var(--font-size-sm);
  }

  :global(.markdown-body th),
  :global(.markdown-body td) {
    padding: 9px 13px;
    border: 1px solid var(--border-default);
    text-align: left;
  }

  :global(.markdown-body th) {
    background: var(--bg-elevated);
    font-weight: 600;
    color: var(--text-secondary);
    font-size: var(--font-size-xs);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  :global(.markdown-body tr:nth-child(even) td) {
    background: rgba(255,255,255,0.02);
  }

  /* ─── Input bar ─────────────────────────────────────────────────── */
  .input-zone {
    flex-shrink: 0;
    padding: 12px 24px 14px;
    border-top: 1px solid var(--border-subtle);
    background: var(--bg-app);
    display: flex;
    flex-direction: column;
    gap: 6px;
    max-width: 800px;
    width: 100%;
    margin: 0 auto;
    /* Break out of the flex centering */
    align-self: stretch;
  }

  /* Workaround: center the input bar */
  .main .input-zone {
    padding-left: max(24px, calc((100% - 800px) / 2 + 24px));
    padding-right: max(24px, calc((100% - 800px) / 2 + 24px));
    max-width: none;
  }

  .input-bar {
    display: flex;
    align-items: flex-end;
    gap: 8px;
    background: var(--bg-input);
    border: 1.5px solid var(--border-default);
    border-radius: var(--border-radius-xl);
    padding: 10px 10px 10px 14px;
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
  }

  .model-picker-wrapper {
    flex-shrink: 0;
    margin-bottom: 2px;
  }

  .input-bar:focus-within {
    border-color: var(--accent-primary);
    box-shadow: var(--shadow-input);
  }

  textarea#chat-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-family: var(--font-sans);
    font-size: var(--font-size-base);
    line-height: 1.6;
    resize: none;
    max-height: 200px;
    overflow-y: auto;
    padding: 0;
    caret-color: var(--accent-primary);
  }

  textarea#chat-input::placeholder {
    color: var(--text-tertiary);
  }

  .input-actions {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  .hint-text {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
    white-space: nowrap;
  }

  .send-btn {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--accent-primary);
    border: none;
    color: #000;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background var(--transition-fast), transform var(--transition-fast), box-shadow var(--transition-fast);
    flex-shrink: 0;
  }

  .send-btn:hover:not(:disabled) {
    background: var(--accent-secondary);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px var(--accent-glow);
  }

  .send-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    background: var(--text-tertiary);
  }

  .send-btn:active:not(:disabled) {
    transform: scale(0.96);
  }

  .stop-btn {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: #ff4d4d;
    border: none;
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    flex-shrink: 0;
    box-shadow: 0 0 10px rgba(255, 77, 77, 0.4);
  }

  .stop-btn:hover {
    background: #ff3333;
    transform: scale(1.1);
    box-shadow: 0 0 15px rgba(255, 77, 77, 0.6);
  }

  .stop-btn:active {
    transform: scale(0.9);
  }

  .spin {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .disclaimer {
    text-align: center;
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
  }

  /* ─── Citation Chips ────────────────────────────────────────────── */
  :global(.cite-chip) {
    display: inline-flex;
    align-items: center;
    background: var(--bg-elevated);
    color: var(--accent-primary);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    padding: 0 4px;
    margin: 0 2px;
    font-size: 0.8em;
    font-weight: 600;
    cursor: pointer;
    vertical-align: baseline;
    transition: all 0.2s;
    user-select: none;
    line-height: 1.4;
  }

  :global(.cite-chip:hover) {
    background: var(--accent-dim);
    border-color: var(--accent-primary);
    box-shadow: 0 0 8px var(--accent-glow);
  }

  :global(.cite-chip:active) {
    transform: scale(0.95);
  }

  /* ─── Typing Indicator ─────────────────────────────────────────── */
  .typing-indicator {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 12px 16px;
    margin-top: 4px;
    background: var(--bg-elevated);
    border-radius: var(--border-radius-lg);
    width: fit-content;
    border: 1px solid var(--border-subtle);
    box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  }

  .typing-indicator span {
    width: 6px;
    height: 6px;
    background: var(--accent-primary);
    border-radius: 50%;
    display: block;
    animation: bounce 1.4s infinite ease-in-out both;
  }

  .typing-indicator span:nth-child(1) { animation-delay: -0.32s; }
  .typing-indicator span:nth-child(2) { animation-delay: -0.16s; }
</style>