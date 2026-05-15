import { create } from 'zustand';

interface Project {
  id: string;
  name: string;
  description?: string;
  persona_template: string;
  model_small?: string;
  model_heavy?: string;
}

interface AppState {
  activeProject: Project | null;
  projects: Project[];
  setActiveProject: (project: Project | null) => void;
  setProjects: (projects: Project[]) => void;
  addProject: (project: Project) => void;
  isSidebarOpen: boolean;
  toggleSidebar: () => void;
}

export const useAppStore = create<AppState>((set) => ({
  activeProject: null,
  projects: [],
  setActiveProject: (project) => set({ activeProject: project }),
  setProjects: (projects) => set({ projects }),
  addProject: (project) => set((state) => ({ projects: [...state.projects, project] })),
  isSidebarOpen: true,
  toggleSidebar: () => set((state) => ({ isSidebarOpen: !state.isSidebarOpen })),
}));
