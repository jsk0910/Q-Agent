# Q-Agent 네트워크 및 보안 설계 v1.0

> **Version**: 1.0 · **Authored**: 2026-06-01  
> **Status**: 확정 — Phase 1.5 네트워크/보안 설계 확정 단계 산출물  
> **대응 Plan**: Plan.md §10 (보안/네트워크 설계 원칙) · §9 (AaaS API) · §6 (권한 오케스트레이터)

---

## 0. 설계 원칙 요약

| 원칙 | 내용 |
|---|---|
| **Secure-by-Default** | 모든 외부 접근은 기본 비활성화. 사용자가 명시적으로 활성화해야만 열림 |
| **최소 권한** | 원격 클라이언트는 Tauri 로컬 UI보다 항상 더 제한된 권한 세트를 적용 |
| **감사 가능성** | 승인·거절·도구 실행·원격 접속 이력을 모두 이벤트 로그로 영구 보존 |
| **비밀정보 분리** | 토큰·키는 코드·DB 평문에 절대 저장하지 않고 OS Keychain 또는 암호화 볼트에 보관 |
| **격리 우선** | 에이전트 작업 범위를 프로젝트 폴더 내부로 강제 격리 |

---

## 1. 네트워크 모드 정책

### 1.1 모드 정의

| 모드 | 바인딩 주소 | 설명 | 활성화 조건 |
|---|---|---|---|
| **Local-only** (기본) | `127.0.0.1:8765` | Tauri 내부 IPC 전용. 외부에서 절대 접근 불가 | 앱 기동 시 자동 |
| **LAN Share** | `0.0.0.0:8765` (필터 적용) | 동일 서브넷(`/24`) IP만 허용. 방화벽 예외 사용자 동의 필요 | 설정 화면 토글 ON + 재시작 |
| **Remote (고급)** | `0.0.0.0:8765` (TLS 필수) | 인터넷 경유 접속. TLS + 강화된 인증 + Rate Limit 적용 | 고급 설정에서 명시적 활성화 + TLS 인증서 구성 완료 |

### 1.2 모드 전환 규칙

```text
Local-only (기본)
  ↓  사용자 LAN 토글 ON + 확인 다이얼로그 승인
LAN Share
  ↓  고급 설정 → Remote 활성화 + TLS 인증서 업로드 + 재확인
Remote
```

- **LAN → Local 전환** 시 기존 열린 외부 세션을 즉시 종료한다.
- 모드 전환 이력은 `audit_log` 테이블에 `event_type = "network_mode_change"`로 기록한다.
- UI Global Header의 네트워크 배지는 `Local-only` / `LAN` / `Remote` 세 가지 상태를 실시간 표시한다.

### 1.3 포트 정책

| 포트 | 용도 | 비고 |
|---|---|---|
| `8765` | AaaS REST + WebSocket | 단일 포트 집중. 불필요 포트 일체 비활성화 |
| 기타 | 미사용 | 방화벽·OS 레벨 모두 차단 |

---

## 2. 인증 (Authentication)

### 2.1 Tauri 로컬 클라이언트 (IPC)

- Tauri IPC 명령은 프로세스 경계 내부에서만 호출 가능하므로 **별도 토큰 없이 프로세스 신뢰** 모델을 적용한다.
- 단, 민감한 명령(모델 삭제, 설정 초기화, 원격 세션 강제 종료)에는 **재확인 다이얼로그** (HITL)를 요구한다.

### 2.2 API 클라이언트 (REST / WebSocket)

```text
로그인 플로우:
  POST /auth/login-local
    Body: { "passphrase": "<사용자 설정 패스프레이즈>" }
    → 200: { "access_token": "<JWT>", "refresh_token": "<Opaque>", "expires_in": 3600 }

토큰 갱신:
  POST /auth/token
    Body: { "refresh_token": "<Opaque>" }
    → 200: { "access_token": "<JWT>", "expires_in": 3600 }
```

#### JWT 구조

