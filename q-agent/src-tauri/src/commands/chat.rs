use serde::{Deserialize, Serialize};
use futures_util::StreamExt;
use tauri::Emitter;

// ─── Ollama Request/Response Types ───────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f64,
}

#[derive(Deserialize)]
struct OllamaChatMessage {
    content: String,
}

#[derive(Deserialize)]
struct OllamaChatStreamResponse {
    message: Option<OllamaChatMessage>,
}

// ─── Tauri Commands ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn chat_with_ollama(
    window: tauri::Window,
    messages: Vec<OllamaMessage>,
    model: Option<String>,
    temperature: Option<f64>,
    system_prompt: Option<String>,
    stream: Option<bool>,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    let model_name = model.unwrap_or_else(|| "qwen3.5:4b".to_string());
    let temp = temperature.unwrap_or(0.7);
    let should_stream = stream.unwrap_or(true);
    let sys = system_prompt.unwrap_or_else(|| {
        "You are Q-Agent, a helpful AI research and development assistant.".to_string()
    });

    let mut final_messages = vec![OllamaMessage {
        role: "system".to_string(),
        content: sys,
    }];
    final_messages.extend(messages);

    let payload = OllamaChatRequest {
        model: model_name,
        messages: final_messages,
        stream: should_stream,
        options: OllamaOptions { temperature: temp },
    };

    let res = client
        .post("http://localhost:11434/api/chat")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("API request failed: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(format!("Ollama returned {}: {}", status, body));
    }

    if should_stream {
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
                
                if trimmed.is_empty() {
                    continue;
                }

                if let Ok(parsed) = serde_json::from_str::<OllamaChatStreamResponse>(trimmed) {
                    if let Some(msg) = parsed.message {
                        full_response.push_str(&msg.content);
                        // Emit token to frontend
                        let _ = window.emit("chat_token", &msg.content);
                    }
                }
            }
        }
        
        // Signal stream finished
        let _ = window.emit("chat_finished", ());
        
        Ok(full_response)
    } else {
        // Non-streaming fallback
        let json_res: OllamaChatStreamResponse = res
            .json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        if let Some(msg) = json_res.message {
            Ok(msg.content)
        } else {
            Ok("".to_string())
        }
    }
}

/// List available Ollama models
#[tauri::command]
pub async fn list_models() -> Result<Vec<String>, String> {
    #[derive(Deserialize)]
    struct ModelInfo { name: String }
    #[derive(Deserialize)]
    struct ModelsResponse { models: Vec<ModelInfo> }

    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:11434/api/tags")
        .send()
        .await
        .map_err(|e| format!("Failed to list models: {}", e))?;

    let body: ModelsResponse = res
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))?;

    Ok(body.models.into_iter().map(|m| m.name).collect())
}
