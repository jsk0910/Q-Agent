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
  import Sidebar from '$lib/components/Sidebar.svelte';
  import WidgetPanel from '$lib/components/WidgetPanel.svelte';
  import ModelPicker from '$lib/components/ModelPicker.svelte';
  import ModelInstallModal from '$lib/components/ModelInstallModal.svelte';

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
    chatStore.updateMessage(convId, asstId, { content: "<span class='gathering-sources'>Agent is gathering sources...</span>\n" });
    
    let contextStr = "";
    try {
      contextStr = await invoke('intercept_and_route', { prompt: text });
    } catch(e) {
      console.error("Routing error:", e);
    }
    
    chatStore.updateMessage(convId, asstId, { content: "" });
    const finalSystemPrompt = $effectiveSettings.systemPrompt + (contextStr ? "\n\n" + contextStr : "");

    let unlistenToken: UnlistenFn | undefined;
    let unlistenFinished: UnlistenFn | undefined;
    let streamRaw = "";

    try {
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
        stream: $settings.streamTokens
      });

      // Cleanup listeners
      if (unlistenToken) unlistenToken();
      if (unlistenFinished) unlistenFinished();

      // Final reconciliation
      const finalRaw = $settings.streamTokens ? streamRaw : raw;
      const { thinking, answer } = parseResponse(finalRaw);
      chatStore.updateMessage(convId, asstId, {
        content: answer,
        thinking: thinking ?? undefined,
      });
      
    } catch (err) {
      if (unlistenToken) unlistenToken();
      
      chatStore.updateMessage(convId, asstId, {
        content: `⚠️ Error: ${err}`,
      });
    } finally {
      isLoading = false;
    }
  }

  // ─── Export Chat ────────────────────────────────────────────────────────
  function exportChat() {
    if (!currentConv) return;
    
    let md = `# ${currentConv.title}\n\n`;
    currentConv.messages.forEach((m: Message) => {
      md += `### ${m.role === 'user' ? 'User' : 'Assistant'}\n`;
      if (m.thinking && $settings.showThinking) {
        md += `<details><summary>Thinking process</summary>\n\n${m.thinking}\n</details>\n\n`;
      }
      md += `${m.content}\n\n---\n\n`;
    });

    const blob = new Blob([md], { type: 'text/markdown' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${currentConv.title.replace(/[^a-zA-Z0-9\s]/g, '_').trim()}.md`;
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
  });

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
        <ModelPicker onrequestInstall={m => installModel = m} />
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
          title="Export Conversation"
          onclick={exportChat}
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
        <div class="messages-list" oncite-click={() => openWidgets('sources')}>
          {#each messages as msg, i (msg.id)}
            <div class="message-wrap" class:user={msg.role === 'user'} class:asst={msg.role === 'assistant'}>

              {#if msg.role === 'user'}
                <!-- User bubble -->
                <div class="user-bubble">
                  <div class="bubble-content">{msg.content}</div>
                  <span class="msg-time">{formatTimestamp(msg.timestamp)}</span>
                </div>

              {:else}
                <!-- Assistant response -->
                <div class="asst-block">
                  <div class="asst-avatar">
                    <span>⬡</span>
                  </div>
                  <div class="asst-body">

                    <!-- Thinking block -->
                    {#if msg.thinking && $settings.showThinking}
                      <button
                        class="thinking-toggle"
                        onclick={() => toggleThinking(msg.id)}
                        aria-expanded={expandedThinking.has(msg.id)}
                      >
                        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                          <path d="M9 18c-4.51 2-5-2-7-2M22 6c-1 3-2 4-4 5s-3 1-3 3v1"/>
                          <path d="M2 2l20 20"/>
                        </svg>
                        <span>Thinking process</span>
                        <svg
                          class="chevron"
                          class:rotated={expandedThinking.has(msg.id)}
                          width="12" height="12" viewBox="0 0 24 24"
                          fill="none" stroke="currentColor" stroke-width="2.5"
                        >
                          <path d="M6 9l6 6 6-6"/>
                        </svg>
                      </button>

                      {#if expandedThinking.has(msg.id)}
                        <div class="thinking-content">
                          <div class="thinking-inner">
                            {msg.thinking}
                          </div>
                        </div>
                      {/if}
                    {/if}

                    <!-- Answer content -->
                    {#if msg.content}
                      {#await renderMd(msg.content) then html}
                        <div class="markdown-body">{@html html}</div>
                      {/await}
                    {/if}

                    {#if isLoading && i === messages.length - 1 && msg.role === 'assistant'}
                      <div class="typing-indicator">
                        <span></span><span></span><span></span>
                      </div>
                    {/if}

                    <!-- Footer -->
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
          {/each}
          <div bind:this={chatEndEl}></div>
        </div>
      {/if}
    </div>

    <!-- Input bar -->
    <div class="input-zone">
      <div class="input-bar" class:focused={false} id="input-bar">
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
            {$settings.sendOnEnter ? '↵ Send' : 'Ctrl+↵ Send'}
          </span>
          <button
            class="send-btn"
            id="send-btn"
            onclick={send}
            disabled={isLoading || !prompt.trim()}
            aria-label="Send message"
          >
            {#if isLoading}
              <svg class="spin" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4"/>
              </svg>
            {:else}
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                <path d="M22 2L11 13M22 2L15 22l-4-9-9-4 20-7z"/>
              </svg>
            {/if}
          </button>
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

  .message-wrap {
    display: flex;
    flex-direction: column;
    padding: 6px 0;
  }

  /* User bubble */
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

  /* Assistant block (Perplexity style: left-aligned, full width, no bubble) */
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

  /* Typing indicator animation */
  @keyframes bounce {
    0%, 80%, 100% { transform: scale(0); opacity: 0.3; }
    40% { transform: scale(1); opacity: 1; }
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

  /* Thinking toggle */
  .thinking-toggle {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    padding: 5px 12px;
    background: var(--thinking-bg);
    border: 1px solid var(--thinking-border);
    color: var(--thinking-text);
    border-radius: var(--border-radius-sm);
    font-size: var(--font-size-xs);
    font-weight: 600;
    cursor: pointer;
    font-family: var(--font-sans);
    transition: background var(--transition-fast);
    align-self: flex-start;
  }

  .thinking-toggle:hover {
    background: rgba(255, 180, 0, 0.10);
  }

  .chevron {
    transition: transform var(--transition-base);
  }

  .chevron.rotated {
    transform: rotate(180deg);
  }

  .thinking-content {
    animation: fadeUp 150ms ease;
  }

  .thinking-inner {
    background: var(--thinking-bg);
    border: 1px solid var(--thinking-border);
    border-radius: var(--border-radius-md);
    padding: 12px 14px;
    font-size: var(--font-size-sm);
    color: var(--thinking-text);
    line-height: 1.7;
    white-space: pre-wrap;
    font-family: var(--font-mono);
    border-left: 3px solid var(--thinking-border);
    max-height: 300px;
    overflow-y: auto;
  }

  /* Typing indicator */
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

  /* Assistant footer */
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
    padding: 10px 10px 10px 18px;
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
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