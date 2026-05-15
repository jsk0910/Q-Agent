# Q-Agent Data Schema v4.1

> **Version**: 4.1 · **Authored**: 2026-05-13
> **Database**: SurrealDB 2.x
> **Convention**: SCHEMAFULL, UUIDs, 소프트 삭제(`deleted_at`)

---

## 0. 공통 규칙

```surql
-- 모든 테이블의 공통 필드 패턴
DEFINE FIELD id         ON TABLE <T> TYPE string;   -- UUID (auto or manual)
DEFINE FIELD created_at ON TABLE <T> TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE <T> TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON TABLE <T> TYPE option<datetime>;  -- 소프트 삭제
```

---

## 1. Project (프로젝트)

```surql
DEFINE TABLE project SCHEMAFULL;

DEFINE FIELD id          ON project TYPE string;
DEFINE FIELD name        ON project TYPE string;
DEFINE FIELD description ON project TYPE option<string>;
DEFINE FIELD icon        ON project TYPE option<string>;  -- 이모지 or 아이콘 이름
DEFINE FIELD color       ON project TYPE option<string>;  -- hex color

-- 템플릿 설정
DEFINE FIELD persona_template     ON project TYPE string DEFAULT 'default';
DEFINE FIELD harness_template     ON project TYPE string DEFAULT 'standard';
DEFINE FIELD orchestration_template ON project TYPE string DEFAULT 'balanced';

-- 모델 설정
DEFINE FIELD model_small  ON project TYPE option<string>;  -- 라우팅용 소형 모델 ID
DEFINE FIELD model_heavy  ON project TYPE option<string>;  -- 복잡 태스크 대형 모델 ID

-- 자원 한도
DEFINE FIELD max_iterations ON project TYPE int DEFAULT 5;
DEFINE FIELD token_budget   ON project TYPE int DEFAULT 8000;
DEFINE FIELD fvcore_limit   ON project TYPE option<float>;  -- null = 무제한

-- 보안
DEFINE FIELD security_level  ON project TYPE int DEFAULT 2;  -- 1~5
DEFINE FIELD allowed_paths   ON project TYPE array<string> DEFAULT [];

-- 메타
DEFINE FIELD is_active   ON project TYPE bool DEFAULT false;
DEFINE FIELD created_at  ON project TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at  ON project TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at  ON project TYPE option<datetime>;

DEFINE INDEX idx_project_name ON project FIELDS name;
```

---

## 2. Conversation & Message (대화)

```surql
DEFINE TABLE conversation SCHEMAFULL;

DEFINE FIELD id         ON conversation TYPE string;
DEFINE FIELD project_id ON conversation TYPE option<record<project>>;
DEFINE FIELD title      ON conversation TYPE string DEFAULT 'New Conversation';
DEFINE FIELD mode       ON conversation TYPE string  -- 'research' | 'code' | 'mlops'
    ASSERT $value IN ['research', 'code', 'mlops', 'chat'];
DEFINE FIELD created_at ON conversation TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON conversation TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at ON conversation TYPE option<datetime>;

DEFINE INDEX idx_conv_project ON conversation FIELDS project_id;

-- ────────────────────────────────────────────────────

DEFINE TABLE message SCHEMAFULL;

DEFINE FIELD id              ON message TYPE string;
DEFINE FIELD conversation_id ON message TYPE record<conversation>;
DEFINE FIELD role            ON message TYPE string  -- 'user' | 'assistant' | 'system'
    ASSERT $value IN ['user', 'assistant', 'system'];
DEFINE FIELD content         ON message TYPE string;
DEFINE FIELD citations       ON message TYPE array<object>;
    -- [ { index: int, source_id: string, excerpt: string, confidence: float } ]
DEFINE FIELD thought_trace   ON message TYPE array<object> DEFAULT [];
    -- [ { agent: string, step: string, timestamp: datetime } ]
DEFINE FIELD artifact_ids    ON message TYPE array<string> DEFAULT [];
DEFINE FIELD model_used      ON message TYPE option<string>;
DEFINE FIELD tokens_used     ON message TYPE option<int>;
DEFINE FIELD duration_ms     ON message TYPE option<int>;
DEFINE FIELD created_at      ON message TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_msg_conv ON message FIELDS conversation_id;
```

