import React from 'react';
import { User, Bot } from 'lucide-react';
import { ThoughtTrace } from './ThoughtTrace';

export interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  traces?: { agent: string; step: string }[];
}

interface ChatMessageProps {
  message: Message;
}

export const ChatMessage: React.FC<ChatMessageProps> = ({ message }) => {
  const isUser = message.role === 'user';

  return (
    <div className={`flex gap-4 p-4 ${isUser ? '' : 'bg-[var(--bg-elevated)]'} rounded-[var(--radius-lg)] mb-4 transition-all`}>
      <div className="flex-shrink-0 mt-1">
        {isUser ? (
          <div className="w-8 h-8 rounded-full bg-[var(--border)] flex items-center justify-center">
            <User size={18} className="text-[var(--text-primary)]" />
          </div>
        ) : (
          <div className="w-8 h-8 rounded-full bg-[var(--accent-primary)] flex items-center justify-center text-white shadow-glow">
            <Bot size={18} />
          </div>
        )}
      </div>
      
      <div className="flex-1 overflow-hidden">
        <div className="font-semibold text-sm mb-1 text-[var(--text-primary)]">
          {isUser ? 'User' : 'Q-Agent'}
        </div>
        
        {!isUser && message.traces && <ThoughtTrace traces={message.traces} />}
        
        <div className="text-[var(--text-primary)] text-base leading-relaxed whitespace-pre-wrap">
          {message.content}
        </div>
      </div>
    </div>
  );
};
