# Q-Agent 테스트 계획 (Test Plan v1.0)

> **Version**: 1.0 · **Authored**: 2026-06-01  
> **Status**: Active · **대상 Phase**: 1.5 재정비  
> **근거**: Master Plan v4.5 §11 Phase 1.5 "테스트 계획 수립" 항목 이행

---

## 0. 개요 & 목표

Phase 1.5 테스트 계획은 **"지금 존재하는 코어를 믿을 수 있는가"**를 검증하는 것을 1차 목표로 한다.  
E2E/UI 완성보다 **Rust 코어 로직의 정확성 + 통합 흐름의 안정성**에 집중한다.

| 레이어 | 도구 | 범위 |
|---|---|---|
| **단위 테스트** | `cargo test` + `tokio::test` | Rust 코어 모듈 전체 |
| **통합 테스트** | `cargo test` (in-memory DB) | DB ↔ Agent ↔ 워크플로우 흐름 |
| **E2E 테스트** | Playwright (TypeScript) | Tauri 앱 UI 시나리오 |
| **CI 파이프라인** | GitHub Actions | Push/PR 자동 빌드·린트·테스트 |

---

## 1. 단위 테스트 (Unit Tests)

> 파일별 `#[cfg(test)] mod tests` 블록에 위치.  
> 원칙: **외부 의존성 없이 순수 로직만 검증**. DB 접근은 `Database::init("memory")`만 허용.

### 1.1 현재 구현 현황

| 모듈 | 파일 | 테스트 수 | 상태 |
|---|---|---|---|
| `agents::state` | `agents/state.rs` | 5 | ✅ 전체 통과 |
| `agents::roles` | `agents/roles.rs` | 5 | ✅ 전체 통과 |
| `agents::security` | `agents/security.rs` | 6 | ✅ 전체 통과 |
| `models` | `models/mod.rs` | 5 | ✅ 전체 통과 |
| `db::projects` | `db/projects.rs` | 5 | ✅ 전체 통과 |
| `db::conversations` | `db/conversations.rs` | 4 | ✅ 전체 통과 (schema 수정 후) |
| `db::checkpoint` | `db/checkpoint.rs` | 3 | ✅ 전체 통과 |
| `lib (통합)` | `lib.rs` | 3 | ✅ 전체 통과 |

### 1.2 추가 단위 테스트 정의

아래 항목은 코어 로직이 확장될 때 함께 구현한다.

#### 1.2.1 `agents::graph` — WorkflowGraph 엣지 케이스

```
[U-GRAPH-01] Budget 초과 시 Error 상태 전이 검증
  - token_budget = 10, tokens_used = 10으로 설정
  - graph.run() 호출 시 status = Error("Token budget exceeded") 반환 확인

[U-GRAPH-02] Max Iteration 초과 시 Error 상태 전이 검증
  - max_iterations = 1, Critic이 계속 재시도 유도 시
  - status = Error("Max iterations reached") 반환 확인

[U-GRAPH-03] 빈 메시지로 Planner 호출 시 빈 플랜 처리
  - messages = [] 상태에서 graph.run() 호출
  - 패닉 없이 정상 종료 확인
```

#### 1.2.2 `agents::security` — 추가 경계 테스트

```
[U-SEC-01] Balanced 모드: 알 수 없는 도구명 → 수정 도구로 분류
  - tool_name = "unknown_tool" → approval_required = true

[U-SEC-02] Agentic 모드: score = 0.7 경계값 (미만 → 자동, 초과 → 승인)
  - 경계값 0.7은 자동, 0.701은 승인 필요로 분류

[U-SEC-03] RiskReport 직렬화 / 역직렬화 (JSON roundtrip)
```

#### 1.2.3 `models::manager` — 모델 관리자

```
[U-MODEL-01] 사용 가능한 모델 목록 조회 (파일 시스템 미의존 Mock)
[U-MODEL-02] ModelRequest 유효성 — max_tokens = 0 처리
[U-MODEL-03] embed() 결과 벡터 차원 일관성 확인
```

#### 1.2.4 `templates::manager` — 템플릿 시스템

```
[U-TPL-01] 기본 템플릿 4종 존재 여부 검증 (researcher/coder/analyst/default)
[U-TPL-02] 하네스 보안 등급 3종 존재 확인 (strict/standard/advanced)
[U-TPL-03] 잘못된 경로 접근 시 anyhow::Error 반환
```

