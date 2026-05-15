import React from 'react';
import { Command, Send, Activity, Maximize2 } from 'lucide-react';
import { useAppStore } from '../../stores/useAppStore';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';

export const Hud: React.FC = () => {
  const { activeProject } = useAppStore();

  const expandToDashboard = async () => {
    try {
      const appWindow = getCurrentWindow();
      await appWindow.setSize(new LogicalSize(1200, 800));
      await appWindow.center();
      await appWindow.setFocus();
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="w-full h-full flex flex-col bg-[var(--bg-overlay)] backdrop-blur-xl border border-[var(--border)] rounded-[var(--radius-lg)] shadow-glass overflow-hidden">
      {/* HUD Header */}
      <div className="flex items-center justify-between px-4 py-2 border-b border-[var(--border)]">
        <div className="flex items-center gap-2 flex-1 h-full cursor-move" data-tauri-drag-region>
          <Command size={16} className="text-[var(--accent-primary)] pointer-events-none" />
          <span className="text-sm font-semibold text-[var(--text-primary)] select-none pointer-events-none">
            Q-Agent HUD
          </span>
          {activeProject && (
            <span className="text-xs px-2 py-0.5 bg-[var(--bg-elevated)] text-[var(--text-secondary)] rounded-full pointer-events-none">
              {activeProject.name}
            </span>
          )}
        </div>
        
        {/* Resource Monitor & Controls */}
        <div className="flex items-center gap-4 text-xs text-[var(--text-muted)] select-none z-10">
          <div className="flex items-center gap-1">
            <Activity size={14} className="text-[var(--accent-success)]" />
            <span>Mem: 42MB</span>
          </div>
          <button 
            onClick={expandToDashboard}
            title="대시보드로 확장"
            className="p-1 hover:text-[var(--text-primary)] hover:bg-[var(--bg-elevated)] rounded-md transition-colors"
          >
            <Maximize2 size={14} />
          </button>
        </div>
      </div>

      {/* Input Area */}
      <div className="flex-1 p-4 flex flex-col justify-end">
        <div className="flex items-center bg-[var(--bg-surface)] border border-[var(--border)] rounded-[var(--radius-md)] p-2 shadow-sm focus-within:border-[var(--accent-primary)] focus-within:shadow-glow transition-all">
          <input 
            type="text" 
            placeholder="Type a command or ask a question..." 
            className="flex-1 bg-transparent outline-none text-[var(--text-primary)] placeholder-[var(--text-muted)] text-sm px-2"
          />
          <button className="p-1.5 bg-[var(--accent-primary)] text-white rounded-[var(--radius-sm)] hover:bg-[var(--accent-secondary)] transition-colors">
            <Send size={14} />
          </button>
        </div>
      </div>
    </div>
  );
};
