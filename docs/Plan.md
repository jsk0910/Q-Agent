# Q-Agent Master Plan v4.1

> **Version**: 4.1 · **Authored**: 2026-05-13 · **Updated**: 2026-05-13
> **Status**: Active Blueprint — 방향 확정 및 로드맵 정제
> **Replaces**: Plan.md v4.0 (2026-05-13)

---

## 0. 비전 & 핵심 철학

**Q-Agent**는 개인 PC에서 완전히 동작하는 자율형 AI 에이전트 플랫폼이다.

> *"클라우드 없이, 데이터 유출 없이 — 내 PC가 곧 나만의 AI 연구실."*

### 핵심 원칙

| 원칙 | 설명 |
|---|---|
| **Privacy-First** | 모든 추론·지식·재무 데이터는 로컬에서만 처리 |
| **On-device Intelligence** | llama.cpp 기반 온디바이스 추론으로 데이터 주권 확보 |
| **Transparent Agency** | LangGraph 사고 과정을 실시간으로 사용자에게 공개 |
| **Human in the Loop** | 고위험 작업은 반드시 사용자 승인 후 실행 |
| **Resource Governance** | fvcore 연산량 상한 + Iteration Cap으로 자원 점유 방지 |

---

## 1. 기술 스택 (Foundation)

### 1.1 코어 스택

| 레이어 | 기술 | 비고 |
|---|---|---|
| **앱 프레임워크** | Tauri v2 | Rust 백엔드 + 크로스 플랫폼 (Windows, macOS, Mobile) |
| **UI 프레임워크** | React + Tailwind CSS v4 | ⚠️ 현재 SvelteKit에서 마이그레이션 필요 |
| **추론 엔진** | llama.cpp (GGUF/EXL2) | 온디바이스 실행, 레이어 오프로딩 지원 |
| **오케스트레이션** | LangGraph (Rust 자체 구현) | 상태 유지형 순환 그래프 워크플로우 |
| **데이터베이스** | SurrealDB | 벡터 + 그래프 + 관계형 통합 |
| **임베딩** | nomic-embed-text / ko-sroberta | 다국어 지원 |

> ⚠️ **마이그레이션 결정 필요**: 기획서는 React 기반을 정의함. 현재 SvelteKit 코드베이스와의 전환 계획을 확정해야 한다.

### 1.2 아키텍처 조감도

```
┌────────────────────────────────────────────────────┐
│              Antigravity HUD (런처)                 │
│    [Project Switcher] [Command Input] [Resource]    │
└────────────────────┬───────────────────────────────┘
                     │ 확장
┌────────────────────▼───────────────────────────────┐
│           Main Mission Control (대시보드)            │
│  ┌──────────────┐ ┌──────────────┐ ┌─────────────┐ │
│  │Harness Studio│ │  Chat &      │ │  Artifacts  │ │
│  │(프로젝트/페르│ │  Citation    │ │  Ready-Zone │ │
│  │소나 관리)    │ │  (대화 영역) │ │  (결과물)   │ │
│  └──────────────┘ └──────────────┘ └─────────────┘ │
│  ┌──────────────┐                                   │
│  │Local         │                                   │
│  │NotebookLM    │                                   │
│  │(지식 소스)   │                                   │
│  └──────────────┘                                   │
└────────────────────────────────────────────────────┘
                     │
┌────────────────────▼───────────────────────────────┐
│                 Q-Agent Core (Rust)                 │
│  ┌──────────────────────────────────────────────┐   │
│  │       Multi-Agent Orchestrator (LangGraph)   │   │
│  │  Planner → Executor → Critic → Reflection   │   │
│  └──────────────────────────────────────────────┘   │
│  ┌───────────┐  ┌───────────┐  ┌───────────────┐   │
│  │ Model     │  │GraphRAG   │  │ Tool Registry │   │
│  │ Orch.     │  │ Engine    │  │ (MCP/Harness) │   │
│  │(llama.cpp)│  │(SurrealDB)│  │               │   │
│  └───────────┘  └───────────┘  └───────────────┘   │
└────────────────────────────────────────────────────┘
                     │
┌────────────────────▼───────────────────────────────┐
│           External Bridges (사용자 승인 필요)        │
│   [SSH Lab Server]    [Web Intelligence Engine]     │
│   [Ghost Prototyping] [MCP Tool Plugins]            │
└────────────────────────────────────────────────────┘
```

