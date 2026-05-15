import React from 'react';
import { X, Copy, Check } from 'lucide-react';
import { Artifact } from '../../stores/useArtifactStore';

interface ArtifactViewerModalProps {
  artifact: Artifact;
  onClose: () => void;
}

export const ArtifactViewerModal: React.FC<ArtifactViewerModalProps> = ({ artifact, onClose }) => {
  const [copied, setCopied] = React.useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(artifact.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="fixed inset-0 z-[110] flex items-center justify-center bg-black/60 backdrop-blur-sm p-8">
      <div className="w-full max-w-5xl h-full bg-[var(--bg-surface)] border border-[var(--border)] rounded-lg shadow-2xl flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-200">
        
        <div className="flex items-center justify-between px-6 py-4 border-b border-[var(--border)] bg-[var(--bg-elevated)]">
          <div>
            <h2 className="text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2">
              {artifact.title}
              <span className="px-2 py-0.5 text-xs rounded bg-[var(--bg-surface)] border border-[var(--border)] uppercase text-[var(--text-muted)]">
                {artifact.language || artifact.type}
              </span>
            </h2>
          </div>
          <div className="flex items-center gap-2">
            <button 
              onClick={handleCopy}
              className="flex items-center gap-1 px-3 py-1.5 text-sm bg-[var(--bg-surface)] border border-[var(--border)] rounded hover:bg-[var(--bg-base)] transition-colors text-[var(--text-secondary)]"
            >
              {copied ? <Check size={16} className="text-green-500" /> : <Copy size={16} />}
              {copied ? 'Copied' : 'Copy'}
            </button>
            <button onClick={onClose} className="p-1.5 text-[var(--text-muted)] hover:bg-[var(--bg-surface)] rounded transition-colors">
              <X size={20} />
            </button>
          </div>
        </div>

        <div className="flex-1 overflow-auto bg-[#0d1117] p-6">
          <pre className="font-mono text-sm leading-relaxed text-[#c9d1d9] whitespace-pre-wrap">
            <code>{artifact.content}</code>
          </pre>
        </div>
      </div>
    </div>
  );
};
