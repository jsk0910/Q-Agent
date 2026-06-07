import { useEffect, useState } from 'react';
import { MainLayout } from './components/layout/MainLayout';
import { ChatArea } from './components/chat/ChatArea';
import { Hud } from './components/layout/Hud';
import { HitlModal } from './components/harness/HitlModal';

function App() {
  const [isHudMode, setIsHudMode] = useState(false);

  useEffect(() => {
    // 윈도우 크기에 따른 초기 HUD 모드 설정
    const handleResize = () => {
      if (window.innerHeight < 250) {
         setIsHudMode(true);
      }
    };
    handleResize();
    window.addEventListener('resize', handleResize);

    // Alt+Space 단축키로 HUD 모드 토글
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.altKey && e.code === 'Space') {
        e.preventDefault();
        setIsHudMode((prev) => !prev);
      }
    };
    window.addEventListener('keydown', handleKeyDown);

    // HUD에서 메시지 전송 시 대시보드로 자동 전환
    const handleHudExpand = () => {
      setIsHudMode(false);
    };
    window.addEventListener('hud-expand', handleHudExpand);

    return () => {
      window.removeEventListener('resize', handleResize);
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('hud-expand', handleHudExpand);
    };
  }, []);

  return (
    <>
      {isHudMode ? (
        <Hud />
      ) : (
        <MainLayout>
          <ChatArea />
        </MainLayout>
      )}
      <HitlModal />
    </>
  );
}

export default App;
