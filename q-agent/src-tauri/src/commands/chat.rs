use serde::{Deserialize, Serialize};
use futures_util::StreamExt;
use tauri::{Emitter, State, Window};
use crate::models::runner::{OllamaRunner, ModelRunner, ChatMessage, ChatOptions};

// ─── Tauri Commands ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn chat_with_ollama(
    window: Window,
    runner: State<'_, OllamaRunner>,
    messages: Vec<ChatMessage>,
    model: Option<String>,
    temperature: Option<f64>,
    system_prompt: Option<String>,
    stream: Option<bool>,
) -> Result<String, String> {
    let model_name = model.unwrap_or_else(|| "qwen3.5:4b".to_string());
    let temp = temperature.unwrap_or(0.7);
    let should_stream = stream.unwrap_or(true);
    let sys = system_prompt.unwrap_or_else(|| {
        "You are Q-Agent, a helpful AI research and development assistant.".to_string()
    });

    let mut final_messages = vec![ChatMessage {
        role: "system".to_string(),
        content: sys,
    }];
    final_messages.extend(messages);

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

#[derive(Serialize, Clone)]
pub struct PullProgress {
    pub status: String,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
    pub percentage: Option<f64>,
}

#[tauri::command]
pub async fn pull_model(window: Window, model: String) -> Result<(), String> {
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
