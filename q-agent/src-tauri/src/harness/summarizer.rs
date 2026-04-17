use crate::models::runner::{ChatMessage, ChatOptions, ModelRunner};
use std::error::Error;

pub struct ContextSummarizer;

impl ContextSummarizer {
    pub fn new() -> Self {
        Self
    }

    /// Summarizes a set of messages into a single concise paragraph.
    pub async fn summarize(
        &self,
        runner: &dyn ModelRunner,
        model_name: &str,
        messages: &[ChatMessage],
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        if messages.is_empty() {
            return Ok(String::new());
        }

        let mut conversation_text = String::new();
        for msg in messages {
            conversation_text.push_str(&format!("{}: {}\n", msg.role, msg.content));
        }

        let prompt = format!(
            "You are a context compression engine. Your goal is to summarize the following conversation history \
            into a concise, dense paragraph that preserves all key facts, decisions, and user preferences. \
            Do not include any preamble. Output ONLY the summary.\n\n\
            CONVERSATION:\n{}",
            conversation_text
        );

        let sum_messages = vec![ChatMessage {
            role: "user".to_string(),
            content: prompt,
        }];

        let summary = runner.chat(model_name, sum_messages, ChatOptions { temperature: 0.3 }).await?;
        
        Ok(summary.trim().to_string())
    }
}
