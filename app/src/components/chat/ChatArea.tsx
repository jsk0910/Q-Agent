import React, { useState } from 'react';
import { Send, Command } from 'lucide-react';
import { ChatMessage, Message } from './ChatMessage';

export const ChatArea: React.FC = () => {
  const [input, setInput] = useState('');
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
    
    // Simulate streaming response
    // (This will be replaced by actual Tauri IPC call later)
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
          <div className="flex items-end bg-[var(--bg-elevated)] border border-[var(--border)] rounded-[var(--radius-lg)] p-2 focus-within:border-[var(--accent-primary)] focus-within:shadow-glow transition-all">
            <button type="button" className="p-2 text-[var(--text-muted)] hover:text-[var(--accent-primary)] transition-colors">
              <Command size={20} />
            </button>
            
            <textarea 
              value={input}
              onChange={e => setInput(e.target.value)}
              placeholder="Ask anything or use Alt+Space for HUD..."
              className="flex-1 max-h-32 min-h-[44px] bg-transparent resize-none outline-none text-[var(--text-primary)] placeholder-[var(--text-muted)] p-2"
              onKeyDown={(e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                  e.preventDefault();
                  handleSubmit(e);
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
