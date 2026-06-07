import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { X, Save, Edit3 } from 'lucide-react';

interface HarnessStudioModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const HarnessStudioModal: React.FC<HarnessStudioModalProps> = ({ isOpen, onClose }) => {
  const [category, setCategory] = useState<'personas' | 'harness' | 'orchestration'>('personas');
  const [templates, setTemplates] = useState<string[]>([]);
  const [selectedTemplate, setSelectedTemplate] = useState<string>('');
  const [yamlContent, setYamlContent] = useState<string>('');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (isOpen) {
      loadTemplates(category);
    }
  }, [isOpen, category]);

  const loadTemplates = async (cat: string) => {
    if (!('__TAURI_INTERNALS__' in window)) {
      setTemplates(["mock_template"]);
      setSelectedTemplate("mock_template");
      return;
    }
    try {
      const list = await invoke<string[]>('list_templates', { category: cat });
      setTemplates(list);
      if (list.length > 0) {
        setSelectedTemplate(list[0]);
      } else {
        setSelectedTemplate('');
        setYamlContent('');
      }
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    if (selectedTemplate) {
      loadContent(selectedTemplate);
    }
  }, [selectedTemplate]);

  const loadContent = async (name: string) => {
    if (!('__TAURI_INTERNALS__' in window)) {
      setYamlContent("# Mock YAML content\nname: " + name);
      return;
    }
    try {
      const content = await invoke<string>('get_template_content', { category, name });
      setYamlContent(content);
    } catch (e) {
      console.error(e);
    }
  };

  const handleSave = async () => {
    if (!selectedTemplate) return;
    setSaving(true);
    if (!('__TAURI_INTERNALS__' in window)) {
      setTimeout(() => {
        setSaving(false);
        alert('Template saved successfully! (Mock)');
      }, 500);
      return;
    }
    try {
      await invoke('save_template_content', { category, name: selectedTemplate, content: yamlContent });
      alert('Template saved successfully!');
    } catch (e) {
      console.error(e);
      alert('Failed to save template');
    } finally {
      setSaving(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div className="w-[900px] h-[650px] bg-[var(--bg-surface)] border border-[var(--border)] rounded-[var(--radius-lg)] shadow-2xl flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-200">
        
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-[var(--border)]">
          <div className="flex items-center gap-2">
            <Edit3 size={20} className="text-[var(--accent-primary)]" />
            <h2 className="text-lg font-semibold text-[var(--text-primary)]">Harness Studio</h2>
          </div>
          <button onClick={onClose} className="p-1 text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors">
            <X size={20} />
          </button>
        </div>

        <div className="flex-1 flex overflow-hidden">
          {/* Sidebar */}
          <div className="w-56 bg-[var(--bg-elevated)] border-r border-[var(--border)] p-4 flex flex-col gap-6">
            <div>
              <h3 className="text-xs font-semibold text-[var(--text-muted)] uppercase tracking-wider mb-2 px-1">Categories</h3>
              <div className="space-y-1">
                {['personas', 'harness', 'orchestration'].map(cat => (
                  <button 
                    key={cat}
                    onClick={() => setCategory(cat as any)}
                    className={`w-full text-left px-3 py-2 text-sm rounded-md capitalize transition-colors ${category === cat ? 'bg-[var(--accent-primary)] text-white' : 'text-[var(--text-secondary)] hover:bg-[var(--bg-surface)] hover:text-[var(--text-primary)]'}`}
                  >
                    {cat}
                  </button>
                ))}
              </div>
            </div>

            <div>
              <h3 className="text-xs font-semibold text-[var(--text-muted)] uppercase tracking-wider mb-2 px-1">Templates</h3>
              <div className="space-y-1">
                {templates.map(tpl => (
                  <button 
                    key={tpl}
                    onClick={() => setSelectedTemplate(tpl)}
                    className={`w-full flex items-center gap-2 px-3 py-2 text-sm rounded-md transition-colors ${selectedTemplate === tpl ? 'bg-[var(--bg-surface)] text-[var(--text-primary)] font-medium border border-[var(--border)]' : 'text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-surface)]'}`}
                  >
                    <span>📄</span>
                    <span className="truncate">{tpl}.yaml</span>
                  </button>
                ))}
              </div>
            </div>
          </div>

          {/* Editor Content */}
          <div className="flex-1 flex flex-col bg-[var(--bg-surface)]">
            <div className="flex-1 p-4">
              <textarea 
                value={yamlContent}
                onChange={e => setYamlContent(e.target.value)}
                className="w-full h-full p-4 font-mono text-sm leading-relaxed bg-[#0d1117] text-[#c9d1d9] border border-[var(--border)] rounded-md focus:outline-none focus:border-[var(--accent-primary)] resize-none shadow-inner"
                spellCheck={false}
              />
            </div>
            <div className="px-4 py-3 border-t border-[var(--border)] bg-[var(--bg-elevated)] flex justify-between items-center">
              <div className="text-xs text-[var(--text-muted)]">
                {selectedTemplate ? `Editing: templates/${category}/${selectedTemplate}.yaml` : 'No template selected'}
              </div>
              <button 
                onClick={handleSave}
                disabled={saving || !selectedTemplate}
                className="flex items-center gap-2 px-4 py-2 bg-[var(--accent-primary)] text-white rounded-md text-sm font-medium hover:bg-[var(--accent-secondary)] transition-colors disabled:opacity-50"
              >
                <Save size={16} />
                {saving ? 'Saving...' : 'Save Changes'}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