---

## 3. Knowledge (지식 베이스)

```surql
-- 소스 문서 (PDF, URL, 코드 파일 등)
DEFINE TABLE knowledge_source SCHEMAFULL;

DEFINE FIELD id          ON knowledge_source TYPE string;
DEFINE FIELD project_id  ON knowledge_source TYPE option<record<project>>;
    -- null = Global Commons, record = Project Private
DEFINE FIELD scope       ON knowledge_source TYPE string DEFAULT 'project'
    ASSERT $value IN ['global', 'shared', 'project'];
DEFINE FIELD source_type ON knowledge_source TYPE string
    ASSERT $value IN ['pdf', 'url', 'code', 'text', 'folder'];
DEFINE FIELD title       ON knowledge_source TYPE string;
DEFINE FIELD uri         ON knowledge_source TYPE string;  -- 로컬 경로 or URL
DEFINE FIELD mime_type   ON knowledge_source TYPE option<string>;
DEFINE FIELD size_bytes  ON knowledge_source TYPE option<int>;
DEFINE FIELD page_count  ON knowledge_source TYPE option<int>;
DEFINE FIELD indexed_at  ON knowledge_source TYPE option<datetime>;
DEFINE FIELD is_active   ON knowledge_source TYPE bool DEFAULT true;
DEFINE FIELD created_at  ON knowledge_source TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at  ON knowledge_source TYPE option<datetime>;

DEFINE INDEX idx_ks_project ON knowledge_source FIELDS project_id, scope;

-- ────────────────────────────────────────────────────

-- 청크 (벡터 임베딩 단위)
DEFINE TABLE knowledge_chunk SCHEMAFULL;

DEFINE FIELD id        ON knowledge_chunk TYPE string;
DEFINE FIELD source_id ON knowledge_chunk TYPE record<knowledge_source>;
DEFINE FIELD chunk_idx ON knowledge_chunk TYPE int;   -- 소스 내 순서
DEFINE FIELD content   ON knowledge_chunk TYPE string;
DEFINE FIELD embedding ON knowledge_chunk TYPE array<float>;  -- 768 or 1024dim
DEFINE FIELD embedding_model ON knowledge_chunk TYPE string;  -- 임베딩 모델 ID
DEFINE FIELD page_num  ON knowledge_chunk TYPE option<int>;
DEFINE FIELD char_start ON knowledge_chunk TYPE option<int>;
DEFINE FIELD char_end   ON knowledge_chunk TYPE option<int>;
DEFINE FIELD created_at ON knowledge_chunk TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_kc_source  ON knowledge_chunk FIELDS source_id;
DEFINE INDEX idx_kc_embed   ON knowledge_chunk FIELDS embedding HNSW DIMENSION 768
    DIST COSINE TYPE F32;

-- ────────────────────────────────────────────────────

-- GraphRAG 관계 (소스 간 연결)
DEFINE TABLE knowledge_relation SCHEMALESS;
-- source: knowledge_source
-- target: knowledge_source
-- DEFINE FIELD weight    ON knowledge_relation TYPE float;  -- 관련도 0.0~1.0
-- DEFINE FIELD rel_type  TYPE string; -- 'similar' | 'references' | 'contradicts'
```

---

## 4. Agent State & Task (에이전트)

