import { create } from 'zustand';

export interface Artifact {
  id: string;
  title: string;
  type: 'code' | 'markdown' | 'plan';
  content: string;
  language?: string;
  createdAt: number;
}

interface ArtifactState {
  artifacts: Artifact[];
  activeArtifactId: string | null;
  addArtifact: (artifact: Omit<Artifact, 'id' | 'createdAt'>) => void;
  setActiveArtifact: (id: string | null) => void;
  updateArtifact: (id: string, content: string) => void;
  clearArtifacts: () => void;
}

export const useArtifactStore = create<ArtifactState>((set) => ({
  artifacts: [],
  activeArtifactId: null,
  addArtifact: (artifact) => set((state) => {
    const newArtifact = {
      ...artifact,
      id: Math.random().toString(36).substring(2, 9),
      createdAt: Date.now(),
    };
    return {
      artifacts: [newArtifact, ...state.artifacts],
      activeArtifactId: newArtifact.id,
    };
  }),
  setActiveArtifact: (id) => set({ activeArtifactId: id }),
  updateArtifact: (id, content) => set((state) => ({
    artifacts: state.artifacts.map(a => a.id === id ? { ...a, content } : a)
  })),
  clearArtifacts: () => set({ artifacts: [], activeArtifactId: null }),
}));
