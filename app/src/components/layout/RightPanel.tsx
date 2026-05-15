import React, { useState } from 'react';
import { Package, FileCode, FileText, LayoutTemplate, Maximize2 } from 'lucide-react';
import { useArtifactStore } from '../../stores/useArtifactStore';
import { ArtifactViewerModal } from './ArtifactViewerModal';

export const RightPanel: React.FC = () => {
  const { artifacts, activeArtifactId, setActiveArtifact } = useArtifactStore();
  const [viewerOpen, setViewerOpen] = useState(false);

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
        <div className="p-3 border-b border-[var(--border)] flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Package size={18} className="text-[var(--accent-primary)]" />
            <span className="font-semibold text-[var(--text-primary)]">Artifacts ({artifacts.length})</span>
          </div>
        </div>
        
        <div className="flex-1 overflow-y-auto p-4 space-y-3">
          {artifacts.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-40 text-center opacity-50">
              <FileCode size={32} className="mb-2" />
              <p className="text-sm">No artifacts generated yet.</p>
            </div>
          ) : (
            artifacts.map((artifact) => (
              <div 
                key={artifact.id}
                onClick={() => setActiveArtifact(artifact.id)}
                className={`p-3 rounded-md border cursor-pointer transition-all ${
                  activeArtifactId === artifact.id 
                    ? 'bg-[var(--bg-elevated)] border-[var(--accent-primary)] shadow-sm' 
                    : 'bg-[var(--bg-base)] border-[var(--border)] hover:border-gray-500'
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
