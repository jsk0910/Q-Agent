import React, { useState, useRef, useEffect } from 'react';
import { Send, Command, File, Archive, Database } from 'lucide-react';
import { ChatMessage } from './ChatMessage';
import { Message, useAppStore } from '../../stores/useAppStore';
import { useArtifactStore } from '../../stores/useArtifactStore';

export const ChatArea: React.FC = () => {
  const { 
    messages, setMessages, activeProject, activeCitations,
    isRightPanelOpen, permissionMode 
  } = useAppStore();
  const [input, setInput] = useState('');
  const [showMentions, setShowMentions] = useState(false);
  const [mentionFilter, setMentionFilter] = useState('');

  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const mockContexts = [
    { name: 'main.rs', type: 'file', icon: <File size={14} /> },
    { name: 'lib.rs', type: 'file', icon: <File size={14} /> },
    { name: 'Plan.md', type: 'file', icon: <File size={14} /> },
    { name: 'AgentReport_v1', type: 'artifact', icon: <Archive size={14} /> },
    { name: 'SurrealDB_Schema', type: 'knowledge', icon: <Database size={14} /> },
  ];

  const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const value = e.target.value;
    setInput(value);

    // Simple mention detection
    const lastWord = value.split(/\s/).pop() || '';
    if (lastWord.startsWith('@')) {
      setShowMentions(true);
      setMentionFilter(lastWord.slice(1));
    } else {
      setShowMentions(false);
    }
  };

  const selectMention = (name: string) => {
    const words = input.split(/\s/);
    words.pop(); // remove @filter
    const newValue = [...words, `@${name} `].join(' ');
    setInput(newValue);
    setShowMentions(false);
    textareaRef.current?.focus();
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const messageToSend = input.trim();
    if (!messageToSend) return;

    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: messageToSend,
    };
    
    // Save user message to DB if we have a project
    if (activeProject && window.__TAURI_INTERNALS__) {
      const pid = typeof activeProject.id === 'string' ? activeProject.id : (activeProject.id?.id?.String || activeProject.id?.id);
      import('@tauri-apps/api/core').then(({ invoke }) => {
        // Need to fetch conv ID first, or just use send_chat_message which we'll adapt
        // Actually, we can fetch history to get the conversation ID, but let's assume we can fetch it, 
        // or just ignore saving for now and do it inside the backend.
      });
    }

    setMessages([...messages, userMessage]);
    setInput('');
    setShowMentions(false);
    
    // Set initial loading state
    const loadingId = (Date.now() + 1).toString();
    setMessages(prev => [...prev, {
      id: loadingId,
      role: 'assistant',
      content: '',
      traces: [
        { agent: 'Planner', step: 'Calling Llama.cpp backend...' }
      ]
    }]);

    // Call Q-Agent API
    const callBackend = async () => {
      try {
        let replyContent = '';
        let citations: any[] = [];
        
        // Extract mentions from message
        const mentionRegex = /@([\w\-\.]+)/g;
        const mentions = [];
        let match;
        while ((match = mentionRegex.exec(messageToSend)) !== null) {
          mentions.push(match[1]);
        }
        
        // @ts-ignore
        if (window.__TAURI_INTERNALS__) {
          // Tauri Desktop Environment -> Use Rust LangGraph Workflow
          // @ts-ignore
          const { invoke } = window.__TAURI__?.core || await import('@tauri-apps/api/core');
          
          let pId = null;
          if (activeProject) {
            if (typeof activeProject.id === 'string') {
              pId = activeProject.id;
            } else if (activeProject.id?.id?.String) {
              pId = activeProject.id.id.String;
            } else if (activeProject.id?.id) {
              pId = activeProject.id.id;
            }
          }

          const response = await invoke('run_agent_workflow', {
            message: messageToSend,
            projectId: pId,
            harnessType: activeProject?.harness || 'deep_research',
            permissionMode: permissionMode,
            mentions: mentions
          }) as { answer: string; citations: any[]; artifacts?: any[] };
          
          replyContent = response.answer;
          citations = response.citations || [];
          
          if (response.artifacts) {
            response.artifacts.forEach(artifact => {
              useArtifactStore.getState().addArtifact(artifact);
            });
          }

          setMessages(prev => prev.map(msg => {
            if (msg.id === loadingId) {
              return { ...msg, content: replyContent, citations, traces: [{ agent: 'Executor', step: 'Generated response' }] };
            }
            return msg;
          }));
        } else {
          // Web Browser Environment -> SSE 스트리밍 엔드포인트 호출
          const apiBase = import.meta.env.VITE_API_URL || 'http://localhost:8000';
          const response = await fetch(`${apiBase}/chat/stream`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              message: messageToSend,
              harness_type: activeProject?.harness || 'general',
              permission_mode: permissionMode,
            }),
          });

          if (!response.ok || !response.body) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
          }

          const reader = response.body.getReader();
          const decoder = new TextDecoder();

          while (true) {
            const { value, done } = await reader.read();
            if (done) break;

            const chunk = decoder.decode(value, { stream: true });
            const lines = chunk.split('\n');

            for (const line of lines) {
              if (!line.startsWith('data: ')) continue;
              try {
                const data = JSON.parse(line.slice(6));
                if (data.event === 'token') {
                  replyContent += data.token;
                  setMessages(prev => prev.map(msg =>
                    msg.id === loadingId
                      ? { ...msg, content: replyContent, traces: [{ agent: 'Executor', step: 'Streaming...' }] }
                      : msg
                  ));
                } else if (data.event === 'done') {
                  citations = (data.citations || []).map((c: any) => ({
                    index: c.index,
                    source_id: c.source_id,
                    excerpt: c.excerpt,
                    confidence: c.confidence,
                  }));
                  setMessages(prev => prev.map(msg =>
                    msg.id === loadingId
                      ? { ...msg, content: replyContent, citations, traces: [{ agent: 'Executor', step: 'Generated response' }] }
                      : msg
                  ));
                } else if (data.event === 'error') {
                  throw new Error(data.detail);
                }
              } catch (parseErr) {
                // 파싱 오류는 무시 (빈 줄 등)
              }
            }
          }
        }
      } catch (error) {
        console.error('Failed to fetch from backend:', error);
        setMessages(prev => prev.map(msg => {
          if (msg.id === loadingId) {
            return {
              ...msg,
              content: '백엔드 연결에 실패했습니다. 서버가 실행 중인지 확인해주세요.',
              traces: [{ agent: 'System', step: 'Connection error' }]
            };
          }
          return msg;
        }));
      }
    };

    callBackend();
  };

  const filteredContexts = mockContexts.filter(c => 
    c.name.toLowerCase().includes(mentionFilter.toLowerCase())
  );

  return (
    <div className="flex flex-col h-full bg-[var(--bg-base)]">
      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-4 md:p-8 space-y-6">
        <div className="max-w-4xl mx-auto">
          {messages.map(msg => (
            <ChatMessage key={msg.id} message={msg} />
          ))}
          <div ref={messagesEndRef} />
        </div>
      </div>

      {/* Input Area */}
      <div className="p-4 bg-[var(--bg-surface)] border-t border-[var(--border)]">
        <form onSubmit={handleSubmit} className="max-w-4xl mx-auto relative">
          
          {/* Mention Popup */}
          {showMentions && (
            <div className="absolute bottom-full left-0 mb-2 w-64 bg-[var(--bg-elevated)] border border-[var(--border)] rounded-[var(--radius-lg)] shadow-xl overflow-hidden z-50 animate-in fade-in slide-in-from-bottom-2">
              <div className="p-2 border-b border-[var(--border)] bg-[var(--bg-surface)] text-[10px] font-bold uppercase text-[var(--text-muted)]">
                Reference Context
              </div>
              <div className="max-h-48 overflow-y-auto">
                {filteredContexts.map(ctx => (
                  <button
                    key={ctx.name}
                    type="button"
                    onClick={() => selectMention(ctx.name)}
                    className="w-full flex items-center gap-2 px-3 py-2 hover:bg-[var(--accent-primary)] hover:text-white transition-colors text-sm text-left"
                  >
                    <span className="text-[var(--text-muted)] group-hover:text-white/80">{ctx.icon}</span>
                    <span>{ctx.name}</span>
                  </button>
                ))}
                {filteredContexts.length === 0 && (
                  <div className="p-3 text-xs text-[var(--text-muted)] italic text-center">No matches found</div>
                )}
              </div>
            </div>
          )}

          <div className="flex items-end bg-[var(--bg-elevated)] border border-[var(--border)] rounded-[var(--radius-lg)] p-2 focus-within:border-[var(--accent-primary)] focus-within:shadow-glow transition-all">
            <button type="button" className="p-2 text-[var(--text-muted)] hover:text-[var(--accent-primary)] transition-colors">
              <Command size={20} />
            </button>
            
            <textarea 
              ref={textareaRef}
              value={input}
              onChange={handleInputChange}
              placeholder="Ask anything or use @ to reference context..."
              className="flex-1 max-h-32 min-h-[44px] bg-transparent resize-none outline-none text-[var(--text-primary)] placeholder-[var(--text-muted)] p-2"
              onKeyDown={(e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                  if (showMentions && filteredContexts.length > 0) {
                    e.preventDefault();
                    selectMention(filteredContexts[0].name);
                  } else {
                    e.preventDefault();
                    handleSubmit(e);
                  }
                }
                if (e.key === 'Escape') {
                  setShowMentions(false);
                }
              }}
            />
            
            <button 
              type="submit"
              disabled={!input.trim()}
              className="p-2 bg-[var(--accent-primary)] text-white rounded-[var(--radius-md)] hover:bg-[var(--accent-secondary)] disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              <Send size={18} />
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
