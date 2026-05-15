# Q-Agent Design System v4.1

> **Version**: 4.1 · **Authored**: 2026-05-13
> **Stack**: React 19 + Tailwind CSS v4 + Tauri v2
> **Fonts**: [Geist](https://vercel.com/font) (UI) + [Geist Mono](https://vercel.com/font) (Code)

---

## 1. 디자인 철학

| 원칙 | 설명 |
|---|---|
| **Clarity** | 정보 계층이 명확하여 한눈에 파악 가능 |
| **Depth** | 글래스모피즘 + 레이어드 쉐도우로 공간감 표현 |
| **Alive** | 마이크로 애니메이션으로 반응하는 UI |
| **Focus** | 불필요한 장식 최소화, 콘텐츠 중심 레이아웃 |

---

## 2. 테마 시스템

두 가지 테마를 지원하며 런타임에 전환 가능하다.

### 2.1 Minimalist White (기본)

```css
/* 배경 계층 */
--bg-base:       #F9FAFB;   /* Off-white 메인 배경 */
--bg-surface:    #FFFFFF;   /* 카드/패널 표면 */
--bg-elevated:   rgba(255,255,255,0.72); /* 글래스 레이어 */
--bg-overlay:    rgba(249,250,251,0.85); /* 팝업 오버레이 */

/* 포인트 컬러 */
--accent-primary:   #4F46E5; /* Indigo 600 */
--accent-secondary: #7C3AED; /* Violet 600 */
--accent-success:   #059669; /* Emerald 600 */
--accent-warning:   #D97706; /* Amber 600 */
--accent-danger:    #DC2626; /* Red 600 */

/* 텍스트 */
--text-primary:   #0F172A;  /* Slate 900 */
--text-secondary: #475569;  /* Slate 600 */
--text-muted:     #94A3B8;  /* Slate 400 */
--text-code:      #4F46E5;  /* Indigo (인라인 코드) */

/* 테두리/그림자 */
--border:       rgba(15,23,42,0.08);
--shadow-sm:    0 1px 3px rgba(15,23,42,0.06), 0 1px 2px rgba(15,23,42,0.04);
--shadow-md:    0 4px 16px rgba(15,23,42,0.08), 0 2px 4px rgba(15,23,42,0.04);
--shadow-lg:    0 12px 40px rgba(15,23,42,0.12);
--shadow-glass: 0 8px 32px rgba(79,70,229,0.08);
```

### 2.2 Dark Modern (다크 모드)

```css
/* 배경 계층 */
--bg-base:       #0D0D0D;   /* Jet Black */
--bg-surface:    #161616;   /* 카드/패널 */
--bg-elevated:   rgba(22,22,22,0.80); /* 글래스 */
--bg-overlay:    rgba(13,13,13,0.92); /* 팝업 */

/* 포인트 컬러 */
--accent-primary:   #06B6D4; /* Cyan 500 (Electric) */
--accent-secondary: #8B5CF6; /* Violet 500 */
--accent-success:   #10B981; /* Emerald 500 */
--accent-warning:   #F59E0B; /* Amber 500 */
--accent-danger:    #EF4444; /* Red 500 */

/* 텍스트 */
--text-primary:   #F1F5F9;  /* Slate 100 */
--text-secondary: #94A3B8;  /* Slate 400 */
--text-muted:     #475569;  /* Slate 600 */
--text-code:      #4ADE80;  /* Neon Green */

/* 테두리/그림자 */
--border:         rgba(255,255,255,0.06);
--shadow-sm:      0 1px 3px rgba(0,0,0,0.4);
--shadow-md:      0 4px 16px rgba(0,0,0,0.5);
--shadow-lg:      0 12px 40px rgba(0,0,0,0.6);
--shadow-glow:    0 0 20px rgba(6,182,212,0.15);
```

---

## 3. 타이포그래피

```css
/* 폰트 패밀리 */
--font-ui:   'Geist', 'Pretendard', system-ui, sans-serif;
--font-code: 'Geist Mono', 'JetBrains Mono', monospace;

/* 스케일 (rem 기준, 1rem = 16px) */
--text-xs:   0.75rem;   /* 12px — 레이블, 뱃지 */
--text-sm:   0.875rem;  /* 14px — 보조 텍스트, 메타 */
--text-base: 1rem;      /* 16px — 본문 */
--text-lg:   1.125rem;  /* 18px — 서브헤딩 */
--text-xl:   1.25rem;   /* 20px — 헤딩 */
--text-2xl:  1.5rem;    /* 24px — 패널 제목 */
--text-3xl:  1.875rem;  /* 30px — 페이지 제목 */

/* 자간/행간 */
--leading-tight:  1.25;
--leading-normal: 1.5;
--leading-relaxed: 1.75;
--tracking-tight: -0.02em;
--tracking-normal: 0;
```

---

## 4. 간격 & 레이아웃

```css
/* 간격 스케일 (4px 기준) */
--space-1: 4px;
--space-2: 8px;
--space-3: 12px;
--space-4: 16px;
--space-5: 20px;
--space-6: 24px;
--space-8: 32px;
--space-10: 40px;
--space-12: 48px;
--space-16: 64px;

/* 테두리 반경 */
--radius-sm:   6px;
--radius-md:   12px;
--radius-lg:   18px;
--radius-xl:   24px;
--radius-full: 9999px;

/* 전환 */
--duration-fast:   120ms;
--duration-normal: 200ms;
--duration-slow:   350ms;
--ease-out:  cubic-bezier(0.16, 1, 0.3, 1);
--ease-in:   cubic-bezier(0.4, 0, 1, 1);
--ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);
```

---

## 5. 레이아웃 구조

### 5.1 Antigravity HUD (런처)

```
┌─────────────────────────────────────────────────────┐
│  ▣ [Project: AI Research ▼]  │  ···  │  ⬛ ⬜ ✕  │
│─────────────────────────────────────────────────────│
│  🔍  무엇을 도와드릴까요?            [🌐] [📂]      │
│─────────────────────────────────────────────────────│
│  ⚡ Qwen 2.5 14B  │  VRAM 9.2/16GB  │  Ops: 82%  │
└─────────────────────────────────────────────────────┘

크기: 680px × 180px (기본) → 확장 시 전체 대시보드
위치: 화면 중앙 상단 (기본), 사용자 지정 가능
트리거: Alt+Space
```

**HUD 컴포넌트 규격:**
- 배경: `var(--bg-elevated)` + `backdrop-filter: blur(20px)`
- 테두리: 1px solid `var(--border)`
- 그림자: `var(--shadow-glass)`
- 입력창: 전체 너비, 높이 48px, 아이콘 좌측
- 리소스 바: 하단 고정, 높이 32px

### 5.2 Main Mission Control (확장 대시보드)

```
┌──────────────────────────────────────────────────────────────────┐
│  Antigravity HUD (상단 고정)                                      │
├────────┬──────────────────────────────────────┬─────────────────┤
│Harness │                                      │  Artifacts      │
│Studio  │      Chat & Citation                 │  Ready-Zone     │
│        │      (중앙 메인)                      │                 │
│  260px │                                      │  320px          │
├────────┤                                      │                 │
│Local   │                                      │                 │
│Notebook│                                      │                 │
│LM      │                                      │                 │
└────────┴──────────────────────────────────────┴─────────────────┘

전체: 1280px ~ 1920px 반응형
좌측 사이드바: 260px (접기 가능 → 48px 아이콘 바)
우측 패널: 320px ~ 480px (Artifact 뷰 시 50% 분할)
```

### 5.3 분할 뷰 (Coding / Ghost Prototyping)

```
┌─────────────────────┬──────────────────────────────────┐
│  Chat & Thought     │  Artifact / WASM Sandbox          │
│  Trace              │                                   │
│                     │  [WASM] [Remote] [Local]         │
│  50%                │  50%                              │
└─────────────────────┴──────────────────────────────────┘
분할 전환: 에이전트가 코드 생성 시작 시 자동
리사이즈: 드래그 핸들로 비율 조정 가능
```

---

## 6. 핵심 컴포넌트

### 6.1 ProjectCard (Harness Studio)

```
┌────────────────────────────────┐
│  🔬 AI Research                │
│  Mistral NeMo 12B              │
│  ────────────────────          │
│  🔒 Standard  │  Cap: 5회      │
│  Ops ████░░░░ 60%              │
│  [설정]  [활성화]               │
└────────────────────────────────┘

너비: 240px, 높이: 140px
테두리 반경: var(--radius-lg)
호버: translateY(-2px) + shadow-md
```

### 6.2 ChatMessage

```
[USER]
┌─────────────────────────────────────────────────┐
│  이 프로젝트 구조에서 회원가입 API를 짜줘        │
└─────────────────────────────────────────────────┘

[AGENT]
📍 Planner → 🔍 Critic → 💻 Coder  [펼치기 ▸]

┌─────────────────────────────────────────────────┐
│  회원가입 API 구조입니다 [1][2]                   │
│  ...                                             │
│                                                  │
│  [1] src/api/auth.rs  [2] docs/api-spec.md      │
└─────────────────────────────────────────────────┘
```

**Thought Trace**: 기본 접힌 상태, 클릭 시 펼쳐지며 단계별 로그 표시.
**Citation 뱃지**: 인라인 `[N]` — 클릭 시 오른쪽에 Source Card 슬라이드 인.

### 6.3 HITL Approval Modal

```
┌────────────────────────────────────────────────┐
│  ⚠️  작업 승인 필요                              │
│─────────────────────────────────────────────────│
│  📁 파일 쓰기   💻 터미널 실행                  │
│                                                  │
│  범위: ~/projects/q-agent/src/ 내부              │
│                                                  │
│  > npm install react@19 react-dom@19            │
│  > npm install @tauri-apps/api                  │
│                                                  │
│  실행 환경:  [◉ WASM]  [○ Remote]  [○ Local]   │
│─────────────────────────────────────────────────│
│           [거부]              [승인]             │
└────────────────────────────────────────────────┘

승인 버튼: accent-primary, 거부: text-muted
Dark 모드: "ACCESS GRANTED" 기계적 애니메이션
```

### 6.4 KnowledgeGraph Visualizer

- **라이브러리**: `@xyflow/react` (React Flow v12)
- **노드 색상**: Project Private `#3B82F6`, Shared Pool `#10B981`, Global Commons `#F59E0B`
- **Dark 모드**: 각 노드에 Glow 효과 (`box-shadow: 0 0 12px <color>40`)
- **엣지**: 답변 생성 시 관련 노드 연결선 두껍게 + 애니메이션 (점선 흐름)
- **인터랙션**: 노드 클릭 → 오른쪽 Document Preview 슬라이드 인

### 6.5 ResourceMonitor (HUD 하단)

```
⚡ Qwen 2.5 14B  │  ████████░░ VRAM 9.2/16GB  │  Ops 82%  │  🟢
```

- 실시간 업데이트: 2초 간격 폴링
- VRAM 80%+ → 주황 경고색
- VRAM 95%+ → 빨간 경고 + 애니메이션 펄스
- Ops(fvcore) 막대: 남은 연산량 퍼센트

### 6.6 TemplateSelector (Harness Studio)

```
페르소나    [researcher ▼]  [편집]
하네스      [standard   ▼]  [편집]
오케스트레이션 [balanced ▼]  [편집]
모델 (Small) [Qwen2.5 3B ▼]
모델 (Heavy) [Mistral 12B ▼]
fvcore 상한  [━━━━━━●━━━] 70%
Iteration    [━━━━●━━━━━] 5회
```

---

## 7. 애니메이션 가이드

| 요소 | 트리거 | 효과 | 시간 |
|---|---|---|---|
| HUD 등장 | Alt+Space | scale(0.96)→1 + opacity 0→1 | 160ms ease-spring |
| 대시보드 확장 | 입력 시작 | height 180px→100vh | 280ms ease-out |
| 카드 호버 | hover | translateY(-2px) + shadow-md | 150ms ease-out |
| Thought Trace | 펼치기 | height 0→auto | 200ms ease-out |
| 노드 생성 | Drag&Drop | scale(0)→1 + glow | 250ms ease-spring |
| 모달 등장 | HITL 트리거 | backdrop-blur 증가 + scale | 200ms ease-out |
| 코드 스트리밍 | Agent 생성 중 | 타이핑 커서 블링크 | — |
| Split View | 코딩 시작 | right panel slide in | 300ms ease-out |

---

## 8. 아이콘 시스템

- **라이브러리**: `lucide-react` (일관된 선 굵기 1.5px)
- **크기**: 16px (인라인), 20px (버튼), 24px (메뉴), 32px (헤딩)
- **색상**: 컨텍스트에 따라 `text-primary` / `text-muted` / `accent-primary`

**주요 아이콘 매핑:**

| 기능 | 아이콘 |
|---|---|
| 파일 작업 | `FileText`, `FolderOpen`, `Upload` |
| 터미널 실행 | `Terminal`, `Play`, `Square` |
| 브라우저 제어 | `Globe`, `MousePointer` |
| SSH | `Server`, `Link` |
| 보안/승인 | `ShieldCheck`, `AlertTriangle`, `Lock` |
| 에이전트 상태 | `Brain`, `Loader2` (spin), `CheckCircle2` |
| GraphRAG | `Network`, `GitBranch` |
| 플러그인 | `Puzzle`, `Store` |

---

## 9. 반응형 브레이크포인트

```css
/* Tailwind v4 커스텀 */
@custom-variant sm  { @media (width >= 640px) }
@custom-variant md  { @media (width >= 768px) }
@custom-variant lg  { @media (width >= 1024px) }
@custom-variant xl  { @media (width >= 1280px) }
@custom-variant 2xl { @media (width >= 1536px) }

/* Q-Agent 전용 */
@custom-variant compact { @media (width < 900px) }  /* HUD 전용 모드 */
@custom-variant panel   { @media (width >= 1200px) } /* 3패널 레이아웃 */
```

| 화면 | 레이아웃 |
|---|---|
| < 900px | HUD 전용 + 전체화면 채팅 (사이드바 숨김) |
| 900~1199px | 2패널 (사이드바 + 채팅) |
| 1200px+ | 3패널 (Harness + 채팅 + Artifacts) |

---

## 10. 접근성 (A11y)

- 모든 인터랙티브 요소에 `aria-label` 필수
- 키보드 네비게이션: `Tab` 순서 논리적 배치
- 포커스 링: `outline: 2px solid var(--accent-primary)`, `outline-offset: 2px`
- 색상 대비: WCAG AA 준수 (텍스트 최소 4.5:1)
- 모션 감소: `@media (prefers-reduced-motion)` 애니메이션 비활성화

---

*마지막 업데이트: 2026-05-13 · Design System v4.1*
*이 문서는 `docs/` 내에서만 관리됩니다.*
