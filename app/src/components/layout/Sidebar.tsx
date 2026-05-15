import React, { useState } from 'react';
import { useAppStore } from '../../stores/useAppStore';
import { PanelLeftClose, PanelLeftOpen, FolderOpen, Database, Settings, Minimize2, Settings2 } from 'lucide-react';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { SettingsModal } from '../settings/SettingsModal';
import { HarnessStudioModal } from '../harness/HarnessStudioModal';

export const Sidebar: React.FC = () => {
  const { isSidebarOpen, toggleSidebar, projects, activeProject } = useAppStore();
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [isHarnessOpen, setIsHarnessOpen] = useState(false);

  const minimizeToHud = async () => {
    try {
      const appWindow = getCurrentWindow();
      await appWindow.setSize(new LogicalSize(680, 180));
      await appWindow.center();
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <>
      <div className="flex flex-col h-full text-[var(--text-secondary)] bg-[var(--bg-surface)]">
        <div className="flex items-center justify-between p-3 border-b border-[var(--border)]">
          {isSidebarOpen && (
            <div 
              className="flex items-center gap-2 cursor-pointer hover:text-[var(--accent-primary)] transition-colors"
              onClick={() => setIsHarnessOpen(true)}
              title="Open Harness Studio"
            >
              <span className="font-semibold text-[var(--text-primary)]">Harness Studio</span>
              <Settings2 size={14} />
            </div>
          )}
          <div className="flex gap-1">
            <button 
              onClick={minimizeToHud}
              title="HUD 모드로 축소"
              className="p-1 hover:bg-[var(--bg-elevated)] rounded-md transition-colors"
            >
              <Minimize2 size={18} />
            </button>
            <button 
              onClick={toggleSidebar}
              className="p-1 hover:bg-[var(--bg-elevated)] rounded-md transition-colors"
            >
              {isSidebarOpen ? <PanelLeftClose size={18} /> : <PanelLeftOpen size={18} />}
            </button>
          </div>
        </div>

        {isSidebarOpen && (
          <div className="flex-1 overflow-y-auto p-3 space-y-6">
            {/* Projects Section */}
            <div>
              <div className="flex items-center gap-2 mb-2 px-1 text-xs font-semibold uppercase tracking-wider">
                <FolderOpen size={14} />
                <span>Projects</span>
              </div>
              <div className="space-y-2">
                {projects.length === 0 ? (
                  <div className="text-sm px-2 py-3 bg-[var(--bg-elevated)] rounded-[var(--radius-md)] border border-[var(--border)] shadow-sm text-center cursor-pointer hover:border-[var(--accent-primary)] transition-colors">
                    + New Project
                  </div>
                ) : (
                  projects.map(p => (
                    <div 
                      key={p.id} 
                      className={`p-3 rounded-[var(--radius-md)] border shadow-sm cursor-pointer transition-all duration-150 ${
                        activeProject?.id === p.id 
                          ? 'border-[var(--accent-primary)] bg-[var(--bg-surface)] shadow-md' 
                          : 'border-[var(--border)] bg-[var(--bg-elevated)] hover:border-[var(--text-muted)]'
                      }`}
                    >
                      <div className="font-medium text-[var(--text-primary)]">{p.name}</div>
                      <div className="text-xs mt-1 text-[var(--text-muted)] truncate">
                        {p.persona_template}
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>

            {/* Local NotebookLM Section */}
            <div>
              <div className="flex items-center gap-2 mb-2 px-1 text-xs font-semibold uppercase tracking-wider">
                <Database size={14} />
                <span>Knowledge Base</span>
              </div>
              <div className="text-xs p-3 border border-dashed border-[var(--border)] rounded-[var(--radius-md)] text-center text-[var(--text-muted)]">
                Drag & Drop files here
              </div>
            </div>
          </div>
        )}
        
        {/* Settings at bottom */}
        {isSidebarOpen && (
          <div className="p-3 border-t border-[var(--border)]">
             <div 
                className="flex items-center gap-2 px-2 py-2 cursor-pointer hover:bg-[var(--bg-elevated)] rounded-md transition-colors"
                onClick={() => setIsSettingsOpen(true)}
             >
               <Settings size={16} />
               <span className="text-sm">Settings</span>
             </div>
          </div>
        )}
      </div>

      <SettingsModal 
        isOpen={isSettingsOpen} 
        onClose={() => setIsSettingsOpen(false)} 
      />

      <HarnessStudioModal
        isOpen={isHarnessOpen}
        onClose={() => setIsHarnessOpen(false)}
      />
    </>
  );
};