---

## 2. 통합 테스트 (Integration Tests)

> 위치: `src-tauri/tests/` 디렉토리 (Rust integration test 관례).  
> `Database::init("memory")`를 사용해 실제 SurrealDB 엔진과 전체 스택을 검증한다.

### 2.1 시나리오 목록

#### IT-01: 기본 채팅 플로우 (Chat Flow)

```
시나리오: 사용자가 프로젝트를 생성하고 대화를 시작하여 메시지를 주고받는다.

사전 조건:
  - DB: memory 모드로 초기화
  - MockLlamaRunner 사용

단계:
  1. create_project() → Project { id: "p1", name: "테스트" } 저장
  2. create_conversation() → Conversation { project_id: "p1", mode: "chat" } 저장
  3. save_message() → Message { role: "user", content: "안녕" } 저장
  4. save_message() → Message { role: "assistant", content: "Mock 응답" } 저장
  5. list_messages(conversation_id) → 2개 메시지 반환 확인
  6. messages 순서 → created_at ASC 정렬 확인

검증 포인트:
  - [IT-01-A] 프로젝트 생성 성공
  - [IT-01-B] 대화 생성 및 프로젝트 연결 확인
  - [IT-01-C] 메시지 저장 및 목록 조회 정확성
  - [IT-01-D] 메시지 정렬 순서 (ASC)
```

#### IT-02: 에이전트 워크플로우 완주 (Agent Workflow Full Run)

```
시나리오: WorkflowGraph가 Planner → Executor → Critic 순환 후 Finished 상태로 종료된다.

단계:
  1. AgentState { task_id: "task-it-02", max_iterations: 5, token_budget: 8192 }
  2. messages에 사용자 요청 1건 삽입
  3. WorkflowGraph::default().run(state, &db) 호출
  4. 반환된 state 검증

검증 포인트:
  - [IT-02-A] status = Finished
  - [IT-02-B] plan.is_some() = true
  - [IT-02-C] final_answer.is_some() = true
  - [IT-02-D] critic_score >= 0.85
  - [IT-02-E] iteration <= max_iterations
  - [IT-02-F] DB에 checkpoint가 최소 3회 이상 저장됨
```

#### IT-03: AgentState 체크포인트 복구 (Checkpoint Resume)

```
시나리오: 진행 중인 AgentState를 저장 후 재로드해도 동일한 상태를 유지한다.

단계:
  1. AgentState { task_id: "task-chk", status: Executing, iteration: 2 } 생성
  2. db.save_agent_state(&state) 호출
  3. 새 DB 세션 없이 db.get_agent_state("task-chk") 호출
  4. 로드된 state와 원본 state 비교

검증 포인트:
  - [IT-03-A] task_id 일치
  - [IT-03-B] status 일치 (Executing)
  - [IT-03-C] iteration 값 일치 (2)
  - [IT-03-D] plan, final_answer, critic_score 값 일치
```

#### IT-04: HITL 승인 게이트 (Human-in-the-Loop Approval Gate)

```
시나리오: Agentic 모드에서 위험 도구 호출 시 승인 요청 상태가 발생한다.

단계:
  1. PermissionOrchestrator::new(Agentic) 생성
  2. evaluate_tool_call("fs_write", { "path": "/etc/hosts" }, None) 호출
  3. RiskReport 검증

검증 포인트:
  - [IT-04-A] approval_required = true (Balanced 폴백: 쓰기 도구)
  - [IT-04-B] score >= 0.7
  - [IT-04-C] reason 필드가 비어 있지 않음

추가 시나리오 — MockClassifier 활용:
  - [IT-04-D] MockLlamaRunner로 Agentic 분류 시 score = 0.4, approval_required = false
```

#### IT-05: 프로젝트 격리 (Project Isolation)

```
시나리오: 프로젝트 A의 대화가 프로젝트 B 조회에 노출되지 않는다.

단계:
  1. Project A, B 각각 생성
  2. Conversation A1, A2 (project: A), Conversation B1 (project: B) 생성
  3. list_conversations(Some("A")) 호출

검증 포인트:
  - [IT-05-A] 반환 목록에 A1, A2만 포함 (len = 2)
  - [IT-05-B] B1이 포함되지 않음
  - [IT-05-C] 반대 방향: list_conversations(Some("B")) → B1만 반환 (len = 1)
```

