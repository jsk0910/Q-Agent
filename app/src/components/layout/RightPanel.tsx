import React, { useState } from 'react';
import { Package, FileCode, FileText, LayoutTemplate, Maximize2, Link2, ExternalLink } from 'lucide-react';
import { useArtifactStore } from '../../stores/useArtifactStore';
import { useAppStore } from '../../stores/useAppStore';
import { ArtifactViewerModal } from './ArtifactViewerModal';

type RightPanelTab = 'artifacts' | 'sources';

export const RightPanel: React.FC = () => {
  const { artifacts, activeArtifactId, setActiveArtifact } = useArtifactStore();
  const { activeCitations } = useAppStore();
  const [activeTab, setActiveTab] = useState<RightPanelTab>('artifacts');
  const [viewerOpen, setViewerOpen] = useState(false);

  const renderStars = (score: number) => {
    // Assuming score is 0 to 5 or 0 to 1. If it's max 1.0, multiply by 5.
    // The bug report mentions "4.2, 3.5" so it's 0 to 5 scale.
    // In lib.rs we set it to 0.95 for the mock. So if it's <= 1, multiply by 5.
    const normalizedScore = score <= 1.0 ? score * 5 : score;
    const filled = Math.min(5, Math.max(0, Math.round(normalizedScore)));
    const empty = 5 - filled;
    return (
      <span className="inline-flex tracking-widest text-[var(--accent-warning)]">
        {'★'.repeat(filled)}
        <span className="text-[var(--text-muted)] opacity-50">{'★'.repeat(empty)}</span>
      </span>
    );
  };

  const getIcon = (type: string) => {
    switch (type) {
      case 'code': return <FileCode size={16} className="text-blue-400" />;
      case 'markdown': return <FileText size={16} className="text-emerald-400" />;
      case 'plan': return <LayoutTemplate size={16} className="text-purple-400" />;
      default: return <FileText size={16} />;
    }
  };

  return (
    <>
      <div className="flex flex-col h-full text-[var(--text-secondary)] bg-[var(--bg-surface)]">
        <div className="flex px-3 pt-3 border-b border-[var(--border)] gap-4 select-none">
          <button 
            onClick={() => setActiveTab('artifacts')}
            className={`pb-2 px-1 font-semibold text-sm border-b-2 transition-colors flex items-center gap-2 ${
              activeTab === 'artifacts' ? 'border-[var(--accent-primary)] text-[var(--text-primary)]' : 'border-transparent text-[var(--text-muted)] hover:text-[var(--text-secondary)]'
            }`}
          >
            <Package size={16} />
            Artifacts ({artifacts.length})
          </button>
          <button 
            onClick={() => setActiveTab('sources')}
            className={`pb-2 px-1 font-semibold text-sm border-b-2 transition-colors flex items-center gap-2 ${
              activeTab === 'sources' ? 'border-[var(--accent-primary)] text-[var(--text-primary)]' : 'border-transparent text-[var(--text-muted)] hover:text-[var(--text-secondary)]'
            }`}
          >
            <Link2 size={16} />
            Sources {activeCitations.length > 0 && `(${activeCitations.length})`}
          </button>
        </div>
        
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          {activeTab === 'artifacts' && (
            artifacts.length === 0 ? (
              <div className="flex flex-col items-center justify-center h-40 text-center opacity-50">
                <FileCode size={32} className="mb-2" />
                <p className="text-sm">No artifacts generated yet.</p>
              </div>
            ) : (
              artifacts.map((artifact) => (
                <div 
                  key={artifact.id}
                  onClick={() => setActiveArtifact(artifact.id)}
                  className={`p-3 rounded-[var(--radius-md)] border cursor-pointer transition-all duration-150 hover:-translate-y-[2px] hover:shadow-[var(--shadow-md)] ${
                    activeArtifactId === artifact.id 
                      ? 'bg-[var(--bg-elevated)] border-[var(--accent-primary)] shadow-[var(--shadow-sm)]' 
                      : 'bg-[var(--bg-base)] border-[var(--border)] hover:border-[var(--text-muted)]'
                  }`}
                >
                  <div className="flex items-center justify-between mb-2">
                    <div className="flex items-center gap-2 font-medium text-sm text-[var(--text-primary)] truncate">
                      {getIcon(artifact.type)}
                      <span className="truncate">{artifact.title}</span>
                    </div>
                    {activeArtifactId === artifact.id && (
                      <button 
                        onClick={(e) => {
                          e.stopPropagation();
                          setViewerOpen(true);
                        }}
                        className="p-1 hover:bg-[var(--bg-surface)] rounded text-[var(--text-muted)] hover:text-[var(--text-primary)] flex-shrink-0"
                      >
                        <Maximize2 size={14} />
                      </button>
                    )}
                  </div>
                  <div className="text-xs text-[var(--text-muted)] flex justify-between">
                    <span>{new Date(artifact.createdAt).toLocaleTimeString()}</span>
                    {artifact.language && <span className="uppercase">{artifact.language}</span>}
                  </div>
                </div>
              ))
            )
          )}

          {activeTab === 'sources' && (
            activeCitations.length === 0 ? (
              <div className="flex flex-col items-center justify-center h-40 text-center opacity-50">
                <Link2 size={32} className="mb-2 text-[var(--text-muted)]" />
                <p className="text-sm">No sources to display.</p>
              </div>
            ) : (
              activeCitations.map((c) => (
                <div key={c.index} className="p-3 bg-[var(--bg-base)] border border-[var(--border)] rounded-[var(--radius-md)] shadow-[var(--shadow-sm)]">
                  <div className="flex items-center gap-2 mb-2 text-sm font-semibold text-[var(--text-primary)]">
                    <span className="inline-flex items-center justify-center w-5 h-5 text-[10px] bg-[var(--accent-primary-subtle)] text-[var(--accent-primary)] rounded-[var(--radius-sm)] border border-[var(--accent-primary)]/30">
                      {c.index}
                    </span>
                    <span className="truncate flex-1">{c.source_id.split('/').pop()}</span>
                  </div>
                  <div className="text-[10px] text-[var(--text-muted)] flex items-center gap-1.5 mb-3 font-medium">
                    {renderStars(c.confidence || 0)} 
                    <span className="ml-1 opacity-70">
                      {c.confidence ? (c.confidence <= 1 ? (c.confidence * 5).toFixed(1) : c.confidence.toFixed(1)) : '0.0'}
                    </span>
                    <span className="mx-1">&middot;</span> 
                    <span className="truncate">{c.source_id.replace(/^https?:\/\//, '').split('/')[0] || 'Local File'}</span>
                  </div>
                  <div className="text-xs text-[var(--text-secondary)] leading-relaxed bg-[var(--bg-surface)] p-2 rounded border border-[var(--border-subtle)] overflow-hidden line-clamp-4">
                    "{c.excerpt}"
                  </div>
                  <div className="mt-3 flex gap-2">
                    <button className="flex items-center gap-1 text-[10px] font-medium px-2 py-1 bg-[var(--bg-elevated)] border border-[var(--border)] rounded hover:bg-[var(--bg-surface)] transition-colors text-[var(--text-primary)]">
                      <ExternalLink size={12} /> 원문 열기
                    </button>
                    <button className="flex items-center gap-1 text-[10px] font-medium px-2 py-1 bg-[var(--bg-elevated)] border border-[var(--border)] rounded hover:bg-[var(--bg-surface)] transition-colors text-[var(--text-primary)]">
                      <LayoutTemplate size={12} /> 컨텍스트 보기
                    </button>
                  </div>
                </div>
              ))
            )
          )}
        </div>
      </div>
      
      {viewerOpen && activeArtifactId && (
        <ArtifactViewerModal 
          artifact={artifacts.find(a => a.id === activeArtifactId)!}
          onClose={() => setViewerOpen(false)} 
        />
      )}
    </>
  );
};
