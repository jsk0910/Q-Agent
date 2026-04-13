// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[tauri::command]
async fn chat_with_ollama(prompt: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    let payload = ChatRequest {
        model: "qwen3.5:4b".to_string(),
        prompt: prompt.clone(),
        stream: false,
    };

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("API 요청 실패: {}", e))?;

    let json_res: OllamaResponse = res
        .json()
        .await
        .map_err(|e| format!("JSON 파싱 실패: {}", e))?;

    Ok(json_res.response)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![chat_with_ollama])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}