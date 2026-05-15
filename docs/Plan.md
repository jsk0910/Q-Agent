# Q-Agent Master Plan v4.4

> **Version**: 4.4 · **Authored**: 2026-05-15 · **Updated**: 2026-05-15  
> **Status**: Active Blueprint — 서버-클라이언트 확장성, 보안 재정비, 디자인 시스템 고도화 반영  
> **Replaces**: Plan.md v4.3 (2026-05-15)

***

## 0. 비전 & 핵심 철학

**Q-Agent**는 개인 PC에서 완전히 동작하면서도, 필요 시 다른 PC·웹 클라이언트와 안전하게 연결될 수 있는 자율형 AI 에이전트 플랫폼이다.

> *"클라우드 없이, 데이터 유출 없이 — 내 PC가 곧 나만의 AI 연구실."*

### 핵심 원칙

| 원칙 | 설명 |
|---|---|
| **Privacy-First** | 모든 추론·지식·재무 데이터는 기본적으로 로컬에서만 처리 |
| **On-device Intelligence** | llama.cpp 기반 온디바이스 추론으로 데이터 주권 확보 |
| **Transparent Agency** | LangGraph 사고 과정과 도구 호출을 사용자에게 투명하게 공개 |
| **Human in the Loop** | 고위험 작업은 반드시 사용자 승인 후 실행 |
| **Resource Governance** | fvcore 연산량 상한 + Iteration Cap으로 자원 점유 방지 |
| **Secure-by-Default** | 외부 접근은 기본 비활성화, 명시적 허용 시에만 네트워크/API 개방 |
| **Client-Ready Core** | 로컬 서버 코어와 다양한 클라이언트(Web/Desktop/Mobile)를 분리 가능한 구조 유지 |

***

## 1. 기술 스택 (Foundation)

### 1.1 코어 스택

| 레이어 | 기술 | 비고 |
|---|---|---|
| **앱 프레임워크** | Tauri v2 | Rust 백엔드 + 크로스 플랫폼 (Windows, macOS, 추후 Mobile) |
| **UI 프레임워크** | React + Tailwind CSS v4 | 단일 디자인 시스템 기반으로 통합 |
| **추론 엔진** | llama.cpp (GGUF/EXL2) | 온디바이스 실행, 레이어 오프로딩 지원 |
| **오케스트레이션** | LangGraph (Rust 자체 구현) | 상태 유지형 순환 그래프 워크플로우 |
| **데이터베이스** | SurrealDB | 벡터 + 그래프 + 관계형 통합 |
| **임베딩** | nomic-embed-text / ko-sroberta | 다국어 및 한국어 특화 지원 |
| **검색 계층** | tantivy + Searxng + 다중 Provider 폴백 | 로컬/외부 검색 이중화 |
| **API 계층** | axum + WebSocket | AaaS 및 멀티 클라이언트 대응 |

### 1.2 아키텍처 조감도

```text
┌───────────────────────────────────────────────────────┐
│                Client Surfaces                        │
│  Tauri Desktop  |  Browser/PWA  |  Future Mobile     │
└──────────────────────┬────────────────────────────────┘
                       │
┌──────────────────────▼────────────────────────────────┐
│             Q-Agent Local Server Core                 │
│   Auth / Session / Policy / Streaming / API Gateway   │
└───────────────┬──────────────────────┬────────────────┘
                │                      │
┌───────────────▼──────────────┐ ┌─────▼────────────────┐
│ Multi-Agent Orchestrator     │ │ Knowledge/RAG Core   │
│ Planner/Executor/Critic/...  │ │ GraphRAG + Search    │
└───────────────┬──────────────┘ └─────┬────────────────┘
                │                      │
┌───────────────▼───────────────────────────────────────┐
│ Model Orchestrator / Tool Registry / Permission Guard │
└───────────────┬───────────────────────────────────────┘
                │
┌───────────────▼───────────────────────────────────────┐
│ Local FS / Terminal / Browser / SSH / OS Native Hooks │
└───────────────────────────────────────────────────────┘
```

### 1.3 아키텍처 방향성

