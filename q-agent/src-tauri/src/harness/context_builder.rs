use crate::models::runner::ChatMessage;
use tiktoken_rs::cl100k_base;

pub struct AdaptiveContextBuilder {
    pub max_tokens: usize,
}

impl AdaptiveContextBuilder {
    pub fn new(max_tokens: usize) -> Self {
        Self { max_tokens }
    }

    /// Truncates and builds the optimal context within the token budget using precise token counting.
    pub fn build(
        &self,
        system_guidelines: &str,
        chat_history: &[ChatMessage],
        rag_chunks: &[String],
        summary: Option<&str>,
    ) -> Vec<ChatMessage> {
        let bpe = cl100k_base().unwrap();
        let mut final_context = Vec::new();
        let mut current_tokens = 0;

        // 1. Always prioritize System Guidelines
        final_context.push(ChatMessage {
            role: "system".to_string(),
            content: system_guidelines.to_string(),
        });
        current_tokens += bpe.encode_with_special_tokens(system_guidelines).len() + 4;

        // 2. Add Summary if exists (High priority)
        if let Some(sum_text) = summary {
            let content = format!("Relevant Background Summary:\n{}", sum_text);
            current_tokens += bpe.encode_with_special_tokens(&content).len() + 4;
            final_context.push(ChatMessage {
                role: "system".to_string(),
                content,
            });
        }

        // 3. Add RAG chunks (Up to 40% of the remaining budget after guidelines)
        let mut compiled_chunks = String::new();
        let rag_budget = self.max_tokens / 2;
        let mut rag_tokens = 0;

        for chunk in rag_chunks {
            let chunk_tokens = bpe.encode_with_special_tokens(chunk).len() + 2;
            if current_tokens + rag_tokens + chunk_tokens > rag_budget {
                break; 
            }
            compiled_chunks.push_str(chunk);
            compiled_chunks.push_str("\n---\n");
            rag_tokens += chunk_tokens;
        }

        if !compiled_chunks.is_empty() {
            let content = format!("Context Documents:\n{}", compiled_chunks);
            current_tokens += bpe.encode_with_special_tokens(&content).len() + 4;
            final_context.push(ChatMessage {
                role: "system".to_string(),
                content,
            });
        }

        // 3. Add History (Recent first, within remaining budget)
        let mut history_to_add = Vec::new();
        for msg in chat_history.iter().rev() {
            let msg_tokens = bpe.encode_with_special_tokens(&msg.content).len() + 4;
            if current_tokens + msg_tokens > self.max_tokens {
                break;
            }
            history_to_add.push(msg.clone());
            current_tokens += msg_tokens;
        }

        // Reverse back to chronological order
        history_to_add.reverse();
        final_context.extend(history_to_add);

        final_context
    }
}