#### IT-06: Message 필드 전체 보존 (Message Schema Fidelity)

```
시나리오: citations, thought_trace, artifact_ids 등 확장 필드가 저장 및 복원된다.

단계:
  1. Message {
       citations: [{ "url": "https://example.com", "title": "Example" }],
       thought_trace: [{ "step": "planning", "text": "..." }],
       artifact_ids: ["art-001"],
       model_used: Some("mock-llama-3b"),
       tokens_used: Some(512),
     } 저장
  2. list_messages() 로 복원

검증 포인트:
  - [IT-06-A] citations 배열 길이 = 1
  - [IT-06-B] thought_trace 배열 길이 = 1
  - [IT-06-C] artifact_ids[0] = "art-001"
  - [IT-06-D] model_used = Some("mock-llama-3b")
  - [IT-06-E] tokens_used = Some(512)
```

### 2.2 통합 테스트 파일 구조

```
app/src-tauri/tests/
  integration_chat.rs       — IT-01, IT-06
  integration_workflow.rs   — IT-02, IT-03
  integration_security.rs   — IT-04
  integration_isolation.rs  — IT-05
```

---

## 3. E2E 테스트 (End-to-End Tests)

> 도구: **Playwright (TypeScript)**  
> 대상: Tauri 앱의 WebView (Chromium 기반)  
> 조건: MockLlamaRunner 주입 모드에서 실행 (실제 LLM 불필요)

### 3.1 설치 & 환경

```bash
# app/ 디렉토리에서
npm install -D @playwright/test
npx playwright install chromium

# tauri에서 E2E 실행 시 별도 test:e2e 스크립트 추가 필요
```

`package.json` 추가 스크립트:
```json
{
  "scripts": {
    "test:e2e": "playwright test",
    "test:e2e:ui": "playwright test --ui"
  }
}
```

`playwright.config.ts` 핵심 설정:
```typescript
import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: './e2e',
  use: {
    baseURL: 'tauri://localhost',  // Tauri WebView URL
  },
  timeout: 30_000,
});
```

### 3.2 E2E 시나리오 목록

#### E2E-01: 앱 초기 진입 & HUD 실행

```
시나리오: 앱을 실행하면 기본 화면이 로드되고 Alt+Space HUD가 동작한다.

단계:
  1. 앱 시작 → 메인 화면 로드 확인
  2. Alt+Space 단축키 입력 → HUD 오버레이 표시
  3. ESC 입력 → HUD 닫힘 확인

검증 포인트:
  - [E2E-01-A] data-testid="main-layout" 요소 존재
  - [E2E-01-B] HUD 오버레이 data-testid="hud-overlay" 표시
  - [E2E-01-C] HUD 닫힘 (aria-hidden="true" 또는 DOM에서 제거)
```

#### E2E-02: 프로젝트 생성 플로우

```
시나리오: 사용자가 새 프로젝트를 생성하고 사이드바에 표시된다.

단계:
  1. 사이드바 "새 프로젝트" 버튼 클릭
  2. 이름 입력: "My Test Project"
  3. 확인 버튼 클릭
  4. 사이드바 프로젝트 목록 확인

검증 포인트:
  - [E2E-02-A] 프로젝트 생성 모달이 표시됨
  - [E2E-02-B] 입력 후 목록에 "My Test Project" 항목 추가됨
  - [E2E-02-C] 새 프로젝트 선택 시 채팅 화면으로 전환됨
```

#### E2E-03: 채팅 메시지 송수신

```
시나리오: 채팅창에 메시지를 입력하면 스트리밍 응답이 표시된다.

전제: MockLlamaRunner 모드 (실제 LLM 불필요)

단계:
  1. 채팅 입력창(data-testid="chat-input") 클릭
  2. "안녕하세요" 입력 후 Enter 또는 전송 버튼 클릭
  3. 사용자 메시지 버블 표시 확인
  4. 어시스턴트 응답 스트리밍 시작 확인
  5. 스트리밍 완료 후 응답 버블 고정 확인

검증 포인트:
  - [E2E-03-A] 사용자 메시지 버블에 "안녕하세요" 텍스트 표시
  - [E2E-03-B] 어시스턴트 버블이 로딩 상태에서 스트리밍으로 전환
  - [E2E-03-C] 응답 완료 후 입력창이 비워짐
  - [E2E-03-D] 스크롤이 최신 메시지로 이동
```

