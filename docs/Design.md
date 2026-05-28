# Q-Agent Design System v4.5

> **Version**: 4.5 · **Authored**: 2026-05-28  
> **Stack**: React 19 + Tailwind CSS v4 + Tauri v2  
> **Fonts**: [Geist](https://vercel.com/font) (UI) + [Geist Mono](https://vercel.com/font) (Code)  
> **Replaces**: Design System v4.2 (2026-05-28)

---

## 0. 제품 디자인 한 줄 정의

> **Q-Agent는 Research-grade local agent workspace다.**  
> 장시간 집중 작업에 적합한 중성 컬러 기반 생산성 UI를 지향한다.  
> 감성적 데모 효과보다 작업성·일관성·확장성·접근성을 우선한다.

### 핵심 레퍼런스 (역할별)

| 레퍼런스 | 참고 영역 |
|---|---|
| **Perplexity** | 검색 결과 인용, 출처 카드, 스레드형 후속 질문 가독성 |
| **Claude** | Artifacts 작업 흐름, Preview/Code 탭, 분할 뷰 |
| **Open WebUI** | 멀티 패널 탐색성, 세션·모델 전환 구조 |
| **Linear / Raycast** | HUD 런처, 키보드 중심 조작, 밀도 높은 도구 UI |

### 디자인 방향 전환 선언

- `v4.1`의 **네온(Cyan Electric) / 글로우(glow) 중심** 다크 모드는 **단기 데모형 표현**으로 판단하여 폐기한다.
- `v4.5`부터는 **저채도 다크 톤 + 최소한의 강조색(Indigo 단일)** 으로 재설계한다.
- 라이트 모드는 **문서형 가독성 + 카드 구분** 중심 밝은 톤을 유지한다.
- 강조색은 **Indigo 600** 하나만 사용하며, 이모셔널 컬러(성공/경고/위험)는 의미 전달 목적으로만 제한 사용한다.

---

## 1. 디자인 철학

| 원칙 | 설명 |
|---|---|
| **Clarity** | 정보 계층이 명확하여 한눈에 파악 가능 |
| **Density** | 전문가용 도구답게 정보 밀도는 높되 클릭 타깃은 충분히 확보 |
| **Focus** | 불필요한 장식 최소화, 콘텐츠 중심 레이아웃 |
| **Consistency** | 화면·모드·클라이언트 간 동일한 정보 구조 유지 |
| **Accessible** | WCAG AA 이상, 키보드 전용 조작, 모션 감소 대응 |

---

## 2. 테마 시스템

두 가지 테마를 지원하며 런타임에 전환 가능하다.  
고대비 모드(`forced-colors: active`)는 시스템 색상으로 자동 대응한다.

### 2.1 Light Mode — Warm Document (기본)

```css
/* 배경 계층 */
--bg-base:       #F8F9FA;   /* Warm Off-white 메인 배경 */
--bg-surface:    #FFFFFF;   /* 카드/패널 표면 */
--bg-elevated:   rgba(255,255,255,0.80); /* 글래스 레이어 */
--bg-overlay:    rgba(248,249,250,0.92); /* 팝업 오버레이 */
--bg-subtle:     #F1F3F5;   /* 구분선 배경, 인라인 코드 배경 */

/* 강조색 — Indigo 단일 계열 */
--accent-primary:        #4F46E5; /* Indigo 600 — 주 강조 */
--accent-primary-hover:  #4338CA; /* Indigo 700 */
--accent-primary-subtle: #EEF2FF; /* Indigo 50 — 배경 강조 */

/* 의미 컬러 (상태 전달 목적만) */
--color-success: #059669; /* Emerald 600 */
--color-warning: #D97706; /* Amber 600 */
--color-danger:  #DC2626; /* Red 600 */
--color-info:    #0284C7; /* Sky 600 */

/* 텍스트 위계 4단계 */
--text-primary:   #111827;  /* Gray 900 */
--text-secondary: #374151;  /* Gray 700 */
--text-tertiary:  #6B7280;  /* Gray 500 */
--text-muted:     #9CA3AF;  /* Gray 400 */
--text-code:      #4F46E5;  /* Indigo (인라인 코드) */

/* 테두리 / 그림자 */
--border:        rgba(17,24,39,0.08);
--border-subtle: rgba(17,24,39,0.05);
--shadow-sm:     0 1px 3px rgba(17,24,39,0.06), 0 1px 2px rgba(17,24,39,0.04);
--shadow-md:     0 4px 16px rgba(17,24,39,0.08), 0 2px 4px rgba(17,24,39,0.04);
--shadow-lg:     0 12px 40px rgba(17,24,39,0.10);
--shadow-panel:  0 2px 8px rgba(17,24,39,0.06);
```

### 2.2 Dark Mode — Slate Workspace

> 순수 네온/글로우 제거. 저채도 슬레이트 계열 다크 톤 + 최소한의 Indigo 강조.

```css
/* 배경 계층 */
--bg-base:       #0F1117;   /* Near-black Slate — 메인 배경 */
--bg-surface:    #1A1D27;   /* 카드/패널 표면 */
--bg-elevated:   rgba(26,29,39,0.85); /* 글래스 레이어 */
--bg-overlay:    rgba(15,17,23,0.95); /* 팝업 오버레이 */
--bg-subtle:     #242736;   /* 구분선 배경, 인라인 코드 배경 */

/* 강조색 — Indigo (다크 대응) */
--accent-primary:        #6366F1; /* Indigo 500 — 다크에서 밝게 조정 */
--accent-primary-hover:  #818CF8; /* Indigo 400 */
--accent-primary-subtle: rgba(99,102,241,0.12); /* 배경 강조 */

/* 의미 컬러 */
--color-success: #10B981; /* Emerald 500 */
--color-warning: #F59E0B; /* Amber 500 */
--color-danger:  #EF4444; /* Red 500 */
--color-info:    #38BDF8; /* Sky 400 */

/* 텍스트 위계 4단계 */
--text-primary:   #E5E7EB;  /* Gray 200 */
--text-secondary: #9CA3AF;  /* Gray 400 */
--text-tertiary:  #6B7280;  /* Gray 500 */
--text-muted:     #4B5563;  /* Gray 600 */
--text-code:      #A5B4FC;  /* Indigo 300 (코드) */

/* 테두리 / 그림자 — glow 없음 */
--border:        rgba(255,255,255,0.07);
--border-subtle: rgba(255,255,255,0.04);
--shadow-sm:     0 1px 3px rgba(0,0,0,0.35);
--shadow-md:     0 4px 16px rgba(0,0,0,0.45);
--shadow-lg:     0 12px 40px rgba(0,0,0,0.55);
--shadow-panel:  0 2px 8px rgba(0,0,0,0.40);
```

### 2.3 고대비 모드 (Forced Colors)

```css
@media (forced-colors: active) {
  --accent-primary: Highlight;
  --text-primary:   CanvasText;
  --bg-surface:     Canvas;
  --border:         ButtonText;
  /* 모든 box-shadow, backdrop-filter 비활성화 */
  * { box-shadow: none !important; backdrop-filter: none !important; }
}
```

---

## 3. 타이포그래피

화면 역할별로 타입 스케일을 분리하여 일관성을 확보한다.

```css
/* 폰트 패밀리 */
--font-ui:   'Geist', 'Pretendard', system-ui, sans-serif;
--font-code: 'Geist Mono', 'JetBrains Mono', monospace;

/* 스케일 (rem, 1rem = 16px) */
--text-xs:   0.75rem;   /* 12px — 레이블, 뱃지, 수치 데이터 */
--text-sm:   0.875rem;  /* 14px — 보조 텍스트, 메타, 툴팁 */
--text-base: 1rem;      /* 16px — 채팅 본문, 문서 본문 */
--text-lg:   1.125rem;  /* 18px — 서브헤딩, 패널 내 소제목 */
--text-xl:   1.25rem;   /* 20px — 패널 제목, 헤딩 */
--text-2xl:  1.5rem;    /* 24px — 섹션 제목 */
--text-3xl:  1.875rem;  /* 30px — 페이지/모달 제목 */

/* 행간 */
--leading-tight:   1.25;  /* 제목, 패널 헤더 */
--leading-normal:  1.5;   /* 채팅 메시지, UI 레이블 */
--leading-relaxed: 1.75;  /* 문서형 긴 답변, 인용문 */

/* 자간 */
--tracking-tight:  -0.02em; /* 대형 제목 */
--tracking-normal: 0;

/* 줄 길이 (가독성) */
--prose-width: 72ch; /* 채팅/문서 본문 최대 줄 길이 */
```

### 화면별 타입 역할

| 화면 | 크기 | 행간 | 비고 |
|---|---|---|---|
| 채팅 본문 | `text-base` | `leading-relaxed` | `prose-width` 제한 |
| 인용 / 출처 | `text-sm` | `leading-normal` | `text-secondary` |
| 코드 블록 | `text-sm` | `leading-normal` | `font-code` |
| 설정 / 메타 | `text-sm` | `leading-normal` | `text-tertiary` |
| 수치 데이터 | `text-xs` | `leading-tight` | `tabular-nums` |
| 패널 헤더 | `text-xl` | `leading-tight` | `font-weight: 600` |
| 모달 제목 | `text-2xl` | `leading-tight` | `font-weight: 700` |

---

## 4. 간격 & 레이아웃 토큰

```css
/* 간격 스케일 (4px 기준) */
--space-1:  4px;
--space-2:  8px;
--space-3:  12px;
--space-4:  16px;
--space-5:  20px;
--space-6:  24px;
--space-8:  32px;
--space-10: 40px;
--space-12: 48px;
--space-16: 64px;

/* 테두리 반경 */
--radius-sm:   4px;    /* 뱃지, 인라인 요소 */
--radius-md:   8px;    /* 버튼, 입력창 */
--radius-lg:   12px;   /* 카드, 패널 */
--radius-xl:   16px;   /* 모달, 시트 */
--radius-full: 9999px; /* 칩, 아바타 */

/* 전환 */
--duration-fast:   120ms;
--duration-normal: 200ms;
--duration-slow:   320ms;
--ease-out:    cubic-bezier(0.16, 1, 0.3, 1);
--ease-in:     cubic-bezier(0.4, 0, 1, 1);
--ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);
```

### 밀도 기준표

| 요소 | 높이 | 패딩 |
|---|---|---|
| 버튼 (기본) | 36px | 8px 16px |
| 버튼 (대) | 44px | 10px 20px |
| 입력창 (채팅) | 48px~auto | 12px 16px |
| 리스트 행 | 40px | 8px 12px |
| 패널 헤더 | 48px | 0 16px |
| 카드 (ProjectCard) | 140px | 16px |
| 툴바 | 44px | — |
| HUD 리소스 바 | 32px | 0 12px |

---

## 5. 레이아웃 구조

### 5.1 기본 레이아웃 원칙

3영역 구조를 기본값으로 정의한다. 모든 클라이언트(Desktop, Browser/PWA)에서 동일한 정보 구조를 유지한다.

```
┌──────────────────────────────────────────────────────────────────┐
│  Global Header / HUD Bar (고정, 48px)                            │
│  [Project ▼] [Mode: Balanced] [Network: 🟢 Local] [VRAM: 9.2GB] │
├─────────────┬──────────────────────────────────┬─────────────────┤
│             │                                  │                 │
│  Left Nav   │     Main Content Area            │  Context Panel  │
│  (260px)    │     (중앙 — 가변 너비)            │  (320~480px)    │
│             │                                  │                 │
│  접기 →     │                                  │  접기 가능      │
│  48px 아이콘│                                  │                 │
└─────────────┴──────────────────────────────────┴─────────────────┘
```

**좌측 내비 (260px)**
- 프로젝트 목록, 세션 목록, Harness Studio 진입
- 접기 시: 48px 아이콘 바로 전환 (툴팁 표시)

**중앙 작업 영역 (flex-1, 최소 480px)**
- Chat & Citation, Artifacts Studio, Computer Tab, Knowledge Map 등 탭 전환

**우측 컨텍스트 패널 (320~480px)**
- Source Cards, Artifact Ready-Zone, Session 목록
- 기본 숨김 → 아티팩트 생성 / 인용 클릭 시 자동 슬라이드 인

### 5.2 패널 UX 규칙

| 동작 | 방식 |
|---|---|
| 패널 접기/펼치기 | 토글 버튼 또는 드래그 핸들 클릭 |
| 패널 리사이즈 | 드래그 핸들, 최소/최대 너비 제한 |
| 패널 고정 (Pin) | 핀 아이콘 — 자동 접힘 방지 |
| 레이아웃 저장 | 프로젝트별 패널 상태 로컬 저장 |
| 포커스 이동 | `Ctrl+[` / `Ctrl+]` 패널 간 이동 |

### 5.3 Antigravity HUD (런처)

```
┌─────────────────────────────────────────────────────┐
│  ▣ [Project: AI Research ▼]  │  ···  │  ⬛ ⬜ ✕  │
│─────────────────────────────────────────────────────│
│  🔍  무엇을 도와드릴까요?            [🌐] [📂]      │
│─────────────────────────────────────────────────────│
│  ⚡ Qwen 2.5 14B  │  VRAM 9.2/16GB  │  🟢 Local    │
└─────────────────────────────────────────────────────┘

크기: 680px × 180px (기본) → 확장 시 전체 대시보드
위치: 화면 중앙 상단 (기본), 사용자 지정 가능
트리거: Alt+Space
```

**HUD 컴포넌트 규격:**
- 배경: `var(--bg-elevated)` + `backdrop-filter: blur(16px)`
- 테두리: 1px solid `var(--border)`
- 그림자: `var(--shadow-lg)`
- 입력창: 전체 너비, 높이 48px, 아이콘 좌측
- 리소스 바: 하단 고정, 높이 32px

### 5.4 Main Mission Control (확장 대시보드)

```
┌──────────────────────────────────────────────────────────────────┐
│  Global Header (상단 고정 48px)                                   │
├──────────────┬──────────────────────────────────┬────────────────┤
│  Harness     │                                  │  Artifacts     │
│  Studio      │      Chat & Citation             │  Ready-Zone    │
│  Sessions    │      (중앙 메인)                  │  / Source Card │
│              │                                  │                │
│  260px       │  flex-1 (최소 480px)              │  320~480px     │
│  (접기 가능) │                                  │  (접기 가능)   │
└──────────────┴──────────────────────────────────┴────────────────┘
```

### 5.5 분할 뷰 (Coding / Ghost Prototyping)

```
┌─────────────────────┬──────────────────────────────────┐
│  Chat & Thought     │  Artifact / WASM Sandbox          │
│  Trace              │                                   │
│                     │  [WASM] [Remote] [Local]          │
│  50%                │  50%                              │
└─────────────────────┴──────────────────────────────────┘
분할 전환: 에이전트가 코드 생성 시작 시 자동
리사이즈: 드래그 핸들로 비율 조정 가능
```

---

## 6. 버튼 & 입력 체계

### 6.1 버튼 (Button)

| Variant | 용도 | 스타일 |
|---|---|---|
| **Primary** | 주 행동 (승인, 실행) | `accent-primary` 배경, 흰 텍스트 |
| **Secondary** | 보조 행동 | `bg-subtle` 배경, `text-primary` |
| **Ghost** | 텍스트형, 패널 내 | 배경 없음, hover 시 `bg-subtle` |
| **Danger** | 위험·삭제 | `color-danger` 배경 또는 텍스트 |
| **Icon** | 아이콘 전용 | 36px 정사각형, `aria-label` 필수 |

### 6.2 입력창 (Input)

| 용도 | 높이 | 스타일 |
|---|---|---|
| **Chat Input** | 48px~auto | 다중 행 자동 확장, Shift+Enter 줄바꿈 |
| **Search** | 40px | 좌측 검색 아이콘, 우측 단축키 힌트 |
| **Form Field** | 36px | 레이블 상단, 오류 메시지 하단 |
| **Code Editor** | auto | `font-code`, 줄 번호, 탭 들여쓰기 |

---

## 7. 상태 배지 & 칩

### 7.1 네트워크/권한 상태 배지

| 배지 | 색상 | 의미 |
|---|---|---|
| 🟢 **Local-only** | `color-success` | 외부 연결 없음, 완전 로컬 |
| 🟡 **LAN Share** | `color-warning` | 동일 네트워크 공유 중 |
| 🔴 **Approval Needed** | `color-danger` | HITL 승인 대기 |
| 🔵 **Agentic** | `accent-primary` | 자율 실행 모드 |
| ⚪ **Offline** | `text-muted` | 인터넷 단절, 로컬 지식만 사용 |

> 이 배지는 항상 Global Header에 표시되어야 한다. 3단계 이내 핵심 작업 접근을 보장하는 기준점이다.

### 7.2 Citation Chip

- 인라인 `[N]` — `text-xs`, `accent-primary-subtle` 배경
- 클릭 시 오른쪽 Source Card 슬라이드 인
- 신뢰도 점수 ★~★★★★★ 함께 표시 (최대 5개, 회색 미채움)

---

## 8. 핵심 컴포넌트

### 8.1 ProjectCard (Harness Studio)

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
radius: var(--radius-lg)
hover: translateY(-2px) + shadow-md (200ms ease-out)
```

### 8.2 ChatMessage

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
│  ─────────────────────────────────────           │
│  [1] src/api/auth.rs  ★★★★☆                    │
│  [2] docs/api-spec.md ★★★★★                    │
└─────────────────────────────────────────────────┘
```

- **Thought Trace**: 기본 접힌 상태, 클릭 시 펼쳐지며 단계별 로그 표시
- **Citation Chip**: 인라인 `[N]` — 클릭 시 Source Card 슬라이드 인
- **최대 줄 길이**: `prose-width` (72ch) 제한

### 8.3 Artifact Card

| 탭 | 기능 |
|---|---|
| **Preview** | 렌더링된 결과 (HTML / Markdown / SVG / Mermaid) |
| **Code** | 소스 코드 + 복사 버튼 + 언어 레이블 |
| **Diff** | 이전 버전과 변경 사항 (추가: 초록, 삭제: 빨강) |
| **Fullscreen** | 전체화면 보기, Escape 로 복귀 |

### 8.4 Source Card

```
┌────────────────────────────────────────┐
│  [1] src/api/auth.rs                   │
│  ★★★★☆ — 로컬 파일                   │
│  "...회원가입 핸들러 함수..."           │
│  [원문 열기]  [컨텍스트 보기]           │
└────────────────────────────────────────┘
```

### 8.5 HITL Approval Modal

```
┌────────────────────────────────────────────────┐
│  ⚠️  작업 승인 필요                              │
│  위험도: ████░░░░░░ 0.42                       │
│─────────────────────────────────────────────────│
│  📁 파일 쓰기   💻 터미널 실행                  │
│  범위: ~/projects/q-agent/src/ 내부              │
│                                                  │
│  > npm install react@19 react-dom@19            │
│  > npm install @tauri-apps/api                  │
│                                                  │
│  실행 환경:  [◉ WASM]  [○ Remote]  [○ Local]   │
│─────────────────────────────────────────────────│
│  [이력 보기]  [거부]          [승인]             │
└────────────────────────────────────────────────┘

위험도 바 색상: 0~0.5 → color-warning | 0.5~0.7 → amber | 0.7+ → color-danger
승인 버튼: accent-primary | 거부: ghost
```

### 8.6 KnowledgeGraph Visualizer

- **라이브러리**: `@xyflow/react` (React Flow v12)
- **노드 색상**:
  - Project Private: `#3B82F6` (Blue 500)
  - Shared Pool: `#10B981` (Emerald 500)
  - Global Commons: `#F59E0B` (Amber 500)
- **다크 모드**: 글로우 없음, 노드 테두리(`1.5px solid`)만 강조
- **엣지**: 관련 노드 연결선 두껍게 + 점선 흐름 애니메이션 (파란색 고정)
- **인터랙션**: 노드 클릭 → 우측 Context Panel에 Document Preview 슬라이드 인
- **성능**: 노드 100개 초과 시 가상화(virtualization) 적용

### 8.7 ResourceMonitor (HUD 하단)

```
⚡ Qwen 2.5 14B  │  ████████░░ VRAM 9.2/16GB  │  Ops 82%  │  🟢 Local
```

- 실시간 업데이트: 2초 간격
- VRAM 80%+ → `color-warning`
- VRAM 95%+ → `color-danger` + 펄스 애니메이션
- 네트워크 상태 배지 (Local-only / LAN / Offline) 항상 표시

### 8.8 Session Card

```
┌──────────────────────────────────────┐
│  🖥️ MacBook Air (LAN)  │  🟡         │
│  sangk · 12분 전 접속               │
│  권한: 읽기 전용 | 채팅 전용         │
│  [세부 정보]  [세션 종료]            │
└──────────────────────────────────────┘
```

### 8.9 TemplateSelector (Harness Studio)

```
페르소나       [researcher ▼]  [편집]
하네스         [standard   ▼]  [편집]
오케스트레이션  [balanced   ▼]  [편집]
모델 (Small)   [Qwen2.5 3B ▼]
모델 (Heavy)   [Mistral 12B ▼]
fvcore 상한    [━━━━━━●━━━] 70%
Iteration      [━━━━●━━━━━] 5회
```

### 8.10 Model Hub Card (Settings > Models)

```
┌────────────────────────────────────────────┐
│  Qwen2.5-Coder 14B           [추천] ★      │
│  ─────────────────────────────────────     │
│  역할: Coder     양자화: Q6_K              │
│  VRAM: 10.2GB    속도: ~28 tok/s          │
│  ██████████░░ 점유율 예상                   │
│  [다운로드]  [활성화]                       │
└────────────────────────────────────────────┘

너비: 280px | radius: var(--radius-lg)
추천 배지: accent-primary-subtle 배경 + accent-primary 텍스트
```

---

## 9. Trajectory Timeline & Self-Eval Dashboard

에이전트 작업 흐름과 자가 평가 결과를 시각화하는 패널.

### 9.1 Trajectory Timeline

```
[타임라인]
00:00  📁 파일 읽기    src/api/auth.rs
00:02  🔍 검색         "JWT 인증 패턴"   [3개 결과]
00:05  🧠 계획 수립    TaskPlan (4 steps)
00:08  💻 코드 생성    auth_handler.rs
00:12  🧪 테스트 실행  cargo test → ✅ 3/3 통과
00:15  📝 문서 작성    API spec 업데이트
         └─ 총 소요: 15초 | 토큰: 1,240 / 8,000
```

- 각 이벤트 아이콘: Tool Call 종류별 색상 구분
- 클릭 시 해당 스텝의 입출력 전체 보기
- 토큰 예산 소비 게이지 함께 표시

### 9.2 Self-Eval Dashboard

```
┌────────────────────────────────────────────┐
│  📊 태스크 성능 리포트                      │
│─────────────────────────────────────────────│
│  답변 신뢰도       ████████░░  82%          │
│  답변 관련성       █████████░  90%          │
│  계획 완수율       ████████░░  85%          │
│─────────────────────────────────────────────│
│  💡 개선 제안                               │
│  · 외부 검색 결과 2개 미활용됨              │
│  · 파일 쓰기 전 단위 테스트 추가 권장       │
│─────────────────────────────────────────────│
│  [상세 리포트]  [히스토리]                  │
└────────────────────────────────────────────┘
```

- 각 지표 색상: 90%+ → `color-success`, 70~89% → `color-warning`, <70% → `color-danger`
- 태스크 종료 시 자동 슬라이드 인 (우측 Context Panel)

---

## 10. 빈 / 로딩 / 오류 상태

| 상태 | 표현 방식 |
|---|---|
| **빈 상태** | 중앙 아이콘(32px) + 짧은 안내 문구 + 행동 버튼 |
| **로딩** | 스켈레톤 UI — 텍스트 영역 회색 블록, 카드 펄스 |
| **스트리밍** | 타이핑 커서 블링크, 레이아웃 점프 없는 점진적 렌더링 |
| **오류** | `color-danger` 인라인 배너 + 재시도 버튼 |
| **검색 폴백** | 상단 안내 배너: "외부 검색 장애 — 로컬 지식으로 전환 중" |
| **오프라인** | Global Header 배지 → `⚪ Offline` + 채팅 입력 하단 안내 |
| **재시도** | 오류 배너 + 카운트다운 + 수동 재시도 버튼 |

---

## 11. 애니메이션 가이드

> `prefers-reduced-motion: reduce` 환경에서는 모든 전환을 즉시(0ms)로 대체한다.

| 요소 | 트리거 | 효과 | 시간 |
|---|---|---|---|
| HUD 등장 | Alt+Space | scale(0.97)→1 + opacity 0→1 | 160ms ease-spring |
| 대시보드 확장 | 입력 시작 | height 180px→100vh | 280ms ease-out |
| 카드 호버 | hover | translateY(-2px) + shadow-md | 150ms ease-out |
| Thought Trace | 펼치기 | height 0→auto | 200ms ease-out |
| Source Card | 인용 클릭 | right panel slide in | 250ms ease-out |
| Artifact 탭 | 탭 전환 | content fade + slide | 150ms ease-out |
| 모달 등장 | HITL 트리거 | backdrop 점진 + scale | 200ms ease-out |
| 코드 스트리밍 | Agent 생성 중 | 타이핑 커서 블링크 | — |
| Split View | 코딩 시작 | right panel slide in | 300ms ease-out |
| 노드 생성 | GraphRAG | scale(0)→1 | 200ms ease-spring |
| Self-Eval 슬라이드 | 태스크 완료 | right panel slide in | 280ms ease-out |

---

## 12. 아이콘 시스템

- **라이브러리**: `lucide-react` (선 굵기 1.5px 일관)
- **크기**: 16px (인라인), 20px (버튼/리스트), 24px (메뉴), 32px (헤딩/빈 상태)
- **색상**: 컨텍스트에 따라 `text-primary` / `text-muted` / `accent-primary`
- **아이콘 전용 버튼**: 반드시 `aria-label` 또는 `title` 툴팁 제공

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
| 네트워크 상태 | `Wifi`, `WifiOff`, `Share2` |
| 타임라인/이력 | `History`, `Clock`, `BarChart3` |
| 모델 허브 | `Cpu`, `Download`, `Sparkles` |

---

## 13. 반응형 & 멀티 클라이언트

### 13.1 브레이크포인트

```css
@custom-variant sm     { @media (width >= 640px) }
@custom-variant md     { @media (width >= 768px) }
@custom-variant lg     { @media (width >= 1024px) }
@custom-variant xl     { @media (width >= 1280px) }
@custom-variant 2xl    { @media (width >= 1536px) }

/* Q-Agent 전용 */
@custom-variant compact { @media (width < 900px) }   /* HUD 전용 모드 */
@custom-variant panel   { @media (width >= 1200px) } /* 3패널 레이아웃 */
```

| 화면 | 레이아웃 |
|---|---|
| < 900px | HUD 전용 + 전체화면 채팅 (사이드바 숨김) |
| 900~1199px | 2패널 (Left Nav + 채팅) |
| 1200px+ | 3패널 (Left Nav + 채팅 + Context Panel) |

### 13.2 Browser/PWA 기능 대응표

| 기능 | Tauri Desktop | Browser/PWA |
|---|---|---|
| 파일 시스템 직접 접근 | ✅ 전체 | ❌ API 경유만 |
| HITL 승인 | ✅ | ✅ (네트워크 경유) |
| 채팅 / Artifacts | ✅ | ✅ |
| HUD (Alt+Space) | ✅ | ⚠️ 단축키 불가, 버튼 대체 |
| 로컬 모델 실행 | ✅ | ❌ 원격 서버 연결만 |
| GraphRAG 시각화 | ✅ | ✅ |
| Trajectory Timeline | ✅ | ✅ |
| Self-Eval Dashboard | ✅ | ✅ |
| 권한 / 세션 관리 | ✅ | ✅ (읽기 중심) |
| Model Hub | ✅ | ⚠️ 조회만, 다운로드 불가 |

---

## 14. 접근성 (A11y)

- **ARIA**: 모든 인터랙티브 요소에 `aria-label` 또는 `aria-labelledby` 필수
- **키보드 네비게이션**:
  - `Tab` 순서 논리적 배치 (좌→우, 위→아래)
  - HUD, 채팅 입력, 승인 모달, 탭 전환 모두 키보드만으로 완료 가능
  - `Escape` 로 모달/패널 닫기
  - `Ctrl+[` / `Ctrl+]` 패널 간 포커스 이동
- **포커스 링**: `outline: 2px solid var(--accent-primary)`, `outline-offset: 2px`
- **색상 대비**:
  - 텍스트 최소 4.5:1 (WCAG AA)
  - 아이콘/UI 컨트롤 최소 3:1
- **모션 감소**: `@media (prefers-reduced-motion)` 시 모든 전환·애니메이션 비활성화
- **고대비 모드**: `forced-colors: active` 대응 (섹션 2.3)
- **스크린 리더**: 에이전트 상태 변화 시 `aria-live="polite"` 알림

---

## 15. 성능 체감

| 항목 | 전략 |
|---|---|
| 초기 진입 | 채팅 패널 우선 렌더 (GraphRAG 맵, Artifact 뷰는 lazy) |
| 스트리밍 | 점진적 DOM 삽입, 레이아웃 점프 방지 (`min-height` 예약) |
| 스켈레톤 | 패널 진입 시 회색 블록 스켈레톤 즉시 표시 |
| GraphRAG | 노드 100개 초과 시 가상화(virtualization) 적용 |
| Artifact Preview | iframe sandbox + 점진적 로딩 |
| 저사양 대응 | `prefers-reduced-motion` 적용, 파티클·이펙트 전혀 없음 |
| Model Hub | 다운로드 상태 스트리밍 진행률 표시 |

---

## 16. 문서화 & 거버넌스

### 산출물 현황 (Phase 1.5)

| 산출물 | 상태 | 위치 |
|---|---|---|
| 디자인 방향 문서 | ✅ 확정 | `docs/Design.md` (이 문서) |
| 디자인 토큰 정의서 | ✅ 확정 | 섹션 2~4 |
| 핵심 화면 와이어프레임 | 🔄 진행 중 | 섹션 5~8 ASCII 스펙 — Storybook 전환 예정 |
| 핵심 컴포넌트 목록 | 🔄 진행 중 | 섹션 6~9 — 구현 검증 필요 |
| 다크/라이트/고대비 기준안 | ✅ 확정 | 섹션 2 |
| Desktop / Browser(PWA) 레이아웃 가이드 | ✅ 확정 | 섹션 13 |
| 접근성 및 성능 점검 결과 | ✅ 확정 | 섹션 14~15 |

### 변경 이력

| 버전 | 날짜 | 주요 변경 |
|---|---|---|
| v4.1 | 2026-05-13 | 초기 디자인 시스템 (네온/글로우 다크 모드) |
| v4.2 | 2026-05-28 | Plan.md 8-1 기준 재정비: 저채도 다크 톤, Indigo 단일 강조, Research-grade UI 방향, 멀티클라이언트·접근성·성능 섹션 |
| v4.5 | 2026-05-28 | Plan v4.5 연동: Trajectory Timeline, Self-Eval Dashboard, Model Hub Card 추가, 버전 통합 |

---

*마지막 업데이트: 2026-05-28 · Design System v4.5*  
*이 문서는 `docs/` 내에서만 관리됩니다.*