| 필드 | 값 | 설명 |
|---|---|---|
| `sub` | `session_id (UUID)` | 세션 식별자 |
| `role` | `local` \| `lan` \| `remote` | 클라이언트 유형 |
| `scope` | `["chat","artifacts","knowledge"]` | 허용된 기능 범위 |
| `exp` | Unix timestamp | 만료 (기본 1시간) |
| `iat` | Unix timestamp | 발급 시각 |

- **서명 알고리즘**: HS256 (시크릿은 OS Keychain 보관, §5 참조)
- **Refresh Token**: 불투명 랜덤 토큰 (32 bytes hex), DB `sessions` 테이블에 해시 저장, TTL 7일
- Access Token 만료 후 갱신 없으면 WebSocket 연결 자동 종료

### 2.3 패스프레이즈 정책

- 최초 앱 실행 시 사용자에게 패스프레이즈 설정을 요구한다 (선택 아님).
- 길이 최소 8자, 복잡성 검사는 강제하지 않되 UI에 강도 게이지 표시.
- 패스프레이즈 원문은 **저장하지 않음**. bcrypt(cost=12) 해시만 `settings` 테이블에 저장.
- 패스프레이즈 분실 시 복구 코드(16 words BIP-39 subset) 제공 옵션 — 복구 코드는 초기 설정 시 1회 표시.

---

## 3. 인가 (Authorization)

### 3.1 클라이언트 역할별 권한 범위

| 기능 | Local (Tauri IPC) | LAN 클라이언트 | Remote 클라이언트 |
|---|---|---|---|
| 대화 / 채팅 | ✅ | ✅ | ✅ |
| 아티팩트 조회 | ✅ | ✅ | ✅ |
| 아티팩트 수정/삭제 | ✅ | ✅ | ❌ (읽기 전용) |
| 지식 베이스 검색 | ✅ | ✅ | ✅ |
| 지식 베이스 인제스트 | ✅ | ✅ | ❌ |
| HITL 승인 처리 | ✅ | ✅ | ⚠️ 별도 PIN 재확인 필요 |
| 모델 설정 변경 | ✅ | ❌ | ❌ |
| 네트워크 모드 변경 | ✅ | ❌ | ❌ |
| 원격 세션 강제 종료 | ✅ | ❌ | ❌ |
| 로컬 파일 직접 접근 | ✅ | ❌ | ❌ |
| 터미널 도구 실행 | ✅ (권한 모드 적용) | ⚠️ Strict 전용 | ❌ |
| SSH 도구 연결 | ✅ (권한 모드 적용) | ❌ | ❌ |

### 3.2 세션별 권한 범위 분리

- 각 원격 세션은 생성 시 `scope` 배열이 JWT에 포함되며, 이후 해당 scope 외 API 호출은 `403 Forbidden` 반환.
- 세션 만료·종료 시 해당 세션의 승인 큐는 로컬 클라이언트로 자동 이관(대기 상태).
- 동일 계정의 여러 세션이 동시에 HITL 승인 요청을 수신하는 경우, **최초 응답 세션 적용** 규칙 (First-Write-Wins) 적용. 나머지 세션에는 `approval_resolved` 이벤트 전송.

---

## 4. 네트워크 보안

### 4.1 CORS / Origin 화이트리스트

```text
허용 Origin (Local-only 모드):
  - tauri://localhost          (Tauri WebView)
  - http://127.0.0.1:*        (개발 서버)

추가 허용 Origin (LAN Share 모드):
  - http://<LAN_SUBNET>/24 범위 IP만 허용
  - 사용자가 설정 UI에서 Origin 목록을 직접 추가/삭제 가능

추가 허용 Origin (Remote 모드):
  - 사용자가 설정한 명시적 도메인/IP 화이트리스트만 허용
```

- `Access-Control-Allow-Origin` 헤더는 와일드카드(`*`) 절대 사용 금지.
- Preflight 요청은 허용 Origin + 메서드(`GET, POST, PUT, DELETE, OPTIONS`) + 헤더(`Authorization, Content-Type`)만 통과.
- CORS 오류 시 응답 바디에 원인 노출 금지 (빈 바디 또는 `{"error":"cors_denied"}`).

