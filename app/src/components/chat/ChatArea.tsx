import React, { useState, useRef, useEffect } from 'react';
import { Send, Command, File, Archive, Database } from 'lucide-react';
import { ChatMessage, Message } from './ChatMessage';

export const ChatArea: React.FC = () => {
  const [input, setInput] = useState('');
  const [showMentions, setShowMentions] = useState(false);
  const [mentionFilter, setMentionFilter] = useState('');
  const [messages, setMessages] = useState<Message[]>([
    {
      id: '1',
      role: 'assistant',
      content: 'Hello! I am Q-Agent. How can I help you with your project today?',
      traces: [
        { agent: 'System', step: 'Initialized environment' },
        { agent: 'Router', step: 'Ready for input' }
      ]
    }
  ]);

  const textareaRef = useRef<HTMLTextAreaElement>(null);

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
    if (!input.trim()) return;

    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: input,
    };

    setMessages([...messages, userMessage]);
    setInput('');
    setShowMentions(false);
    
    // Simulate streaming response
    setTimeout(() => {
      setMessages(prev => [...prev, {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: 'This is a simulated response from the Mock Llama backend.',
        traces: [
          { agent: 'Planner', step: 'Analyzed request' },
          { agent: 'Executor', step: 'Generated mock response' }
        ]
      }]);
    }, 1000);
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
