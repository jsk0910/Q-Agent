use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Thing,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub persona_template: String,
    pub harness_template: String,
    pub orchestration_template: String,
    pub model_small: Option<String>,
    pub model_heavy: Option<String>,
    pub max_iterations: i32,
    pub token_budget: i32,
    pub security_level: i32,
    pub is_active: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Thing,
    pub project_id: Option<Thing>,
    pub title: String,
    pub mode: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Thing,
    pub conversation_id: Thing,
    pub role: String,
    pub content: String,
    /// 인용 목록 — RAG 검색 출처 (기본값: [])
    #[serde(default)]
    pub citations: Vec<serde_json::Value>,
    /// 사고 과정 추적 — Planner/Critic 단계별 로그 (기본값: [])
    #[serde(default)]
    pub thought_trace: Vec<serde_json::Value>,
    /// 연결된 Artifact ID 목록 (기본값: [])
    #[serde(default)]
    pub artifact_ids: Vec<String>,
    /// 응답 생성에 사용된 모델 ID
    pub model_used: Option<String>,
    /// 생성에 사용된 토큰 수
    pub tokens_used: Option<i64>,
    /// 응답 생성 소요 시간 (ms)
    pub duration_ms: Option<i64>,
    pub created_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: Thing,
    pub conversation_id: Thing,
    pub type_name: String, // 'type' is a reserved keyword
    pub name: String,
    pub content: String,
    pub language: Option<String>,
    pub path: Option<String>,
    pub created_at: Datetime,
}