### 4.2 CSRF 대응

- REST API는 `Authorization: Bearer <JWT>` 헤더를 필수로 요구하므로, 브라우저 자동 쿠키 방식의 CSRF는 해당 없음.
- PWA 클라이언트가 쿠키 기반 세션을 사용하게 될 경우(추후): `SameSite=Strict` + CSRF Double-Submit Cookie 패턴 적용.

### 4.3 보안 HTTP 헤더

모든 API 응답에 아래 헤더를 기본 포함한다:

```
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
Referrer-Policy: no-referrer
Content-Security-Policy: default-src 'none'; frame-ancestors 'none'
Permissions-Policy: geolocation=(), microphone=(), camera=()
```

### 4.4 입력 정제 (XSS 대응)

- API 레이어에서 수신한 모든 문자열 필드는 `serde` 역직렬화 후 HTML 엔티티 이스케이프 처리를 거친다.
- 아티팩트·메시지 내용 렌더링 시 React의 `dangerouslySetInnerHTML` 사용 금지. Markdown은 `sanitize-html` 후처리.
- 파일명·경로 파라미터에는 `..` / 절대경로 탐지 후 즉시 `400 Bad Request` 반환.

### 4.5 TLS 정책

| 모드 | TLS 적용 |
|---|---|
| Local-only | 불필요 (loopback) |
| LAN Share | **선택** — 자체 서명 인증서 or Let's Encrypt (mkcert 도구 가이드 제공) |
| Remote | **필수** — 유효한 도메인 인증서 또는 Cloudflare Tunnel 등 Reverse Proxy 권장 |

- TLS 없이 Remote 모드 활성화 시도 시 → 강제 경고 다이얼로그 차단 (우회 불가).
- 최소 TLS 버전: TLS 1.2 (axum/rustls 기본값이 1.2+이므로 별도 설정 불필요).

### 4.6 Rate Limiting

| 엔드포인트 | 제한 |
|---|---|
| `POST /auth/login-local` | IP당 5회/분, 초과 시 429 + 30초 Lock |
| `POST /agent/chat` | 세션당 30 req/분 |
| `POST /knowledge/ingest` | 세션당 10 req/분 |
| 기타 GET 엔드포인트 | 세션당 120 req/분 |

---

## 5. 비밀정보 저장 전략

### 5.1 저장 위치 분류

| 데이터 | 저장 위치 | 암호화 |
|---|---|---|
| JWT 서명 시크릿 | OS Keychain (Windows: DPAPI, macOS: Keychain) | OS 수준 보호 |
| Refresh Token (해시) | SurrealDB `sessions` 테이블 | bcrypt 해시 |
| 사용자 패스프레이즈 (해시) | SurrealDB `settings` 테이블 | bcrypt(cost=12) |
| 외부 API 키 (검색/클라우드) | OS Keychain | OS 수준 보호 |
| 복구 코드 | 사용자 책임 (앱 미저장) | — |
| E2EE 동기화 키 | OS Keychain | OS 수준 보호 |

### 5.2 OS Keychain 접근 인터페이스

- **Rust 크레이트**: `keyring` (Windows DPAPI / macOS Keychain / libsecret 지원)
- **서비스 이름 네임스페이스**: `q-agent/<key_name>` 형식으로 구분
- Keychain 접근 실패 시 → 평문 저장 절대 금지, 기능 비활성화 후 사용자에게 오류 안내

### 5.3 메모리 내 비밀정보 보호

- 시크릿 로드 후 사용 완료 시 Rust `Zeroize` 크레이트를 활용해 메모리 0-fill
- 로그 출력 시 비밀정보 포함 구조체는 `#[derive(Debug)]` 제외 또는 `redact` 처리

---

## 6. 감사 로그 (Audit Log)

### 6.1 로깅 대상 이벤트

