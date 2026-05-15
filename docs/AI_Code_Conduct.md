# Q-Agent AI Code Conduct v4.1

> **Version**: 4.1 · **Authored**: 2026-05-13
> **목적**: Q-Agent 프로젝트에서 작업하는 AI 코딩 에이전트를 위한 행동 규범 및 개발 지침

---

## 0. 핵심 원칙

1. **Privacy-First**: 어떠한 코드도 사용자 데이터를 외부로 전송해서는 안 된다
2. **Human-in-the-Loop**: 파일 삭제, 외부 API 호출, 시스템 설정 변경은 반드시 HITL 승인 흐름을 거친다
3. **Minimal Footprint**: 필요한 최소한의 권한만 요청하고, 작업 범위를 명확히 제한한다
4. **Transparent Agency**: 에이전트의 모든 사고 과정과 도구 호출은 UI에 실시간으로 노출한다

---

## 1. 프로젝트 구조

```
Q-Agent/
├── docs/                   ← 내부 문서 (공개 금지)
│   ├── Plan.md             ← 마스터 플랜 (v4.1)
│   ├── Design.md           ← 디자인 시스템
│   ├── AI_Code_Conduct.md  ← 이 문서
│   └── Schema.md           ← 데이터 스키마
├── app/                    ← React + Tauri v2 앱 루트
│   ├── src/                ← React 프론트엔드
│   │   ├── components/     ← UI 컴포넌트
│   │   ├── pages/          ← 페이지/뷰
│   │   ├── stores/         ← 상태 관리 (Zustand)
│   │   ├── hooks/          ← 커스텀 훅
│   │   ├── types/          ← TypeScript 타입 정의
│   │   └── lib/            ← 유틸리티, API 클라이언트
│   ├── src-tauri/          ← Rust 백엔드
│   │   └── src/
│   │       ├── commands/   ← Tauri 커맨드 (프론트엔드 API)
│   │       ├── agents/     ← 에이전트 구현 (LangGraph)
│   │       ├── harness/    ← RAG, 검색, 인용 엔진
│   │       ├── models/     ← ModelRunner, llama.cpp 인터페이스
│   │       ├── db/         ← SurrealDB 레이어
│   │       └── templates/  ← YAML 템플릿 로더
│   └── templates/          ← 기본 제공 YAML 템플릿
│       ├── personas/
│       ├── harness/
│       └── orchestration/
├── 기획서.md               ← 원본 기획 문서 (참조용)
└── README.md
```

---

## 2. 기술 스택 규범

### 2.1 프론트엔드 (React)

| 항목 | 선택 | 비고 |
|---|---|---|
| **프레임워크** | React 19 | Server Components 미사용 (Tauri 환경) |
| **빌드** | Vite | Tauri 기본 연동 |
| **스타일** | Tailwind CSS v4 | CSS Variables 기반 테마 |
| **상태 관리** | Zustand | 글로벌 상태. 컴포넌트 로컬은 useState |
| **그래프 UI** | @xyflow/react | Knowledge Graph 시각화 |
| **아이콘** | lucide-react | 전용. 다른 아이콘 라이브러리 혼용 금지 |
| **애니메이션** | CSS transitions + Framer Motion | 복잡한 애니메이션만 Framer Motion |
| **타입** | TypeScript strict | `any` 사용 금지 |

**금지 사항:**
- `console.log` 프로덕션 코드 잔류 금지
- `useEffect` 내 비동기 데이터 패칭은 커스텀 훅으로 추상화
- 인라인 스타일(`style={{}}`) 사용 금지 — Tailwind 클래스 사용
- 하드코딩된 색상 값 금지 — CSS 변수 참조

### 2.2 백엔드 (Rust)

| 항목 | 선택 | 비고 |
|---|---|---|
| **에러 처리** | `anyhow` + `thiserror` | `unwrap()` / `expect()` 프로덕션 코드 금지 |
| **비동기** | `tokio` | async/await 일관 사용 |
| **직렬화** | `serde` + `serde_json` | 모든 구조체에 `Serialize/Deserialize` derive |
| **HTTP 클라이언트** | `reqwest` | 외부 요청에만. 내부 통신은 Tauri IPC |
| **DB** | `surrealdb` crate | SurrealDB 공식 Rust SDK |
| **벡터 검색** | `surrealdb` 내장 벡터 | 별도 벡터 DB 추가 금지 |
| **키워드 검색** | `tantivy` | BM25 검색 |
| **로깅** | `tracing` + `tracing-subscriber` | `println!` 프로덕션 코드 금지 |

**코드 스타일:**
- 공개 API는 반드시 `///` docstring 작성
- `clippy` 경고 0개 유지
- 모든 `pub fn`은 에러를 `Result<T, E>`로 반환

### 2.3 Tauri IPC 규범

```rust
// 모든 커맨드는 commands/ 디렉터리 내 모듈별 분리
// 커맨드 명명: snake_case
#[tauri::command]
pub async fn get_projects(
    state: State<'_, AppState>,
) -> Result<Vec<Project>, String> {
    // 에러는 String으로 직렬화하여 프론트엔드에 전달
    Ok(state.db.get_projects().await.map_err(|e| e.to_string())?)
}
```

```typescript
// 프론트엔드에서 Tauri 커맨드 호출
import { invoke } from '@tauri-apps/api/core';

const projects = await invoke<Project[]>('get_projects');
```

---

## 3. 보안 규범 (HITL)

### 3.1 위험도 등급