```surql
DEFINE TABLE agent_task SCHEMAFULL;

DEFINE FIELD id           ON agent_task TYPE string;
DEFINE FIELD conversation_id ON agent_task TYPE record<conversation>;
DEFINE FIELD project_id   ON agent_task TYPE option<record<project>>;
DEFINE FIELD status       ON agent_task TYPE string DEFAULT 'pending'
    ASSERT $value IN ['pending', 'planning', 'executing', 'reviewing', 'approved', 'failed', 'cancelled'];
DEFINE FIELD mode         ON agent_task TYPE string
    ASSERT $value IN ['research', 'code', 'mlops', 'chat'];
DEFINE FIELD user_input   ON agent_task TYPE string;
DEFINE FIELD iteration    ON agent_task TYPE int DEFAULT 0;
DEFINE FIELD max_iterations ON agent_task TYPE int DEFAULT 5;
DEFINE FIELD tokens_used  ON agent_task TYPE int DEFAULT 0;
DEFINE FIELD token_budget ON agent_task TYPE int DEFAULT 8000;
DEFINE FIELD critic_score ON agent_task TYPE option<float>;
DEFINE FIELD created_at   ON agent_task TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at   ON agent_task TYPE datetime DEFAULT time::now();
DEFINE FIELD completed_at ON agent_task TYPE option<datetime>;

-- ────────────────────────────────────────────────────

-- 에이전트 상태 스냅샷 (체크포인팅)
DEFINE TABLE agent_checkpoint SCHEMAFULL;

DEFINE FIELD id        ON agent_checkpoint TYPE string;
DEFINE FIELD task_id   ON agent_checkpoint TYPE record<agent_task>;
DEFINE FIELD iteration ON agent_checkpoint TYPE int;
DEFINE FIELD state_json ON agent_checkpoint TYPE string;  -- AgentState JSON
DEFINE FIELD created_at ON agent_checkpoint TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_ckpt_task ON agent_checkpoint FIELDS task_id, iteration;

-- ────────────────────────────────────────────────────

-- 도구 호출 로그
DEFINE TABLE tool_call_log SCHEMAFULL;

DEFINE FIELD id        ON tool_call_log TYPE string;
DEFINE FIELD task_id   ON tool_call_log TYPE record<agent_task>;
DEFINE FIELD tool_name ON tool_call_log TYPE string;
DEFINE FIELD input_json  ON tool_call_log TYPE option<string>;
DEFINE FIELD output_json ON tool_call_log TYPE option<string>;
DEFINE FIELD error       ON tool_call_log TYPE option<string>;
DEFINE FIELD duration_ms ON tool_call_log TYPE option<int>;
DEFINE FIELD approved    ON tool_call_log TYPE option<bool>;  -- HITL 결과
DEFINE FIELD created_at  ON tool_call_log TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_tcl_task ON tool_call_log FIELDS task_id;
```

---

## 5. Artifact (생성물)

```surql
DEFINE TABLE artifact SCHEMAFULL;

DEFINE FIELD id          ON artifact TYPE string;
DEFINE FIELD project_id  ON artifact TYPE option<record<project>>;
DEFINE FIELD task_id     ON artifact TYPE option<record<agent_task>>;
DEFINE FIELD message_id  ON artifact TYPE option<string>;
DEFINE FIELD title       ON artifact TYPE string;
DEFINE FIELD artifact_type ON artifact TYPE string
    ASSERT $value IN ['code', 'document', 'chart', 'diagram', 'data_table', 'diff'];
DEFINE FIELD language    ON artifact TYPE option<string>;  -- 코드인 경우 언어
DEFINE FIELD content     ON artifact TYPE string;
DEFINE FIELD file_path   ON artifact TYPE option<string>;  -- 로컬 파일 연동 시
DEFINE FIELD is_pinned   ON artifact TYPE bool DEFAULT false;
DEFINE FIELD version     ON artifact TYPE int DEFAULT 1;
DEFINE FIELD parent_id   ON artifact TYPE option<string>;  -- 이전 버전 참조
DEFINE FIELD created_at  ON artifact TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at  ON artifact TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at  ON artifact TYPE option<datetime>;

DEFINE INDEX idx_art_project ON artifact FIELDS project_id;
DEFINE INDEX idx_art_task    ON artifact FIELDS task_id;
```

---

## 6. Personal Memory (장기 기억)

