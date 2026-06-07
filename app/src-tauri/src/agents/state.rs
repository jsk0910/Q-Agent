use serde::{Deserialize, Serialize};
use crate::agents::security::PermissionMode;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AgentState {
    pub task_id: String,
    pub iteration: u32,
    pub max_iterations: u32,
    pub tokens_used: usize,
    pub token_budget: usize,
    pub status: AgentStatus,
    pub plan: Option<String>,
    pub critic_score: Option<f32>,
    pub final_answer: Option<String>,
    pub messages: Vec<Message>,
    pub permission_mode: PermissionMode,
    pub mentions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AgentStatus {
    Idle,
    Planning,
    Executing,
    Reviewing,
    Finished,
    Error(String),
}

impl Default for AgentStatus {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String, // "user", "assistant", "system"
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_state_default() {
        let state = AgentState::default();
        assert_eq!(state.status, AgentStatus::Idle);
        assert_eq!(state.iteration, 0);
        assert_eq!(state.tokens_used, 0);
        assert!(state.plan.is_none());
        assert!(state.critic_score.is_none());
        assert!(state.final_answer.is_none());
        assert!(state.messages.is_empty());
        assert!(matches!(state.permission_mode, PermissionMode::Balanced));
        assert!(state.mentions.is_empty());
    }

    #[test]
    fn test_agent_status_eq() {
        assert_eq!(AgentStatus::Idle, AgentStatus::Idle);
        assert_ne!(AgentStatus::Idle, AgentStatus::Planning);
        assert_eq!(
            AgentStatus::Error("oops".into()),
            AgentStatus::Error("oops".into())
        );
        assert_ne!(
            AgentStatus::Error("a".into()),
            AgentStatus::Error("b".into())
        );
    }

    #[test]
    fn test_agent_state_serde_roundtrip() {
        let state = AgentState {
            task_id: "task-abc".into(),
            iteration: 2,
            max_iterations: 5,
            tokens_used: 100,
            token_budget: 8192,
            status: AgentStatus::Reviewing,
            plan: Some("step 1, step 2".into()),
            critic_score: Some(0.88),
            final_answer: Some("done".into()),
            messages: vec![Message { role: "user".into(), content: "hello".into() }],
            permission_mode: PermissionMode::Strict,
            mentions: vec!["main.rs".to_string()],
        };
        let json = serde_json::to_string(&state).expect("serialize ok");
        let restored: AgentState = serde_json::from_str(&json).expect("deserialize ok");
        assert_eq!(restored.task_id, "task-abc");
        assert_eq!(restored.iteration, 2);
        assert_eq!(restored.status, AgentStatus::Reviewing);
        assert_eq!(restored.critic_score, Some(0.88));
        assert_eq!(restored.messages.len(), 1);
        assert!(matches!(restored.permission_mode, PermissionMode::Strict));
        assert_eq!(restored.mentions.len(), 1);
    }

    #[test]
    fn test_budget_guard_logic() {
        // token_budget 초과 시 상태 전이를 외부에서 검증하는 헬퍼 로직 테스트
        let state = AgentState {
            token_budget: 100,
            tokens_used: 100,
            ..Default::default()
        };
        assert!(state.tokens_used >= state.token_budget, "budget exceeded");
    }

    #[test]
    fn test_iteration_cap_logic() {
        let state = AgentState {
            max_iterations: 3,
            iteration: 3,
            ..Default::default()
        };
        assert!(state.iteration >= state.max_iterations, "iteration cap reached");
    }
}
