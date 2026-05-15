import React from 'react';
import { useAppStore } from '../../stores/useAppStore';
import { Sidebar } from './Sidebar';
import { RightPanel } from './RightPanel';

interface MainLayoutProps {
  children: React.ReactNode;
}

export const MainLayout: React.FC<MainLayoutProps> = ({ children }) => {
  const { isSidebarOpen } = useAppStore();

  return (
    <div className="flex h-screen w-full bg-[var(--bg-base)] text-[var(--text-primary)] overflow-hidden">
      {/* 좌측 사이드바 (Harness Studio & NotebookLM) */}
      <div 
        className={`transition-all duration-300 ease-out border-r border-[var(--border)] bg-[var(--bg-surface)] ${
          isSidebarOpen ? 'w-[260px]' : 'w-[48px]'
        } flex-shrink-0 flex flex-col`}
      >
        <Sidebar />
      </div>

      {/* 중앙 메인 뷰 (Chat & Citation) */}
      <main className="flex-1 flex flex-col h-full relative overflow-hidden">
        {children}
      </main>

      {/* 우측 패널 (Artifacts Ready-Zone) */}
      <div className="w-[320px] border-l border-[var(--border)] bg-[var(--bg-surface)] flex-shrink-0 flex flex-col hidden lg:flex">
        <RightPanel />
      </div>
    </div>
  );
};