- **로컬 서버 + 다중 클라이언트 구조**를 정식 지원 대상으로 정의한다.
- 기본 사용 시에는 Tauri 단일 앱처럼 동작하지만, 설정에서 원격 접속을 켜면 다른 PC/브라우저가 API 및 WebSocket으로 접속할 수 있다.
- 모바일은 후순위지만, 코어는 처음부터 **PWA/모바일 친화적 API**를 전제로 설계한다.

***

## 2. 지능형 지식 관리 (Hierarchical GraphRAG)

단순 벡터 검색을 넘어 데이터 간 '관계'를 이해하는 시스템.

### 2.1 3계층 그래프 구조

```text
Global Commons (황금 노드)
  └── 사용자 선호도, 공통 규칙, 전역 페르소나

Shared Pool (초록 노드)
  └── 선택된 프로젝트 간 공유 맥락

Project Private (파란 노드)
  └── 특정 프로젝트 전용 (PDF, 코드, URL)
```

### 2.2 RAG 2.0 파이프라인

```text
쿼리 입력
  → Query Expansion (NER + 동의어 → 3~5 서브쿼리)
  → Parallel Search:
      ├─ Vector Search (nomic-embed-text, SurrealDB)
      ├─ BM25 Keyword Search (tantivy)
      └─ External Search Provider Chain (Searxng → Provider Fallback)
  → Merge & Deduplicate
  → Cross-Encoder Re-ranking (ms-marco-MiniLM)
  → Citation Assignment [1][2][3] + 신뢰도 점수
  → Context Budget (최대 8,000 토큰)
  → LLM 생성
  → Iterative Check (추가 검색 필요 시 재탐색)
  → Final Answer + Source Cards
```

### 2.3 오프라인 검색 폴백 (Internal-only Mode)

네트워크 단절 또는 외부 검색 장애 시 자동으로 로컬 지식만을 사용하는 모드로 전환.
- **동작 규칙**: 외부 웹 검색을 생략하고 `Project Private` 및 `Shared Pool` 지식 그래프만 탐색.
- **답변 재구성**: "현재 오프라인 상태입니다. 로컬 지식 기반으로 답변을 생성합니다" 안내 및 신뢰도 점수 조정.

### 2.4 로컬-클라우드 하이브리드 싱크 (E2EE)

여러 기기 간 Mem0(장기 기억) 및 GraphRAG 인덱스 공유를 위한 동기화 메커니즘.
- **E2EE (End-to-End Encryption)**: 모든 데이터는 로컬에서 AES-256-GCM으로 암호화된 후 클라우드(S3/WebDAV)로 전송.
- **Sync Logic**: 기기별 타임스탬프 기반 충돌 해결 및 증분 업데이트 지원.

### 2.5 외부 지식 신뢰도 시스템

- **Perplexity형 출처 표기**: 모든 답변에 `[N]` 인용 번호 + 원문 팝업
- **Reflection 패턴**: 정보의 최신성·도메인 권위도를 자기 성찰로 평가
- **신뢰 등급**: ★★★★★ ~ ★☆☆☆☆ 시각적 표시
- **스레드형 탐색 누적**: 후속 질문이 이전 검색/인용 문맥과 연결되도록 컨텍스트 체인 유지

### 2.6 신규 Rust 모듈

```text
harness/
  query_expander.rs   — LLM 기반 서브쿼리 생성
  bm25_search.rs      — tantivy 키워드 검색
  reranker.rs         — Cross-Encoder 점수 계산
  citation.rs         — 인용 번호 + 신뢰도
  rag_pipeline.rs     — RAG 2.0 파이프라인 통합
  graph_rag.rs        — SurrealDB 그래프 관계 추론
  provider_chain.rs   — 외부 검색 공급자 폴백 체인
```

***

## 3. 동적 LLM 오케스트레이션

### 3.1 Cascade Routing (단계별 라우팅)

