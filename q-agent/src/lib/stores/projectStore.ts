import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Project {
  id: any;
  name: string;
  description: string;
}

export function parseId(idRaw: any): string {
  if (!idRaw) return '';
  if (typeof idRaw === 'string') return idRaw;
  if (idRaw.StringRecordId) return idRaw.StringRecordId;
  if (idRaw.tb && idRaw.id && idRaw.id.String) return `${idRaw.tb}:${idRaw.id.String}`;
  return JSON.stringify(idRaw);
}

function createProjectStore() {
  const { subscribe, set, update } = writable<{
    projects: Project[];
    activeProjectId: string | null;
    isLoading: boolean;
  }>({
    projects: [],
    activeProjectId: null,
    isLoading: false,
  });

  return {
    subscribe,
    async loadProjects() {
      update(s => ({ ...s, isLoading: true }));
      try {
        const projects = await invoke<Project[]>('list_projects');
        update(s => {
          // If no active project but we have projects, auto-select the first one
          let activeId = s.activeProjectId;
          if (!activeId && projects.length > 0) {
            activeId = parseId(projects[0].id);
          }
          return { ...s, projects, activeProjectId: activeId, isLoading: false };
        });
      } catch (e) {
        console.error("Failed to load projects", e);
        update(s => ({ ...s, isLoading: false }));
      }
    },
    async createProject(name: string, description: string) {
      try {
        const newProj = await invoke<Project>('create_project', { name, description });
        update(s => {
          const newId = parseId(newProj.id);
          return {
            ...s,
            projects: [newProj, ...s.projects],
            activeProjectId: newId
          };
        });
        return newProj;
      } catch (e) {
        console.error("Failed to create project", e);
        throw e;
      }
    },
    setActive(id: string | null) {
      update(s => ({ ...s, activeProjectId: id }));
    }
  };
}

export const projectStore = createProjectStore();
