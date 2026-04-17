import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { modeStore, type ModeType } from './mode';

export interface ModeModelMap {
  default: string;
  academic: string;
  coding: string;
  finance: string;
  custom: string;
}

export interface UserProfile {
  id: string;
  name: string;
  avatar: string; // Emoji or icon
  mode_models: ModeModelMap;
  theme: string;
  system_prompts: Record<string, string>;
  created_at: string;
}

const defaultProfile: UserProfile = {
  id: 'default',
  name: 'Default User',
  avatar: '👤',
  mode_models: {
    default: 'qwen3.5:4b',
    academic: 'gemma3:4b',
    coding: 'qwen2.5-coder:7b',
    finance: 'qwen3.5:4b',
    custom: 'qwen3.5:4b',
  },
  theme: 'dark',
  system_prompts: {},
  created_at: new Date().toISOString(),
};

function createProfileStore() {
  const { subscribe, set, update } = writable<{
    profiles: UserProfile[];
    activeId: string;
  }>({
    profiles: [defaultProfile],
    activeId: 'default',
  });

  async function load() {
    try {
      const remoteProfiles = await invoke<UserProfile[]>('load_profiles');
      if (remoteProfiles && remoteProfiles.length > 0) {
        // Load active ID from localStorage
        const activeId = localStorage.getItem('q-agent-active-profile') || remoteProfiles[0].id;
        set({ profiles: remoteProfiles, activeId });
      } else {
        // Initial save of default profile
        await invoke('save_profile', { profile: defaultProfile });
      }
    } catch (e) {
      console.error('Failed to load profiles', e);
    }
  }

  return {
    subscribe,
    load,
    setActive: (id: string) => {
      update(s => {
        localStorage.setItem('q-agent-active-profile', id);
        return { ...s, activeId: id };
      });
    },
    updateActiveProfile: async (patch: Partial<UserProfile>) => {
      let updated: UserProfile | null = null;
      update(s => {
        const profiles = s.profiles.map(p => {
          if (p.id === s.activeId) {
            updated = { ...p, ...patch };
            return updated;
          }
          return p;
        });
        return { ...s, profiles };
      });
      if (updated) {
        await invoke('save_profile', { profile: updated });
      }
    },
    setModelForMode: async (mode: ModeType, model: string) => {
      let updated: UserProfile | null = null;
      update(s => {
        const profiles = s.profiles.map(p => {
          if (p.id === s.activeId) {
            updated = {
              ...p,
              mode_models: { ...p.mode_models, [mode]: model }
            };
            return updated;
          }
          return p;
        });
        return { ...s, profiles };
      });
      if (updated) {
        await invoke('save_profile', { profile: updated });
      }
    },
    getActiveProfile: () => {
      const s = get(profileStore);
      return s.profiles.find(p => p.id === s.activeId) || defaultProfile;
    },
    getCurrentModel: () => {
      const p = profileStore.getActiveProfile();
      const mode = get(modeStore);
      return p.mode_models[mode] || p.mode_models.default;
    }
  };
}

export const profileStore = createProfileStore();