```text
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

### 3.3 Resource Governance & Auto-Mapping

- **하드웨어 티어별 자동 매핑**: VRAM(8GB, 12-16GB, 24GB) 감지 후 최적 모델 및 양자화(Q4/Q5/Q6) 자동 추천.
- **자원 동적 스로틀링 (Power-Save Mode)**:
  - 하드웨어 온도(Thermal) 및 배터리 상태 실시간 감지.
  - 임계치 초과 시 모델 양자화 수준 하향 또는 fvcore 연산 속도 제한.
- **fvcore 연산량 슬라이더**: 단일 태스크 최대 GPU 연산량 상한 설정.
- **Iteration Cap**: Reflection 루프 최대 횟수 제한 (기본 5회).
- **Budget Guard**: 토큰 예산 초과 시 자동 중단 + 사용자 알림.
- **모델 허브 UI**: 하드웨어별 추천 모델, 예상 속도, 메모리 점유율을 카드 형태로 시각화.

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

***

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

```text
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
    pub eval_report:    Option<EvalReport>,
    pub status:         AgentStatus,
    pub checkpoint_at:  DateTime<Utc>,
}
```

### 4.4 에이전트 자가 평가 (Self-Eval)

RAGAS 스타일의 평가 프레임워크를 내재화하여 에이전트 성능을 수치화.
- **평가 지표**: 답변 신뢰도(Faithfulness), 답변 관련성(Answer Relevance), 계획 완수율(Task Completion).
- **리포트**: 태스크 종료 시 사용자에게 성능 대시보드 및 개선 제안 노출.
- **Trajectory 타임라인**: 파일 읽기 → 검색 → 생성 → 테스트 → 수정 흐름을 시간축으로 시각화.

***

## 5. Computer Mode: OS-Native 지능형 제어

### 5.1 Native Context Retrieval (Hybrid Logic)

단순 스크린샷 기반의 Vision 분석을 넘어, OS 고유의 API를 통해 구조적 데이터를 직접 추출.
- **UI Tree Extraction**: Tauri Native Rust Hooks를 통해 활성 창의 텍스트와 UI 구성 요소(Tree)를 직접 읽어옴.
  - Windows: UI Automation (UIA)
  - macOS: Accessibility API (AXUI)
- **장점**: 시각적 지연 시간 제거, 텍스트의 정확한 추출, 백그라운드 탭 데이터 접근 가능.

### 5.2 Self-healing Loop

```text
코드 생성
  → Ghost Prototyping (WASM 샌드박스)
  → 오류 감지 시: 로그 캡처 → 원인 분석 → 재생성
  → 성공 시: "실제 로컬에 반영할까요?" HITL 승인 요청
  → 로컬 적용 (Live Artifacts와 연동)
```

### 5.3 MCP (Model Context Protocol)

| 도구 | 설명 |
|---|---|
| `fs_tool` | 로컬 파일 시스템 읽기/쓰기 |
| `terminal_tool` | 터미널 명령 실행 (승인 필요) |
| `browser_tool` | 브라우저 제어 + Vision 인식 |
| `ssh_tool` | 원격 GPU 서버 연결 |

### 5.4 Ghost Prototyping (이중 검증)

1. **WASM 샌드박스** (기본): 브라우저 내 가상 환경 — 네트워크/파일 접근 차단 상태에서 UI 렌더링
2. **Remote Server** (선택): SSH/API로 Linux 실제 환경 배포 → 결과 스트리밍 수신

***

## 6. 선택형 권한 오케스트레이터 (Permission Orchestrator)

모든 MCP 도구 호출은 Rust 기반의 **Backend Guardrail Interceptor**를 통해 필터링됨. 사용자는 UI 상단에서 실행 권한 강도를 실시간 조절 가능.

### 6.1 실행 모드 정의

| 모드 | 동작 방식 | 대상 명령 |
|---|---|---|
| **Strict** | 모든 도구 호출 시 반드시 사용자 승인 필요 | 전종류 |
| **Balanced** | 읽기 전용 작업은 자동 실행, 수정/삭제 작업은 승인 필요 | [자동] ls, cat, grep / [승인] rm, write, push |
| **Agentic** | 위험도 분류기 기반 자동화. 이상 패턴 감지 시에만 승인 요청 | 시스템 파일 접근, 대량 삭제 등 비정상 패턴 |

### 6.2 Agentic 위험도 분류기 (Risk Classifier)

- **Engine**: Qwen 2.5 1.5B (소형 온디바이스 모델)
- **역할**: 에이전트의 다음 행동(Tool Call)을 가로채어 의도의 위험성을 0~1 사이 점수로 평가.
- **임계치**: `score > 0.7` 일 경우 강제 HITL 팝업 발생.

### 6.3 Sandboxing & Isolation

- **작업 격리**: 모든 작업 범위를 특정 프로젝트 폴더(`~/projects/q-agent/...`) 내부로 강제 격리.
- **Command Filter**: 화이트리스트 기반의 명령어 필터링 및 위험 인자 사전 검증.
- **원격 세션 격리**: 네트워크 경유 접속 클라이언트는 세션별 권한 범위와 승인 기록을 별도 보관.

***

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
| **PWA Client** | 브라우저/태블릿용 경량 클라이언트 | Phase 3 |
| **App Connectors** | VS Code, Obsidian 등 외부 앱 연동 | Phase 4 |

***

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

```text
templates/
  personas/
    researcher.yaml
    coder.yaml
    analyst.yaml
    default.yaml
  harness/
    strict.yaml
    standard.yaml
    advanced.yaml
  orchestration/
    fast.yaml
    balanced.yaml
    deep.yaml
