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
