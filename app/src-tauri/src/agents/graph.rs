use super::state::{AgentState, AgentStatus};
use super::roles;
use crate::models::{ModelRequest, ModelRunner, MockLlamaRunner};
use anyhow::Result;

use std::sync::Arc;

pub struct WorkflowGraph {
    runner: Arc<dyn ModelRunner>,
}

impl Default for WorkflowGraph {
    fn default() -> Self {
        Self { runner: Arc::new(crate::models::RealLlamaRunner::new("http://localhost:8080/v1".to_string(), "qwen2.5-coder-7b".to_string())) }
    }
}

impl WorkflowGraph {
    pub fn new(runner: Arc<dyn ModelRunner>) -> Self {
        Self { runner }
    }
    pub async fn run(
        &self, 
        mut state: AgentState, 
        db: &crate::db::Database,
        app_handle: &tauri::AppHandle,
        hitl_manager: &crate::agents::hitl::HitlManager
    ) -> Result<AgentState> {
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
                    
                    let mut context = String::new();
                    for mention in &state.mentions {
                        let path = std::path::Path::new(mention);
                        if let Ok(content) = std::fs::read_to_string(path) {
                            context.push_str(&format!("--- FILE: {} ---\n{}\n\n", mention, content));
                        } else {
                            context.push_str(&format!("--- FILE: {} ---\n[File not found or unreadable]\n\n", mention));
                        }
                    }
                    
                    let prompt = roles::get_planner_prompt(&task_desc, &context);
                    
                    let req = ModelRequest { prompt, max_tokens: 200 };
                    let response = self.runner.generate(req).await?;
                    state.plan = Some(response);
                    state.status = AgentStatus::Executing;
                }
                AgentStatus::Executing => {
                    tracing::info!("Executing Coder Node...");
                    
                    // --- Permission Orchestrator Mock Simulation ---
                    use crate::agents::security::PermissionOrchestrator;
                    let orchestrator = PermissionOrchestrator::new(state.permission_mode.clone());
                    
                    // Simulate a dangerous tool call "fs_write"
                    let simulated_args = serde_json::json!({"path": "/system/config"});
                    // Evaluate tool call
                    let risk_report = orchestrator.evaluate_tool_call(
                        "fs_write", 
                        &simulated_args, 
                        None::<&crate::models::MockLlamaRunner>
                    ).await.unwrap_or_else(|_| crate::agents::security::RiskReport {
                        score: 0.0,
                        reason: "Error".to_string(),
                        approval_required: false
                    });

                    if risk_report.approval_required {
                        use tauri::Emitter;
                        tracing::warn!("[Permission Denied or Required] Tool: fs_write, Mode: {:?}, Score: {}", state.permission_mode, risk_report.score);
                        
                        let hitl_id = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0).to_string();
                        let (tx, rx) = tokio::sync::oneshot::channel();
                        
                        hitl_manager.add_request(hitl_id.clone(), tx).await;
                        
                        let payload = crate::agents::hitl::HitlRequestPayload {
                            id: hitl_id,
                            action_type: "fs_write".to_string(),
                            description: "Writing to /system/config".to_string(),
                            command: None,
                            path: Some("/system/config".to_string()),
                            risk_score: Some(risk_report.score),
                        };
                        
                        // Emit to frontend
                        let _ = app_handle.emit("hitl_request", payload);
                        
                        // Wait for response
                        tracing::info!("Paused. Waiting for HITL response...");
                        let approved = rx.await.unwrap_or(false);
                        
                        if approved {
                            tracing::info!("HITL Approved!");
                        } else {
                            tracing::warn!("HITL Rejected!");
                            state.final_answer = Some(format!(
                                "[Permission Alert] I tried to use a tool (fs_write), but under your {:?} permission mode, it requires approval (Risk Score: {}), and the user rejected it. I will not proceed further.", 
                                state.permission_mode, risk_report.score
                            ));
                            state.status = AgentStatus::Finished;
                            continue;
                        }
                    } else {
                        tracing::info!("[Permission Auto-Approved] Tool: fs_write, Mode: {:?}, Score: {}", state.permission_mode, risk_report.score);
                    }
                    // ------------------------------------------------

                    let plan = state.plan.clone().unwrap_or_default();
                    
                    let mut context = String::new();
                    for mention in &state.mentions {
                        let path = std::path::Path::new(mention);
                        if let Ok(content) = std::fs::read_to_string(path) {
                            context.push_str(&format!("--- FILE: {} ---\n{}\n\n", mention, content));
                        } else {
                            context.push_str(&format!("--- FILE: {} ---\n[File not found or unreadable]\n\n", mention));
                        }
                    }
                    
                    let prompt = roles::get_coder_prompt(&plan, &context);
                    
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
                    let critic_response = self.runner.generate(req).await?; 
                    // Parse Critic score
                    let mut score = 0.5; // Default if parsing fails
                    if let Some(idx) = critic_response.find("SCORE:") {
                        let score_str = &critic_response[idx + 6..].trim();
                        let score_str = score_str.split_whitespace().next().unwrap_or("0.5");
                        if let Ok(s) = score_str.parse::<f32>() {
                            score = s;
                        }
                    }

                    state.critic_score = Some(score);
                    if score < 0.85 {
                        tracing::info!("Critic rejected (score {}). Retrying Executing phase...", score);
                        state.status = AgentStatus::Executing;
                    } else {
                        tracing::info!("Critic approved (score {}). Finishing workflow.", score);
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
