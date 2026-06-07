import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { X, Download, Check, AlertTriangle, HardDrive, Cpu, Zap } from 'lucide-react';

interface SystemSpecs {
  total_memory_mb: number;
  used_memory_mb: number;
  cpu_cores: number;
}

interface ModelInfo {
  id: string;
  name: string;
  description: string;
  parameters: string;
  required_ram_mb: number;
  download_url: string;
  file_name: string;
  is_downloaded: boolean;
}

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const SettingsModal: React.FC<SettingsModalProps> = ({ isOpen, onClose }) => {
  const [specs, setSpecs] = useState<SystemSpecs | null>(null);
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (isOpen) {
      loadData();
    }
  }, [isOpen]);

  const loadData = async () => {
    setLoading(true);
    if (!('__TAURI_INTERNALS__' in window)) {
      setSpecs({ total_memory_mb: 16384, used_memory_mb: 8192, cpu_cores: 8 });
      setModels([{ id: 'mock', name: 'Mock Model', description: 'Mock', parameters: '7B', required_ram_mb: 8192, download_url: '', file_name: '', is_downloaded: false }]);
      setLoading(false);
      return;
    }
    try {
      const [sysSpecs, availModels] = await Promise.all([
        invoke<SystemSpecs>('get_system_specs'),
        invoke<ModelInfo[]>('get_available_models')
      ]);
      setSpecs(sysSpecs);
      setModels(availModels);
    } catch (e) {
      console.error("Failed to load settings data:", e);
    } finally {
      setLoading(false);
    }
  };

  const handleDownload = async (modelId: string) => {
    if (!('__TAURI_INTERNALS__' in window)) {
      setModels(models.map(m => m.id === modelId ? { ...m, is_downloaded: true } : m));
      return;
    }
    try {
      await invoke('download_model', { modelId });
      // 낙관적 업데이트 또는 상태 폴링
      setModels(models.map(m => m.id === modelId ? { ...m, is_downloaded: true } : m));
    } catch (e) {
      console.error("Download failed:", e);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div className="w-[800px] h-[600px] bg-[var(--bg-surface)] border border-[var(--border)] rounded-[var(--radius-lg)] shadow-2xl flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-200">
        
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-[var(--border)]">
          <h2 className="text-lg font-semibold text-[var(--text-primary)]">Settings & Models</h2>
          <button onClick={onClose} className="p-1 text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors">
            <X size={20} />
          </button>
        </div>

        <div className="flex-1 flex overflow-hidden">
          {/* Sidebar */}
          <div className="w-48 bg-[var(--bg-elevated)] border-r border-[var(--border)] p-4 space-y-2">
            <div className="px-3 py-2 bg-[var(--bg-surface)] rounded-md font-medium text-[var(--accent-primary)] shadow-sm">
              Local Models
            </div>
            <div className="px-3 py-2 text-[var(--text-secondary)] hover:text-[var(--text-primary)] cursor-pointer rounded-md transition-colors">
              General
            </div>
          </div>

          {/* Content */}
          <div className="flex-1 overflow-y-auto p-6 space-y-8">
            
            {/* System Status */}
            <section>
              <h3 className="text-sm font-semibold uppercase tracking-wider text-[var(--text-muted)] mb-4">System Specs</h3>
              {loading ? (
                <div className="animate-pulse h-16 bg-[var(--bg-elevated)] rounded-md"></div>
              ) : specs ? (
                <div className="grid grid-cols-3 gap-4">
                  <div className="p-4 border border-[var(--border)] rounded-lg bg-[var(--bg-elevated)]">
                    <div className="flex items-center gap-2 text-[var(--text-secondary)] mb-1">
                      <Cpu size={16} />
                      <span className="text-sm">CPU Cores</span>
                    </div>
                    <div className="text-xl font-semibold text-[var(--text-primary)]">{specs.cpu_cores} Cores</div>
                  </div>
                  <div className="p-4 border border-[var(--border)] rounded-lg bg-[var(--bg-elevated)]">
                    <div className="flex items-center gap-2 text-[var(--text-secondary)] mb-1">
                      <HardDrive size={16} />
                      <span className="text-sm">Total RAM</span>
                    </div>
                    <div className="text-xl font-semibold text-[var(--text-primary)]">
                      {(specs.total_memory_mb / 1024).toFixed(1)} GB
                    </div>
                  </div>
                  <div className="p-4 border border-[var(--border)] rounded-lg bg-[var(--bg-elevated)]">
                    <div className="flex items-center gap-2 text-[var(--text-secondary)] mb-1">
                      <Zap size={16} />
                      <span className="text-sm">Available RAM</span>
                    </div>
                    <div className="text-xl font-semibold text-[var(--accent-success)]">
                      {((specs.total_memory_mb - specs.used_memory_mb) / 1024).toFixed(1)} GB
                    </div>
                  </div>
                </div>
              ) : null}
            </section>

            {/* Model Registry */}
            <section>
              <h3 className="text-sm font-semibold uppercase tracking-wider text-[var(--text-muted)] mb-4">Available Models</h3>
              <div className="space-y-4">
                {models.map(model => {
                  const requiredGb = model.required_ram_mb / 1024;
                  const isOOM = specs ? (model.required_ram_mb > specs.total_memory_mb) : false;
                  const isWarning = specs ? (model.required_ram_mb > (specs.total_memory_mb - specs.used_memory_mb)) : false;

                  return (
                    <div key={model.id} className="p-4 border border-[var(--border)] rounded-lg bg-[var(--bg-elevated)] flex flex-col gap-3">
                      <div className="flex items-start justify-between">
                        <div>
                          <div className="flex items-center gap-2">
                            <h4 className="font-semibold text-[var(--text-primary)]">{model.name}</h4>
                            <span className="text-xs px-2 py-0.5 bg-[var(--bg-surface)] border border-[var(--border)] rounded-full text-[var(--text-muted)]">
                              {model.parameters}
                            </span>
                          </div>
                          <p className="text-sm text-[var(--text-secondary)] mt-1">{model.description}</p>
                        </div>
                        
                        <div>
                          {model.is_downloaded ? (
                            <button disabled className="flex items-center gap-1 px-3 py-1.5 bg-[var(--accent-success)]/10 text-[var(--accent-success)] border border-[var(--accent-success)]/20 rounded-md text-sm font-medium">
                              <Check size={16} /> Ready
                            </button>
                          ) : (
                            <button 
                              onClick={() => handleDownload(model.id)}
                              disabled={isOOM}
                              className={`flex items-center gap-1 px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${
                                isOOM ? 'bg-red-500/10 text-red-500 border border-red-500/20 opacity-50 cursor-not-allowed' : 'bg-[var(--accent-primary)] text-white hover:bg-[var(--accent-secondary)]'
                              }`}
                            >
                              <Download size={16} /> Download
                            </button>
                          )}
                        </div>
                      </div>

                      {/* Requirement Indicator */}
                      <div className="flex items-center justify-between text-xs mt-2 p-2 rounded bg-[var(--bg-surface)]">
                        <div className="flex items-center gap-4">
                          <span className="text-[var(--text-muted)]">Req. RAM: <span className="text-[var(--text-primary)] font-medium">{requiredGb.toFixed(1)} GB</span></span>
                          <span className="text-[var(--text-muted)]">Size: <span className="text-[var(--text-primary)] font-medium">~{(requiredGb * 0.8).toFixed(1)} GB</span></span>
                        </div>
                        
                        {isOOM ? (
                          <div className="flex items-center gap-1 text-red-500">
                            <AlertTriangle size={14} /> <span>시스템 RAM 부족 (다운로드 불가)</span>
                          </div>
                        ) : isWarning && !model.is_downloaded ? (
                          <div className="flex items-center gap-1 text-yellow-500">
                            <AlertTriangle size={14} /> <span>현재 가용 RAM 부족 (다른 앱 종료 권장)</span>
                          </div>
                        ) : null}
                      </div>
                    </div>
                  );
                })}
              </div>
            </section>
            
          </div>
        </div>
      </div>
    </div>
  );
};
