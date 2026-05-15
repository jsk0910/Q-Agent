use super::state::{AgentState, AgentStatus};
use super::roles;
use crate::models::{ModelRequest, ModelRunner, MockLlamaRunner};
use anyhow::Result;

pub struct WorkflowGraph {
    runner: MockLlamaRunner,
}

impl Default for WorkflowGraph {
    fn default() -> Self {
        Self { runner: MockLlamaRunner }
    }
}

impl WorkflowGraph {
    pub async fn run(&self, mut state: AgentState, db: &crate::db::Database) -> Result<AgentState> {
        tracing::info!("Starting WorkflowGraph for task: {}", state.task_id);
        
        // 초기 상태 스냅샷 저장
        let _ = db.save_agent_state(&state).await;
        
        while state.status != AgentStatus::Finished {
            if state.iteration >= state.max_iterations {
                state.status = AgentStatus::Error("Max iterations reached".to_string());
                let _ = db.save_agent_state(&state).await;
                break;
            }
            if state.tokens_used >= state.token_budget {
                state.status = AgentStatus::Error("Token budget exceeded".to_string());
                let _ = db.save_agent_state(&state).await;
                break;
            }

            match state.status {
                AgentStatus::Idle => {
                    state.status = AgentStatus::Planning;
                }
                AgentStatus::Planning => {
                    tracing::info!("Executing Planner Node...");
                    let task_desc = state.messages.last().map(|m| m.content.clone()).unwrap_or_default();
                    let prompt = roles::get_planner_prompt(&task_desc);
                    
                    let req = ModelRequest { prompt, max_tokens: 200 };
                    let response = self.runner.generate(req).await?;
                    state.plan = Some(response);
                    state.status = AgentStatus::Executing;
                }
                AgentStatus::Executing => {
                    tracing::info!("Executing Coder Node...");
                    let plan = state.plan.clone().unwrap_or_default();
                    let prompt = roles::get_coder_prompt(&plan);
                    
                    let req = ModelRequest { prompt, max_tokens: 500 };
                    let response = self.runner.generate(req).await?;
                    state.final_answer = Some(response);
                    state.status = AgentStatus::Reviewing;
                }
                AgentStatus::Reviewing => {
                    tracing::info!("Executing Critic Node...");
                    state.iteration += 1;
                    
                    let answer = state.final_answer.clone().unwrap_or_default();
                    let prompt = roles::get_critic_prompt(&answer);
                    let req = ModelRequest { prompt, max_tokens: 50 };
                    let _ = self.runner.generate(req).await?; 
                    
                    // Mock review logic: first time fail, second time pass
                    if state.iteration < 2 {
                        state.critic_score = Some(0.6);
                        tracing::info!("Critic rejected (score 0.6). Retrying Executing phase...");
                        state.status = AgentStatus::Executing;
                    } else {
                        state.critic_score = Some(0.9);
                        tracing::info!("Critic approved (score 0.9). Finishing workflow.");
                        state.status = AgentStatus::Finished;
                    }
                }
                AgentStatus::Finished | AgentStatus::Error(_) => break,
            }
            
            // 매 노드 라우팅이 끝날 때마다 스냅샷 저장 (Checkpointing)
            if let Err(e) = db.save_agent_state(&state).await {
                tracing::warn!("Failed to checkpoint agent state: {}", e);
            }
        }
        
        tracing::info!("Workflow finished with status: {:?}", state.status);
        Ok(state)
    }
}
