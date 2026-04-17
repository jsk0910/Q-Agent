use serde::Serialize;
use futures_util::StreamExt;
use tauri::{Emitter, State, Window};
use std::sync::atomic::Ordering;
use crate::models::runner::{OllamaRunner, ModelRunner, ChatMessage, ChatOptions};
use crate::CancellationState;
use crate::harness::context_builder::AdaptiveContextBuilder;
use crate::harness::eval_loop::GeneratorEvaluatorLoop;
use crate::harness::planner::TaskPlanner;
use crate::harness::summarizer::ContextSummarizer;

// ─── Tauri Commands ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn stop_generation(cancel: State<'_, CancellationState>) -> Result<(), String> {
    cancel.0.store(true, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
pub async fn chat_with_ollama(
    window: Window,
    runner: State<'_, OllamaRunner>,
    cancel: State<'_, CancellationState>,
    messages: Vec<ChatMessage>,
    model: Option<String>,
    temperature: Option<f64>,
    system_prompt: Option<String>,
    stream: Option<bool>,
    auto_plan: Option<bool>,
    summarize_threshold: Option<f64>,
    summary: Option<String>,
) -> Result<String, String> {
    // Reset cancellation flag
    cancel.0.store(false, Ordering::SeqCst);

    let model_name = model.unwrap_or_else(|| "qwen3.5:4b".to_string());
    let temp = temperature.unwrap_or(0.7);
    let should_stream = stream.unwrap_or(true);
    let sys = system_prompt.unwrap_or_else(|| {
        "You are Q-Agent, a helpful AI research and development assistant.".to_string()
    });

    let threshold = summarize_threshold.unwrap_or(0.7);
    let mut current_summary = summary;

    // 1. Check for Summarization Trigger
    let bpe = tiktoken_rs::cl100k_base().unwrap();
    let history_tokens: usize = messages.iter().map(|m| bpe.encode_with_special_tokens(&m.content).len() + 4).sum();
    let max_tokens = 4096;

    if history_tokens > (max_tokens as f64 * threshold) as usize && messages.len() > 4 {
        let _ = window.emit("harness_status", "Compressing conversation history...");
        let summarizer = ContextSummarizer::new();
        // Summarize old messages (excluding the last 2 for immediate context)
        let to_summarize = &messages[..messages.len() - 2];
        match summarizer.summarize(runner.inner(), &model_name, to_summarize).await {
            Ok(sum) => {
                current_summary = Some(sum);
                let _ = window.emit("harness_summary_updated", current_summary.clone());
            }
            Err(e) => eprintln!("Summarization failed: {}", e),
        }
    }

    // 2. Planning Phase
    let is_small_model = model_name.to_lowercase().contains("1.5b") || 
                         model_name.to_lowercase().contains("3b") || 
                         model_name.to_lowercase().contains("4b") || 
                         model_name.to_lowercase().contains("7b") || 
                         model_name.to_lowercase().contains("8b");

    if auto_plan.unwrap_or(false) && is_small_model {
        let last_user_msg = messages.last().map(|m| m.content.clone()).unwrap_or_default();
        if last_user_msg.len() > 50 { // Only plan for long/complex queries
            let _ = window.emit("harness_status", "Planning steps for complex task...");
            let planner = TaskPlanner::new();
            if let Ok(plan) = planner.plan(runner.inner(), &model_name, &last_user_msg).await {
                let _ = window.emit("harness_plan_created", plan.clone());
                
                let mut results: Vec<String> = Vec::new();
                for (i, step) in plan.steps.iter().enumerate() {
                    // Check for cancellation before each step
                    if cancel.0.load(Ordering::SeqCst) {
                        let _ = window.emit("chat_token", "\n\n⚠️ *Generation stopped.*");
                        let _ = window.emit("chat_finished", ());
                        return Ok("Generation stopped.".to_string());
                    }

                    let _ = window.emit("harness_status", format!("Step {}/{}: {}", i+1, plan.steps.len(), step.title));
                    
                    let step_prompt = format!(
                        "Context: {}\nTask: {}\nPrevious Results: {:?}\n\nExecute this specific step and provide the result.",
                        sys, step.task, results
                    );
                    
                    let step_msg = vec![ChatMessage { role: "user".to_string(), content: step_prompt }];
                    if let Ok(res) = runner.inner().chat(&model_name, step_msg, ChatOptions { temperature: 0.3 }).await {
                        // Check for cancellation after LLM call
                        if cancel.0.load(Ordering::SeqCst) {
                             let _ = window.emit("chat_token", "\n\n⚠️ *Generation stopped.*");
                             let _ = window.emit("chat_finished", ());
                             return Ok("Generation stopped.".to_string());
                        }
                        results.push(res.clone());
                        let _ = window.emit("harness_results_updated", results.clone());
                    }
                }
                
                // Final Synthesis
                if cancel.0.load(Ordering::SeqCst) { return Ok("Generation stopped.".to_string()); }
                let _ = window.emit("harness_status", "Synthesizing final answer...");
                let final_prompt = format!(
                    "Original Request: {}\nSteps Taken: {:?}\nExecution Results: {:?}\n\nPlease provide a final comprehensive answer summarized from the results above.",
                    last_user_msg, plan.steps, results
                );
                let final_msg = vec![ChatMessage { role: "user".to_string(), content: final_prompt }];
                let final_ans = runner.inner().chat(&model_name, final_msg, ChatOptions { temperature: temp }).await.map_err(|e| e.to_string())?;
                
                let _ = window.emit("chat_finished", ());
                return Ok(final_ans);
            }
        }
    }

    // 3. Regular Execution (Using optimized context)
    let builder = AdaptiveContextBuilder::new(max_tokens);
    let final_messages = builder.build(&sys, &messages, &[], current_summary.as_deref());

    if should_stream {
        // For streaming, we still use the direct client for now because the trait returns String.
        // In a future phase of "Execution Separation", the trait will support streaming.
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "model": model_name,
            "messages": final_messages,
            "stream": true,
            "options": { "temperature": temp }
        });

        let res = client
            .post("http://localhost:11434/api/chat")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("API request failed: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Ollama returned {}", res.status()));
        }

        let mut byte_stream = res.bytes_stream();
        let mut buffer = Vec::new();
        let mut full_response = String::new();

        while let Some(chunk_res) = byte_stream.next().await {
            // Check for cancellation
            if cancel.0.load(Ordering::SeqCst) {
                let _ = window.emit("chat_token", "\n\n⚠️ *Generation stopped.*");
                break;
            }

            let chunk = chunk_res.map_err(|e| format!("Stream error: {}", e))?;
            buffer.extend_from_slice(&chunk);

            while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                let line_bytes: Vec<u8> = buffer.drain(..=pos).collect();
                let line_str = String::from_utf8_lossy(&line_bytes);
                let trimmed = line_str.trim();
                
                if trimmed.is_empty() { continue; }

                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(trimmed) {
                    if let Some(content) = parsed["message"]["content"].as_str() {
                        full_response.push_str(content);
                        let _ = window.emit("chat_token", content);
                    }
                }
            }
        }
        
        let _ = window.emit("chat_finished", ());
        Ok(full_response)
    } else {
        runner.chat(&model_name, final_messages, ChatOptions { temperature: temp })
            .await
            .map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn list_models(runner: State<'_, OllamaRunner>) -> Result<Vec<String>, String> {
    runner.list_models().await.map_err(|e| e.to_string())
}

/// Runs the Evaluator-Critic pass on a draft answer.
/// Returns the original draft if it passes, or a corrected version.
#[tauri::command]
pub async fn evaluate_response(
    window: Window,
    runner: State<'_, OllamaRunner>,
    cancel: State<'_, CancellationState>,
    draft: String,
    prompt: String,
    model: Option<String>,
) -> Result<String, String> {
    // Check for cancellation before starting
    if cancel.0.load(Ordering::SeqCst) {
        return Ok(draft);
    }

    let model_name = model.unwrap_or_else(|| "qwen3.5:4b".to_string());

    let eval_loop = GeneratorEvaluatorLoop::new();
    let result = eval_loop.evaluate_and_refine(
        runner.inner(),
        &window,
        cancel.inner(),
        &draft,
        &prompt,
        &model_name,
        3
    ).await;

    match result {
        Ok(refined) => Ok(refined),
        Err(e) => {
            eprintln!("Eval loop error: {}", e);
            Ok(draft) // Fallback to original draft on system error
        }
    }
}

#[derive(Serialize, Clone)]
pub struct PullProgress {
    pub status: String,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
    pub percentage: Option<f64>,
}

#[tauri::command]
pub async fn pull_model(window: Window, cancel: State<'_, CancellationState>, model: String) -> Result<(), String> {
    // Reset cancellation flag
    cancel.0.store(false, Ordering::SeqCst);

    let client = reqwest::Client::new();
    let payload = serde_json::json!({ "name": model });

    let res = client
        .post("http://localhost:11434/api/pull")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to start pull: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Ollama pull error: {}", res.status()));
    }

    let mut byte_stream = res.bytes_stream();
    let mut buffer = Vec::new();

    while let Some(chunk_res) = byte_stream.next().await {
        // Check for cancellation
        if cancel.0.load(Ordering::SeqCst) {
            break;
        }

        let chunk = chunk_res.map_err(|e| format!("Stream error: {}", e))?;
        buffer.extend_from_slice(&chunk);

        while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
            let line_bytes: Vec<u8> = buffer.drain(..=pos).collect();
            let line_str = String::from_utf8_lossy(&line_bytes);
            let trimmed = line_str.trim();
            
            if trimmed.is_empty() { continue; }

            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                let status = val["status"].as_str().unwrap_or("unknown").to_string();
                let digest = val["digest"].as_str().map(|s| s.to_string());
                let total = val["total"].as_u64();
                let completed = val["completed"].as_u64();
                
                let percentage = if let (Some(t), Some(c)) = (total, completed) {
                    if t > 0 { Some((c as f64 / t as f64) * 100.0) } else { None }
                } else {
                    None
                };

                let progress = PullProgress {
                    status,
                    digest,
                    total,
                    completed,
                    percentage,
                };

                let _ = window.emit("pull_progress", progress);
            }
        }
    }

    Ok(())
}
