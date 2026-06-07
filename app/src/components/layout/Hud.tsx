import React, { useState } from 'react';
import { Command, Send, Maximize2, FolderPlus, Globe, ChevronDown } from 'lucide-react';
import { useAppStore, Message } from '../../stores/useAppStore';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';

export const Hud: React.FC = () => {
  const { activeProject, networkStatus, vramInfo, setMessages, messages, permissionMode } = useAppStore();
  const [input, setInput] = useState('');

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

  const handleSend = async () => {
    const messageToSend = input.trim();
    if (!messageToSend) return;

    // 1. 사용자 메시지를 store에 추가
    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: messageToSend,
    };
    const loadingId = (Date.now() + 1).toString();
    const loadingMessage: Message = {
      id: loadingId,
      role: 'assistant',
      content: '',
      traces: [{ agent: 'Planner', step: 'Calling Llama.cpp backend...' }],
    };

    setMessages([...messages, userMessage, loadingMessage]);
    setInput('');

    // 2. 대시보드로 전환 (CustomEvent로 App.tsx에 알림)
    window.dispatchEvent(new CustomEvent('hud-expand'));
    await expandToDashboard();

    // 3. 백엔드 호출
    try {
      let replyContent = 'No response';
      let citations: any[] = [];

      // @ts-ignore
      if (window.__TAURI_INTERNALS__) {
        const { invoke } = await import('@tauri-apps/api/core');
        let pId: string | null = null;
        if (activeProject) {
          pId = typeof activeProject.id === 'string'
            ? activeProject.id
            : (activeProject.id?.id?.String || activeProject.id?.id || null);
        }
        const response = await invoke('run_agent_workflow', {
          message: messageToSend,
          projectId: pId,
          harnessType: activeProject?.harness || 'deep_research',
          permissionMode: permissionMode,
          mentions: [],
        }) as { answer: string; citations: any[]; artifacts?: any[] };

        replyContent = response.answer;
        citations = response.citations || [];
      } else {
        await new Promise(resolve => setTimeout(resolve, 1200));
        replyContent = `[HUD → Dashboard] ${activeProject?.name || 'Q-Agent'}: "${messageToSend}"에 대한 응답입니다. (웹 Mock 환경)`;
        citations = [];
      }

      useAppStore.getState().setMessages(prev =>
        prev.map(msg =>
          msg.id === loadingId
            ? { ...msg, content: replyContent, citations, traces: [{ agent: 'Executor', step: 'Generated response' }] }
            : msg
        )
      );
    } catch (error) {
      useAppStore.getState().setMessages(prev =>
        prev.map(msg =>
          msg.id === loadingId
            ? { ...msg, content: 'Failed to connect to the model backend.', traces: [{ agent: 'System', step: 'Connection error' }] }
            : msg
        )
      );
    }
  };

  const vramUsed = vramInfo?.used || 9.2;
  const vramTotal = vramInfo?.total || 16.0;
  const vramRatio = vramUsed / vramTotal;
  const vramColor = vramRatio > 0.95 ? 'var(--accent-danger)' : vramRatio > 0.8 ? 'var(--accent-warning)' : 'var(--accent-primary)';

  return (
    <div className="w-full h-full flex flex-col bg-[var(--bg-elevated)] backdrop-blur-[16px] border border-[var(--border)] rounded-[var(--radius-lg)] shadow-[var(--shadow-lg)] overflow-hidden">
      {/* HUD Header */}
      <div className="flex items-center justify-between px-4 py-2 border-b border-[var(--border)]">
        <div className="flex items-center gap-2 flex-1 h-full cursor-move" data-tauri-drag-region>
          <Command size={16} className="text-[var(--accent-primary)] pointer-events-none" />
          <button className="flex items-center gap-1.5 px-2 py-0.5 rounded-md hover:bg-[var(--bg-subtle)] text-sm font-semibold text-[var(--text-primary)] transition-colors">
            {activeProject ? activeProject.name : 'No Project'}
            <ChevronDown size={14} className="text-[var(--text-muted)]" />
          </button>
        </div>
        
        {/* Window Controls */}
        <div className="flex items-center gap-4 text-xs text-[var(--text-muted)] select-none z-10">
          <button 
            onClick={expandToDashboard}
            title="대시보드로 확장"
            className="p-1 hover:text-[var(--text-primary)] hover:bg-[var(--bg-subtle)] rounded-md transition-colors"
          >
            <Maximize2 size={14} />
          </button>
        </div>
      </div>

      {/* Input Area */}
      <div className="flex-1 px-4 py-3 flex flex-col justify-end">
        <div className="flex items-center bg-[var(--bg-base)] border border-[var(--border)] rounded-[var(--radius-md)] p-2 shadow-[var(--shadow-sm)] focus-within:border-[var(--accent-primary)] transition-all">
          <input 
            type="text" 
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                handleSend();
              }
            }}
            placeholder="Type a command or ask a question..." 
            className="flex-1 bg-transparent outline-none text-[var(--text-primary)] placeholder-[var(--text-muted)] text-sm px-2 min-h-[32px]"
          />
          <div className="flex items-center gap-1 pr-1">
            <button className="p-1.5 text-[var(--text-muted)] hover:bg-[var(--bg-subtle)] hover:text-[var(--text-primary)] rounded-[var(--radius-sm)] transition-colors" title="Attach file">
              <FolderPlus size={16} />
            </button>
            <button className="p-1.5 text-[var(--text-muted)] hover:bg-[var(--bg-subtle)] hover:text-[var(--text-primary)] rounded-[var(--radius-sm)] transition-colors" title="Web Search">
              <Globe size={16} />
            </button>
            <div className="w-[1px] h-4 bg-[var(--border)] mx-1" />
            <button 
              onClick={handleSend}
              disabled={!input.trim()}
              className="p-1.5 bg-[var(--accent-primary)] text-white rounded-[var(--radius-sm)] hover:bg-[var(--accent-primary)] opacity-90 hover:opacity-100 disabled:opacity-40 disabled:cursor-not-allowed transition-opacity"
            >
              <Send size={14} />
            </button>
          </div>
        </div>
      </div>

      {/* Resource Monitor Bottom Bar */}
      <div className="h-[32px] bg-[var(--bg-surface)] border-t border-[var(--border)] flex items-center px-4 text-xs text-[var(--text-secondary)] justify-between select-none">
        <div className="flex items-center gap-3">
          <span className="font-medium">⚡ {activeProject?.model_small || 'Qwen 2.5 14B'}</span>
          <div className="w-[1px] h-3 bg-[var(--border)]" />
          <div className="flex items-center gap-2">
            <div className="w-24 h-1.5 bg-[var(--bg-subtle)] rounded-full overflow-hidden border border-[var(--border-subtle)]">
               <div className="h-full transition-all duration-300" style={{ width: `${vramRatio * 100}%`, backgroundColor: vramColor }} />
            </div>
            <span>VRAM {vramUsed.toFixed(1)}/{vramTotal.toFixed(1)}GB</span>
          </div>
        </div>
        <div className="flex items-center gap-3 font-medium">
           <span>Ops 82%</span>
           <div className="w-[1px] h-3 bg-[var(--border)]" />
           <div className="flex items-center gap-1.5">
             <div className={`w-2 h-2 rounded-full ${networkStatus === 'local' ? 'bg-[var(--accent-success)]' : networkStatus === 'lan' ? 'bg-[var(--accent-warning)]' : 'bg-[var(--text-muted)]'}`} />
             <span className="capitalize">{networkStatus}</span>
           </div>
        </div>
      </div>
    </div>
  );
};