```

### 적용 우선순위 (상속 체계)

```text
Global Default
  ↑ 재정의
Shared Template
  ↑ 재정의
Project Override
```

### UI 방향

- Harness Studio에서 **드롭다운 + YAML 에디터 + 시각적 빌더**를 함께 제공한다.
- 고급 사용자는 YAML을 직접 수정하고, 일반 사용자는 카드 기반 템플릿 조합 방식으로 설정한다.
- 자주 쓰는 워크플로우는 **Action Cards**로 HUD에 고정 배치한다.

***

## 8. UI / 디자인 시스템 (Perplexity + Claude + Open WebUI 참고)

### 8.1 디자인 원칙

- **기본 방향**: "Research-grade productivity" — 지나치게 장식적이지 않고, 정보 구조와 작업 흐름을 우선한다.
- **서버-클라이언트 확장성**을 고려해 데스크톱과 브라우저에서 동일한 정보 구조를 유지한다.
- **Claude식 Artifacts 작업감 + Perplexity식 검색/인용 가독성 + Open WebUI식 실용적인 멀티 패널 탐색성**을 조합한다.
- 초기 다크 네온 중심 그래픽은 축소하고, 장시간 사용에 적합한 **차분한 중성 계열 + 제한된 포인트 컬러** 중심으로 재설계한다.

### 8.2 Antigravity HUD (초기 런처)

- `Alt + Space` 호출
- **Command Input**: Perplexity 스타일 통합 명령창
- **Focus Mode**: '디스커버, 재무, 코딩, 학술' 등 프로젝트 성격에 따른 검색 범위 및 모델 프리셋 전환
- **Action Cards**: '보고서 만들기', '코드 디버깅', '프로젝트 인덱싱', '보안 점검' 등 자주 쓰는 워크플로우 템플릿
- **Resource Monitor**: VRAM 점유율 + fvcore 잔여 + Power-Save 상태 표시
- **Quick Connect**: 로컬 전용 / LAN 공유 / 원격 비활성 상태를 즉시 확인하는 네트워크 배지

### 8.3 Main Mission Control (확장 대시보드)

| 영역 | 기능 |
|---|---|
| **Chat & Citation** | Perplexity형 인용, 스레드형 후속 질문, Source Cards |
| **Artifacts Studio** | Claude식 Preview/Code 탭, 전체화면, Diff 보기, Hot-Reload |
| **Computer Tab** | OS 조작 및 브라우징 과정을 실시간 미러링하는 독립 뷰 |
| **Harness Studio** | 프로젝트 카드, 페르소나 설정, 권한 모드, 연산량 상한 |
| **Knowledge Map** | GraphRAG 노드 맵 + 출처/메모리 연결 시각화 |
| **Sessions & Approvals** | 원격 접속 세션, 승인 기록, 권한 로그 확인 |

### 8.4 디자인 시스템 재정비

| 항목 | 변경 방향 |
|---|---|
| **컬러** | 기본은 Neutral Gray / Warm White / Slate, 포인트는 Indigo 또는 Teal 중 1개만 선택 |
| **다크 모드** | 순수 네온/글로우 대신 저채도 다크 톤 + 최소한의 강조색 사용 |
| **라이트 모드** | 문서형 가독성과 카드 구분이 좋은 밝은 톤 우선 |
| **타이포그래피** | 채팅·문서·코드·설정 화면에 맞춘 역할별 스케일 정리 |
| **레이아웃** | 좌측 내비 + 중앙 작업영역 + 우측 컨텍스트 패널의 3영역 구조를 기본값으로 설계 |
| **패널 UX** | 패널 접기/고정, 드래그 리사이즈, 멀티 클라이언트에서도 동일한 구조 유지 |
| **모바일/PWA 대응** | 카드 스택 구조와 하단 탭 기반의 축약 레이아웃 사전 고려 |

### 8.5 참고 UX 요소 반영

- **PWA 스타일 브라우저 접근 UI**를 사전 고려한다.
- **플러그인/스킬 카드형 마켓 UI**를 Phase 4 설계에 반영한다.
- **에이전트 Trajectory 타임라인**과 **Self-Eval 대시보드**를 시각적으로 연결한다.
- **모델 허브 카드**, **원클릭 공급자 전환**, **검색 공급자 상태 표시**를 Settings/Model 탭에 반영한다.

***

## 9. API-First (AaaS) 아키텍처

```text
Base URL: http://127.0.0.1:8765/api/v1
Optional LAN Mode: http://0.0.0.0:8765/api/v1
```

```text
[Agent]  POST /agent/chat         — 스트리밍 대화
         POST /agent/task         — 배치 태스크
         GET  /agent/status/{id}  — 진행 상태

