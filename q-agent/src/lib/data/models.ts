export interface ModelMetadata {
  id: string;
  name: string;
  provider: string;
  parameters: string;
  description: string;
  useCase: string;
  tags: string[];
  capabilities: string[];
  recommendation?: string;
}

export const ALL_MODELS: ModelMetadata[] = [
  {
    id: 'qwen3.5:1.5b',
    name: 'Qwen 3.5 1.5B',
    provider: 'Alibaba',
    parameters: '1.5B',
    description: 'Ultra-lightweight and fast model optimized for casual tasks.',
    useCase: 'Chit-chat, basic reasoning, low-end hardware.',
    tags: ['Fast', 'Lightweight'],
    capabilities: ['Chat', 'Reasoning']
  },
  {
    id: 'qwen3.5:4b',
    name: 'Qwen 3.5 4B',
    provider: 'Alibaba',
    parameters: '4B',
    description: 'Best balance of speed and intelligence for local operation.',
    useCase: 'General assistant, summarization, research.',
    tags: ['Balanced', 'Popular'],
    capabilities: ['Chat', 'RAG', 'Reasoning'],
    recommendation: 'Target model for Q-Agent'
  },
  {
    id: 'qwen2.5-coder:7b',
    name: 'Qwen 2.5 Coder 7B',
    provider: 'Alibaba',
    parameters: '7B',
    description: 'Specialized model for software development and debugging.',
    useCase: 'Coding, scripts, debugging, architectural advice.',
    tags: ['Coding', 'Powerful'],
    capabilities: ['Code Generation', 'Debugging']
  },
  {
    id: 'llama3.2:1b',
    name: 'Llama 3.2 1B',
    provider: 'Meta',
    parameters: '1B',
    description: 'Metas smallest and fastest open weights model.',
    useCase: 'Mobile devices, fast response, simple extraction.',
    tags: ['Tiny', 'Meta'],
    capabilities: ['Chat']
  },
  {
    id: 'llama3.2:3b',
    name: 'Llama 3.2 3B',
    provider: 'Meta',
    parameters: '3B',
    description: 'Versatile model with solid reasoning performance.',
    useCase: 'Daily errands, email drafting, creative writing.',
    tags: ['Versatile', 'Meta'],
    capabilities: ['Chat', 'Reasoning']
  },
  {
    id: 'gemma3:4b',
    name: 'Gemma 3 4B',
    provider: 'Google',
    parameters: '4B',
    description: 'Googles latest open model with high factual accuracy.',
    useCase: 'Academic research, fact-checking, summarization.',
    tags: ['Research', 'Google'],
    capabilities: ['Academic', 'RAG']
  },
  {
    id: 'gemma3:12b',
    name: 'Gemma 3 12B',
    provider: 'Google',
    parameters: '12B',
    description: 'Higher reasoning capability for complex problem solving.',
    useCase: 'Complex logic, deep analysis, professional research.',
    tags: ['Powerful', 'Logic'],
    capabilities: ['Academic', 'Reasoning', 'Complex Tasks']
  },
  {
    id: 'phi-4',
    name: 'Phi-4',
    provider: 'Microsoft',
    parameters: '14B',
    description: 'State-of-the-art small language model from Microsoft.',
    useCase: 'High-end local reasoning, instruction following.',
    tags: ['SOTA', 'Microsoft'],
    capabilities: ['Reasoning', 'Math', 'Coding']
  },
  {
    id: 'deepseek-coder-v2:16b',
    name: 'DeepSeek Coder V2 16B',
    provider: 'DeepSeek',
    parameters: '16B',
    description: 'One of the best coding models available for local use.',
    useCase: 'Pro-level coding, multi-language support.',
    tags: ['Pro-Coding', 'Large'],
    capabilities: ['Expert Coding', 'Deep Analysis']
  }
];