| 이벤트 유형 | `event_type` 값 | 저장 필드 |
|---|---|---|
| 원격 세션 로그인 성공 | `session_login` | `session_id`, `client_ip`, `role` |
| 원격 세션 로그인 실패 | `login_failed` | `client_ip`, `reason` |
| HITL 승인 | `hitl_approved` | `session_id`, `task_id`, `tool_name`, `risk_score` |
| HITL 거절 | `hitl_rejected` | `session_id`, `task_id`, `tool_name`, `risk_score` |
| 도구 실행 완료 | `tool_executed` | `session_id`, `tool_name`, `args_hash`, `duration_ms` |
| 도구 실행 차단 | `tool_blocked` | `session_id`, `tool_name`, `reason` |
| 네트워크 모드 변경 | `network_mode_change` | `from_mode`, `to_mode` |
| 세션 강제 종료 | `session_terminated` | `session_id`, `terminated_by` |
| 권한 모드 변경 | `permission_mode_change` | `from_mode`, `to_mode` |
| 비밀정보 접근 | `secret_access` | `key_name`, `accessed_by` (session_id) |

### 6.2 로그 스키마 (SurrealDB)

```surql
DEFINE TABLE audit_log SCHEMAFULL;
DEFINE FIELD event_type    ON audit_log TYPE string;
DEFINE FIELD session_id    ON audit_log TYPE option<string>;
DEFINE FIELD client_ip     ON audit_log TYPE option<string>;
DEFINE FIELD payload       ON audit_log TYPE object;   -- 이벤트별 추가 데이터
DEFINE FIELD created_at    ON audit_log TYPE datetime DEFAULT time::now();

DEFINE INDEX audit_log_event_type_idx ON audit_log FIELDS event_type;
DEFINE INDEX audit_log_created_at_idx ON audit_log FIELDS created_at;
```

### 6.3 로그 보존 정책

- 기본 보존 기간: **90일** (설정에서 30/90/180/365일 선택 가능)
- 90일 초과 레코드는 매일 자동 정리 (Scheduled SurrealDB Task)
- 로그 내보내기: Settings → Audit Log → Export (JSON/CSV), E2EE 암호화 선택 옵션 제공
- 로그 자체는 **삭제 불가** (앱 UI에서 수동 삭제 미지원). 설정된 TTL에 의해서만 만료.

---

## 7. 프로젝트 폴더 격리 및 명령 필터

### 7.1 작업 경로 격리 (Sandbox Root)

- 에이전트의 모든 파일 시스템 접근은 **프로젝트 샌드박스 루트** 이하로 제한한다.

```text
샌드박스 루트: {app_data_dir}/projects/{project_id}/
  예) C:\Users\<user>\AppData\Roaming\q-agent\projects\<uuid>\
```

