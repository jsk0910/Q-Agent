import { useEffect, useState } from 'react';
import { MainLayout } from './components/layout/MainLayout';
import { ChatArea } from './components/chat/ChatArea';
import { Hud } from './components/layout/Hud';
import { HitlModal } from './components/harness/HitlModal';

function App() {
  const [isHudMode, setIsHudMode] = useState(true);

  useEffect(() => {
    // 간단한 반응형 크기 감지로 모드 전환 (HUD는 680px)
    const handleResize = () => {
      setIsHudMode(window.innerWidth < 800);
    };
    
    handleResize();
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
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