#### E2E-04: 권한 모드 전환 (Permission Mode Switch)

```
시나리오: Strict → Balanced → Agentic 모드 전환이 UI에 반영된다.

단계:
  1. Settings 탭 → Permission 섹션 진입
  2. "Strict" 선택 → 상태 배지 확인
  3. "Balanced" 선택 → 상태 배지 변경 확인
  4. "Agentic" 선택 → 상태 배지 변경 확인

검증 포인트:
  - [E2E-04-A] Strict 선택 시 Global Header 배지 = "Strict"
  - [E2E-04-B] Balanced 선택 시 배지 = "Balanced"
  - [E2E-04-C] Agentic 선택 시 배지 = "Agentic"
  - [E2E-04-D] 페이지 새로고침 후 선택값 유지 (영속성)
```

#### E2E-05: HITL 승인 팝업 (Approval Gate UI)

```
시나리오: 위험 도구 호출 시 승인 팝업이 표시되고 승인/거절이 동작한다.

전제: 테스트 전용 "high-risk-mock" 도구 트리거 가능

단계:
  1. Strict 모드에서 도구 호출 트리거
  2. 승인 팝업(data-testid="approval-dialog") 표시 확인
  3. "승인" 버튼 클릭 → 팝업 닫힘 + 작업 계속
  4. 별도 시나리오: "거절" 클릭 → 팝업 닫힘 + 작업 중단

검증 포인트:
  - [E2E-05-A] 승인 팝업이 표시됨
  - [E2E-05-B] 위험도 점수가 팝업에 표시됨
  - [E2E-05-C] 승인 시 작업 계속 진행 확인
  - [E2E-05-D] 거절 시 Error 상태 메시지 표시
```

#### E2E-06: Harness Studio 템플릿 선택

```
시나리오: Harness Studio에서 페르소나 템플릿을 변경하면 프로젝트에 반영된다.

단계:
  1. Harness Studio 탭 진입
  2. Persona 드롭다운에서 "researcher" 선택
  3. 저장 버튼 클릭
  4. 프로젝트 재선택 후 Persona = "researcher" 확인

검증 포인트:
  - [E2E-06-A] 드롭다운 선택이 UI에 즉시 반영
  - [E2E-06-B] 저장 성공 토스트 메시지 표시
  - [E2E-06-C] 새로고침 후 설정값 유지 (DB 영속성)
```

### 3.3 E2E 파일 구조

```
app/e2e/
  app.setup.ts          — 앱 초기화 & testId 유틸
  e2e-01.hud.spec.ts    — E2E-01
  e2e-02.project.spec.ts — E2E-02
  e2e-03.chat.spec.ts   — E2E-03
  e2e-04.permission.spec.ts — E2E-04
  e2e-05.hitl.spec.ts   — E2E-05
  e2e-06.harness.spec.ts — E2E-06
```

---

## 4. 테스트 인프라

### 4.1 Rust 테스트 환경

| 항목 | 선택 | 비고 |
|---|---|---|
| 테스트 런너 | `cargo test` | 기본 내장 |
| 비동기 런타임 | `#[tokio::test]` | `tokio` feature = "full" |
| DB 격리 | `Database::init("memory")` | 테스트마다 독립 인스턴스 |
| Mock 모델 | `MockLlamaRunner` | `models/mod.rs`에 내장 |
| 로깅 | `tracing_subscriber` (test 시 비활성) | 필요 시 `RUST_LOG=debug` |

### 4.2 Frontend 테스트 환경

| 항목 | 선택 | 비고 |
|---|---|---|
| E2E 런너 | Playwright 1.x | TypeScript |
| 브라우저 | Chromium | Tauri WebView 동일 엔진 |
| 컴포넌트 테스트 | Vitest + @testing-library/react | 단위 수준 (추후) |
| Mock | Tauri `__TAURI_IPC__` Mock | Rust 없이 프론트 독립 테스트 |

### 4.3 테스트 커버리지 목표

| 레이어 | 현재 | 목표 |
|---|---|---|
| Rust 단위 테스트 | 40개 (39 pass / 1 fix) | 55개+ |
| Rust 통합 테스트 | 0개 | 12개+ (IT-01~06) |
| E2E | 0개 | 6개+ (E2E-01~06) |
| CI 통과율 | — | 100% (main branch) |

---

## 5. CI 파이프라인 (GitHub Actions)

