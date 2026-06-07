import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { X, FolderPlus } from 'lucide-react';
import { useAppStore } from '../../stores/useAppStore';

interface CreateProjectModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const CreateProjectModal: React.FC<CreateProjectModalProps> = ({ isOpen, onClose }) => {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [persona, setPersona] = useState('default');
  const [harness, setHarness] = useState('standard');
  const [loading, setLoading] = useState(false);

  const { addProject, setActiveProject } = useAppStore();

  const handleCreate = async () => {
    if (!name.trim()) return;
    setLoading(true);

    try {
      const idString = `proj-${Date.now()}`;
      // In surrealdb, ID is { id: { String: "raw_id" }, tb: "project" } when serialized from Rust Thing,
      // but creating from frontend can just pass a valid Rust struct equivalent or we can just send the raw fields if we modify the signature.
      // Wait, the Rust signature is `pub async fn create_project(db: State<'_, Database>, project: Project) -> Result<Project, String>`
      // The `Project` struct requires `id: Thing`, which is tricky to construct from JS.
      if (!('__TAURI_INTERNALS__' in window)) {
        // Mock fallback for browser
        const mockProject = {
          id: idString,
          name,
          description: description || undefined,
          persona_template: persona,
          harness: harness,
        };
        addProject(mockProject);
        setActiveProject(mockProject.id);
        setName('');
        setDescription('');
        onClose();
        setLoading(false);
        return;
      }

      const createdProject = await invoke('create_project', {
        name,
        description: description || null,
        personaTemplate: persona,
        harnessTemplate: harness,
      }) as any;
      
      addProject(createdProject);
      setActiveProject(createdProject.id.id?.String || createdProject.id.id || createdProject.id);
      
      setName('');
      setDescription('');
      onClose();
    } catch (error) {
      console.error('Failed to create project:', error);
      alert('Failed to create project.');
    } finally {
      setLoading(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div className="w-[500px] bg-[var(--bg-surface)] border border-[var(--border)] rounded-[var(--radius-lg)] shadow-2xl flex flex-col overflow-hidden animate-in fade-in zoom-in-95 duration-200">
        <div className="flex items-center justify-between px-6 py-4 border-b border-[var(--border)]">
          <div className="flex items-center gap-2">
            <FolderPlus size={20} className="text-[var(--accent-primary)]" />
            <h2 className="text-lg font-semibold text-[var(--text-primary)]">New Project</h2>
          </div>
          <button onClick={onClose} className="p-1 text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors">
            <X size={20} />
          </button>
        </div>

        <div className="p-6 flex flex-col gap-4">
          <div>
            <label className="block text-sm font-medium text-[var(--text-secondary)] mb-1">Project Name</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g. Next.js Refactoring"
              className="w-full px-3 py-2 bg-[var(--bg-elevated)] border border-[var(--border)] rounded-md text-[var(--text-primary)] focus:outline-none focus:border-[var(--accent-primary)] transition-colors"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-[var(--text-secondary)] mb-1">Description (Optional)</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Brief description of the project..."
              className="w-full px-3 py-2 h-20 bg-[var(--bg-elevated)] border border-[var(--border)] rounded-md text-[var(--text-primary)] focus:outline-none focus:border-[var(--accent-primary)] transition-colors resize-none"
            />
          </div>
          
          <div className="flex gap-4">
            <div className="flex-1">
              <label className="block text-sm font-medium text-[var(--text-secondary)] mb-1">Persona Template</label>
              <select 
                value={persona} 
                onChange={e => setPersona(e.target.value)}
                className="w-full px-3 py-2 bg-[var(--bg-elevated)] border border-[var(--border)] rounded-md text-[var(--text-primary)] focus:outline-none focus:border-[var(--accent-primary)] transition-colors"
              >
                <option value="default">Default Persona</option>
                <option value="senior_dev">Senior Developer</option>
              </select>
            </div>
            <div className="flex-1">
              <label className="block text-sm font-medium text-[var(--text-secondary)] mb-1">Harness Template</label>
              <select 
                value={harness} 
                onChange={e => setHarness(e.target.value)}
                className="w-full px-3 py-2 bg-[var(--bg-elevated)] border border-[var(--border)] rounded-md text-[var(--text-primary)] focus:outline-none focus:border-[var(--accent-primary)] transition-colors"
              >
                <option value="standard">Standard</option>
                <option value="deep_research">Deep Research</option>
              </select>
            </div>
          </div>
        </div>

        <div className="px-6 py-4 border-t border-[var(--border)] bg-[var(--bg-elevated)] flex justify-end gap-2">
          <button 
            onClick={onClose}
            className="px-4 py-2 text-sm font-medium text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-colors"
          >
            Cancel
          </button>
          <button 
            onClick={handleCreate}
            disabled={!name.trim() || loading}
            className="px-4 py-2 bg-[var(--accent-primary)] text-white rounded-md text-sm font-medium hover:bg-[var(--accent-secondary)] transition-colors disabled:opacity-50"
          >
            {loading ? 'Creating...' : 'Create Project'}
          </button>
        </div>
      </div>
    </div>
  );
};
