use crate::models::runner::{ChatMessage, ChatOptions, ModelRunner};
use crate::CancellationState;
use tauri::{Window, Emitter};
use serde::Serialize;
use std::sync::atomic::Ordering;

#[derive(Serialize, Clone)]
pub struct EvalProgress {
    pub iteration: u8,
    pub max_iterations: u8,
    pub status: String,
}

pub struct GeneratorEvaluatorLoop;

impl GeneratorEvaluatorLoop {
    pub fn new() -> Self {
        Self
    }

    /// Evaluates a drafted response and attempts to refine it iteratively if issues are found.
    pub async fn evaluate_and_refine(
        &self,
        runner: &dyn ModelRunner,
        window: &Window,
        cancel: &CancellationState,
        draft: &str,
        original_prompt: &str,
        model_name: &str,
        max_retries: u8,
    ) -> Result<String, String> {
        let mut current_output = draft.to_string();
        
        for i in 1..=max_retries {
            // Check for cancellation
            if cancel.0.load(Ordering::SeqCst) {
                return Ok(current_output); // Return what we have so far
            }

            let _ = window.emit("eval_progress", EvalProgress {
                iteration: i,
                max_iterations: max_retries,
                status: format!("Critiquing and refining (Pass {})...", i),
            });

            let eval_prompt = format!(
                "You are a strict Critic evaluating an assistant's response. \
                The User asked: '{}'\n\
                The Assistant drafted this response: '{}'\n\n\
                Check the response for accuracy, logical errors, hallucination, and formatting. \
                If it is high quality and accurate, reply exactly with the word 'PASS'. \
                If it needs improvement, reply with the full corrected and improved response and nothing else. \
                Do not include any preamble or explanation in your response.",
                original_prompt, current_output
            );

            let messages = vec![ChatMessage {
                role: "user".to_string(),
                content: eval_prompt,
            }];

            let result = runner.chat(model_name, messages, ChatOptions { temperature: 0.1 }).await;

            match result {
                Ok(evaluation) => {
                    let ev = evaluation.trim();
                    if ev.to_uppercase().contains("PASS") && ev.len() < 10 {
                        return Ok(current_output);
                    } else {
                        current_output = ev.to_string();
                    }
                }
                Err(e) => return Err(e.to_string()),
            }
        }

        Ok(current_output)
    }
}
