import React from 'react';
import { useAppStore } from '../../stores/useAppStore';
import { Sidebar } from './Sidebar';
import { RightPanel } from './RightPanel';
import { GlobalHeader } from './GlobalHeader';

interface MainLayoutProps {
  children: React.ReactNode;
}

export const MainLayout: React.FC<MainLayoutProps> = ({ children }) => {
  const { isSidebarOpen, isRightPanelOpen } = useAppStore();

  return (
    <div className="flex h-screen w-full bg-[var(--bg-base)] text-[var(--text-primary)] overflow-hidden">
      {/* 좌측 사이드바 (Harness Studio & NotebookLM) */}
      <div 
        className={`transition-all duration-300 ease-out border-r border-[var(--border)] bg-[var(--bg-surface)] ${
          isSidebarOpen ? 'w-[260px]' : 'w-[48px]'
        } flex-shrink-0 flex flex-col z-20`}
      >
        <Sidebar />
      </div>

      {/* 중앙 및 우측 패널 컨테이너 */}
      <div className="flex-1 flex flex-col min-w-0">
        {/* Global Header */}
        <GlobalHeader />

        <div className="flex-1 flex overflow-hidden relative">
          {/* 중앙 메인 뷰 (Chat & Citation) */}
          <main className="flex-1 flex flex-col h-full relative overflow-hidden">
            {children}
          </main>

          {/* 우측 패널 (Artifacts Ready-Zone / Sources) */}
          <div 
            className={`transition-all duration-300 ease-out border-l border-[var(--border)] bg-[var(--bg-surface)] flex-shrink-0 flex flex-col z-10 ${
              isRightPanelOpen ? 'w-[320px] max-w-[480px] translate-x-0' : 'w-[320px] absolute right-0 translate-x-[100%] opacity-0 pointer-events-none'
            }`}
            style={{ height: '100%' }}
          >
            <RightPanel />
          </div>
        </div>
      </div>
    </div>
  );
};
