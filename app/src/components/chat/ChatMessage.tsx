import React from 'react';
import { User, Bot } from 'lucide-react';
import { ThoughtTrace } from './ThoughtTrace';
import { useAppStore, Message, MessageCitation } from '../../stores/useAppStore';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

interface ChatMessageProps {
  message: Message;
}

export const ChatMessage: React.FC<ChatMessageProps> = ({ message }) => {
  const isUser = message.role === 'user';
  const { openRightPanel, setActiveCitations } = useAppStore();

  const handleCitationClick = (e: React.MouseEvent, _citation: MessageCitation) => {
    e.preventDefault();
    if (message.citations) {
      setActiveCitations(message.citations);
    }
    openRightPanel();
  };

  return (
    <div className={`flex gap-4 p-4 ${isUser ? '' : 'bg-[var(--bg-elevated)]'} rounded-[var(--radius-lg)] mb-4 transition-all`}>
      <div className="flex-shrink-0 mt-1">
        {isUser ? (
          <div className="w-8 h-8 rounded-full bg-[var(--border)] flex items-center justify-center">
            <User size={18} className="text-[var(--text-primary)]" />
          </div>
        ) : (
          <div className="w-8 h-8 rounded-full bg-[var(--accent-primary)] flex items-center justify-center text-white">
            <Bot size={18} />
          </div>
        )}
      </div>
      
      <div className="flex-1 overflow-hidden">
        <div className="font-semibold text-sm mb-1 text-[var(--text-primary)]">
          {isUser ? 'User' : 'Q-Agent'}
        </div>
        
        {!isUser && message.traces && <ThoughtTrace traces={message.traces} />}
        
        <div className="text-[var(--text-primary)] text-base leading-relaxed break-words max-w-[var(--prose-width,72ch)]">
          {message.content === '' && !isUser && (
            <div className="space-y-2 mt-2">
              <div className="h-4 bg-[var(--bg-subtle)] rounded animate-pulse w-3/4" />
              <div className="h-4 bg-[var(--bg-subtle)] rounded animate-pulse w-1/2" />
            </div>
          )}
          
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
            components={{
              a: ({ node, href, children, ...props }) => {
                if (href?.startsWith('citation://')) {
                  const idx = parseInt(href.replace('citation://', ''), 10);
                  const citation = message.citations?.find(c => c.index === idx);
                  if (citation) {
                    return (
                      <button
                        onClick={(e) => handleCitationClick(e, citation)}
                        title={citation.excerpt}
                        className="inline-flex items-center justify-center w-5 h-5 mx-0.5 text-[10px] font-bold text-[var(--accent-primary)] bg-[var(--accent-primary-subtle)] rounded-sm border border-[var(--accent-primary)]/30 cursor-pointer hover:bg-[var(--accent-primary)] hover:text-white transition-colors align-super"
                      >
                        {idx}
                      </button>
                    );
                  }
                  return <span className="text-[var(--text-muted)]">[{idx}]</span>;
                }
                return <a href={href} target="_blank" rel="noreferrer" className="text-[var(--accent-primary)] hover:underline" {...props}>{children}</a>;
              }
            }}
          >
            {message.content.replace(/\[(\d+)\]/g, '[CITATION-$1](citation://$1)')}
          </ReactMarkdown>
        </div>

        {message.citations && message.citations.length > 0 && (
          <div className="mt-4 pt-4 border-t border-[var(--border)] max-w-[var(--prose-width,72ch)]">
            <div className="text-xs font-semibold text-[var(--text-muted)] mb-2 uppercase tracking-wide">Sources</div>
            <div className="flex flex-wrap gap-2">
              {message.citations.map(c => (
                <button 
                  key={c.index} 
                  onClick={(e) => handleCitationClick(e, c)}
                  className="flex items-center w-[220px] gap-2 p-2 bg-[var(--bg-base)] border border-[var(--border)] rounded-[var(--radius-md)] hover:border-[var(--accent-primary)] hover:bg-[var(--bg-surface)] transition-all shadow-[var(--shadow-sm)] text-left"
                >
                  <div className="w-5 h-5 flex-shrink-0 rounded-full bg-[var(--bg-surface)] flex items-center justify-center text-[10px] font-bold text-[var(--text-muted)] border border-[var(--border)]">
                    {c.index}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-xs font-medium text-[var(--text-primary)] truncate" title={c.excerpt}>
                      {c.excerpt.substring(0, 40) || c.source_id}
                    </div>
                    <div className="text-[10px] text-[var(--text-muted)] truncate flex items-center gap-1 mt-0.5">
                      <span className="text-[var(--accent-warning)]">★</span> {c.confidence.toFixed(1)} &middot; {c.source_id.replace(/^https?:\/\//, '').split('/')[0]}
                    </div>
                  </div>
                </button>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