[Project] GET/POST/PUT/DELETE /projects/{id}

[Knowledge] POST /knowledge/ingest
            GET  /knowledge/search

[Artifact] GET /artifacts
           GET /artifacts/{id}
           GET /artifacts/{id}/diff

[Session] POST /auth/login-local
          POST /auth/token
          GET  /sessions
          POST /sessions/{id}/approve

[System] GET /health
         GET /models
         GET /providers
         GET /network
```

**WebSocket 이벤트:**
```jsonc
{ "event": "agent_thinking",   "data": { "agent": "Planner", "step": "계획 수립" } }
{ "event": "agent_tool_call",  "data": { "tool": "web_search", "query": "..." } }
{ "event": "artifact_created", "data": { "id": "uuid", "type": "code" } }
{ "event": "approval_needed",  "data": { "session_id": "uuid", "risk": 0.82 } }
{ "event": "stream_token",     "data": { "token": "..." } }
{ "event": "stream_done",      "data": { "usage": { "prompt": 1200, "completion": 450 } } }
```

### 9.1 네트워크 모드

| 모드 | 설명 |
|---|---|
| **Local-only (기본)** | `127.0.0.1` 바인딩, 외부 접근 차단 |
| **LAN Share** | 동일 네트워크 대역에서만 접속 허용 |
| **Remote Disabled by Default** | 외부 인터넷 노출은 기본 미지원 또는 고급 설정에서만 허용 |

***

## 10. 보안 / 네트워크 설계 원칙

### 10.1 기본 원칙

- 외부 연결은 **기본 비활성화**한다.
- 원격 접속을 허용하더라도 **명시적 사용자 설정 + 세션 인증 + 권한 제한**이 선행되어야 한다.
- Tauri 로컬 UI와 웹/PWA 클라이언트는 동일 권한을 가지지 않으며, **원격 클라이언트는 더 제한적인 정책**을 기본 적용한다.

### 10.2 필수 보안 항목

| 항목 | 계획 |
|---|---|
| **인증** | 로컬 세션 + API Token/JWT 기반 인증 도입 |
| **인가** | 세션별 권한 범위, 도구 사용 범위, 승인 정책 분리 |
| **CORS/Origin 정책** | 허용 Origin 화이트리스트 적용 |
| **CSRF/XSS 대응** | Web 클라이언트 대응 보안 헤더 및 입력 정제 |
| **TLS** | LAN 모드 이상에서 선택적 TLS 또는 Reverse Proxy 권장 |
| **감사 로그** | 승인, 실패, 도구 실행, 원격 접속 이력을 이벤트 로그로 저장 |
| **포트/서비스 노출 점검** | 8765 포트 외 불필요 포트 비활성화 |
| **비밀정보 저장** | 토큰/키는 OS Keychain 또는 암호화 저장소 활용 |

### 10.3 네트워크 점검 체크리스트

- 바인딩 주소(`127.0.0.1` / `0.0.0.0`) 설정 검증
- 로컬 방화벽 예외 처리 여부 확인
- 미사용 API 엔드포인트 비활성화
- 원격 세션 타임아웃 및 강제 종료 기능
- WebSocket 인증 누락 여부 테스트
- 승인 요청이 원격 세션 간 교차 노출되지 않는지 검증
- 프로젝트 폴더 격리 우회 가능성 점검

***

## 11. 개발 로드맵

> 완전 새 시작 기준. 기존 구현 코드는 참조하되, 아키텍처는 V4.4 기준으로 재설계.

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
  - [x] YAML 기반 템플릿 정의
  - [x] 기본 템플릿 4종 제공
  - [/] 하네스 보안 등급 3종 (strict, standard 구현 완료)
  - [x] Global / Shared / Project 상속 체계
  - [x] Harness Studio UI 에디터
- [x] **선택형 권한 오케스트레이터 (Strict / Balanced / Agentic)**
- [x] HITL 승인 관문
- [x] Artifact Panel (생성 + 독립 뷰)
- [ ] @ 컨텍스트 참조 UI

### 🟠 Phase 1.5: 프로젝트 재정비
- [ ] **구현 현황 감사(Audit)**: 현재 코드베이스와 Plan v4.4 간 차이 분석
- [ ] **구조 재정비**: UI/상태관리/API 경계를 정리하고 기술 부채 목록화
- [ ] **테스트 계획 수립**
  - [ ] 단위 테스트 범위 정의 (Rust core, agent state, policy)
  - [ ] 통합 테스트 시나리오 정의 (chat, artifact, approval, search)
  - [ ] E2E 테스트 시나리오 정의 (desktop, browser client, permission flow)
- [ ] **디자인 계획 재수립**
  - [ ] 디자인 토큰 정의 (color, spacing, typography, panel layout)
  - [ ] 핵심 화면 와이어프레임/HUD/대시보드/아티팩트 스튜디오 정리
  - [ ] 다크/라이트 모드 기준안 확정
  - [ ] PWA/브라우저 축약 레이아웃 가이드 수립
- [ ] **네트워크 및 보안 점검/계획**
  - [ ] AaaS API 인증/인가 구조 설계
  - [ ] CORS, Origin, WebSocket 인증 전략 설계
  - [ ] Local-only / LAN Share 모드 정책 문서화
  - [ ] 감사 로그 및 승인 이력 보존 정책 수립
  - [ ] 프로젝트 폴더 격리 및 명령 필터 우회 가능성 검토
- [ ] **성능 및 안정화 점검**
  - [ ] 모델 체크포인트 다운로드/연동 검증
  - [ ] 하드웨어별 기본 설정 검증
  - [ ] 발견된 버그 픽스 및 프로파일링 우선순위 선정

### 🟡 Phase 2: 지식 베이스 (GraphRAG 2.0)
- [ ] RAG 2.0 파이프라인
  - [ ] BM25 키워드 검색 (tantivy)
  - [ ] Vector 검색 (nomic-embed-text)
  - [ ] **한국어 임베딩 (ko-sroberta-multitask)**
  - [ ] Cross-Encoder Re-ranking
  - [ ] Query Expansion (서브쿼리 생성)
  - [ ] **검색 Provider Chain / 장애 폴백**
- [ ] Citation Engine (`[N]` + 신뢰도 점수)
- [ ] GraphRAG 3계층 구조 (Project / Shared / Global)
- [ ] GraphRAG 시각화 UI (인터랙티브 노드 맵)
- [ ] Folder Watcher (실시간 파일 인덱싱)
- [ ] Closed RAG 모드 (오프라인 리서치)
- [ ] Personal Memory (Mem0) — 교정·스타일 장기 기억
- [ ] Prompt Inheritance 우선순위 엔진
- [ ] Studio 아티팩트 (Mermaid 마인드맵, SVG 인포그래픽)
- [ ] 스레드형 검색 컨텍스트 누적

### 🟢 Phase 3: Computer Mode + 브릿지
- [ ] Ghost Prototyping WASM 샌드박스
- [ ] Ghost Prototyping Remote Server 연동
- [ ] Self-healing Loop (코드 → 실행 → 오류 → 재생성)
- [ ] **OS-Native UI Tree Extraction (Rust Hooks)**
- [ ] **Live Artifacts Hot-Reload (Tauri Webview)**
- [ ] **Agentic 위험도 분류기 (Qwen 1.5B)**
- [ ] MCP 도구 레지스트리 (fs, terminal, browser, ssh)
- [ ] SSH Lab Bridge (연결 관리 + 로그 스트리밍)
- [ ] Vision-Aided Computer Mode (화면 인식 + 조작)
- [ ] AaaS REST API 서버 (axum 기반)
- [ ] **PWA Client (읽기/채팅/승인 중심)**
- [ ] **세션/승인 센터 UI**
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
  - [ ] 로컬 플러그인 마켓 UI (스킬 카드형)
- [ ] **App Connectors**
  - [ ] VS Code / Obsidian / Browser Extension 연동
- [ ] Finance Agent
  - [ ] 거래 내역 CSV 임포트 + LLM 자동 분류
  - [ ] ISA / CMA 납입 한도 추적
  - [ ] 월별 소비 대시보드
  - [ ] ETF 보유 현황 + 리밸런싱 알림

### ⚪ Phase 5: 후순위 (미래 확장)
- [ ] Mobile 지원 (Tauri Mobile — iOS/Android)
- [ ] Audio Overview (Whisper STT + Kokoro TTS)
- [ ] 팀 워크스페이스 / 멀티 사용자
- [ ] ntransformer 가속화 아키텍처
  - [ ] Qwen 시리즈 모델 추론 최적화용 연산 커널 구현
  - [ ] Rust + SIMD/FlashAttention 기반 추론 효율 극대화

***

## 12. 하드웨어별 추천 구성

| 등급 | VRAM | 자동 매핑 모델 (추천) | 양자화 | 주요 용도 |
|---|---|---|---|---|
| **Entry** | 8GB | Llama 3.2 3B / Qwen 2.5 3B | Q5_K_M | 요약, 간단한 Q&A, 경량 클라이언트 |
| **Mid** | 12~16GB | Mistral NeMo 12B / Qwen 2.5 14B | Q6_K | Computer Mode, RAG 문서 분석 |
| **High** | 20~24GB | Llama 3.1 70B / Qwen 2.5 72B | Q4_K_M | 대규모 프로젝트, Self-healing |

***

## 13. 확정된 방향 (v4.4 기준)

| 항목 | 결정 | 비고 |
|---|---|---|
| **UI 프레임워크** | ✅ 확정 | React + Tailwind CSS v4 |
| **데이터베이스** | ✅ 확정 | SurrealDB 공식 스택 |
| **한국어 임베딩** | ✅ 도입 확정 | ko-sroberta-multitask (Phase 2) |
| **플러그인 생태계** | ✅ 도입 확정 | MCP 기반 Extension Marketplace |
| **템플릿 시스템** | ✅ 도입 확정 | Persona/Harness/Orchestration YAML 템플릿 |
| **백업 & 동기화** | ✅ 도입 확정 | AES-256 암호화 + 선택적 클라우드 |
| **멀티 클라이언트 구조** | ✅ 방향 확정 | Local Server Core + Browser/PWA 확장 |
| **PWA Client** | ✅ 조기 반영 | Mobile 전 단계 대체 수단으로 Phase 3 |
| **보안 기본 정책** | ✅ 확정 | Local-only 기본, 원격 접근은 선택 활성화 |
| **Finance Agent** | 🔶 후순위 확정 | 코어 완성 후 Phase 4 |
| **음성 I/O (TTS/STT)** | ⏸️ 최후순위 | Phase 5 |
| **Mobile** | ⏸️ 최후순위 | Phase 5 — 코어/PWA/데스크톱 우선 |

***

*마지막 업데이트: 2026-05-15 · Master Plan v4.4 (서버-클라이언트 확장성, 보안 재정비, 디자인 시스템 고도화 반영)*
