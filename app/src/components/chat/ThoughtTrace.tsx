import React from 'react';
import { Bot, User, ChevronRight, ChevronDown } from 'lucide-react';

interface ThoughtTraceProps {
  traces: { agent: string; step: string }[];
}

export const ThoughtTrace: React.FC<ThoughtTraceProps> = ({ traces }) => {
  const [expanded, setExpanded] = React.useState(false);

  if (!traces || traces.length === 0) return null;

  return (
    <div className="mb-4">
      <button 
        onClick={() => setExpanded(!expanded)}
        className="flex items-center gap-2 text-xs font-medium text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors"
      >
        {expanded ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
        <span>Thought Trace ({traces.length} steps)</span>
      </button>
      
      {expanded && (
        <div className="mt-2 pl-6 space-y-2 border-l-2 border-[var(--border)] ml-2">
          {traces.map((trace, idx) => (
            <div key={idx} className="flex items-start gap-2 text-xs">
              <span className="font-semibold text-[var(--accent-primary)]">{trace.agent}</span>
              <span className="text-[var(--text-secondary)]">→</span>
              <span className="text-[var(--text-primary)]">{trace.step}</span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
