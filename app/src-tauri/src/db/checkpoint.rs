use anyhow::Result;
use super::Database;
use crate::agents::state::AgentState;

impl Database {
    /// AgentState 스냅샷을 SurrealDB에 저장/업데이트합니다.
    pub async fn save_agent_state(&self, state: &AgentState) -> Result<()> {
        let _result: Option<AgentState> = self.client
            .upsert(("agent_state", state.task_id.clone()))
            .content(state.clone())
            .await?;
        Ok(())
    }

    /// Task ID를 통해 마지막 AgentState 스냅샷을 불러옵니다.
    pub async fn get_agent_state(&self, task_id: &str) -> Result<Option<AgentState>> {
        let result: Option<AgentState> = self.client
            .select(("agent_state", task_id))
            .await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::state::{AgentState, AgentStatus, Message};

    async fn setup_db() -> Database {
        Database::init("memory").await.expect("In-memory DB should init")
    }

    #[tokio::test]
    async fn test_save_and_get_agent_state() {
        let db = setup_db().await;
        let state = AgentState {
            task_id: "task-chk-1".into(),
            iteration: 1,
            max_iterations: 5,
            tokens_used: 200,
            token_budget: 8192,
            status: AgentStatus::Planning,
            plan: Some("Plan step A".into()),
            critic_score: None,
            final_answer: None,
            messages: vec![Message { role: "user".into(), content: "hi".into() }],
        };
        db.save_agent_state(&state).await.unwrap();
        let loaded = db.get_agent_state("task-chk-1").await.unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.task_id, "task-chk-1");
        assert_eq!(loaded.status, AgentStatus::Planning);
        assert_eq!(loaded.plan, Some("Plan step A".into()));
        assert_eq!(loaded.iteration, 1);
    }

    #[tokio::test]
    async fn test_upsert_updates_existing_state() {
        let db = setup_db().await;
        let mut state = AgentState {
            task_id: "task-chk-2".into(),
            status: AgentStatus::Planning,
            ..Default::default()
        };
        db.save_agent_state(&state).await.unwrap();

        // 상태를 Executing으로 전이 후 다시 저장 (upsert)
        state.status = AgentStatus::Executing;
        state.iteration = 1;
        db.save_agent_state(&state).await.unwrap();

        let loaded = db.get_agent_state("task-chk-2").await.unwrap().unwrap();
        assert_eq!(loaded.status, AgentStatus::Executing);
        assert_eq!(loaded.iteration, 1);
    }

    #[tokio::test]
    async fn test_get_agent_state_not_found() {
        let db = setup_db().await;
        let result = db.get_agent_state("does-not-exist").await.unwrap();
        assert!(result.is_none());
    }
}
