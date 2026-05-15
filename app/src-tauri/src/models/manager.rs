use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use crate::system::ensure_app_data_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub parameters: String,
    pub required_ram_mb: u64,
    pub download_url: String,
    pub file_name: String,
    pub is_downloaded: bool,
}

#[tauri::command]
pub async fn get_available_models(app: AppHandle) -> Result<Vec<ModelInfo>, String> {
    let app_data_dir = ensure_app_data_dir(&app)?;
    let models_dir = app_data_dir.join("models");

    let mut models = vec![
        ModelInfo {
            id: "qwen2.5-3b-q4".to_string(),
            name: "Qwen 2.5 3B (Q4_K_M)".to_string(),
            description: "빠른 속도와 적절한 성능을 보여주는 경량 모델. (Router/Simple Tasks)".to_string(),
            parameters: "3B".to_string(),
            required_ram_mb: 4096, // 4GB
            download_url: "https://huggingface.co/Qwen/Qwen2.5-3B-Instruct-GGUF/resolve/main/qwen2.5-3b-instruct-q4_k_m.gguf".to_string(),
            file_name: "qwen2.5-3b-instruct-q4_k_m.gguf".to_string(),
            is_downloaded: false,
        },
        ModelInfo {
            id: "mistral-nemo-12b-q4".to_string(),
            name: "Mistral NeMo 12B (Q4_K_M)".to_string(),
            description: "강력한 추론 능력과 코딩 능력을 갖춘 다목적 모델. (Planner/Coder)".to_string(),
            parameters: "12B".to_string(),
            required_ram_mb: 10240, // 10GB
            download_url: "https://huggingface.co/bartowski/Mistral-Nemo-Instruct-2407-GGUF/resolve/main/Mistral-Nemo-Instruct-2407-Q4_K_M.gguf".to_string(),
            file_name: "Mistral-Nemo-Instruct-2407-Q4_K_M.gguf".to_string(),
            is_downloaded: false,
        },
        ModelInfo {
            id: "llama-3.1-8b-q4".to_string(),
            name: "Llama 3.1 8B (Q4_K_M)".to_string(),
            description: "밸런스가 잘 잡힌 메타의 오픈소스 로컬 표준 모델.".to_string(),
            parameters: "8B".to_string(),
            required_ram_mb: 8192, // 8GB
            download_url: "https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf".to_string(),
            file_name: "Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf".to_string(),
            is_downloaded: false,
        },
    ];

    // 다운로드 여부 체크
    for model in models.iter_mut() {
        let file_path = models_dir.join(&model.file_name);
        if file_path.exists() {
            model.is_downloaded = true;
        }
    }

    Ok(models)
}

#[tauri::command]
pub async fn download_model(_app: AppHandle, model_id: String) -> Result<(), String> {
    // 실제 다운로드 로직은 Phase 1 후반 또는 백그라운드 태스크로 구현
    // 지금은 스텁(Stub) 로직
    tracing::info!("Model download requested for: {}", model_id);
    
    // TODO: spawn async task with reqwest to stream file to app_data_dir/models/...
    // emit progress events to frontend using app.emit(...)

    Ok(())
}
