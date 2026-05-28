import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Plus, Folder, Settings, Trash2 } from 'lucide-react';

export interface Project {
  id: String;
  name: String;
  description?: string;
  is_active: boolean;
}

export const ProjectSwitcher: React.FC = () => {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadProjects();
  }, []);

  const loadProjects = async () => {
    try {
      const list = await invoke<Project[]>('list_projects');
      setProjects(list);
    } catch (error) {
      console.error('Failed to load projects:', error);
    } finally {
      setLoading(false);
    }
  };

  const createProject = async () => {
    const name = prompt('Project Name:');
    if (!name) return;

    const newProject = {
      id: Math.random().toString(36).substring(7),
      name,
      persona_template: 'default',
      harness_template: 'standard',
      orchestration_template: 'balanced',
      max_iterations: 5,
      token_budget: 8000,
      security_level: 2,
      is_active: true,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };

    try {
      await invoke('create_project', { project: newProject });
      loadProjects();
    } catch (error) {
      alert('Failed to create project: ' + error);
    }
  };

  if (loading) return <div className="p-4 text-[var(--text-muted)]">Loading projects...</div>;

  return (
    <div className="flex flex-col gap-2 p-2">
      <div className="flex items-center justify-between px-2 py-1 mb-2">
        <span className="text-xs font-bold uppercase tracking-wider text-[var(--text-muted)]">Projects</span>
        <button 
          onClick={createProject}
          className="p-1 hover:bg-[var(--bg-elevated)] rounded-full transition-colors text-[var(--accent-primary)]"
        >
          <Plus size={16} />
        </button>
      </div>

      <div className="space-y-1">
        {projects.length === 0 && (
          <div className="px-3 py-2 text-sm text-[var(--text-muted)] italic">No projects yet.</div>
        )}
        {projects.map(project => (
          <div 
            key={project.id as string}
            className={`group flex items-center justify-between px-3 py-2 rounded-[var(--radius-md)] cursor-pointer transition-all ${
              project.is_active 
                ? 'bg-[var(--accent-primary)] text-white shadow-md' 
                : 'hover:bg-[var(--bg-elevated)] text-[var(--text-primary)]'
            }`}
          >
            <div className="flex items-center gap-3">
              <Folder size={18} className={project.is_active ? 'text-white' : 'text-[var(--accent-primary)]'} />
              <span className="text-sm font-medium">{project.name}</span>
            </div>
            <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
              <button className="p-1 hover:bg-black/10 rounded">
                <Settings size={14} />
              </button>
              <button 
                onClick={(e) => {
                  e.stopPropagation();
                  if (confirm('Delete project?')) invoke('delete_project', { id: project.id }).then(loadProjects);
                }}
                className="p-1 hover:bg-red-500/20 text-red-500 rounded"
              >
                <Trash2 size={14} />
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