```surql
DEFINE TABLE memory SCHEMAFULL;

DEFINE FIELD id          ON memory TYPE string;
DEFINE FIELD project_id  ON memory TYPE option<record<project>>;
    -- null = 전역 기억, record = 프로젝트 특화 기억
DEFINE FIELD memory_type ON memory TYPE string
    ASSERT $value IN ['correction', 'style', 'preference', 'fact', 'workflow'];
DEFINE FIELD content     ON memory TYPE string;  -- 기억 내용
DEFINE FIELD source_message_id ON memory TYPE option<string>;
DEFINE FIELD confidence  ON memory TYPE float DEFAULT 1.0;  -- 0.0~1.0
DEFINE FIELD use_count   ON memory TYPE int DEFAULT 0;
DEFINE FIELD last_used   ON memory TYPE option<datetime>;
DEFINE FIELD created_at  ON memory TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at  ON memory TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_mem_project ON memory FIELDS project_id, memory_type;
```

---

## 7. Template (사용자 정의 템플릿)

```surql
DEFINE TABLE template SCHEMAFULL;

DEFINE FIELD id             ON template TYPE string;
DEFINE FIELD project_id     ON template TYPE option<record<project>>;
    -- null = 공통(Shared), record = 프로젝트 전용
DEFINE FIELD template_type  ON template TYPE string
    ASSERT $value IN ['persona', 'harness', 'orchestration'];
DEFINE FIELD name           ON template TYPE string;
DEFINE FIELD description    ON template TYPE option<string>;
DEFINE FIELD base_template  ON template TYPE option<string>;  -- 상속 원본 ID
DEFINE FIELD yaml_content   ON template TYPE string;          -- YAML 원문
DEFINE FIELD is_builtin     ON template TYPE bool DEFAULT false;  -- 기본 제공 여부
DEFINE FIELD is_shared      ON template TYPE bool DEFAULT false;  -- 공유 풀 노출
DEFINE FIELD created_at     ON template TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at     ON template TYPE datetime DEFAULT time::now();
DEFINE FIELD deleted_at     ON template TYPE option<datetime>;

DEFINE INDEX idx_tpl_type    ON template FIELDS template_type;
DEFINE INDEX idx_tpl_project ON template FIELDS project_id;
```

---

## 8. Model Registry (모델 목록)

```surql
DEFINE TABLE model_registry SCHEMAFULL;

DEFINE FIELD id           ON model_registry TYPE string;  -- 예: "qwen2.5-14b-q4"
DEFINE FIELD display_name ON model_registry TYPE string;
DEFINE FIELD provider     ON model_registry TYPE string
    ASSERT $value IN ['llama_cpp', 'ollama', 'remote_api'];
DEFINE FIELD model_path   ON model_registry TYPE option<string>;  -- 로컬 GGUF 경로
DEFINE FIELD api_endpoint ON model_registry TYPE option<string>;  -- 원격 API URL
DEFINE FIELD context_size ON model_registry TYPE int DEFAULT 8192;
DEFINE FIELD vram_required_gb ON model_registry TYPE option<float>;
DEFINE FIELD supports_vision  ON model_registry TYPE bool DEFAULT false;
DEFINE FIELD is_embedding     ON model_registry TYPE bool DEFAULT false;
DEFINE FIELD embedding_dim    ON model_registry TYPE option<int>;  -- 임베딩 차원
DEFINE FIELD tier             ON model_registry TYPE string DEFAULT 'heavy'
    ASSERT $value IN ['router', 'small', 'heavy', 'embedder', 'vision'];
DEFINE FIELD is_active    ON model_registry TYPE bool DEFAULT true;
DEFINE FIELD created_at   ON model_registry TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at   ON model_registry TYPE datetime DEFAULT time::now();
```

---

## 9. Settings (앱 설정)

