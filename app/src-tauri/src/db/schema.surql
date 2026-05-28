-- Project (프로젝트)
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

-- Conversation (대화)
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

-- Message
DEFINE TABLE message SCHEMAFULL;
DEFINE FIELD id              ON message TYPE string;
DEFINE FIELD conversation_id ON message TYPE record<conversation>;
DEFINE FIELD role            ON message TYPE string  -- 'user' | 'assistant' | 'system'
    ASSERT $value IN ['user', 'assistant', 'system'];
DEFINE FIELD content         ON message TYPE string;
DEFINE FIELD citations       ON message TYPE array<object>;
DEFINE FIELD thought_trace   ON message TYPE array<object> DEFAULT [];
DEFINE FIELD artifact_ids    ON message TYPE array<string> DEFAULT [];
DEFINE FIELD model_used      ON message TYPE option<string>;
DEFINE FIELD tokens_used     ON message TYPE option<int>;
DEFINE FIELD duration_ms     ON message TYPE option<int>;
DEFINE FIELD created_at      ON message TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_msg_conv ON message FIELDS conversation_id;

-- Artifact (결과물)
DEFINE TABLE artifact SCHEMAFULL;
DEFINE FIELD id              ON artifact TYPE string;
DEFINE FIELD conversation_id ON artifact TYPE record<conversation>;
DEFINE FIELD type            ON artifact TYPE string  -- 'code' | 'doc' | 'diagram'
    ASSERT $value IN ['code', 'doc', 'diagram'];
DEFINE FIELD name            ON artifact TYPE string;
DEFINE FIELD content         ON artifact TYPE string;
DEFINE FIELD language        ON artifact TYPE option<string>;
DEFINE FIELD path            ON artifact TYPE option<string>;  -- 로컬 저장 경로
DEFINE FIELD created_at      ON artifact TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at      ON artifact TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_art_conv ON artifact FIELDS conversation_id;

-- Knowledge (지식 베이스)
DEFINE TABLE knowledge SCHEMAFULL;
DEFINE FIELD id         ON knowledge TYPE string;
DEFINE FIELD project_id ON knowledge TYPE record<project>;
DEFINE FIELD source     ON knowledge TYPE string;  -- 파일명 or URL
DEFINE FIELD type       ON knowledge TYPE string  -- 'file' | 'web' | 'graph'
    ASSERT $value IN ['file', 'web', 'graph'];
DEFINE FIELD content    ON knowledge TYPE string;  -- 원문 or 요약
DEFINE FIELD embedding  ON knowledge TYPE option<array<float>>;  -- 벡터
DEFINE FIELD metadata   ON knowledge TYPE object;
DEFINE FIELD created_at ON knowledge TYPE datetime DEFAULT time::now();
DEFINE INDEX idx_knw_project ON knowledge FIELDS project_id;

-- Settings (앱 설정)
DEFINE TABLE app_settings SCHEMAFULL;
-- ... (rest of the file remains same)

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

-- 초기 데이터 (Seed)
INSERT INTO app_settings {
    id: 'global',
    theme: 'system',
    language: 'ko',
    hud_shortcut: 'alt+space'
};