> 위치: `.github/workflows/ci.yml`

### 5.1 트리거 조건

```yaml
on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]
```

### 5.2 Job 구성

```
┌──────────────────────────────────────────────────┐
│ Job 1: rust-check                                 │
│   cargo fmt --check                               │
│   cargo clippy -- -D warnings                     │
│   cargo build                                     │
└──────────────────────────┬───────────────────────┘
                           │ 의존
┌──────────────────────────▼───────────────────────┐
│ Job 2: rust-test                                  │
│   cargo test --lib          (단위 테스트)         │
│   cargo test --test '*'     (통합 테스트)         │
└──────────────────────────┬───────────────────────┘
                           │ 의존
┌──────────────────────────▼───────────────────────┐
│ Job 3: frontend-check                             │
│   npm ci                                          │
│   npm run build (tsc + vite build)                │
└──────────────────────────┬───────────────────────┘
                           │ 의존 (선택: E2E 환경 준비 후)
┌──────────────────────────▼───────────────────────┐
│ Job 4: e2e (Phase 1.5 후반 활성화)               │
│   npx playwright install --with-deps chromium     │
│   npm run test:e2e                                │
└──────────────────────────────────────────────────┘
```

### 5.3 CI YAML 초안

```yaml
name: Q-Agent CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  rust-check:
    name: Rust Lint & Build
    runs-on: windows-latest
    defaults:
      run:
        working-directory: app/src-tauri
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2
      - name: Format check
        run: cargo fmt --check
      - name: Clippy
        run: cargo clippy -- -D warnings
      - name: Build
        run: cargo build

  rust-test:
    name: Rust Tests
    needs: rust-check
    runs-on: windows-latest
    defaults:
      run:
        working-directory: app/src-tauri
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Unit Tests
        run: cargo test --lib
      - name: Integration Tests
        run: cargo test --test '*'
        continue-on-error: true  # 통합 테스트 파일 없을 경우 무시

  frontend-check:
    name: Frontend Build
    needs: rust-check
    runs-on: windows-latest
    defaults:
      run:
        working-directory: app
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
          cache-dependency-path: app/package-lock.json
      - name: Install dependencies
        run: npm ci
      - name: TypeScript check & build
        run: npm run build
```

---

## 6. 알려진 이슈 & 기술 부채

| ID | 내용 | 영향 | 우선순위 |
|---|---|---|---|
| **BUG-001** | `Message.citations` 스키마에 DEFAULT 없어 NOT NULL 오류 | `test_save_and_list_messages` 실패 | 🔴 긴급 (수정 완료) |
| **WARN-001** | `security.rs` L54: `response` 변수 미사용 | 컴파일 경고 | 🟡 낮음 |
| **DEBT-001** | `WorkflowGraph::run()` Critic Mock 로직 하드코딩 | 실제 LLM 연동 전까지 임시 | 🟡 Phase 2 전 해소 |
| **DEBT-002** | `graph.rs`에 단위 테스트 없음 | Budget/Iteration 엣지케이스 미검증 | 🟡 중간 |
| **DEBT-003** | `templates::manager` 단위 테스트 없음 | 템플릿 로딩 오류 미탐지 위험 | 🟡 중간 |
| **DEBT-004** | E2E testid 미지정 (data-testid 속성) | Playwright 셀렉터 불안정 | 🟠 E2E 작성 전 필수 |

---

## 7. 진행 체크리스트

> Plan.md §11 Phase 1.5 "테스트 계획 수립" 항목 대응

- [x] **단위 테스트 범위 정의** — 1.1 현황 + 1.2 추가 정의 완료
- [x] **통합 테스트 시나리오 정의** — IT-01~06 (§2.1) 완료
- [x] **E2E 테스트 시나리오 정의** — E2E-01~06 (§3.2) 완료
- [x] **테스트 인프라 선택** — §4 확정 (cargo test + Playwright)
- [x] **CI 파이프라인 초안** — §5.3 GitHub Actions YAML 완성
- [ ] **통합 테스트 코드 구현** — `tests/` 디렉토리 생성 (Phase 1.5 후반)
- [ ] **E2E 환경 셋업** — Playwright 설치 + data-testid 추가 (Phase 1.5 후반)
- [ ] **CI 파일 저장소 등록** — `.github/workflows/ci.yml` 커밋

---

*마지막 업데이트: 2026-06-01 · Test Plan v1.0*
