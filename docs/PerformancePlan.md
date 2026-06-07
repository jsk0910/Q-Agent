# Q-Agent Performance & Stabilization Plan v1.0

> **Version**: 1.0 · **Authored**: 2026-06-01
> **Status**: Active Blueprint — Phase 1.5 성능 및 안정화 점검 설계
> **Target**: 로컬 온디바이스 AI 에이전트 성능의 한계 검증 및 프로파일링 전략

---

## 1. 개요 및 목적

본 문서는 Q-Agent가 개인 PC 환경(로컬 전용)에서 동작할 때 요구되는 **최소 성능 기준**을 정의하고, 병목 현상을 방지하기 위한 최적화 전략과 프로파일링 계획을 다룹니다. 클라우드 의존성 없이 로컬 자원만을 활용하므로, 철저한 메모리 관리 및 자원 분배가 필수적입니다.

---

## 2. 하드웨어 티어 및 검증 기준

Q-Agent는 시스템 VRAM을 기준으로 3단계 등급으로 나누어 모델을 자동 매핑하며, 각 등급별로 성능 최소 기준을 만족해야 합니다.

### 2.1 하드웨어 등급

| 등급 | 시스템 사양 | 타겟 VRAM | 추천 라우팅 모델 | 추천 주력 모델 | 타겟 양자화 수준 |
|---|---|---|---|---|---|
| **Entry** | 사무용 PC, 구형 랩탑 | ~ 8GB | Qwen 2.5 1.5B | Qwen 2.5 3B / Llama 3.2 3B | Q5_K_M |
| **Mid** | 일반 개발자 워크스테이션 | 12 ~ 16GB | Qwen 2.5 3B | Mistral NeMo 12B / Qwen 2.5 14B | Q6_K |
| **High** | 고사양 GPU 탑재 데스크톱 | 20 ~ 24GB+ | Qwen 2.5 3B | Llama 3.1 70B (일부 오프로드) | Q4_K_M |

### 2.2 성능 검증 지표 (KPIs)

- **TTFT (Time To First Token)**: 로컬 모델 구동 후 첫 토큰 응답까지의 시간. 목표 1.5초 이내.
- **Generation Speed**: 토큰 생성 속도. 목표 15+ tokens/sec (Mid 티어 기준).
- **VRAM 점유율 (Overhead)**: 모델 가중치 로드 후 여유 VRAM 모니터링. 컨텍스트 윈도우 처리 시 Out Of Memory(OOM) 방지.

---

## 3. LLM 엔진 (llama.cpp) 안정화 및 검증

### 3.1 모델 체크포인트 다운로드 및 캐싱
- **Hugging Face 연동**: `hf-hub`를 활용한 GGUF 모델 자동 다운로드 및 무결성 검증 (SHA256).
- **동적 오프로딩 (Dynamic Offloading)**: 사용 가능한 VRAM에 따라 GPU/CPU 레이어 분배.
- **Lazy Loading**: 애플리케이션 시작 시 모든 모델을 메모리에 적재하지 않고, 호출 시점에 모델 로드 및 LRU 캐시 교체 알고리즘 적용.

### 3.2 컨텍스트 및 프롬프트 처리 최적화
- **Flash Attention**: llama.cpp 프롬프트 처리 시 Flash Attention을 활성화하여 메모리 대역폭 절약.
- **컨텍스트 제한**: 하드웨어 티어에 맞춰 최대 컨텍스트 윈도우 제한 (예: Entry 8k, Mid 32k, High 128k).

---

## 4. 데이터베이스 (SurrealDB) 성능 프로파일링

그래프 관계 탐색과 벡터 검색이 동시에 발생하는 구조이므로, 쿼리 병목을 사전에 파악합니다.

### 4.1 인덱싱 전략 검증
- **벡터 인덱스 (HNSW)**: `embedding` 필드에 대한 HNSW 인덱스의 빌드 속도 및 K-NN 검색 속도 프로파일링.
- **전통적 인덱스**: `task_id`, `session_id`, `created_at` 등 빈번히 조회되는 필드에 대한 B-Tree 인덱스 검증.

### 4.2 커넥션 풀 및 동시성
- SurrealDB 연결 시 풀링 설정을 최적화하여 다중 에이전트(Planner, Critic, Coder)가 동시에 로그를 기록하거나 검색할 때의 데드락 방지.
- **Profile 툴**: `surreal sql` 명령을 통한 쿼리 실행 계획(EXPLAIN) 분석으로 테이블 풀 스캔 여부 확인.

---

## 5. 코어(Rust) 및 UI 성능 최적화

### 5.1 Rust Backend (Tokio)
- **비동기 런타임 튜닝**: 워커 스레드 풀 크기를 조절하여 CPU 병목 완화.
- **메모리 릭 탐지**: `valgrind` / `jemalloc` 프로파일러를 통한 메모리 사용량 추적. LLM 스트리밍 채널 및 WebSocket 연결 유지 중 누수 확인.
- **Tracing**: `tracing-subscriber`를 이용한 스팬(Span) 기반 실행 시간 측정 및 병목 구간 시각화 (Jaeger 연동 고려).

### 5.2 UI Rendering (React + Tailwind)
- **가상화 렌더링**: GraphRAG 지식 맵(수백 개의 노드) 및 대용량 채팅 스레드 렌더링 시 Virtual DOM 부하 감소를 위해 가상화 리스트(예: `@tanstack/react-virtual`) 적용.
- **디바운싱/스로틀링**: 실시간 토큰 스트리밍 시 DOM 업데이트 주기 조절 (초당 30프레임 이하로 제한)하여 메인 스레드 부하 경감.
- **Lazy Loading**: 사용자가 선택하지 않은 탭(Artifact Preview 등)은 마운트 시점에 렌더링하도록 지연.

---

## 6. 버그 픽스 및 프로파일링 우선순위

성능 이슈 발생 시 다음과 같은 우선순위에 따라 해결합니다.

1. **P0 (Critical)**: VRAM OOM으로 인한 강제 종료, 시스템 프리징 (우선 해결).
2. **P1 (High)**: RAG 파이프라인 지연(3초 이상), 데드락에 의한 응답 중단.
3. **P2 (Medium)**: 스트리밍 끊김 현상, 불필요한 DB 풀 스캔, UI 버벅임.
4. **P3 (Low)**: 미세한 메모리 증가, 로깅 오버헤드.

> [!TIP]
> 디버깅 시에는 Rust의 `flamegraph`를 생성하여 호출 빈도와 병목 시간을 시각적으로 확인한 뒤 타겟을 지정하는 것을 권장합니다.