```surql
DEFINE TABLE app_settings SCHEMAFULL;

-- 싱글톤 레코드 (id: "global")
DEFINE FIELD id          ON app_settings TYPE string;
DEFINE FIELD theme       ON app_settings TYPE string DEFAULT 'light'
    ASSERT $value IN ['light', 'dark', 'system'];
DEFINE FIELD language    ON app_settings TYPE string DEFAULT 'ko';
DEFINE FIELD hud_position ON app_settings TYPE string DEFAULT 'center-top';
DEFINE FIELD hud_shortcut ON app_settings TYPE string DEFAULT 'alt+space';

-- 기본 모델 설정
DEFINE FIELD default_model_small  ON app_settings TYPE option<string>;
DEFINE FIELD default_model_heavy  ON app_settings TYPE option<string>;
DEFINE FIELD default_model_embed  ON app_settings TYPE option<string>;

-- 기본 템플릿
DEFINE FIELD default_persona      ON app_settings TYPE string DEFAULT 'default';
DEFINE FIELD default_harness      ON app_settings TYPE string DEFAULT 'standard';
DEFINE FIELD default_orchestration ON app_settings TYPE string DEFAULT 'balanced';

-- 백업 설정
DEFINE FIELD backup_enabled       ON app_settings TYPE bool DEFAULT false;
DEFINE FIELD backup_interval_hours ON app_settings TYPE int DEFAULT 24;
DEFINE FIELD backup_path          ON app_settings TYPE option<string>;
DEFINE FIELD cloud_sync_enabled   ON app_settings TYPE bool DEFAULT false;
DEFINE FIELD cloud_sync_endpoint  ON app_settings TYPE option<string>;

-- 개인정보
DEFINE FIELD telemetry_enabled    ON app_settings TYPE bool DEFAULT false;

DEFINE FIELD created_at ON app_settings TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON app_settings TYPE datetime DEFAULT time::now();
```

---

## 10. 초기 데이터 (Seed)

```surql
-- 기본 앱 설정
INSERT INTO app_settings {
    id: 'global',
    theme: 'system',
    language: 'ko',
    hud_shortcut: 'alt+space'
};

-- 기본 제공 템플릿 (빌트인)
INSERT INTO template [
    { id: 'persona-default',    template_type: 'persona',        name: '기본',       is_builtin: true, yaml_content: '' },
    { id: 'persona-researcher', template_type: 'persona',        name: '연구원',     is_builtin: true, yaml_content: '' },
    { id: 'persona-coder',      template_type: 'persona',        name: '엔지니어',   is_builtin: true, yaml_content: '' },
    { id: 'persona-analyst',    template_type: 'persona',        name: '분석가',     is_builtin: true, yaml_content: '' },
    { id: 'harness-strict',     template_type: 'harness',        name: '엄격',       is_builtin: true, yaml_content: '' },
    { id: 'harness-standard',   template_type: 'harness',        name: '표준',       is_builtin: true, yaml_content: '' },
    { id: 'harness-advanced',   template_type: 'harness',        name: '고급',       is_builtin: true, yaml_content: '' },
    { id: 'orch-fast',          template_type: 'orchestration',  name: '빠름',       is_builtin: true, yaml_content: '' },
    { id: 'orch-balanced',      template_type: 'orchestration',  name: '균형',       is_builtin: true, yaml_content: '' },
    { id: 'orch-deep',          template_type: 'orchestration',  name: '정확',       is_builtin: true, yaml_content: '' }
];
```

---

## 11. 관계 다이어그램

```
project ──1:N──► conversation ──1:N──► message
   │                                      │
   ├──1:N──► knowledge_source ──1:N──► knowledge_chunk
   │              │
   │         (GraphRAG 관계)
   │         knowledge_relation
   │
   ├──1:N──► template (override)
   ├──1:N──► artifact
   ├──1:N──► memory (프로젝트 특화)
   └──1:N──► agent_task ──1:N──► agent_checkpoint
                          └──1:N──► tool_call_log

[전역]
  app_settings (싱글톤)
  model_registry
  template (is_builtin = true)
  memory (project_id = null)
  knowledge_source (scope = 'global')
```

---

*마지막 업데이트: 2026-05-13 · Schema v4.1*
*이 문서는 `docs/` 내에서만 관리되며 외부에 공개하지 않습니다.*