| 등급 | 작업 유형 | 처리 방식 |
|---|---|---|
| **L1 — 읽기** | 파일 읽기, DB 조회, 모델 실행 | 자동 허용 |
| **L2 — 생성** | 새 파일 생성, DB 삽입 | HITL 팝업 (Command Preview 포함) |
| **L3 — 수정** | 기존 파일 수정, DB 업데이트 | HITL 팝업 + 영향 범위 표시 |
| **L4 — 삭제/실행** | 파일 삭제, 터미널 명령, 브라우저 제어 | HITL 팝업 + 작업 전 백업 |
| **L5 — 외부** | SSH 접속, 외부 API 호출, 네트워크 요청 | HITL 팝업 + 명시적 사용자 활성화 |

### 3.2 Permission Scope 원칙

- 파일 작업은 반드시 현재 프로젝트 디렉터리 내부로 제한
- 경로 탈출(Path Traversal) 시도는 즉시 차단 및 로깅
- 외부 네트워크 요청은 사용자가 명시적으로 설정한 URL만 허용

### 3.3 Harness 보안 프로필 적용

```yaml
# harness/strict.yaml (예시)
profile: strict
allowed_tools:
  - read_file
  - search_kb
denied_tools:
  - write_file
  - terminal
  - browser
  - ssh
path_restrictions:
  - deny: "**"  # 모든 경로 차단
```

---

## 4. 에이전트 개발 규범

### 4.1 AgentState 불변 원칙

- `AgentState`는 각 전환마다 SurrealDB에 스냅샷 저장
- 에이전트는 이전 상태를 직접 수정하지 않고 새 상태를 반환
- 모든 도구 호출은 `tool_log`에 기록

### 4.2 Budget Guard 필수 구현

모든 에이전트 루프는 반드시 아래 두 조건을 검사해야 한다:

```rust
if state.iteration >= state.max_iterations {
    return Err(AgentError::IterationCapReached);
}
if state.tokens_used >= state.token_budget {
    return Err(AgentError::TokenBudgetExhausted);
}
```

### 4.3 Thought Trace 노출 의무

에이전트의 각 단계는 반드시 `tauri::emit`으로 프론트엔드에 스트리밍:

```rust
app_handle.emit("agent_thinking", ThinkingEvent {
    agent: "Planner".to_string(),
    step:  "태스크 그래프 생성 중...".to_string(),
    iteration: state.iteration,
}).ok();
```

### 4.4 템플릿 YAML 스키마

```yaml
# personas/ 예시
id: researcher
name: "AI 연구원"
description: "논문 분석 및 요약 특화"
system_prompt: |
  당신은 AI/ML 분야의 전문 연구원입니다.
  모든 답변에 논문 인용과 수식을 포함하고,
  불확실한 정보는 명확히 표시하십시오.
tone: academic
language: ko

# harness/ 예시
id: standard
name: "표준 하네스"
security_level: 2
allowed_tools: [read_file, write_file, search_kb, web_search]
path_scope: "project"  # project | workspace | unrestricted
max_file_size_mb: 10

# orchestration/ 예시
id: balanced
name: "균형 오케스트레이션"
loop:
  planner: true
  critic: true
  max_iterations: 5
  critic_threshold: 0.85
routing:
  simple_model: "small"   # 3B~8B
  complex_model: "heavy"  # 10B+
  complexity_threshold: 0.6
```

---

## 5. 데이터 규범

### 5.1 개인정보 보호

- 사용자의 대화 내용, 파일, 재무 데이터는 **절대** 외부로 전송 금지
- Finance 관련 데이터는 AES-256-GCM으로 암호화하여 저장
- 외부 웹 검색 시 쿼리 내용은 최소화 (PII 제거 후 전송)

### 5.2 SurrealDB 규범

- 모든 테이블은 `SCHEMAFULL` 선언 (스키마 없는 저장 금지)
- 레코드 ID는 `Uuid` 사용 (`ulid()` 가능)
- 소프트 삭제: `deleted_at: Option<DateTime>` 필드 사용
- 인덱스는 자주 조회하는 필드에만 추가

---

## 6. 테스트 규범

| 계층 | 도구 | 대상 |
|---|---|---|
| **Unit** | Rust `#[test]`, Vitest | 순수 함수, 유틸리티 |
| **Integration** | Rust `#[tokio::test]` | DB 레이어, 에이전트 루프 |
| **E2E** | Playwright (Tauri 연동) | 핵심 사용자 흐름 |

**최소 커버리지 목표:**
- Rust 백엔드 핵심 모듈: 80%+
- React 컴포넌트: 주요 인터랙션 E2E 보장

---

## 7. 커밋 & 브랜치 규범

```
feat: 새 기능 추가
fix:  버그 수정
refactor: 기능 변화 없는 리팩토링
docs: 문서 수정
test: 테스트 추가/수정
chore: 빌드, 설정 변경
```

**브랜치 전략:**
- `main`: 릴리즈 가능한 안정 버전
- `dev`: 통합 개발 브랜치
- `feature/<name>`: 기능 개발
- `fix/<name>`: 버그 수정

---

## 8. AI 에이전트 작업 금지 목록

다음 작업은 **사용자 명시 요청 없이 절대 수행 금지:**

- `docs/` 디렉터리 내 파일 수정 (Plan.md, Design.md 등)
- `.git/` 디렉터리 접근
- 환경변수 파일(`.env*`) 수정
- 패키지 매니저 글로벌 설치 (`npm install -g`)
- `git push` / `git force-push`
- 외부 API 키 하드코딩
- Finance 데이터의 외부 전송

---

*마지막 업데이트: 2026-05-13 · AI Code Conduct v4.1*
*이 문서는 `docs/` 내에서만 관리되며 외부에 공개하지 않습니다.*
