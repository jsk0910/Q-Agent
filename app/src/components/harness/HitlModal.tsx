import React, { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { ShieldAlert, Check, X } from 'lucide-react';

interface HitlRequest {
  id: string;
  action_type: string; // 'fs_write', 'terminal', 'ssh'
  description: string;
  command?: string;
  path?: string;
}

export const HitlModal: React.FC = () => {
  const [request, setRequest] = useState<HitlRequest | null>(null);
  
  useEffect(() => {
    const unlisten = listen<HitlRequest>('hitl_request', (event) => {
      setRequest(event.payload);
    });
    
    return () => {
      unlisten.then(f => f());
    };
  }, []);

  const handleResponse = async (approved: boolean) => {
    if (!request) return;
    try {
      await invoke('hitl_respond', { id: request.id, approved });
    } catch (e) {
      console.error("Failed to send HITL response:", e);
    }
    setRequest(null);
  };

  // For testing purposes during UI dev, you can uncomment this to show a mock request
  /*
  useEffect(() => {
    setTimeout(() => {
      setRequest({
        id: "mock-123",
        action_type: "terminal",
        description: "Installing pip dependencies",
        command: "pip install -r requirements.txt",
        path: "C:/workspace/test-project"
      });
    }, 5000);
  }, []);
  */

  if (!request) return null;

  return (
    <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div className="w-[500px] bg-[var(--bg-surface)] border border-red-500/30 rounded-[var(--radius-lg)] shadow-2xl flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-200">
        
        <div className="flex items-center gap-3 px-6 py-4 border-b border-[var(--border)] bg-red-500/10 text-red-500">
          <ShieldAlert size={24} />
          <h2 className="text-lg font-semibold">사용자 승인 필요 (HITL)</h2>
        </div>

        <div className="p-6 space-y-4">
          <p className="text-sm text-[var(--text-secondary)]">
            에이전트가 시스템에 직접적인 영향을 줄 수 있는 동작을 수행하려고 합니다. 위험 요소를 확인하고 승인해 주세요.
          </p>

          <div className="p-4 bg-[var(--bg-elevated)] border border-[var(--border)] rounded-md space-y-2">
            <div className="flex justify-between items-center text-sm">
              <span className="text-[var(--text-muted)]">Action:</span>
              <span className="font-semibold text-[var(--text-primary)] uppercase">{request.action_type}</span>
            </div>
            <div className="flex justify-between items-start text-sm">
              <span className="text-[var(--text-muted)]">Description:</span>
              <span className="text-[var(--text-primary)] text-right max-w-[60%]">{request.description}</span>
            </div>
            
            {request.path && (
              <div className="mt-3 text-xs">
                <span className="text-[var(--text-muted)] block mb-1">Target Path:</span>
                <code className="block p-2 bg-[#0d1117] text-[#c9d1d9] rounded break-all">{request.path}</code>
              </div>
            )}
            
            {request.command && (
              <div className="mt-3 text-xs">
                <span className="text-[var(--text-muted)] block mb-1">Command:</span>
                <code className="block p-2 bg-[#0d1117] text-orange-300 rounded break-all whitespace-pre-wrap">
                  {request.command}
                </code>
              </div>
            )}
          </div>
        </div>

        <div className="px-6 py-4 border-t border-[var(--border)] bg-[var(--bg-elevated)] flex justify-end gap-3">
          <button 
            onClick={() => handleResponse(false)}
            className="flex items-center gap-2 px-4 py-2 bg-[var(--bg-surface)] text-[var(--text-primary)] border border-[var(--border)] rounded-md text-sm font-medium hover:bg-[var(--bg-elevated)] transition-colors"
          >
            <X size={16} />
            거절 (Reject)
          </button>
          <button 
            onClick={() => handleResponse(true)}
            className="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded-md text-sm font-medium hover:bg-red-700 transition-colors shadow-lg shadow-red-900/20"
          >
            <Check size={16} />
            승인 (Approve)
          </button>
        </div>
      </div>
    </div>
  );
};
