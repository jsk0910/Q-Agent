import React, { useState } from 'react';

interface ThoughtTraceProps {
  traces: { agent: string; step: string }[];
}

export const ThoughtTrace: React.FC<ThoughtTraceProps> = ({ traces }) => {
  const [isExpanded, setIsExpanded] = useState(false);

  if (!traces || traces.length === 0) return null;

  return (
    <div className="mb-2">
      <button 
        onClick={() => setIsExpanded(!isExpanded)} 
        className="flex items-center gap-2 text-xs text-[var(--text-muted)] hover:text-[var(--text-secondary)] transition-colors"
      >
        <span>📍 {traces[0].agent}</span>
        {traces.length > 1 && <span>→ {traces[traces.length - 1].agent}</span>}
        <span className="text-[var(--accent-primary)] font-medium">
          {isExpanded ? '▾' : '▸'} {isExpanded ? '접기' : '펼치기'}
        </span>
      </button>
      
      {isExpanded && (
        <div className="space-y-1.5 mt-2 mb-3 pl-3 border-l-2 border-[var(--border)] animate-in fade-in slide-in-from-top-1 duration-200">
          {traces.map((trace, idx) => (
            <div key={idx} className="text-xs text-[var(--text-muted)]">
              <span className="font-medium text-[var(--text-secondary)]">{trace.agent}</span>
              <span className="mx-1">→</span>
              <span className="text-[var(--text-primary)]">{trace.step}</span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
