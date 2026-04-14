# Q-Agent — Project Plan
> **Core Engine**: Ollama + Qwen 3.5 4B (Local) · **Stack**: Tauri 2.0 / SvelteKit 5 / Rust
> **Last Updated**: 2026-04-13 (Phase 1 완료, Phase 2 RAG 진행 중)

---

## 📍 현재 상태 요약 (2026-04-13 기준)

### ✅ 완료된 것들

| 항목 | 세부 내용 |
|---|---|
| 프로젝트 스캐폴딩 | Tauri 2.0 + SvelteKit 5 + TypeScript + Runes (v5) |
| Ollama 연동 | `/api/chat` 및 스트리밍(`chat_token`) 지원 |
| 마크다운 렌더링 | `marked v18` + `highlight.js` (Copy 버튼 포함) |
| **GUI 재설계** | Integrated vs Overlay 듀얼 모드 Widget Panel |
| **CSS 토큰 시스템** | Dark / Light / OLED 테마 + 5가지 Accent 컬러 |
| **UI Store** | Svelte 5 Runes 기반 영속적 설정 및 채팅 이력 |
| **Agentic Routing** | LLM 기반 쿼리 라우팅 (Local / Web / Internal) |
| **Web Search** | DuckDuckGo 연동 실시간 정보 수집 |
| **Welcome 화면** | Quick Prompts, 타이핑 애니메이션, 글로우 효과 |
| **입력창** | 자동 높이 조절, 단축키 지원 (Ctrl+N, Ctrl+,) |

### ⚠️ 당장 고쳐야 할 사항 (Phase 1 버그/미완)

| 우선순위 | 항목 | 이유 |
|---|---|---|
| 🔴 CRITICAL | **임베딩 최적화** | 대용량 PDF 로딩 시 속도 저하 개선 필요 |
| 🔴 HIGH | **RAG 인용 UI** | 답변 내 [Source N] 클릭 시 해당 소스 하이라이트 |
| 🟡 MEDIUM | **대화 Export 기능 확장** | PDF/HTML 내보내기 추가 |
| 🟡 MEDIUM | **반응형 레이아웃** | 모바일/소형 창에서의 측면 패널 동작 최적화 |
| 🟢 LOW | **다국어 지원** | i18n 적용 (한국어/영어) |

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

### 🟡 Phase 2 — SurrealDB RAG & Agentic Routing `[진행 중 — 40% 완료]`

**목표**: 로컬 Knowledge Base 구축 및 인텔리전트 라우팅 — 정보 수집 후 답변 생성

#### 작업 목록
- [x] SurrealDB Embedded 연동 (`kv-rocksdb`)
- [x] LLM 기반 쿼리 라우터 (`intercept_and_route`)
- [x] DuckDuckGo 웹 검색 연동 및 컨텍스트 주입
- [x] Ollama `nomic-embed-text` 기반 벡터 임베딩 파이프라인
- [x] 벡터 유사도 검색 (코사인) → Top-K 추출
- [ ] PDF 업로드 및 텍스트 청크화 고도화
- [ ] Knowledge Base UI (`/knowledge` 패널) 개선
- [ ] 논문 인용 그래프 시각화 (D3.js)

#### 핵심 스키마
```sql
DEFINE TABLE paper SCHEMAFULL;
DEFINE FIELD title ON paper TYPE string;
DEFINE FIELD embedding ON paper TYPE array;
DEFINE FIELD arxiv_id ON paper TYPE string;

DEFINE TABLE chunk SCHEMAFULL;
DEFINE FIELD content ON chunk TYPE string;
DEFINE FIELD embedding ON chunk TYPE array;

DEFINE TABLE cites SCHEMAFULL;     -- paper -> paper
DEFINE TABLE contains SCHEMAFULL; -- paper -> chunk
```

#### 추가할 의존성
```toml
surrealdb = { version = "2", features = ["kv-rocksdb"] }
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
- [ ] Knowledge Base UI 고도화

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

*마지막 업데이트: 2026-04-13 · 모드 시스템 / 인라인 모델 선택기 / NotebookLM 에이전트 기능 기획 추가*
