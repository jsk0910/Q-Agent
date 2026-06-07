import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface Project {
  id: any;
  name: string;
  description?: string;
  persona_template: string;
  harness?: string;
  model_small?: string;
  model_heavy?: string;
}

export interface MessageCitation {
  index: number;
  source_id: string;
  excerpt: string;
  confidence: number;
}

export interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  traces?: { agent: string; step: string }[];
  citations?: MessageCitation[];
}

interface AppState {
  // Existing state
  activeProject: Project | null;
  projects: Project[];
  setActiveProject: (projectId: string | null) => Promise<void>;
  setProjects: (projects: Project[]) => void;
  addProject: (project: Project) => void;
  loadProjects: () => Promise<void>;
  isSidebarOpen: boolean;
  toggleSidebar: () => void;
  
  // New state
  isRightPanelOpen: boolean;
  networkStatus: 'local' | 'lan' | 'offline';
  permissionMode: 'strict' | 'balanced' | 'agentic';
  activeCitations: MessageCitation[];
  vramInfo: { total: number; used: number } | null;
  messages: Message[];
  
  // New actions
  openRightPanel: () => void;
  closeRightPanel: () => void;
  setNetworkStatus: (status: 'local' | 'lan' | 'offline') => void;
  setPermissionMode: (mode: 'strict' | 'balanced' | 'agentic') => void;
  setActiveCitations: (citations: MessageCitation[]) => void;
  setVramInfo: (info: { total: number; used: number } | null) => void;
  setMessages: (updater: Message[] | ((prev: Message[]) => Message[])) => void;
}

export const useAppStore = create<AppState>((set) => ({
  activeProject: null,
  projects: [],
  setActiveProject: async (projectId) => {
    set((state) => {
      const activeProj = projectId 
        ? state.projects.find((p) => {
            const pid = typeof p.id === 'string' ? p.id : (p.id?.id?.String || p.id?.id || p.id);
            return pid === projectId;
          }) || null 
        : null;
      return { activeProject: activeProj };
    });
    
    // Fetch history
    if (projectId) {
      try {
        set({ messages: [] });
        const history = await invoke('get_project_history', { projectId }) as any[];
        if (history && history.length > 0) {
          set({ messages: history });
        } else {
          set({
            messages: [{
              id: '1',
              role: 'assistant',
              content: '안녕하세요! 새로운 프로젝트입니다. 무엇을 도와드릴까요?',
            }]
          });
        }
      } catch (e) {
        console.error('Failed to load project history', e);
      }
    }
  },
  setProjects: (projects) => set({ projects }),
  addProject: (project) => set((state) => ({ projects: [...state.projects, project] })),
  loadProjects: async () => {
    try {
      const projects = await invoke('list_projects') as Project[];
      set({ projects });
    } catch (e) {
      console.error('Failed to list projects', e);
    }
  },
  isSidebarOpen: true,
  toggleSidebar: () => set((state) => ({ isSidebarOpen: !state.isSidebarOpen })),
  
  isRightPanelOpen: false,
  networkStatus: 'local',
  permissionMode: 'balanced',
  activeCitations: [],
  vramInfo: null,
  messages: [
    {
      id: '1',
      role: 'assistant',
      content: '안녕하세요! 저는 Q-Agent입니다. 무엇을 도와드릴까요?',
      traces: [
        { agent: 'Planner', step: 'Initializing context and plugins...' },
        { agent: 'MLOps', step: 'Checking model VRAM allocation: OK' }
      ]
    }
  ],
  
  openRightPanel: () => set({ isRightPanelOpen: true }),
  closeRightPanel: () => set({ isRightPanelOpen: false }),
  setNetworkStatus: (status) => set({ networkStatus: status }),
  setPermissionMode: (mode) => set({ permissionMode: mode }),
  setActiveCitations: (citations) => set({ activeCitations: citations }),
  setVramInfo: (info) => set({ vramInfo: info }),
  setMessages: (updater) => set((state) => ({
    messages: typeof updater === 'function' ? updater(state.messages) : updater
  })),
}));
