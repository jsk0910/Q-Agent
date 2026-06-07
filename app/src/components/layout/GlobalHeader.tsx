import React from 'react';
import { useAppStore } from '../../stores/useAppStore';
import { ChevronDown, Database, Shield, Wifi, WifiOff, Sidebar } from 'lucide-react';

export const GlobalHeader: React.FC = () => {
  const { 
    activeProject, 
    permissionMode, 
    networkStatus, 
    vramInfo,
    isRightPanelOpen
  } = useAppStore();

  const handleTogglePanel = () => {
    if (isRightPanelOpen) {
      useAppStore.getState().closeRightPanel();
    } else {
      useAppStore.getState().openRightPanel();
    }
  };

  return (
    <header className="h-[48px] flex-shrink-0 flex items-center justify-between px-4 border-b border-[var(--border)] bg-[var(--bg-surface)]">
      {/* 좌측: 프로젝트 정보 */}
      <div className="flex items-center gap-2">
        <button className="flex items-center gap-2 text-sm font-medium hover:bg-[var(--bg-subtle)] px-2 py-1 rounded-md transition-colors">
          <span>{activeProject ? activeProject.name : 'No Project Selected'}</span>
          <ChevronDown size={14} className="text-[var(--text-muted)]" />
        </button>
      </div>

      {/* 중앙: 실행 모드 */}
      <div className="flex items-center gap-2">
        <div className="flex items-center gap-1.5 px-2.5 py-1 text-xs font-medium bg-[var(--bg-subtle)] text-[var(--text-secondary)] rounded-full">
          <Shield size={12} className={
            permissionMode === 'strict' ? 'text-[var(--accent-danger)]' :
            permissionMode === 'agentic' ? 'text-[var(--accent-warning)]' :
            'text-[var(--accent-success)]'
          } />
          <span className="capitalize">{permissionMode} Mode</span>
        </div>
      </div>

      {/* 우측: 상태 및 패널 토글 */}
      <div className="flex items-center gap-3 text-xs text-[var(--text-secondary)]">
        {/* 네트워크 배지 */}
        <div className="flex items-center gap-1.5" title="Network Status">
          {networkStatus === 'offline' ? (
            <WifiOff size={14} className="text-[var(--text-muted)]" />
          ) : (
            <Wifi size={14} className={networkStatus === 'local' ? 'text-[var(--accent-success)]' : 'text-[var(--accent-primary)]'} />
          )}
          <span className="capitalize">{networkStatus}</span>
        </div>

        <div className="w-[1px] h-4 bg-[var(--border)] mx-1" />

        {/* VRAM 배지 */}
        <div className="flex items-center gap-1.5" title="VRAM Usage">
          <Database size={14} className="text-[var(--text-muted)]" />
          <span>
            {vramInfo ? `${vramInfo.used.toFixed(1)} / ${vramInfo.total.toFixed(1)}GB` : 'N/A'}
          </span>
        </div>

        <div className="w-[1px] h-4 bg-[var(--border)] mx-1" />

        {/* 우측 패널 토글 */}
        <button 
          onClick={handleTogglePanel}
          className={`p-1.5 rounded-md transition-colors ${isRightPanelOpen ? 'bg-[var(--accent-primary-subtle)] text-[var(--accent-primary)]' : 'hover:bg-[var(--bg-subtle)] text-[var(--text-muted)]'}`}
          title="Toggle Right Panel"
        >
          <Sidebar size={16} />
        </button>
      </div>
    </header>
  );
};
