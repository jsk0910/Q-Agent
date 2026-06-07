import React, { useState, useEffect } from 'react';
import { useAppStore } from '../../stores/useAppStore';
import { PanelLeftClose, PanelLeftOpen, FolderOpen, Database, Settings, Minimize2, Settings2, Users, Moon, Sun } from 'lucide-react';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { SettingsModal } from '../settings/SettingsModal';
import { HarnessStudioModal } from '../harness/HarnessStudioModal';
import { CreateProjectModal } from './CreateProjectModal';

export const Sidebar: React.FC = () => {
  const { isSidebarOpen, toggleSidebar, projects, activeProject, setActiveProject } = useAppStore();
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [isHarnessOpen, setIsHarnessOpen] = useState(false);
  const [isCreateProjectOpen, setIsCreateProjectOpen] = useState(false);
  const [isDarkMode, setIsDarkMode] = useState(() => document.documentElement.classList.contains('dark'));

  const toggleDarkMode = () => {
    const isDark = document.documentElement.classList.toggle('dark');
    setIsDarkMode(isDark);
  };

  useEffect(() => {
    if ('__TAURI_INTERNALS__' in window) {
      useAppStore.getState().loadProjects();
    }
  }, []);

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
        <div data-tauri-drag-region className={`flex items-center p-3 border-b border-[var(--border)] ${isSidebarOpen ? 'justify-between' : 'justify-center flex-col gap-2'}`}>
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
          <div className={`flex ${isSidebarOpen ? 'gap-1' : 'flex-col gap-2 mt-2'}`}>
            <button 
              onClick={minimizeToHud}
              title="HUD 모드로 축소"
              className="p-1.5 hover:bg-[var(--bg-elevated)] rounded-md transition-colors flex items-center justify-center"
            >
              <Minimize2 size={18} />
            </button>
            <button 
              onClick={toggleSidebar}
              title={isSidebarOpen ? "Collapse Sidebar" : "Expand Sidebar"}
              className="p-1.5 hover:bg-[var(--bg-elevated)] rounded-md transition-colors flex items-center justify-center"
            >
              {isSidebarOpen ? <PanelLeftClose size={18} /> : <PanelLeftOpen size={18} />}
            </button>
          </div>
        </div>

        {isSidebarOpen ? (
          <div className="flex-1 overflow-y-auto p-3 space-y-6">
            {/* Projects Section */}
            <div>
              <div className="flex items-center gap-2 mb-2 px-1 text-xs font-semibold uppercase tracking-wider">
                <FolderOpen size={14} />
                <span>Projects</span>
              </div>
              <div className="space-y-2">
                {projects.length === 0 ? (
                  <div 
                    className="text-sm px-2 py-3 bg-[var(--bg-elevated)] rounded-[var(--radius-md)] border border-[var(--border)] shadow-sm text-center cursor-pointer hover:border-[var(--accent-primary)] transition-colors"
                    onClick={() => setIsCreateProjectOpen(true)}
                  >
                    + New Project
                  </div>
                ) : (
                  projects.map(p => {
                    const pid = typeof p.id === 'string' ? p.id : (p.id?.id?.String || p.id?.id || p.id);
                    return (
                      <div 
                        key={pid} 
                        onClick={() => setActiveProject(pid)}
                        className={`p-3 rounded-[var(--radius-md)] border shadow-[var(--shadow-sm)] cursor-pointer transition-all duration-150 ${
                          activeProject?.id === p.id || activeProject?.id?.id?.String === pid
                            ? 'border-[var(--accent-primary)] bg-[var(--bg-surface)] shadow-[var(--shadow-md)]' 
                            : 'border-[var(--border)] bg-[var(--bg-elevated)] hover:border-[var(--text-muted)]'
                        }`}
                      >
                        <div className="font-medium text-[var(--text-primary)]">{p.name}</div>
                        <div className="text-xs mt-1 text-[var(--text-muted)] truncate">
                          {p.persona_template}
                        </div>
                      </div>
                    );
                  })
                )}
                {projects.length > 0 && (
                  <div 
                    className="mt-2 text-xs px-2 py-2 bg-[var(--bg-elevated)] rounded-[var(--radius-md)] border border-dashed border-[var(--border)] text-center cursor-pointer hover:border-[var(--accent-primary)] transition-colors text-[var(--text-muted)] hover:text-[var(--text-primary)]"
                    onClick={() => setIsCreateProjectOpen(true)}
                  >
                    + Create Project
                  </div>
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

            {/* Sessions Section */}
            <div>
              <div className="flex items-center gap-2 mb-2 px-1 text-xs font-semibold uppercase tracking-wider">
                <Users size={14} />
                <span>Sessions</span>
              </div>
              <div className="text-xs p-3 border border-[var(--border)] bg-[var(--bg-elevated)] rounded-[var(--radius-md)] text-center text-[var(--text-muted)]">
                No active sessions
              </div>
            </div>
          </div>
        ) : (
          <div className="flex-1 flex flex-col items-center gap-4 py-4 overflow-y-auto">
            <button title="Projects" className="p-2 hover:bg-[var(--bg-elevated)] rounded-md transition-colors" onClick={toggleSidebar}>
              <FolderOpen size={20} className="text-[var(--text-muted)]" />
            </button>
            <button title="Knowledge Base" className="p-2 hover:bg-[var(--bg-elevated)] rounded-md transition-colors" onClick={toggleSidebar}>
              <Database size={20} className="text-[var(--text-muted)]" />
            </button>
            <button title="Sessions" className="p-2 hover:bg-[var(--bg-elevated)] rounded-md transition-colors" onClick={toggleSidebar}>
              <Users size={20} className="text-[var(--text-muted)]" />
            </button>
          </div>
        )}
        
        {/* Settings at bottom */}
        <div className={`p-3 border-t border-[var(--border)] flex flex-col gap-2 ${isSidebarOpen ? '' : 'items-center'}`}>
           <div 
              className={`flex items-center gap-2 p-2 cursor-pointer hover:bg-[var(--bg-elevated)] rounded-md transition-colors ${isSidebarOpen ? 'w-full px-2' : 'justify-center'}`}
              onClick={toggleDarkMode}
              title={isDarkMode ? "Light Mode" : "Dark Mode"}
           >
             {isDarkMode ? (
               <Sun size={isSidebarOpen ? 16 : 20} className={isSidebarOpen ? '' : 'text-[var(--text-muted)]'} />
             ) : (
               <Moon size={isSidebarOpen ? 16 : 20} className={isSidebarOpen ? '' : 'text-[var(--text-muted)]'} />
             )}
             {isSidebarOpen && <span className="text-sm font-medium">{isDarkMode ? 'Light Mode' : 'Dark Mode'}</span>}
           </div>
           <div 
              className={`flex items-center gap-2 p-2 cursor-pointer hover:bg-[var(--bg-elevated)] rounded-md transition-colors ${isSidebarOpen ? 'w-full px-2' : 'justify-center'}`}
              onClick={() => setIsSettingsOpen(true)}
              title="Settings"
           >
             <Settings size={isSidebarOpen ? 16 : 20} className={isSidebarOpen ? '' : 'text-[var(--text-muted)]'} />
             {isSidebarOpen && <span className="text-sm font-medium">Settings</span>}
           </div>
        </div>
      </div>

      <SettingsModal 
        isOpen={isSettingsOpen} 
        onClose={() => setIsSettingsOpen(false)} 
      />

      <HarnessStudioModal
        isOpen={isHarnessOpen}
        onClose={() => setIsHarnessOpen(false)}
      />

      <CreateProjectModal
        isOpen={isCreateProjectOpen}
        onClose={() => setIsCreateProjectOpen(false)}
      />
    </>
  );
};