---

## 2. 지능형 지식 관리 (Hierarchical GraphRAG)

단순 벡터 검색을 넘어 데이터 간 '관계'를 이해하는 시스템.

### 2.1 3계층 그래프 구조

```
Global Commons (황금 노드)
  └── 사용자 선호도, 공통 규칙, 전역 페르소나

Shared Pool (초록 노드)
  └── 선택된 프로젝트 간 공유 맥락

Project Private (파란 노드)
  └── 특정 프로젝트 전용 (PDF, 코드, URL)
```

### 2.2 RAG 2.0 파이프라인

```
쿼리 입력
  → Query Expansion (NER + 동의어 → 3~5 서브쿼리)
  → Parallel Search:
      ├─ Vector Search (nomic-embed-text, SurrealDB)
      └─ BM25 Keyword Search (tantivy)
  → Merge & Deduplicate
  → Cross-Encoder Re-ranking (ms-marco-MiniLM)
  → Citation Assignment [1][2][3] + 신뢰도 점수
  → Context Budget (최대 8,000 토큰)
  → LLM 생성
  → Iterative Check (추가 검색 필요 시 재탐색)
  → Final Answer + Source Cards
```

### 2.3 외부 지식 신뢰도 시스템

- **Perplexity형 출처 표기**: 모든 답변에 `[N]` 인용 번호 + 원문 팝업
- **Reflection 패턴**: 정보의 최신성·도메인 권위도를 자기 성찰로 평가
- **신뢰 등급**: ★★★★★ ~ ★☆☆☆☆ 시각적 표시

### 2.4 신규 Rust 모듈

```
harness/
  query_expander.rs   — LLM 기반 서브쿼리 생성
  bm25_search.rs      — tantivy 키워드 검색
  reranker.rs         — Cross-Encoder 점수 계산
  citation.rs         — 인용 번호 + 신뢰도
  rag_pipeline.rs     — RAG 2.0 파이프라인 통합
  graph_rag.rs        — SurrealDB 그래프 관계 추론
```

---

## 3. 동적 LLM 오케스트레이션

### 3.1 Cascade Routing (단계별 라우팅)

```
질문 입력
  → Intent Classifier (복잡도 판단)
      ├─ Simple: 3B~8B 모델 즉각 처리
      └─ Complex: 10B+ 대형 모델로 전환
          └─ Coding/Planning: Coder/Planner 전문 모델
```

### 3.2 모델 분류 체계

| 역할 | 모델 | VRAM |
|---|---|---|
| **Router** | Qwen2.5 3B / Phi-3.5 Mini | Entry (8GB) |
| **Planner** | Mistral NeMo 12B | Mid (12~16GB) |
| **Coder** | Qwen2.5-Coder 14B | Mid (12~16GB) |
| **Heavy** | Llama 3.1 70B / Qwen 2.5 72B | High (20~24GB) |
| **Embedder** | nomic-embed-text | CPU |
| **Vision** | Qwen2-VL 7B | Phase 4 |

### 3.3 Resource Governance

- **fvcore 연산량 슬라이더**: 단일 태스크 최대 GPU 연산량 상한 설정
- **Iteration Cap**: Reflection 루프 최대 횟수 제한 (기본 5회)
- **Budget Guard**: 토큰 예산 초과 시 자동 중단 + 사용자 알림

### 3.4 ModelRunner Trait (Rust)

```rust
#[async_trait]
pub trait ModelRunner: Send + Sync {
    async fn generate_stream(&self, req: ModelRequest) -> Result<impl Stream<Item=String>>;
    async fn generate(&self, req: ModelRequest) -> Result<String>;
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    fn supports_vision(&self) -> bool { false }
    fn context_window(&self) -> usize;
    fn model_id(&self) -> &str;
}
```

---

## 4. Multi-Agent 워크플로우

### 4.1 에이전트 팀 구성

