use crate::models::runner::{ChatMessage, ChatOptions, ModelRunner};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlanStep {
    pub title: String,
    pub task: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecutionPlan {
    pub steps: Vec<PlanStep>,
}

pub struct TaskPlanner;

impl TaskPlanner {
    pub fn new() -> Self {
        Self
    }

    /// Analyzes a prompt and generates a structured plan.
    pub async fn plan(
        &self,
        runner: &dyn ModelRunner,
        model_name: &str,
        user_prompt: &str,
    ) -> Result<ExecutionPlan, Box<dyn Error + Send + Sync>> {
        let system_msg = "You are a master strategist. Your task is to break down complex user requests into a list of simple, executable steps. \
            Output ONLY valid JSON in the following format: {\"steps\": [{\"title\": \"Step Title\", \"task\": \"Specific instruction for this step\"}]}. \
            Do not include any preamble or extra text.";

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_msg.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: format!("Break down this task: {}", user_prompt),
            },
        ];

        let response = runner.chat(model_name, messages, ChatOptions { temperature: 0.2 }).await?;
        
        // Clean JSON in case of markdown blocks
        let clean_json = response.trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let plan: ExecutionPlan = serde_json::from_str(clean_json)?;
        Ok(plan)
    }
}
