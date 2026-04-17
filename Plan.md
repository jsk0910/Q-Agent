# Q-Agent — Project Plan
> **Core Engine**: Ollama + Qwen 3.5 4B (Local) · **Stack**: Tauri 2.0 / SvelteKit 5 / Rust
> **Last Updated**: 2026-04-15 (SLM Intelligent Harness 패러다임 추가)

---

## 📍 현재 상태 요약 (2026-04-17 기준)

### ✅ 완료된 핵심 기능

| 항목 | 세부 내용 |
|---|---|
| **Intelligent Harness** | Context Builder, Eval-Loop, Router 등 SLM 최적화 루프 구현 완료 |
| **Project Isolation** | 프로젝트 단위 컨텍스트/RAG 격리 및 활성 프로젝트 스위칭 안정화 |
| **Inline Model Selector** | 실시간 모델 설치(`pull`), 프로필 기반 모델 매핑 및 영속화 |
| **Agentic Routing** | LLM 기반 의도 분석 및 도구(Web/Internal/RAG) 동적 라우팅 |
| **Web Search** | DuckDuckGo 연동 및 실시간 웹 정보 컨텍스트 주입 |
| **GUI/UX 시스템** | Svelte 5 Runes 기반 듀얼 모드 위젯 패널, 테마 엔진, 타이핑 애니메이션 |
| **Core Architecture** | Tauri 2.0 / Rust 기반의 모듈화된 명령 구조 및 에러 핸들링 |


---

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                     User Interfaces                     │
│  ┌──────────────────────┐  ┌───────────────────────────┐ │
│  │  GUI (Tauri + Svelte) │  │  TUI (Rust + Ratatui) [P5]│ │
│  └──────────┬───────────┘  └──────────┬────────────────┘ │
└─────────────┼─────────────────────────┼─────────────────┘
              │      IPC (invoke)        │ CLI
┌─────────────▼─────────────────────────▼─────────────────┐
│                  Rust Core (Tauri Backend)               │
│  ┌──────────┐  ┌──────────┐  ┌────────┐  ┌──────────┐   │
│  │ Ollama   │  │SurrealDB │  │ ssh2   │  │ ArXiv/   │   │
│  │ Client   │  │ RAG/Vec  │  │Monitor │  │ Notion   │   │
│  │ [P1 WIP] │  │ [Phase2] │  │[Phase3]│  │ [Phase4] │   │
│  └────┬─────┘  └────┬─────┘  └───┬────┘  └────┬─────┘   │
└───────┼─────────────┼────────────┼─────────────┼─────────┘
        │             │            │             │
  Ollama (LLM)   SurrealDB    Remote GPU    External APIs
  Qwen 3.5 4B    (Embedded)   (SSH)         ArXiv/Notion/Slack
```

---

## 🗺️ 전체 로드맵

---

### 🔴 Phase 0 — 전체 점검 및 안정화 (Full System Audit & Stabilization) `[진행 중]`

**목표**: Phase 2.5까지 구현된 핵심 기능들의 정합성을 전수 점검하고, 잠재적 버그 및 미진한 구현부를 수정하여 프로덕션급 안정성 확보.

#### 작업 목록 (긴급 수정 사항 포함)
- [ ] 🔴 **[BUG] 프로젝트 생성 기능 오작동**: 프로젝트 생성 명령 실행 시 정상적으로 저장되지 않는 문제 해결.
- [ ] 🔴 **[BUG] 모델 다운로드 에러**: `pull` 명령 부재 에러 (`pull` API 매핑 및 상태 처리 확인).
- [ ] 🔴 **[BUG] 인라인 모델 선택기 제한**: 대화 창 내 모델 선택 시 전체 목록이 아닌 일부만 노출되는 필터링 로직 수정.
- [ ] **회귀 테스트**: 프로젝트 생성, 모델 설치, 채팅, RAG 인용, 웹 검색 등 주요 경로 전수 점검.
- [ ] **Harness 신뢰도 검증**: `ContextBuilder`의 토큰 계산 및 트런케이션 로직 정밀도 확인.
- [ ] **Eval-Loop 최적화**: 다단계 추론 시의 레이턴시 측정 및 수렴 실패 케이스 방어 로직 강화.
- [ ] **RAG 파이프라인 강화**: 대용량 문서 파싱 시의 병목 구간 개선 및 임베딩 처리 속도 최적화.
- [ ] **데이터 영속화 점검**: 프로젝트/프로필 데이터의 UI-Backend 간 상태 불일치 및 저장 누락 해결.
- [ ] **UI/UX 폴리싱**: 반응형 레이아웃 결함, 애니메이션 끊김, 미흡한 다국어 처리 및 접근성 개선.


---

### 🟢 Phase 1 — Base UI & Ollama Integration `[완료]`

#### 완료
- [x] Perplexity 스타일 GUI 재설계 (Integrated Widget Panel)
- [x] Svelte 5 Runes 마이그레이션 완료
- [x] CSS 토큰 시스템 (Dark/Light/OLED + 5 Accent)
- [x] Sidebar 컴포넌트 (대화 목록, 핀, 접기)
- [x] UI Store (설정 영속화, 대화 히스토리)
- [x] 스트리밍 응답 구현 (Tauri event emit)
- [x] 멀티턴 컨텍스트 전달 (messages[] → Rust)
- [x] `lib.rs` 역할 정리 및 모듈화 (`commands/` 분리)
- [x] Qwen `<think>` 토글 렌더링 및 파이핑
- [x] 마크다운/코드 하이라이팅 (Copy 버튼 포함)
- [x] 모델 목록 자동 로드 (`list_models` → 드롭다운)
- [x] 컨텍스트 토큰 카운터 구현
- [x] 키보드 단축키 (Ctrl+N, Ctrl+,)
- [x] 대화 Export (Markdown)

---

### 🧠 Phase 2 — SLM Intelligent Harness & RAG `[진행 중]`

> **목표**: 로컬 Knowledge Base 고도화 및 인텔리전트 하니스 구축. 
> Qwen 3.5 4B의 약점을 보완하기 위해 Rust 백엔드가 컨텍스트를 조립하고 출력을 검증하는 소프트웨어 계층을 완성합니다.

---

#### 패러다임 1 — Intelligent Harness & Context Separation (Storage vs. View)

**개념**: LLM에게 날것의 채팅 히스토리나 RAG 청크를 그대로 먹이지 않는다.

| 레이어 | 역할 | 구현체 |
|---|---|---|
| **Storage** | 모든 원시 로그·툴 출력·전체 히스토리 보존 | SurrealDB |
| **Harness** | 매 턴마다 최소·최적화된 Working Context를 동적으로 조립 | Rust Backend |
| **View (Working Context)** | LLM이 실제로 보는 컨텍스트 (오래된 메시지 트런케이트, 필수 상태만 포함) | 조립된 `WorkingContext` struct |

##### 핵심 Rust 구조체

```rust
// src-tauri/src/harness/context.rs