- `fs_tool`이 샌드박스 루트 외부 경로를 요청하는 경우:
  1. 요청을 즉시 차단 (`403 Forbidden`)
  2. `tool_blocked` 이벤트를 audit_log에 기록
  3. HITL 알림 팝업 표시 (\"범위 이탈 시도가 감지되었습니다\")

- 경로 탈출 탐지 패턴:
  - `..` 포함 경로 정규화 후 루트 외부 여부 확인
  - Windows 절대 경로 (`C:\`, `\\`) 직접 지정 차단
  - 심볼릭 링크 대상이 샌드박스 외부인 경우 차단

### 7.2 명령 화이트리스트 (terminal_tool)

#### 허용 명령 (Balanced 모드 자동 실행)

| 명령 | 허용 이유 |
|---|---|
| `ls`, `dir` | 디렉토리 목록 읽기 |
| `cat`, `type` | 파일 내용 읽기 |
| `grep`, `findstr` | 패턴 검색 |
| `pwd`, `echo` | 상태 확인 |
| `git status`, `git log`, `git diff` | 버전 제어 읽기 |
| `cargo check`, `cargo test` | 빌드/테스트 읽기 |
| `node --version`, `python --version` | 버전 확인 |

#### 조건부 허용 (Balanced 모드 승인 필요 / Strict 항상 승인)

| 명령 | 위험 분류 |
|---|---|
| `git commit`, `git push`, `git pull` | 상태 변경 |
| `npm install`, `pip install` | 외부 패키지 설치 |
| `cargo build`, `cargo run` | 빌드/실행 |
| `cp`, `mv`, `mkdir`, `rmdir` | 파일 조작 |
| `ssh`, `scp` | 원격 연결 |

#### 차단 명령 (모든 모드에서 항상 차단)

| 명령/패턴 | 차단 이유 |
|---|---|
| `rm -rf`, `del /f /q` | 대량 삭제 |
| `format`, `diskpart` | 디스크 포맷 |
| `shutdown`, `reboot` | 시스템 종료 |
| `curl \| bash`, `wget \| sh` | 원격 코드 실행 |
| `sudo`, `runas` | 권한 상승 |
| `> /dev/null 2>&1` 방식 출력 억제 | 감사 우회 |
| 배경 실행 (`&`, `nohup`) | 프로세스 이탈 |

### 7.3 명령 필터 우회 가능성 점검 결과

| 우회 시도 패턴 | 대응 방법 |
|---|---|
| 명령 분할 (`r` + `m` 등 문자열 조합) | 실행 전 전체 명령어 토큰화 후 최종 resolved 명령어 기준으로 재검사 |
| Shell 내장 변수 활용 (`$IFS` 등) | 쉘 자체 실행 금지 (`sh -c`, `bash -c`, `cmd /c` 직접 전달 차단) |
| 환경변수 PATH 조작 | `terminal_tool` 실행 시 `PATH` 고정 (시스템 기본 + 프로젝트 `.venv/bin`) |
| 파이프라인 악용 | 파이프라인(`|`) 포함 명령은 각 세그먼트를 개별 검사 |
| 별칭(alias) 활용 | alias 정의 명령 차단, 실행 전 alias 해석 |

---

## 8. WebSocket 인증

### 8.1 연결 시 인증 플로우

```text
1. 클라이언트 → WS 핸드셰이크: GET /ws?token=<JWT>
2. 서버 → JWT 검증 (서명 + 만료 + scope)
3. 실패 시: HTTP 401로 업그레이드 거부
4. 성공 시: WS 연결 수립 + 서버 측 세션 레지스트리에 등록

토큰 만료 처리:
5. 서버 → {"event": "token_expiring_soon", "data": {"remaining_seconds": 60}} 발송 (만료 60초 전)
6. 클라이언트 → REST /auth/token 으로 갱신 후 새 JWT를 다음 WS 재연결 시 사용
7. 만료 후 기존 WS는 서버에서 Clean Close (4001)로 종료
```

### 8.2 WS 메시지 인가 검사

- WS 연결 후에도 각 메시지의 `event` 타입이 JWT `scope` 내에 속하는지 재검사
- scope 외 이벤트 요청 시 → `{"event": "error", "data": {"code": "scope_denied"}}` 반환 후 연결 유지

### 8.3 다중 세션 충돌 방지

- 동일 `session_id`의 WS 연결이 2개 이상 시도될 경우: 기존 연결 종료 후 신규 연결 수립 (Single-Session 정책)
- HITL 승인 메시지는 **모든 활성 세션**에 브로드캐스트 단, **처리는 최초 응답 1건만** 적용 (First-Write-Wins)

---

## 9. 보안 점검 체크리스트 (완료 기준)

> Phase 1.5 → Phase 2 진입 전 아래 체크리스트를 모두 통과해야 한다.

| # | 항목 | 담당 레이어 | 상태 |
|---|---|---|---|
| N-01 | 바인딩 주소(`127.0.0.1`/`0.0.0.0`) 설정이 네트워크 모드와 연동되는가 | axum 서버 | 설계 확정 |
| N-02 | LAN Share 활성화 시 방화벽 예외 사용자 동의 다이얼로그가 표시되는가 | Tauri UI | 설계 확정 |
| N-03 | 미사용 API 엔드포인트가 비활성화(404 또는 미라우팅)되어 있는가 | axum 라우터 | 설계 확정 |
| N-04 | 원격 세션 타임아웃(기본 1시간) 및 강제 종료 기능이 구현되는가 | 세션 관리 | 설계 확정 |
| N-05 | WebSocket 연결 시 JWT 검증이 누락 없이 수행되는가 | WS 핸드셰이크 | 설계 확정 |
| N-06 | HITL 승인 요청이 타 세션에 교차 노출되지 않는가 | 세션 레지스트리 | 설계 확정 |
| N-07 | 프로젝트 폴더 외부 경로 접근 시도가 차단되는가 | fs_tool Interceptor | 설계 확정 |
| N-08 | 차단 명령 목록의 우회 패턴(분할 실행, 쉘 직접 호출 등)이 방어되는가 | Command Filter | 설계 확정 |
| N-09 | JWT 서명 시크릿이 OS Keychain에 보관되는가 | keyring 크레이트 | 설계 확정 |
| N-10 | 패스프레이즈가 bcrypt 해시만 저장되고 원문은 미저장되는가 | DB 저장 로직 | 설계 확정 |
| N-11 | CORS Origin 화이트리스트가 적용되는가 (와일드카드 없음) | axum CORS Layer | 설계 확정 |
| N-12 | 보안 HTTP 헤더 5종이 모든 응답에 포함되는가 | axum Middleware | 설계 확정 |
| N-13 | 인증 실패 Rate Limit(5회/분)이 동작하는가 | Rate Limit Layer | 설계 확정 |
| N-14 | audit_log 이벤트 10종이 의도한 시점에 기록되는가 | 이벤트 훅 | 설계 확정 |
| N-15 | 로그 TTL 정책(기본 90일)에 따른 자동 만료가 동작하는가 | DB Scheduled Task | 설계 확정 |
| N-16 | TLS 없이 Remote 모드 활성화 시도가 차단되는가 | 설정 검증 로직 | 설계 확정 |
| N-17 | Refresh Token이 DB에 해시만 저장되고 원문 전송은 초기 1회로 제한되는가 | 인증 로직 | 설계 확정 |

---

## 10. 구현 단계별 우선순위

Phase 3 (AaaS API 서버 구축) 시 아래 순서로 구현한다:

| 순위 | 구현 항목 | 대응 섹션 |
|---|---|---|
| 1 | JWT 발급/검증 미들웨어 (axum) | §2.2 |
| 2 | WS 연결 시 JWT 검증 + 세션 레지스트리 | §8 |
| 3 | CORS Layer + 보안 헤더 미들웨어 | §4.1, §4.3 |
| 4 | Rate Limit Layer (로그인 엔드포인트 우선) | §4.6 |
| 5 | 프로젝트 폴더 격리 인터셉터 (fs_tool) | §7.1 |
| 6 | 명령 화이트리스트/블랙리스트 필터 | §7.2 |
| 7 | OS Keychain 연동 (JWT 시크릿, API 키) | §5.2 |
| 8 | audit_log 이벤트 훅 전체 | §6 |
| 9 | 네트워크 모드 전환 로직 + UI 연동 | §1.2 |
| 10 | Rate Limit Layer (일반 엔드포인트) | §4.6 |

---

## 11. 관련 문서

| 문서 | 내용 |
|---|---|
| [Plan.md §10](./Plan.md) | 보안/네트워크 설계 원칙 원본 |
| [Plan.md §9](./Plan.md) | AaaS API 엔드포인트 목록 |
| [Plan.md §6](./Plan.md) | 선택형 권한 오케스트레이터 정의 |
| [Schema.md](./Schema.md) | SurrealDB 스키마 (sessions, audit_log 테이블) |
| [TestPlan.md](./TestPlan.md) | 보안 관련 통합/E2E 테스트 시나리오 |
| [AI_Code_Conduct.md](./AI_Code_Conduct.md) | 하네스 보안 등급 및 도구 허용 범위 |

---

*마지막 업데이트: 2026-06-01 · Security.md v1.0 — Phase 1.5 네트워크/보안 설계 확정*
