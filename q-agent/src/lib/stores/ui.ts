// UI customization store
import { writable, derived, get } from 'svelte/store';
import { profileStore } from './profile';
import { modeStore } from './mode';

export type Theme = 'dark' | 'light' | 'oled';
export type AccentColor = 'teal' | 'violet' | 'orange' | 'green' | 'pink';
export type FontSize = 'sm' | 'md' | 'lg';
export type SidebarPosition = 'left' | 'right';

export interface UISettings {
  theme: Theme;
  accent: AccentColor;
  fontSize: FontSize;
  sidebarCollapsed: boolean;
  sendOnEnter: boolean;
  showThinking: boolean;
  streamTokens: boolean;
  temperature: number;
}

const defaultSettings: UISettings = {
  theme: 'dark',
  accent: 'teal',
  fontSize: 'md',
  sidebarCollapsed: false,
  sendOnEnter: true,
  showThinking: true,
  streamTokens: true,
  temperature: 0.7,
};

// Load from localStorage if available
function loadSettings(): UISettings {
  if (typeof window === 'undefined') return defaultSettings;
  try {
    const stored = localStorage.getItem('q-agent-settings');
    if (stored) return { ...defaultSettings, ...JSON.parse(stored) };
  } catch {}
  return defaultSettings;
}

function createSettingsStore() {
  const { subscribe, set, update } = writable<UISettings>(loadSettings());

  return {
    subscribe,
    update: (fn: (s: UISettings) => UISettings) => {
      update(s => {
        const next = fn(s);
        if (typeof window !== 'undefined') {
          localStorage.setItem('q-agent-settings', JSON.stringify(next));
        }
        return next;
      });
    },
    set: (s: UISettings) => {
      if (typeof window !== 'undefined') {
        localStorage.setItem('q-agent-settings', JSON.stringify(s));
      }
      set(s);
    },
    reset: () => {
      if (typeof window !== 'undefined') {
        localStorage.removeItem('q-agent-settings');
      }
      set(defaultSettings);
    },
  };
}

export const settings = createSettingsStore();

// Derived store to get the "Effective Settings" combining UI global settings and Profile-specific ones
export const effectiveSettings = derived(
  [settings, profileStore, modeStore],
  ([$settings, $profile, $mode]) => {
    const activeProfile = $profile.profiles.find(p => p.id === $profile.activeId);
    return {
      ...$settings,
      theme: (activeProfile?.theme as Theme) || $settings.theme,
      modelName: activeProfile?.mode_models[$mode] || activeProfile?.mode_models.default || 'qwen3.5:4b',
      systemPrompt: activeProfile?.system_prompts[$mode] || 'You are Q-Agent, a helpful AI research and development assistant.'
    };
  }
);

// Chat history
export interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  thinking?: string;
  timestamp: Date;
  model?: string;
  tokens?: number;
}

export interface Conversation {
  id: string;
  title: string;
  messages: Message[];
  createdAt: Date;
  updatedAt: Date;
  pinned?: boolean;
}

function loadChatState() {
  if (typeof window === 'undefined') return { conversations: [], activeId: null };
  try {
    const stored = localStorage.getItem('q-agent-chats');
    if (stored) {
      const parsed = JSON.parse(stored);
      parsed.conversations.forEach((c: any) => {
        if (c.createdAt) c.createdAt = new Date(c.createdAt);
        if (c.updatedAt) c.updatedAt = new Date(c.updatedAt);
        c.messages.forEach((m: any) => {
          if (m.timestamp) m.timestamp = new Date(m.timestamp);
        });
      });
      return parsed;
    }
  } catch (e) {
    console.error('Failed to load chat store', e);
  }
  return { conversations: [], activeId: null };
}

function createChatStore() {
  const { subscribe, set, update: rawUpdate } = writable<{
    conversations: Conversation[];
    activeId: string | null;
  }>(loadChatState());

  const update = (fn: any) => {
    rawUpdate(s => {
      const next = fn(s);
      if (typeof window !== 'undefined') {
        localStorage.setItem('q-agent-chats', JSON.stringify(next));
      }
      return next;
    });
  };

  const persistSet = (val: any) => {
    if (typeof window !== 'undefined') {
      localStorage.setItem('q-agent-chats', JSON.stringify(val));
    }
    set(val);
  };

  return {
    subscribe,
    newConversation: () => {
      const id = crypto.randomUUID();
      const conv: Conversation = {
        id,
        title: 'New Chat',
        messages: [],
        createdAt: new Date(),
        updatedAt: new Date(),
      };
      update(s => ({
        conversations: [conv, ...s.conversations],
        activeId: id,
      }));
      return id;
    },
    setActive: (id: string) => update(s => ({ ...s, activeId: id })),
    addMessage: (convId: string, msg: Message) => {
      update(s => ({
        ...s,
        conversations: s.conversations.map(c =>
          c.id === convId
            ? { ...c, messages: [...c.messages, msg], updatedAt: new Date() }
            : c
        ),
      }));
    },
    updateMessage: (convId: string, msgId: string, patch: Partial<Message>) => {
      update(s => ({
        ...s,
        conversations: s.conversations.map(c =>
          c.id === convId
            ? {
                ...c,
                messages: c.messages.map(m =>
                  m.id === msgId ? { ...m, ...patch } : m
                ),
              }
            : c
        ),
      }));
    },
    renameConversation: (id: string, title: string) => {
      update(s => ({
        ...s,
        conversations: s.conversations.map(c =>
          c.id === id ? { ...c, title } : c
        ),
      }));
    },
    deleteConversation: (id: string) => {
      update(s => {
        const conversations = s.conversations.filter(c => c.id !== id);
        const activeId = s.activeId === id
          ? (conversations[0]?.id ?? null)
          : s.activeId;
        return { conversations, activeId };
      });
    },
    pinConversation: (id: string) => {
      update(s => ({
        ...s,
        conversations: s.conversations.map(c =>
          c.id === id ? { ...c, pinned: !c.pinned } : c
        ),
      }));
    },
    clear: () => persistSet({ conversations: [], activeId: null }),
  };
}

export const chatStore = createChatStore();

// Derived: active conversation
export const activeConversation = derived(chatStore, $s =>
  $s.conversations.find(c => c.id === $s.activeId) ?? null
);