#### 🧭 Planner
- **역할**: 사용자 요청 분석 → TaskGraph 생성
- **페르소나**: 논리적 프로젝트 매니저. 최단 경로 탐색
- **출력**: `TaskPlan { steps, dependencies, estimated_tokens }`

#### 🔍 Critic
- **역할**: Planner 계획 및 Executor 결과 검토
- **페르소나**: 냉혹한 시니어 리뷰어
- **루프 종료**: `score >= 0.85` 또는 최대 3회

#### 💻 Coder / Designer
- **역할**: 코드 생성, 문서 작성, UI 설계
- **페르소나**: 실용주의 엔지니어

#### 🔬 MLOps
- **역할**: 딥러닝 실험 설계 + SSH 브릿지 실행 + 결과 분석
- **페르소나**: 재현성 중시 데이터 과학자

### 4.2 LangGraph 상태 머신

```
START (User Input)
  → Router (3B, 태스크 분류)
      ├─ Research Mode
      ├─ Code Mode
      └─ MLOps Mode
  → Planner (TaskGraph 생성)
  → Executor (Tool Call via Harness/MCP)
  → Critic
      ├─ Approved → Output
      └─ Rejected → Executor (max 3회)
```

### 4.3 AgentState (Rust)

```rust
pub struct AgentState {
    pub task_id:        Uuid,
    pub iteration:      u32,
    pub max_iterations: u32,
    pub token_budget:   usize,
    pub tokens_used:    usize,
    pub plan:           Option<TaskPlan>,
    pub artifacts:      Vec<Artifact>,
    pub citations:      Vec<Citation>,
    pub tool_log:       Vec<ToolCallRecord>,
    pub critic_score:   Option<f32>,
    pub status:         AgentStatus,
    pub checkpoint_at:  DateTime<Utc>,
}
// 모든 상태 전환 → SurrealDB 자동 스냅샷
```

---

## 5. Computer Mode: 자가 치유 코딩 루프

### 5.1 Self-healing Loop

```
코드 생성
  → Ghost Prototyping (WASM 샌드박스)
  → 오류 감지 시: 로그 캡처 → 원인 분석 → 재생성
  → 성공 시: "실제 로컬에 반영할까요?" HITL 승인 요청
  → 로컬 적용
```

### 5.2 MCP (Model Context Protocol)

| 도구 | 설명 |
|---|---|
| `fs_tool` | 로컬 파일 시스템 읽기/쓰기 |
| `terminal_tool` | 터미널 명령 실행 (승인 필요) |
| `browser_tool` | 브라우저 제어 + Vision 인식 |
| `ssh_tool` | 원격 GPU 서버 연결 |

### 5.3 Ghost Prototyping (이중 검증)

1. **WASM 샌드박스** (기본): 브라우저 내 가상 환경 — 네트워크/파일 접근 차단 상태에서 UI 렌더링
2. **Remote Server** (선택): SSH/API로 Linux 실제 환경 배포 → 결과 스트리밍 수신

---

## 6. HITL 보안 관문 (Human-in-the-Loop)

고위험 작업 전 반드시 사용자 승인:

| 요소 | 상세 |
|---|---|
| **작업 요약 뱃지** | 파일 생성·터미널·브라우저 제어 아이콘 분류 |
| **Permission Scope** | "작업 범위: ~/projects/ 내부로 한정됨" 명시 |
| **Command Preview** | 실행 예정 명령어 모노스페이스 폰트로 노출 |
| **실행 환경 선택** | [WASM 실행] / [Remote 서버] / [로컬 즉시 적용] |

**보안 프로필 등급:**
- `Read-only`: 읽기만 허용
- `Standard`: 프로젝트 폴더 내부만
- `Advanced`: 원격 서버 접속 허용

---

## 7. 자율성 극대화 기능