/// LLM에 실제로 전달되는 최적화된 뷰 (Working Context)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingContext {
    /// 시스템 프롬프트 (동적으로 조립된 가이드라인 포함)
    pub system_prompt: String,
    /// 트런케이트된 대화 히스토리 (최근 N턴만 포함)
    pub messages: Vec<ContextMessage>,
    /// 현재 턴에 주입된 RAG 청크 (프로젝트 격리됨)
    pub rag_chunks: Vec<RagChunk>,
    /// 현재 턴에 주입된 웹 검색 결과 (필요 시)
    pub web_results: Option<Vec<WebSnippet>>,
    /// 조립 메타데이터 (디버깅·로깅용)
    pub meta: ContextMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMessage {
    pub role: MessageRole,
    pub content: String,
    /// 이 메시지의 SurrealDB record ID (원본 추적용)
    pub source_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole { System, User, Assistant, Tool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagChunk {
    pub content: String,
    pub score: f32,
    pub source_title: String,
    pub source_id: String,
    /// 어느 프로젝트에서 왔는지
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMeta {
    pub total_tokens_estimated: u32,
    pub messages_truncated: u32,
    pub guidelines_injected: Vec<String>,
    pub project_id: String,
    pub turn_id: String,
}

/// Working Context를 조립하는 빌더 trait
pub trait ContextBuilder: Send + Sync {
    async fn build(
        &self,
        user_message: &str,
        project: &Project,
        history: &[StoredMessage],
        guidelines: &[GuidelineCard],
    ) -> Result<WorkingContext, HarnessError>;
}

/// 기본 구현: 토큰 예산 초과 시 오래된 메시지부터 트런케이트
pub struct AdaptiveContextBuilder {
    pub max_context_tokens: u32,
    pub min_history_turns: usize,
    pub embedder: Arc<dyn Embedder>,
}
```

---

#### 패러다임 2 — Active Guideline Routing (Dynamic Prompt Builder)

**개념**: 단일 거대 `Design.md` 시스템 프롬프트 대신, 의도에 맞는 1~3개의 가이드라인 카드만 동적으로 주입한다.

##### 핵심 Rust 구조체

```rust
// src-tauri/src/harness/guidelines.rs

/// 개별 가이드라인 카드 (Markdown 파일 or SurrealDB 레코드)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuidelineCard {
    pub id: String,
    pub title: String,
    /// 마크다운 본문 (LLM 시스템 프롬프트에 삽입될 내용)
    pub content: String,
    /// 용도 태그 (의미 검색 및 필터용)
    pub tags: Vec<GuidelineTag>,
    /// 임베딩 벡터 (SurrealDB에 저장)
    pub embedding: Option<Vec<f32>>,
    /// 이 카드가 적용되는 프로젝트 (None = 전역)
    pub project_scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GuidelineTag {
    Academic,
    SafeCoding,
    RustStyle,
    Finance,
    FactChecking,
    Conciseness,
    Custom(String),
}

/// 가이드라인 라우터 trait
pub trait GuidelineRouter: Send + Sync {
    /// 사용자 메시지를 분석하여 관련 가이드라인 카드 1~3개를 반환
    async fn route(
        &self,
        user_message: &str,
        project: &Project,
        top_k: usize,
    ) -> Result<Vec<GuidelineCard>, HarnessError>;
}

/// 경량 의미 검색 기반 라우터 (nomic-embed-text 임베딩 재활용)
pub struct SemanticGuidelineRouter {
    pub db: Arc<SurrealDbClient>,
    pub embedder: Arc<dyn Embedder>,
}

/// 키워드 분류 기반 라우터 (임베딩 비용 0, 빠른 응답)
pub struct KeywordGuidelineRouter {
    /// 키워드 → 가이드라인 ID 매핑 테이블
    pub rules: Vec<(Vec<String>, String)>,
}
```

##### SurrealDB 스키마 (가이드라인)

```sql
DEFINE TABLE guideline SCHEMAFULL;
DEFINE FIELD id          ON guideline TYPE string;
DEFINE FIELD title       ON guideline TYPE string;
DEFINE FIELD content     ON guideline TYPE string;
DEFINE FIELD tags        ON guideline TYPE array;
DEFINE FIELD embedding   ON guideline TYPE array;
DEFINE FIELD project_id  ON guideline TYPE option<string>;
DEFINE FIELD created_at  ON guideline TYPE datetime;
```

---

#### 패러다임 3 — Project Workspace Isolation (Context Boundaries)

**개념**: RAG 데이터와 시스템 룰을 "프로젝트" 단위로 격리하여 토큰 낭비와 정보 오염을 방지한다.

##### 핵심 Rust 구조체

```rust
// src-tauri/src/harness/project.rs

/// 프로젝트 엔티티 — 격리된 작업 공간
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: String,              // 이모지 or URL
    /// 이 프로젝트에 기본 적용되는 가이드라인 ID 목록
    pub default_guideline_ids: Vec<String>,
    /// 이 프로젝트 소속 노트북 (RAG 소스) ID 목록
    pub notebook_ids: Vec<String>,
    /// 프롬프트 라이브러리 (프로젝트 전용 저장 프롬프트)
    pub prompt_library: Vec<SavedPrompt>,
    pub created_at: String,
    pub updated_at: String,
}

/// 프로젝트 내 노트북 (격리된 RAG 소스 컨테이너)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notebook {
    pub id: String,
    pub project_id: String,        // 반드시 부모 프로젝트에 귀속
    pub name: String,
    pub sources: Vec<NotebookSource>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotebookSource {
    Pdf   { path: String, title: String },
    Url   { url: String, title: String },
    ArXiv { arxiv_id: String, title: String },
    Code  { path: String, language: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPrompt {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

/// 프로젝트 매니저 — CRUD + 컨텍스트 격리 보장
pub struct ProjectManager {
    pub db: Arc<SurrealDbClient>,
}

impl ProjectManager {
    /// 프로젝트 전용 RAG 검색 (타 프로젝트 데이터 완전 차단)
    pub async fn search_chunks_isolated(
        &self,
        query_embedding: Vec<f32>,
        project_id: &str,
        top_k: usize,
    ) -> Result<Vec<RagChunk>, HarnessError> {
        // SurrealDB 쿼리에 `WHERE project_id = $project_id` 조건 필수
        todo!()
    }
}
```

##### SurrealDB 스키마 (프로젝트 격리)

```sql
DEFINE TABLE project SCHEMAFULL;
DEFINE FIELD id           ON project TYPE string;
DEFINE FIELD name         ON project TYPE string;
DEFINE FIELD description  ON project TYPE option<string>;

DEFINE TABLE notebook SCHEMAFULL;
DEFINE FIELD id           ON notebook TYPE string;
DEFINE FIELD project_id   ON notebook TYPE string;  -- 격리 키

-- chunk 테이블에 project_id 추가 (기존 Phase 2 스키마 확장)
DEFINE FIELD project_id ON chunk TYPE string;
DEFINE INDEX chunk_project_idx ON chunk FIELDS project_id;
```

---

#### 패러다임 4 — Generator-Evaluator Loop (Multi-Agent Validation)

**개념**: SLM 오류를 숨겨진 백그라운드 이터레이션으로 완화. 같은 4B 모델로 초안 생성 → 자기비평 → 수정을 최대 3회 반복한다.

##### 핵심 Rust 구조체

```rust
// src-tauri/src/harness/eval_loop.rs

/// 평가-루프의 한 이터레이션 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalIteration {
    pub iteration: u8,
    pub draft: String,
    pub critique: Option<String>,
    pub errors_found: bool,
    pub revision_needed: bool,
}

/// 평가 루프 최종 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalLoopResult {
    pub final_output: String,
    pub iterations: Vec<EvalIteration>,
    pub total_iterations: u8,
    pub converged: bool,
}

/// 평가 루프를 실행하는 harness
pub struct GeneratorEvaluatorLoop {
    pub ollama: Arc<OllamaClient>,
    pub max_retries: u8,             // 기본값: 3
}

impl GeneratorEvaluatorLoop {
    pub async fn run(
        &self,
        context: &WorkingContext,
        evaluator_system_prompt: &str,
        emit_progress: impl Fn(EvalLoopProgress),
    ) -> Result<EvalLoopResult, HarnessError> {
        let mut iterations = Vec::new();

        for i in 0..self.max_retries {
            // 1. Generator: 초안 생성
            let draft = self.generate(&context).await?;

            // 2. Evaluator: 비평 (시스템 프롬프트 교체)
            let critique = self.evaluate(&draft, evaluator_system_prompt).await?;
            let errors_found = self.parse_errors(&critique);

            iterations.push(EvalIteration {
                iteration: i + 1,
                draft: draft.clone(),
                critique: Some(critique),
                errors_found,
                revision_needed: errors_found,
            });

            // UI에 진행 상황 스트리밍 (Svelte가 단계별 표시 가능)
            emit_progress(EvalLoopProgress { iteration: i + 1, errors_found });

            if !errors_found { 
                return Ok(EvalLoopResult { 
                    final_output: draft, 
                    iterations, 
                    total_iterations: i + 1,
                    converged: true,
                });
            }
            // context에 비평 내용 추가하여 다음 이터레이션에 활용
        }

        // 최대 재시도 후 마지막 초안 반환
        Ok(EvalLoopResult {
            final_output: iterations.last().unwrap().draft.clone(),
            iterations,
            total_iterations: self.max_retries,
            converged: false,
        })
    }
}

/// Svelte UI에 평가 루프 진행 상황을 스트리밍하는 이벤트 payload
#[derive(Debug, Clone, Serialize)]
pub struct EvalLoopProgress {
    pub iteration: u8,
    pub errors_found: bool,
}
```

---

#### 통합 Harness 에러 타입

```rust
// src-tauri/src/harness/error.rs

#[derive(Debug, thiserror::Error)]
pub enum HarnessError {
    #[error("SurrealDB 오류: {0}")]
    Database(#[from] surrealdb::Error),
    #[error("Ollama 연결 오류: {0}")]
    Ollama(String),
    #[error("임베딩 오류: {0}")]
    Embedding(String),
    #[error("컨텍스트 토큰 예산 초과: {used} > {budget}")]
    TokenBudgetExceeded { used: u32, budget: u32 },
    #[error("가이드라인 없음: project_id={0}")]
    NoGuidelinesFound(String),
    #[error("프로젝트 격리 위반: chunk project={chunk_pid} != active project={active_pid}")]
    IsolationViolation { chunk_pid: String, active_pid: String },
    #[error("평가 루프 수렴 실패: {max_retries}회 초과")]
    EvalLoopNotConverged { max_retries: u8 },
}
```

---

#### 디렉토리 구조 (harness 모듈)

```
src-tauri/src/
├── harness/
│   ├── mod.rs           # pub use 재수출
│   ├── context.rs       # WorkingContext, ContextBuilder trait
│   ├── guidelines.rs    # GuidelineCard, GuidelineRouter trait
│   ├── project.rs       # Project, Notebook, ProjectManager
│   ├── eval_loop.rs     # GeneratorEvaluatorLoop
│   └── error.rs         # HarnessError
├── commands/
│   ├── chat.rs          # harness를 호출하는 Tauri command
│   ├── project.rs       # 프로젝트 CRUD Tauri commands
│   └── guidelines.rs    # 가이드라인 CRUD Tauri commands
└── ...
```

---

#### Svelte 5 (프론트엔드) 통합 전략

| 패러다임 | Svelte 컴포넌트 / 스토어 | 비고 |
|---|---|---|
| **Working Context** | `contextStore.ts` — 서버에서 `ContextMeta` 수신 후 토큰 사용량 표시 | 기존 토큰 카운터 고도화 |
| **Active Guideline Routing** | `GuidelinePills.svelte` — 현재 턴에 주입된 가이드라인 태그를 입력창 위에 뱃지로 표시 | 투명성/신뢰도 ↑ |
| **Project Isolation** | `ProjectSwitcher.svelte` — Sidebar 상단에서 프로젝트 전환, 활성 프로젝트 컨텍스트 전역 store | `projectStore.ts` |
| **Eval Loop** | `EvalLoopIndicator.svelte` — "검증 중 (2/3)..." 인디케이터 스트리밍 표시 | `eval_loop_progress` Tauri event |

```typescript
// stores/project.ts (Svelte 5 Runes)
interface ProjectState {
  activeProject: Project | null;
  projects: Project[];
}

let _state = $state<ProjectState>({ activeProject: null, projects: [] });

export const projectStore = {
  get active() { return _state.activeProject; },
  get all() { return _state.projects; },
  setActive(p: Project) { _state.activeProject = p; },
};
```

---

#### ⚠️ 아키텍처 비평 — 잠재적 병목 & 완화 전략

| # | 병목 지점 | 위험도 | 완화 전략 |
|---|---|---|---|
| **1** | **평가 루프 지연**: 3회 반복 = 실제 응답 시간 3배. 4B 모델의 ctx 전환 오버헤드까지 더해지면 UX 저하 심각 | 🔴 High | ① 기본은 루프 OFF, 사용자가 "검증 모드" 명시적 활성화. ② Generator 스트리밍을 즉시 UI에 노출하고 평가는 백그라운드에서 수행 (낙관적 UX). ③ "오류 없음" 확률 높은 단순 쿼리는 루프 bypass |
| **2** | **가이드라인 임베딩 레이턴시**: 매 턴마다 `nomic-embed-text` 호출 → RAG + 가이드라인 이중 임베딩 부하 | 🟡 Medium | `KeywordGuidelineRouter`를 1차 fast-path로 사용. 키워드 미매칭 시에만 의미 검색 fallback. 가이드라인 임베딩은 삽입 시 1회만 계산, SurrealDB 캐시 |
| **3** | **SurrealDB Embedded vs. 동시 쓰기**: `kv-rocksdb`는 멀티스레드 쓰기 시 락 경합 가능 | 🟡 Medium | Tauri의 `Mutex<SurrealDb>` state를 `Arc<Mutex>` 로 유지. 읽기 중심 설계(쓰기는 하니스 완료 후 비동기 저장). 고부하 시 SurrealDB 외부 프로세스 모드 전환 고려 |
| **4** | **컨텍스트 트런케이션 정확도**: 오래된 메시지 삭제 시 다단계 추론에 필요한 중간 결과가 사라질 위험 | 🟡 Medium | 단순 FIFO 삭제 대신 "중요도 점수" 기반 트런케이션 구현. `AssistantMessage`에 `importance: f32` 필드를 두고 코드 스니펫·수치 결과·의사결정 포함 메시지에 높은 점수 부여 |
| **5** | **Evaluator 프롬프트 오염**: 4B 모델이 Critic 페르소나와 Generator 페르소나를 섞어서 자기 작업을 옹호하는 현상 | 🟠 Medium-High | Evaluator 호출 시 system 프롬프트를 완전히 교체 + 대화 히스토리 없이 초안+비평 지침만 전달. 필요 시 "당신은 저자가 아닙니다. 외부 리뷰어입니다" 명시 |
| **6** | **프로젝트 격리 실수**: 쿼리에 `project_id` 필터 누락 시 데이터 오염 | 🔴 High | `ProjectManager::search_chunks_isolated`를 단일 진입점으로 강제. 직접 SurrealDB 쿼리 금지. `IsolationViolation` 에러로 컴파일 수준이 아닌 런타임 조기 panic |

---

#### 구현 태스크 (Phase 2.5)

- [ ] `src-tauri/src/harness/` 모듈 생성 (mod.rs, context.rs, guidelines.rs, project.rs, eval_loop.rs, error.rs)
- [ ] `AdaptiveContextBuilder` 구현 (토큰 예산 기반 트런케이션)
- [ ] `KeywordGuidelineRouter` 구현 (fast-path)
- [ ] `SemanticGuidelineRouter` 구현 (fallback, nomic-embed-text 재활용)
- [ ] SurrealDB `guideline` 테이블 정의 + 기본 가이드라인 카드 시딩
- [ ] SurrealDB `project`, `notebook` 테이블 정의
- [ ] `chunk` 테이블에 `project_id` 필드 추가 (마이그레이션)
- [ ] `ProjectManager` CRUD + `search_chunks_isolated` 구현
- [ ] `GeneratorEvaluatorLoop` 구현 + Tauri event 스트리밍
- [ ] Tauri commands: `create_project`, `list_projects`, `switch_project`
- [ ] Tauri commands: `list_guidelines`, `create_guideline`, `route_guidelines`
- [ ] `projectStore.ts` + `ProjectSwitcher.svelte` (Sidebar 통합)
- [ ] `GuidelinePills.svelte` (활성 가이드라인 뱃지 표시)
- [ ] `EvalLoopIndicator.svelte` (검증 진행 인디케이터)
- [ ] 설정 패널에 "검증 모드 ON/OFF" 토글 추가

#### 추가할 의존성

```toml
# Cargo.toml
thiserror = "1.0"           # HarnessError 구현
tiktoken-rs = "0.5"         # 토큰 카운팅 (트런케이션 로직)
```

---

### 🔵 Phase 3 — SSH 원격 GPU 모니터링 대시보드 `[계획]`

**목표**: GPU 서버 VRAM/프로세스 실시간 모니터링 + 원격 터미널

#### 작업 목록
- [ ] 다중 서버 프로파일 UI (이름/호스트/키파일/폴링 간격)
- [ ] `ssh2-rs` 연결 → `nvidia-smi --query-gpu=...` 폴링
- [ ] 실시간 차트 (Canvas + requestAnimationFrame)
- [ ] 프로세스 목록 (PID / VRAM / 사용자 / 실행 시간)
- [ ] `xterm.js` 임베드 SSH 터미널
- [ ] VRAM 임계값 초과 시 Slack Webhook 알림
- [ ] 원격 학습 스크립트 실행/중단/로그 스트리밍

#### 핵심 Rust 구조
```rust
#[tauri::command]
async fn get_gpu_stats(host: String, user: String, key_path: String)
  -> Result<GpuStats, String> { /* ssh2 + nvidia-smi */ }

#[derive(Serialize)]
struct GpuStats {
  gpu_util: u8, vram_used_mb: u32, vram_total_mb: u32,
  temperature: u8, processes: Vec<GpuProcess>,
}
```

#### 추가할 의존성
```toml
ssh2 = "0.9"
```

---

### 🔵 Phase 4 — ArXiv 논문 자동화 파이프라인 `[계획]`

**목표**: 최신 논문 수집 → Qwen 요약 → SurrealDB + Notion/Slack 발행

#### 작업 목록
- [ ] ArXiv API 크롤러 (카테고리: cs.CV, cs.LG, cs.AI + 키워드 필터)
- [ ] 구조화된 요약 프롬프트 (핵심 기여 / 방법론 / 실험 결과 / 한계점)
- [ ] Notion API 데이터베이스 페이지 자동 생성
- [ ] Slack Webhook 요약 발행
- [ ] `tokio-cron-scheduler` 기반 일일 스케줄러
- [ ] 논문 관심도 스코어 (별점 피드백 → 개인화 필터)
- [ ] SurrealDB에 저장 (Phase 2 RAG와 연동)

#### 파이프라인 흐름
```
매일 09:00 cron
→ ArXiv API 수집 (최신 50편)
→ 키워드/카테고리 필터
→ Qwen 3.5 요약 (structured prompt)
→ SurrealDB 저장 (RAG 활용)
→ Notion 페이지 생성
→ Slack #papers 알림
```

#### 추가할 의존성
```toml
tokio-cron-scheduler = "0.10"
quick-xml = "0.36"  # ArXiv Atom XML 파싱
```

---

### 🔵 Phase 5 — Ratatui TUI & Agentic Error Loop `[계획]`

**목표**: 터미널 전용 인터페이스 + 에러 자동 수정 루프

#### 작업 목록
- [ ] `q-tui` 별도 Rust crate 생성
- [ ] Ratatui 기반 TUI: Chat / GPU 모니터 / 논문 뷰어 탭
- [ ] Clap CLI: `q chat`, `q gpu <server>`, `q papers`
- [ ] Agentic Error Loop:
  ```
  cargo run → stderr 캡처 → LLM 분석 → diff 제안
  → 사용자 승인 → 파일 패치 → 재실행 (최대 3회)
  ```
- [ ] `q fix <command>` Shell 통합
- [ ] 파일 diff 뷰 (에이전트 제안 코드 변경 미리보기)
- [ ] q-core 공유 라이브러리 분리 (`ollama.rs`, `embedding.rs`)

#### TUI 레이아웃
```
┌──────────────┬────────────────────────────────┐
│  Navigation  │  Main Content                  │
│  > Chat      │  [Chat | GPU | Papers | Logs]  │
│    GPU Stats ├────────────────────────────────┤
│    Papers    │  Status: model / tokens / conn  │
└──────────────┴────────────────────────────────┘
```

#### 추가할 의존성 (q-tui crate)
```toml
ratatui = "0.29"
clap = { version = "4", features = ["derive"] }
crossterm = "0.28"
```

---

### 🔵 Phase 6 — 개인화 & 노코드 워크플로우 빌더 `[계획]`

**목표**: 페르소나 시스템, 노코드 자동화 빌더, 고급 개인화

#### 작업 목록
- [ ] **페르소나 스위칭** 4종: Research / Dev / Review / Debug
  - 각 페르소나: 전용 시스템 프롬프트 + 색상 테마 + 빠른 메뉴
- [ ] 시스템 프롬프트 편집기 (마크다운 미리보기 포함)
- [ ] **노코드 워크플로우 빌더** (드래그&드롭 노드 기반)
  ```
  [ArXiv 수집] → [Qwen 요약] → [Notion 저장] → [Slack 알림]
  ```
- [ ] 프롬프트 라이브러리 (저장 / 검색 / 즐겨찾기)
- [ ] 사용자 행동 기반 프롬프트 자동 최적화 (Few-shot 축적)
- [ ] 로컬 음성 입력 (`whisper.cpp` 연동)

---

## 🆕 Extended Features (추가 제안)

---

### 🌐 F. Perplexity 스타일 모드 시스템 `[신규 제안]`

**개요**: 상황에 맞게 UI·시스템 프롬프트·기본 모델·도구셋이 자동으로 전환되는 **4+1 전문 모드** 체계

| 모드 | 아이콘 | 기본 모델 추천 | 활성화 도구 | 핵심 기능 |
|---|---|---|---|---|
| **기본 (Default)** | 🤖 | `qwen3.5:4b` | 웹 검색 | 범용 Q&A, 웹 정보 수집 |
| **학술 (Academic)** | 📖 | `qwen3.5:4b` / `gemma3:9b` | ArXiv, 로컬 RAG, 인용 그래프 | 논문 검색·요약·인용 관리 |
| **코딩 (Coding)** | 💻 | `qwen2.5-coder:7b` | 코드 실행, GPU 모니터, SSH | 코드 작성·디버깅·서버 모니터링 |
| **재무 (Finance)** | 📈 | `qwen3.5:4b` | 뉴스 검색, 차트 렌더링 | 주식·경제 데이터 분석, 포트폴리오 요약 |
| **커스텀 (Custom)** | ✏️ | 자유 선택 | 사용자 정의 | 페르소나 + 도구 자유 조합 |

#### 각 모드별 세부 설계

**📖 학술 모드**
- ArXiv API 실시간 검색 (쿼리 자동 생성)
- 로컬 RAG + 인용 체인 (`[Source N]` 클릭 → 원문 하이라이트)
- 인용 그래프 시각화 (D3.js)
- 논문 요약 구조화 출력 (배경/방법론/실험/한계)

**💻 코딩 모드**
- `qwen2.5-coder` 계열 모델 우선 적용
- 코드 블록 실행 버튼 (WSL2/Docker 샌드박스)
- SSH 원격 GPU 모니터링 패널 통합
- 에러 메시지 → 자동 디버깅 루프
- Git diff 미리보기

**📈 재무 모드**
- 경제지표/뉴스 검색 (DuckDuckGo Finance)
- 숫자 데이터 → 차트 자동 렌더링 (Chart.js)
- 포트폴리오 입력 → 요약 분석 리포트 생성

#### 구현 계획
- [ ] 모드 상태를 Svelte 스토어에 저장 (`modeStore`)
- [ ] 모드별 전용 시스템 프롬프트 + 토큰 설정
- [ ] 상단 바 또는 입력창 하단에 모드 전환 탭바 UI 추가
- [ ] 모드별 Widget Panel 콘텐츠 자동 전환
- [ ] 모드별 기본 모델 사전 설정 (오버라이드 가능)

---

### 🔧 G. 인라인 모델 선택기 & 모델 설치 관리 `[신규 제안]`

**개요**: 설정 패널을 열지 않고도 **토덕바 에서 직접** 모델을 전환하다; 할당된 모델은 **사용자 프로파일** 단위로 영속 저장 (모드별 독립적)

#### 핵심 UX 시나리오
```
[학술 모드 ▼] [gemma3:9b ▼*] [검색 ON] [Sources] ... [입력창]
                          *이 모드에서 마지막으로 선택한 모델 = 자동 복원
```

| 기능 | 세부 내용 |
|---|---|
| **모델 자유 변경** | 모드 내에서 독립적으로 모델을 바꿼슈 프리셋은 계단마다 팁 표시) |
| **설치된 모델 구분** | ✅ 설치됨 / ⬇ 미설치 시각적 구분 |
| **원클릭 모델 설치** | 미설치 모델 선택 시 확인 모달 → `ollama pull` 실행 |
| **설치 진행 상태 표시** | 다운로드 진행률 바 (Tauri event stream) |
| **프로파일별 저장** | 모드별 모델 선택은 사용자 프로파일에 영속화 |
| **모드 전환 시 자동 복원** | 모드 전환 시 해당 모드에서 마지막으로 선택한 모델 자동 불러오기 |
| **대화별 모델 기록** | 각 대화에 사용된 모델 메타데이터 저장 |

---

#### 프로파일 & 모델 저장 시스템 설계

**프로파일이란?**  사용자 식별 단위의 설정 스냅왔. Q-Agent는 로컬우선 앱이므로 구싸우 인음인증 없이, **로컴 프로파일명** 의 변경만으로 전환 가능 (ex: "혐업 모드", "연구 모드" 등 복수 프로파일)

```
프로파일: "My Research"
├─ mode.default.model:   qwen3.5:4b
├─ mode.academic.model:   gemma3:9b        ← 마지막으로 선택한 모델 저장
├─ mode.coding.model:     qwen2.5-coder:7b
├─ mode.finance.model:    qwen3.5:4b
├─ mode.custom.model:     deepseek-r1:8b
└─ theme, shortcuts, system_prompts ...
```

**저장 방식**: `localStorage` (빠른 응답) + Tauri `app_data_dir()`의 `profiles.json` (영속 백업)

```typescript
// stores/profile.ts
interface ModeModelMap {
  default: string;
  academic: string;
  coding: string;
  finance: string;
  custom: string;
}

interface UserProfile {
  id: string;
  name: string;
  avatar: string;         // 이모지 or 컰드마크
  modeModels: ModeModelMap;  // 모드별 선택 저장
  theme: string;
  systemPrompts: Record<string, string>;
  createdAt: string;
}
```

**프로파일 전환 UX**
```
Sidebar 상단: [프로파일 아바타] [이름 ▼]
                    ├─ My Research   (현재)
                    ├─ Work Mode
                    ├─ + 새 프로파일 만들기
```

#### Rust 병렷 저장 커맨드
```rust
// 모델 설치 명령
#[tauri::command]
async fn pull_model(window: Window, model: String) -> Result<(), String> {
    // ollama pull {model} → 진행률 stream → window.emit("pull_progress", ...)
}

// 프로파일 저장/로드
#[tauri::command]
async fn save_profile(handle: AppHandle, profile: UserProfile) -> Result<(), String> {
    let path = handle.path().app_data_dir()?.join("profiles.json");
    // 프로파일 JSON 직렬화 후 저장
}

#[tauri::command]
async fn load_profiles(handle: AppHandle) -> Result<Vec<UserProfile>, String> { ... }

// 추천 모델 목록
#[tauri::command]
async fn get_recommended_models() -> Vec<ModelInfo> { ... }
```

#### 구현 태스크
- [ ] 상단 바 또는 입력창에 인라인 모델 선택 드롭다운 컴포넌트 (`ModelPicker.svelte`)
- [ ] Rust: `pull_model` 명령어 + 진행률 이벤트 스트리밍
- [ ] 모델 설치 진행 모달 (퍼센트 표시, 취소 가능)
- [ ] `profileStore` + `UserProfile` 타입 정의
- [ ] Rust: `save_profile` / `load_profiles` 커맨드 + `app_data_dir` 파일 I/O
- [ ] Sidebar 상단 프로파일 전환기 UI
- [ ] 모드 전환 시 해당 모드의 마지막 모델 자동 복원 로직
- [ ] 대화 메타데이터에 `modelUsed` 필드 추가
- [ ] 추천 모델 목록 정의 (용도별 태그 포함)

---

### 📚 H. NotebookLM 고도화 + 레퍼런스 에이전트 `[신규 제안]`

**개요**: 단순 RAG를 넘어 **능동적으로 소스를 탐색하고 인용하는 에이전트** 구축

#### 현재 → 목표
| 구분 | 현재 | 목표 |
|---|---|---|
| 소스 수집 | PDF 수동 업로드 | 웹 URL, YouTube, ArXiv, 로컬 폴더 드래그&드롭 다양 지원 |
| 청킹 | 기본 512-token | 의미 단위 청킹 (semantic chunking) + 제목 계층 보존 |
| 인용 | [Source N] 텍스트 | 클릭 시 원문 하이라이트 + 사이드 패널 미리보기 |
| 요약 | 없음 | 소스 자동 요약 카드 생성 |
| 에이전트 | 없음 | 질문 → 소스 탐색 → 증거 수집 → 종합 답변 루프 |

#### 레퍼런스 에이전트 아키텍처
```
사용자 질문
    ↓
[Query Decomposition Agent]  # 질문을 서브-쿼리로 분해
    ↓
[Source Retrieval Agent]     # 로컬 DB + 웹 검색으로 증거 수집
    ↓
[Evidence Scoring Agent]     # 관련성·신뢰도 점수 평가
    ↓
[Synthesis Agent]            # 증거 종합 → 인용 포인트 결정
    ↓
최종 답변 (인용 포인트 포함)
```

#### 소스 타입별 지원 로드맵
| 소스 타입 | 파싱 방법 | 우선순위 |
|---|---|---|
| PDF | `pdf-extract` / `lopdf` | 🔴 HIGH |
| 웹 URL | `scraper` + Readability | 🔴 HIGH |
| ArXiv 논문 | ArXiv API + PDF 파이프라인 | 🟡 MEDIUM |
| YouTube | 자막 추출 (`yt-dlp`) | 🟡 MEDIUM |
| 로컬 코드 폴더 | AST 파싱 + 청킹 | 🟡 MEDIUM |
| Notion 페이지 | Notion API | 🟢 LOW |

#### Knowledge Panel UI 개선 목표
- [ ] 소스 카드에 **자동 요약 + 키워드 태그** 표시
- [ ] 인용 클릭 시 **원문 하이라이트 + 사이드 미리보기** 패널
- [ ] 소스 타입별 아이콘 (PDF/URL/ArXiv/YouTube)
- [ ] 소스 간 **관계 그래프** 시각화 (D3.js)
- [ ] 노트 추가 기능 (소스별 메모)
- [ ] 소스 컬렉션 저장 / 태그 / 검색

#### 에이전트 구현 태스크
- [ ] `QueryDecomposer`: 복잡한 질문 → 서브쿼리 리스트 생성
- [ ] `EvidenceScorer`: 각 청크의 관련성·신뢰도 점수 계산
- [ ] `SynthesisAgent`: 인용 포인트 결정 + 최종 답변 구성
- [ ] URL 소스 추가 UI + Rust 크롤러 (`scraper` 크레이트)
- [ ] YouTube 자막 파이프라인
- [ ] 의미 단위 청킹 (문단/섹션 경계 인식)

---

### A. 멀티모달 지원 (Vision)
| 기능 | 세부 내용 | 모델 |
|---|---|---|
| 이미지 드래그&드롭 | 논문 figure 분석, UI→코드 | `qwen2.5vl:7b` |
| 스크린샷 캡처 | 화면 → 코드 자동 생성 | OS 캡처 API |

### B. 코드 실행 샌드박스
| 기능 | 세부 내용 |
|---|---|
| ▶ Run 버튼 | 코드 블록에 실행 버튼 추가 |
| 격리 환경 | WSL2 / Docker |
| 출력 렌더링 | stdout/stderr + matplotlib 임베드 |

### C. 모델 관리 대시보드
| 기능 | 세부 내용 |
|---|---|
| 모델 목록 | Ollama pull/delete/update UI |
| 벤치마크 비교 | 동일 프롬프트 → 여러 모델 나란히 비교 |
| VRAM 예측 | 모델 크기별 필요 VRAM 자동 계산표 |

### D. 72B+ 모델 확장 (NVMe Streaming)
| 기능 | 세부 내용 |
|---|---|
| 원격 추론 위임 | SSH로 GPU 서버 Ollama 라우팅 |
| 로드 밸런싱 | 4B(로컬) ↔ 72B(원격) 자동 선택 (문맥 길이 기준) |
| NVMe 대응 | ntransformer 지원 설계 |

### E. 실험 추적 통합
| 기능 | 세부 내용 |
|---|---|
| 프롬프트 버전 관리 | Git-like diff 히스토리 |
| A/B 테스트 | 두 프롬프트 전략 자동 비교 & 평가 |
| W&B 연동 | 실험 메타데이터 자동 로깅 |

---

## 📦 전체 의존성 계획

### Rust (Cargo.toml)

```toml
# 현재 설치됨
tauri = "2.0.0"
tauri-plugin-opener = "2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.12", features = ["json", "stream"] }  # stream 추가 필요
tokio = { version = "1", features = ["full"] }

# Phase 2: RAG
surrealdb = { version = "2", features = ["kv-rocksdb"] }

# Phase 3: SSH
ssh2 = "0.9"

# Phase 4: 스케줄러, XML
tokio-cron-scheduler = "0.10"
quick-xml = "0.36"

# Phase 5 (q-tui crate)
ratatui = "0.29"
clap = { version = "4", features = ["derive"] }
crossterm = "0.28"

# 공통 품질
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
```

### Node.js (package.json)

```json
{
  "현재 설치됨": ["@tauri-apps/api", "marked", "highlight.js"],
  "Phase 3 추가": ["xterm", "@xterm/addon-fit"],
  "Phase 2/4 추가": ["d3"],
  "Phase 6 추가": ["@tiptap/core (프롬프트 에디터)"]
}
```

---

## 📁 목표 디렉토리 구조

```
Q-Agent/
├── Plan.md
├── q-agent/                          # Tauri GUI 앱
│   ├── src/
│   │   ├── app.css                   # ✅ 글로벌 CSS 토큰
│   │   ├── app.html                  # ✅ 폰트/메타
│   │   ├── lib/
│   │   │   ├── stores/
│   │   │   │   └── ui.ts             # ✅ 설정/채팅 스토어
│   │   │   └── components/
│   │   │       ├── Sidebar.svelte    # ✅ 대화 목록/핀/접기
│   │   │       ├── SettingsPanel.svelte # ✅ 설정 드로어
│   │   │       ├── ChatBubble.svelte    # [ ] 메시지 분리
│   │   │       ├── ThinkingPanel.svelte # [ ] think 블록 컴포넌트화
│   │   │       ├── GpuDashboard.svelte  # [ ] Phase 3
│   │   │       ├── PaperCard.svelte     # [ ] Phase 4
│   │   │       └── KnowledgeBase.svelte # [ ] Phase 2
│   │   └── routes/
│   │       ├── +layout.ts            # ✅ SSR=false
│   │       ├── +layout.svelte        # ✅ 테마 적용
│   │       ├── +page.svelte          # ✅ 메인 Chat UI
│   │       ├── knowledge/+page.svelte  # [ ] Phase 2
│   │       ├── gpu/+page.svelte        # [ ] Phase 3
│   │       ├── papers/+page.svelte     # [ ] Phase 4
│   │       └── settings/+page.svelte   # [ ] (옵션)
│   └── src-tauri/src/
│       ├── main.rs                   # ✅ chat_with_ollama, list_models
│       ├── lib.rs                    # ⚠️ 정리 필요
│       └── commands/                 # [ ] 모듈화 예정
│           ├── chat.rs
│           ├── rag.rs
│           ├── ssh.rs
│           └── arxiv.rs
├── q-tui/                            # [ ] Phase 5: Ratatui TUI
└── q-core/                           # [ ] Phase 5: 공유 라이브러리
```

---

## 🔄 작업 우선순위 (다음에 할 일)

```
🔴 즉시 (Phase 2 고도화 + 신규 핵심 기능)
  1. 인라인 모델 선택기 (ModelPicker.svelte + pull_model 커맨드)
  2. 모드 시스템 기반 구현 (modeStore + 모드 탭바 UI)
  3. URL 소스 추가 UI + Rust 크롤러 (Knowledge Panel)
  4. PDF 업로드 고도화 (의미 단위 청킹)
  5. RAG 인용 UI (클릭 → 원문 하이라이트)

🟡 단기 (레퍼런스 에이전트 + 코딩/학술 모드)
  6. 레퍼런스 에이전트 파이프라인 (QueryDecomposer → EvidenceScorer → Synthesis)
  7. 학술 모드: ArXiv 연동 + 인용 그래프
  8. 코딩 모드: 코드 실행 샌드박스 + SSH GPU 대시보드 통합
  9. 재무 모드: 뉴스 검색 + 차트 렌더링

🟢 중기 (Phase 3 + 4 + NotebookLM 고도화)
  10. YouTube 자막 파이프라인
  11. ArXiv 크롤러 + Notion/Slack 자동화
  12. 소스 관계 그래프 시각화 (D3.js)

🔵 장기 (Phase 5 + 6)
  13. Ratatui TUI + Agentic Error Loop
  14. 노코드 워크플로우 빌더
  15. 72B+ 원격 모델 라우팅
```

---

## 📋 전체 체크리스트

### 🔴 Phase 0 — Core Stabilization
- [ ] Project Persistence (localStorage)
- [ ] DB Schema Alignment (Project created_at)
- [ ] Chat-Project Scoping (Project-based conversation filtering)
- [ ] Robust Model Pulling (Stream parsing fix)
- [ ] Knowledge Panel UI Polish

### Phase 1 — Base UI ✅ 완료
- [x] Perplexity 스타일 GUI
- [x] CSS 토큰 시스템 (Dark/Light/OLED)
- [x] 5가지 Accent 컬러 커스터마이징
- [x] Sidebar (대화 목록, 핀, 접기)
- [x] Settings Panel (슬라이드 드로어)
- [x] UI Store (localStorage 영속화)
- [x] Qwen `<think>` 토글 렌더링
- [x] Quick Prompts (Welcome 화면)
- [x] 자동 높이 textarea
- [x] 코드/표/링크 마크다운 렌더링
- [x] **스트리밍 응답**
- [x] **멀티턴 컨텍스트 전달**
- [x] lib.rs 정리 / commands/ 분리
- [x] 모델 목록 드롭다운
- [x] 대화 Export (MD/JSON)
- [x] 키보드 단축키
- [x] 토큰 카운터

### Phase 2 — RAG
- [x] SurrealDB Embedded
- [x] 임베딩 파이프라인 (nomic-embed-text)
- [x] 웹 검색 및 쿼리 라우팅 (DuckDuckGo Search)
- [x] 벡터 검색 + RAG 주입 (Hybrid)
- [x] Knowledge Base UI 고도화

### Phase 2.5 — SLM Intelligent Harness
- [ ] `src-tauri/src/harness/` 모듈 생성
- [ ] `AdaptiveContextBuilder` (토큰 예산 기반 트런케이션)
- [ ] `KeywordGuidelineRouter` (fast-path)
- [ ] `SemanticGuidelineRouter` (semantic fallback)
- [ ] SurrealDB `guideline` 테이블 + 기본 카드 시딩
- [ ] SurrealDB `project`, `notebook` 테이블 정의
- [ ] `chunk` 테이블 `project_id` 필드 마이그레이션
- [ ] `ProjectManager` CRUD + `search_chunks_isolated`
- [ ] `GeneratorEvaluatorLoop` + Tauri event 스트리밍
- [ ] Tauri commands: `create_project`, `list_projects`, `switch_project`
- [ ] Tauri commands: `list_guidelines`, `create_guideline`, `route_guidelines`
- [ ] `projectStore.ts` + `ProjectSwitcher.svelte`
- [ ] `GuidelinePills.svelte` (활성 가이드라인 뱃지)
- [ ] `EvalLoopIndicator.svelte` (검증 진행 인디케이터)
- [ ] 설정 패널 "검증 모드 ON/OFF" 토글

### Phase 3 — SSH GPU 모니터링
- [ ] 서버 프로파일 UI
- [ ] nvidia-smi 폴링
- [ ] 실시간 차트
- [ ] xterm.js 터미널
- [ ] Slack 알림

### Phase 4 — ArXiv 자동화
- [ ] ArXiv 크롤러
- [ ] 구조화 요약
- [ ] Notion 연동
- [ ] Slack Webhook
- [ ] cron 스케줄러

### Phase 5 — TUI & Agentic Loop
- [ ] q-tui crate
- [ ] Clap CLI
- [ ] Agentic Error Loop
- [ ] Shell 통합

### Phase 6 — 개인화
- [ ] 페르소나 스위칭
- [ ] 노코드 워크플로우
- [ ] 프롬프트 라이브러리
- [ ] 음성 입력 (Whisper)

### F. 모드 시스템 (Perplexity 스타일)
- [ ] modeStore (기본/학술/코딩/재무/커스텀)
- [ ] 모드 전환 탭바 UI
- [ ] 모드별 시스템 프롬프트 프리셋
- [ ] 모드별 Widget Panel 자동 전환
- [ ] 모드별 기본 모델 사전 설정

### G. 인라인 모델 선택기 & 설치 관리
- [ ] `ModelPicker.svelte` 컴포넌트
- [ ] Rust: `pull_model` + 진행률 스트리밍
- [ ] 미설치 모델 원클릭 설치 모달
- [ ] 대화별 모델 메타데이터 저장
- [ ] 추천 모델 목록 (용도별 태그)

### H. NotebookLM 고도화 + 레퍼런스 에이전트
- [ ] URL 소스 추가 UI + Rust 크롤러
- [ ] YouTube 자막 파이프라인
- [ ] 의미 단위 청킹 (semantic chunking)
- [ ] 소스 자동 요약 카드
- [ ] 인용 클릭 → 원문 하이라이트 + 사이드 미리보기
- [ ] `QueryDecomposer` 에이전트
- [ ] `EvidenceScorer` 에이전트
- [ ] `SynthesisAgent`
- [ ] 소스 관계 그래프 시각화 (D3.js)

### Extended
- [ ] 멀티모달 Vision
- [ ] 코드 샌드박스
- [ ] 모델 대시보드 (벤치마크 비교, VRAM 예측)
- [ ] 72B+ NVMe 스트리밍
- [ ] 실험 추적 (A/B, W&B)

---

*마지막 업데이트: 2026-04-15 · SLM Intelligent Harness 패러다임 4종 추가 (Context Separation / Active Guideline Routing / Project Isolation / Generator-Evaluator Loop) + Rust 구조체 설계 + 아키텍처 비평*
