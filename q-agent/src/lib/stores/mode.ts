import { writable } from 'svelte/store';

export type ModeType = 'default' | 'academic' | 'coding' | 'finance' | 'custom';

export interface ModeInfo {
  id: ModeType;
  label: string;
  icon: string;
  description: string;
  recommendedModels: string[];
}

export const MODES: ModeInfo[] = [
  {
    id: 'default',
    label: 'Default',
    icon: '🤖',
    description: 'General purpose assistant for daily tasks.',
    recommendedModels: ['qwen3.5:4b', 'llama3.2:3b', 'llama3.2:1b']
  },
  {
    id: 'academic',
    label: 'Academic',
    icon: '📖',
    description: 'Literature review, paper summary, and citations.',
    recommendedModels: ['gemma3:4b', 'phi-4', 'gemma3:12b']
  },
  {
    id: 'coding',
    label: 'Coding',
    icon: '💻',
    description: 'Code generation, debugging, and review.',
    recommendedModels: ['qwen2.5-coder:7b', 'deepseek-coder:6.7b', 'deepseek-coder-v2:16b']
  },
  {
    id: 'finance',
    label: 'Finance',
    icon: '📈',
    description: 'Financial news, stock analysis, and charts.',
    recommendedModels: ['qwen3.5:4b', 'llama3.2:3b']
  },
  {
    id: 'custom',
    label: 'Custom',
    icon: '✏️',
    description: 'Your personalized assistant configuration.',
    recommendedModels: []
  }
];

function createModeStore() {
  const { subscribe, set, update } = writable<ModeType>('default');

  return {
    subscribe,
    set: (mode: ModeType) => set(mode),
    getModeInfo: (id: ModeType) => MODES.find(m => m.id === id) || MODES[0]
  };
}

export const modeStore = createModeStore();
