import React, { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { ShieldAlert, Check, X, Server, Cpu, Globe } from 'lucide-react';

interface HitlRequest {
  id: string;
  action_type: string; // 'fs_write', 'terminal', 'ssh'
  description: string;
  command?: string;
  path?: string;
  risk_score?: number;
}

type ExecEnv = 'wasm' | 'remote' | 'local';

export const HitlModal: React.FC = () => {
  const [request, setRequest] = useState<HitlRequest | null>(null);
  const [execEnv, setExecEnv] = useState<ExecEnv>('local');
  
  useEffect(() => {
    if (!('__TAURI_INTERNALS__' in window)) return;
    const unlisten = listen<HitlRequest>('hitl_request', (event) => {
      setRequest(event.payload);
      setExecEnv('local'); // Reset default
    });
    
    return () => {
      unlisten.then(f => f());
    };
  }, []);

  const handleResponse = async (approved: boolean) => {
    if (!request) return;
    if ('__TAURI_INTERNALS__' in window) {
      try {
        await invoke('hitl_respond', { id: request.id, approved, env: execEnv });
      } catch (e) {
        console.error("Failed to send HITL response:", e);
      }
    }
    setRequest(null);
  };

  if (!request) return null;

  const risk = request.risk_score !== undefined ? request.risk_score : 0.42;
  const riskColor = risk < 0.5 ? 'var(--accent-success)' : risk < 0.7 ? 'var(--accent-warning)' : 'var(--accent-danger)';

  return (
    <div className="fixed inset-0 z-[100] flex items-center justify-center bg-[var(--bg-overlay)] backdrop-blur-sm">
      <div className="w-[520px] bg-[var(--bg-surface)] border border-[var(--border)] rounded-[var(--radius-lg)] shadow-[var(--shadow-lg)] flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-200">
        
        <div className="flex items-center gap-3 px-6 py-4 border-b border-[var(--border)] bg-[var(--bg-elevated)]">
          <ShieldAlert size={24} className="text-[var(--accent-warning)]" />
          <h2 className="text-lg font-semibold text-[var(--text-primary)]">사용자 승인 필요 (HITL)</h2>
        </div>

        <div className="p-6 space-y-5">
          <p className="text-sm text-[var(--text-secondary)]">
            에이전트가 시스템에 영향을 줄 수 있는 동작을 수행하려고 합니다. 위험 요소를 확인하고 승인해 주세요.
          </p>

          {/* Risk Score Bar */}
          <div className="flex flex-col gap-1.5">
            <div className="flex justify-between text-xs font-medium">
              <span className="text-[var(--text-muted)]">Risk Score</span>
              <span style={{ color: riskColor }}>{risk.toFixed(2)}</span>
            </div>
            <div className="h-2 bg-[var(--bg-subtle)] rounded-full overflow-hidden border border-[var(--border-subtle)]">
              <div 
                style={{ width: `${Math.min(100, Math.max(0, risk * 100))}%`, backgroundColor: riskColor }} 
                className="h-full rounded-full transition-all duration-500" 
              />
            </div>
          </div>

          <div className="p-4 bg-[var(--bg-elevated)] border border-[var(--border)] rounded-md space-y-3">
            <div className="flex justify-between items-center text-sm">
              <span className="text-[var(--text-muted)]">Action:</span>
              <span className="font-semibold text-[var(--text-primary)] uppercase px-2 py-0.5 bg-[var(--bg-surface)] border border-[var(--border)] rounded">{request.action_type}</span>
            </div>
            <div className="flex flex-col gap-1 text-sm mt-2">
              <span className="text-[var(--text-muted)]">Description:</span>
              <span className="text-[var(--text-primary)]">{request.description}</span>
            </div>
            
            {request.path && (
              <div className="mt-3 text-xs">
                <span className="text-[var(--text-muted)] block mb-1">Target Path:</span>
                <code className="block p-2 bg-[#0d1117] text-[#c9d1d9] rounded border border-[var(--border-subtle)] break-all">{request.path}</code>
              </div>
            )}
            
            {request.command && (
              <div className="mt-3 text-xs">
                <span className="text-[var(--text-muted)] block mb-1">Command:</span>
                <code className="block p-2 bg-[#0d1117] text-[#A5B4FC] rounded border border-[var(--border-subtle)] break-all whitespace-pre-wrap">
                  {request.command}
                </code>
              </div>
            )}
          </div>

          {/* Execution Environment Selection */}
          <div className="space-y-2 pt-1">
            <div className="text-xs font-semibold text-[var(--text-muted)] uppercase tracking-wider mb-2">Execution Environment</div>
            <div className="grid grid-cols-3 gap-2">
              <button 
                onClick={() => setExecEnv('wasm')}
                className={`flex flex-col items-center gap-2 p-3 border rounded-md transition-all text-sm ${execEnv === 'wasm' ? 'border-[var(--accent-primary)] bg-[var(--accent-primary-subtle)] text-[var(--accent-primary)] font-medium' : 'border-[var(--border)] bg-[var(--bg-base)] text-[var(--text-secondary)] hover:border-[var(--text-muted)]'}`}
              >
                <Globe size={20} />
                <span>WASM</span>
              </button>
              <button 
                onClick={() => setExecEnv('remote')}
                className={`flex flex-col items-center gap-2 p-3 border rounded-md transition-all text-sm ${execEnv === 'remote' ? 'border-[var(--accent-primary)] bg-[var(--accent-primary-subtle)] text-[var(--accent-primary)] font-medium' : 'border-[var(--border)] bg-[var(--bg-base)] text-[var(--text-secondary)] hover:border-[var(--text-muted)]'}`}
              >
                <Server size={20} />
                <span>Remote</span>
              </button>
              <button 
                onClick={() => setExecEnv('local')}
                className={`flex flex-col items-center gap-2 p-3 border rounded-md transition-all text-sm ${execEnv === 'local' ? 'border-[var(--accent-primary)] bg-[var(--accent-primary-subtle)] text-[var(--accent-primary)] font-medium' : 'border-[var(--border)] bg-[var(--bg-base)] text-[var(--text-secondary)] hover:border-[var(--text-muted)]'}`}
              >
                <Cpu size={20} />
                <span>Local</span>
              </button>
            </div>
          </div>
        </div>

        <div className="px-6 py-4 border-t border-[var(--border)] bg-[var(--bg-elevated)] flex justify-between items-center">
          <button className="text-xs text-[var(--text-muted)] hover:text-[var(--text-secondary)] underline underline-offset-2 transition-colors">
            이력 보기
          </button>
          <div className="flex gap-3">
            <button 
              onClick={() => handleResponse(false)}
              className="flex items-center gap-2 px-4 py-2 bg-[var(--bg-surface)] text-[var(--text-primary)] border border-[var(--border)] rounded-md text-sm font-medium hover:bg-[var(--bg-base)] transition-colors"
            >
              <X size={16} />
              거절
            </button>
            <button 
              onClick={() => handleResponse(true)}
              className="flex items-center gap-2 px-4 py-2 bg-[var(--accent-primary)] text-white rounded-md text-sm font-medium hover:bg-[var(--accent-primary)]/90 transition-all shadow-sm"
            >
              <Check size={16} />
              승인
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