| 기능 | 상세 | 상태 |
|---|---|---|
| **Personal Memory (Mem0)** | 사용자 교정·코딩 스타일 장기 기억 저장 | Phase 2 |
| **Vision-Aided Computer Mode** | 화면 UI 인식 → 직접 조작 (레거시 앱 자동화) | Phase 3 |
| **@ 컨텍스트 참조** | `@파일`, `@폴더`, `@artifact` 직접 참조 | Phase 1 |
| **Closed RAG 모드** | Project KB만 근거로 답변 (오프라인 리서치) | Phase 2 |
| **Plugin Marketplace** | MCP 기반 서드파티 도구 확장 | Phase 4 |
| **Finance Agent** | 완전 로컬 재무 관리 (거래 분류, ISA/CMA 추적) | Phase 5 |
| **음성 I/O (TTS/STT)** | Whisper 입력 + Kokoro TTS 출력 | 후순위 |

---

## 7-1. 페르소나 & 하네스 템플릿 시스템

에이전트의 페르소나, 오케스트레이션 설정, 하네스 규칙을 사용자가 자유롭게 편집하고 템플릿으로 공유할 수 있는 시스템.

### 편집 가능 항목

| 설정 | 범위 | 설명 |
|---|---|---|
| **시스템 프롬프트 (페르소나)** | 프로젝트별 / 공통 | 에이전트 전문 분야·말투·응답 스타일 정의 |
| **오케스트레이션 규칙** | 프로젝트별 / 공통 | Planner·Critic·Coder 루프 횟수, 라우팅 임계값 |
| **하네스 규칙 (code_conduct)** | 프로젝트별 / 공통 | 허용 도구, 보안 등급, 접근 경로 제한 |
| **모델 매핑** | 프로젝트별 | Small/Heavy 모델 직접 지정 |
| **fvcore 상한 / Iteration Cap** | 프로젝트별 | 자원 사용 한도 |

### 템플릿 구조

```
templates/
  personas/
    researcher.yaml        — 논문 분석·요약 특화 연구자
    coder.yaml             — 코드 생성·리뷰 특화 엔지니어
    analyst.yaml           — 데이터 분석·시각화 특화 분석가
    default.yaml           — 범용 기본 페르소나
  harness/
    strict.yaml            — Read-only, 최소 권한
    standard.yaml          — 프로젝트 폴더 내 표준 권한
    advanced.yaml          — 원격 서버 접속 허용
  orchestration/
    fast.yaml              — Router→Executor 단순 루프 (빠름)
    balanced.yaml          — Planner→Executor→Critic 기본 루프
    deep.yaml              — 다중 Reflection + 병렬 실행 (정확)
```

### 적용 우선순위 (상속 체계)

```
Global Default  (최하위 — 기본값)
  ↑ 재정의
Shared Template (공통 템플릿 — 여러 프로젝트 공유)
  ↑ 재정의
Project Override (프로젝트별 최종 설정 — 최우선)
```

**UI**: Harness Studio 패널에서 드롭다운으로 템플릿 선택 → YAML 에디터로 직접 수정 → 저장 후 즉시 적용.

---

## 8. UI / 디자인 시스템

### 8.1 Antigravity HUD (초기 런처)

- `Alt + Space` 호출
- 글래스모피즘 스타일 투명 바
- **Project Switcher**: 활성 프로젝트 페르소나 빠른 전환
- **Command Input**: Perplexity 스타일 통합 명령창
- **Resource Monitor**: VRAM 점유율 + fvcore 잔여 실시간 표시

### 8.2 Main Mission Control (확장 대시보드)

| 영역 | 기능 |
|---|---|
| **좌측 상단** Harness Studio | 프로젝트 카드 + 페르소나 설정 + 연산량 상한 |
| **좌측 하단** Local NotebookLM | Drag-and-Drop 소스 등록 + Knowledge Graph 시각화 |
| **중앙** Chat & Citation | Perplexity형 인용 + Thought Trace 실시간 노출 |
| **우측** Artifacts Ready-Zone | 생성 중 파일 리스트 + Computer Mode Indicator |

### 8.3 GraphRAG 시각화

- **노드 색상**: 파란(Project Private) / 초록(Shared Pool) / 황금(Global Commons)
- **Edge 가중치**: 관련도 높을수록 굵은 선
- **Dark 테마**: 네온 노드 + 광섬유 애니메이션
- **Light 테마**: 파스텔 노드 + 부드러운 하이라이트

### 8.4 테마 시스템

