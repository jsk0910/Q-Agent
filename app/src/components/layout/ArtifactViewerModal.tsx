import React, { useState } from 'react';
import { X, Copy, Check } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import { Artifact } from '../../stores/useArtifactStore';

interface ArtifactViewerModalProps {
  artifact: Artifact;
  onClose: () => void;
}

type ArtifactTab = 'preview' | 'code' | 'diff';

export const ArtifactViewerModal: React.FC<ArtifactViewerModalProps> = ({ artifact, onClose }) => {
  const [copied, setCopied] = useState(false);
  const [activeTab, setActiveTab] = useState<ArtifactTab>('code');

  const handleCopy = () => {
    navigator.clipboard.writeText(artifact.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="fixed inset-0 z-[110] flex items-center justify-center bg-[var(--bg-overlay)] backdrop-blur-sm p-8">
      <div className="w-full max-w-5xl h-full bg-[var(--bg-surface)] border border-[var(--border)] rounded-[var(--radius-lg)] shadow-[var(--shadow-lg)] flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-200">
        
        <div className="flex items-center justify-between px-6 py-4 border-b border-[var(--border)] bg-[var(--bg-elevated)]">
          <div className="flex flex-col gap-2">
            <h2 className="text-lg font-semibold text-[var(--text-primary)] flex items-center gap-2">
              {artifact.title}
              <span className="px-2 py-0.5 text-xs rounded bg-[var(--bg-surface)] border border-[var(--border)] uppercase text-[var(--text-muted)] font-medium">
                {artifact.language || artifact.type}
              </span>
            </h2>
            <div className="flex items-center gap-4 text-sm font-medium">
              <button 
                onClick={() => setActiveTab('preview')}
                className={`pb-1 border-b-2 transition-colors ${activeTab === 'preview' ? 'border-[var(--accent-primary)] text-[var(--text-primary)]' : 'border-transparent text-[var(--text-muted)] hover:text-[var(--text-secondary)]'}`}
              >
                Preview
              </button>
              <button 
                onClick={() => setActiveTab('code')}
                className={`pb-1 border-b-2 transition-colors ${activeTab === 'code' ? 'border-[var(--accent-primary)] text-[var(--text-primary)]' : 'border-transparent text-[var(--text-muted)] hover:text-[var(--text-secondary)]'}`}
              >
                Code
              </button>
              <button 
                onClick={() => setActiveTab('diff')}
                className={`pb-1 border-b-2 transition-colors ${activeTab === 'diff' ? 'border-[var(--accent-primary)] text-[var(--text-primary)]' : 'border-transparent text-[var(--text-muted)] hover:text-[var(--text-secondary)]'}`}
              >
                Diff
              </button>
            </div>
          </div>
          <div className="flex items-center gap-2 self-start">
            {activeTab === 'code' && (
              <button 
                onClick={handleCopy}
                className="flex items-center gap-1 px-3 py-1.5 text-sm bg-[var(--bg-surface)] border border-[var(--border)] rounded-[var(--radius-sm)] hover:bg-[var(--bg-base)] transition-colors text-[var(--text-secondary)]"
              >
                {copied ? <Check size={16} className="text-[var(--accent-success)]" /> : <Copy size={16} />}
                {copied ? 'Copied' : 'Copy'}
              </button>
            )}
            <button onClick={onClose} className="p-1.5 text-[var(--text-muted)] hover:bg-[var(--bg-surface)] rounded-[var(--radius-sm)] transition-colors">
              <X size={20} />
            </button>
          </div>
        </div>

        <div className="flex-1 overflow-auto bg-[#0d1117] relative">
          {activeTab === 'code' && (
            <div className="p-6">
              <pre className="font-mono text-sm leading-relaxed text-[#c9d1d9] whitespace-pre-wrap">
                <code>{artifact.content}</code>
              </pre>
            </div>
          )}
          {activeTab === 'preview' && (
            <div className="h-full bg-[var(--bg-surface)] text-[var(--text-primary)] p-8 overflow-y-auto">
              <div className="prose prose-invert max-w-none">
                {artifact.language === 'markdown' || artifact.type === 'markdown' || artifact.type === 'plan' ? (
                  <ReactMarkdown>{artifact.content}</ReactMarkdown>
                ) : (
                  <div className="flex items-center justify-center h-full text-[var(--text-muted)]">
                    <p>Preview is only available for Markdown artifacts.</p>
                  </div>
                )}
              </div>
            </div>
          )}
          {activeTab === 'diff' && (
            <div className="h-full bg-[#0d1117] p-6 overflow-y-auto">
              {artifact.type === 'diff' ? (
                <pre className="font-mono text-sm leading-relaxed whitespace-pre-wrap">
                  {artifact.content.split('\n').map((line, i) => {
                    if (line.startsWith('+')) return <div key={i} className="text-[#3fb950] bg-[#23863626] px-2">{line}</div>;
                    if (line.startsWith('-')) return <div key={i} className="text-[#f85149] bg-[#da363326] px-2">{line}</div>;
                    if (line.startsWith('@@')) return <div key={i} className="text-[#79c0ff] px-2 mt-2">{line}</div>;
                    return <div key={i} className="text-[#c9d1d9] px-2">{line}</div>;
                  })}
                </pre>
              ) : (
                <div className="flex items-center justify-center h-full text-[var(--text-muted)]">
                  <div className="text-center">
                    <p className="text-base mb-2">No Diff Available</p>
                    <p className="text-sm">This artifact does not contain diff data.</p>
                  </div>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
