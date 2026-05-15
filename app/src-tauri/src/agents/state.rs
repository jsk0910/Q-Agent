use serde::{Deserialize, Serialize};

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