| 구분 | Minimalist White | Dark Modern |
|---|---|---|
| **배경** | #F9FAFB (Glassmorphism) | #0D0D0D (Jet Black) |
| **포인트** | Indigo Blue (#4F46E5) | Electric Cyan (#06B6D4) |
| **텍스트** | Slate Gray 900 | Gray 100 |
| **그림자** | Soft & Large | Outer Glow |

### 8.5 인터랙션 흐름

1. `Alt+Space` → HUD 호출
2. 프로젝트 선택 → Mission Control 확장
3. 질문 입력 → Thought Trace 실시간 표시
4. 코딩 시작 → Split View 자동 전환 (채팅 | Ghost Prototyping)
5. 완료 → HITL 승인 → 로컬 반영

---

## 9. API-First (AaaS) 아키텍처

```
Base URL: http://localhost:8765/api/v1

[Agent]  POST /agent/chat        — 스트리밍 대화
         POST /agent/task        — 배치 태스크
         GET  /agent/status/{id} — 진행 상태

[Project] GET/POST/PUT/DELETE /projects/{id}

[Knowledge] POST /knowledge/ingest
            GET  /knowledge/search

[Artifact] GET /artifacts, GET /artifacts/{id}

[System] GET /health, GET /models
```

**WebSocket 이벤트:**
```jsonc
{ "event": "agent_thinking",   "data": { "agent": "Planner", "step": "계획 수립" } }
{ "event": "agent_tool_call",  "data": { "tool": "web_search", "query": "..." } }
{ "event": "artifact_created", "data": { "id": "uuid", "type": "code" } }
{ "event": "stream_token",     "data": { "token": "..." } }
{ "event": "stream_done",      "data": { "usage": { "prompt": 1200, "completion": 450 } } }
```

---

## 10. 개발 로드맵

> 완전 새 시작 기준. 기존 구현 코드는 참조하되, 아키텍처는 V4.1 기준으로 재설계.

### 🔴 Phase 0: 코어 기반 구축
- [x] UI 프레임워크 확정 및 프로젝트 초기화 (React + Tauri v2)
- [/] SurrealDB 스키마 설계 (projects, conversations, knowledge, artifacts)
- [x] ModelRunner Trait 추상화 (llama.cpp 백엔드)
- [x] 기본 Chat UI + Streaming 응답
- [/] 프로젝트 생성·전환 기능
- [x] Antigravity HUD 기본 구현 (`Alt+Space` 런처)

### 🟠 Phase 1: 에이전트 팀 + 템플릿 시스템
- [x] LangGraph 상태 머신 (Rust 자체 구현)
- [x] Planner / Critic / Coder / MLOps 에이전트
- [x] AgentState Checkpointing (SurrealDB 자동 스냅샷)
- [x] Budget Guard (토큰 예산 + Iteration Cap)
- [x] **페르소나·하네스·오케스트레이션 템플릿 시스템**
  - [x] YAML 기반 템플릿 정의 (personas/, harness/, orchestration/)
  - [x] 기본 템플릿 4종 제공 (researcher, coder, analyst, default)
  - [/] 하네스 보안 등급 3종 (strict, standard 구현 완료)
  - [x] Global / Shared / Project 상속 체계
  - [x] Harness Studio UI 에디터
- [x] HITL 승인 관문
- [x] Artifact Panel (생성 + 독립 뷰)
- [ ] @ 컨텍스트 참조 UI

### 🟡 Phase 2: 지식 베이스 (GraphRAG 2.0)
- [ ] RAG 2.0 파이프라인
  - [ ] BM25 키워드 검색 (tantivy)
  - [ ] Vector 검색 (nomic-embed-text)
  - [ ] **한국어 임베딩 (ko-sroberta-multitask)**
  - [ ] Cross-Encoder Re-ranking
  - [ ] Query Expansion (서브쿼리 생성)
- [ ] Citation Engine (`[N]` + 신뢰도 점수)
- [ ] GraphRAG 3계층 구조 (Project / Shared / Global)
- [ ] GraphRAG 시각화 UI (인터랙티브 노드 맵)
- [ ] Folder Watcher (실시간 파일 인덱싱)
- [ ] Closed RAG 모드 (오프라인 리서치)
- [ ] Personal Memory (Mem0) — 교정·스타일 장기 기억
- [ ] Prompt Inheritance 우선순위 엔진
- [ ] Studio 아티팩트 (Mermaid 마인드맵, SVG 인포그래픽)

### 🟢 Phase 3: Computer Mode + 브릿지
- [ ] Ghost Prototyping WASM 샌드박스
- [ ] Ghost Prototyping Remote Server 연동
- [ ] Self-healing Loop (코드 → 실행 → 오류 → 재생성)
- [ ] MCP 도구 레지스트리 (fs, terminal, browser, ssh)
- [ ] SSH Lab Bridge (연결 관리 + 로그 스트리밍)
- [ ] Vision-Aided Computer Mode (화면 인식 + 조작)
- [ ] AaaS REST API 서버 (axum 기반)
- [ ] **백업 & 동기화**
  - [ ] AES-256-GCM 암호화 내보내기/가져오기
  - [ ] 로컬 백업 스케줄러
  - [ ] 선택적 클라우드 동기화 (S3 호환, WebDAV)

### 🔵 Phase 4: 확장 (플러그인 + 모델)
- [ ] llama.cpp Native 실행 고도화 (레이어 오프로딩, EXL2)
- [ ] Vision Agent (Qwen2-VL, PDF 도표 분석)
- [ ] **Plugin / Extension Marketplace**
  - [ ] MCP 기반 플러그인 규격 정의
  - [ ] 플러그인 샌드박스 실행 환경
  - [ ] 로컬 플러그인 마켓 UI
- [ ] Finance Agent
  - [ ] 거래 내역 CSV 임포트 + LLM 자동 분류
  - [ ] ISA / CMA 납입 한도 추적
  - [ ] 월별 소비 대시보드
  - [ ] ETF 보유 현황 + 리밸런싱 알림

### ⚪ Phase 5: 후순위 (미래 확장)
- [ ] Mobile 지원 (Tauri Mobile — iOS/Android)
- [ ] Audio Overview (Whisper STT + Kokoro TTS)
- [ ] 팀 워크스페이스 / 멀티 사용자
- [ ] ntransformer 커스텀 커널 (DeepSeek, Qwen 독자 가속)

---

## 11. 하드웨어별 추천 구성

| 등급 | VRAM | 추천 모델 | 주요 용도 |
|---|---|---|---|
| **Entry** | 8GB | Llama 3.2 3B | 요약, 간단한 Q&A, 모바일 |
| **Mid** | 12~16GB | Mistral NeMo 12B | Computer Mode, RAG 문서 분석 |
| **High** | 20~24GB | Llama 3.1 70B / Qwen 2.5 72B | 대규모 프로젝트, Self-healing |

---

## 12. 확정된 방향 (v4.1 기준)

| 항목 | 결정 | 비고 |
|---|---|---|
| **UI 프레임워크** | ✅ 확정 필요 | React + Tailwind CSS v4 (기획서 V4 기준) |
| **데이터베이스** | ✅ SurrealDB 공식 스택 | 벡터 + 그래프 + 관계형 통합 |
| **한국어 임베딩** | ✅ 도입 확정 | ko-sroberta-multitask (Phase 2) |
| **플러그인 생태계** | ✅ 도입 확정 | MCP 기반 Extension Marketplace (Phase 4) |
| **템플릿 시스템** | ✅ 도입 확정 | Persona/Harness/Orchestration YAML 템플릿 (Phase 1) |
| **백업 & 동기화** | ✅ 도입 확정 | AES-256 암호화 + 선택적 클라우드 (Phase 3) |
| **Finance Agent** | 🔶 후순위 확정 | 코어 완성 후 Phase 4 |
| **음성 I/O (TTS/STT)** | ⏸️ 최후순위 | Phase 5 (후순위) |
| **Mobile** | ⏸️ 최후순위 | Phase 5 — 코어/웹/데스크탑 우선 |

---

*마지막 업데이트: 2026-05-13 · Master Plan v4.1 (방향 확정 및 로드맵 정제)*
*이 문서는 `docs/` 내에서만 관리되며 외부에 공개하지 않습니다.*
