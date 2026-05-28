use anyhow::Result;
use tauri::AppHandle;
use std::fs;
use crate::system::ensure_app_data_dir;
use super::models::{PersonaConfig, HarnessConfig, OrchestrationConfig};

pub fn init_default_templates(app: &AppHandle) -> Result<()> {
    let app_data_dir = ensure_app_data_dir(app).map_err(|e| anyhow::anyhow!(e))?;
    
    let persona_dir = app_data_dir.join("templates/personas");
    let default_persona = persona_dir.join("default.yaml");
    if !default_persona.exists() {
        let yaml = r#"name: Default Assistant
description: 범용 기본 페르소나
system_prompt: "You are Q-Agent, a helpful AI assistant. Always think step-by-step."
tone: professional
"#;
        fs::write(default_persona, yaml)?;
    }

    let researcher_persona = persona_dir.join("researcher.yaml");
    if !researcher_persona.exists() {
        let yaml = r#"name: Researcher
description: 논문 분석 및 정보 탐색 특화
system_prompt: "You are a senior researcher. Provide factual, precise, and cited answers."
tone: academic
"#;
        fs::write(researcher_persona, yaml)?;
    }
    
    let coder_persona = persona_dir.join("coder.yaml");
    if !coder_persona.exists() {
        let yaml = r#"name: Senior Coder
description: 코드 생성 및 리뷰 특화 엔지니어
system_prompt: "You are an expert software engineer. Write clean, efficient, and well-documented code."
tone: direct
"#;
        fs::write(coder_persona, yaml)?;
    }

    let harness_dir = app_data_dir.join("templates/harness");
    let default_harness = harness_dir.join("standard.yaml");
    if !default_harness.exists() {
        let yaml = r#"name: Standard
description: 프로젝트 폴더 내 표준 권한
security_level: standard
allowed_tools: ["fs_read", "fs_write", "terminal"]
access_paths: ["./workspace"]
"#;
        fs::write(default_harness, yaml)?;
    }
    
    let strict_harness = harness_dir.join("strict.yaml");
    if !strict_harness.exists() {
        let yaml = r#"name: Strict
description: 시스템 보호 (읽기 전용 모드)
security_level: strict
allowed_tools: ["fs_read"]
access_paths: ["./workspace"]
"#;
        fs::write(strict_harness, yaml)?;
    }

    let advanced_harness = harness_dir.join("advanced.yaml");
    if !advanced_harness.exists() {
        let yaml = r#"name: Advanced
description: 모든 도구 접근 허용 (원격 서버 및 도커 포함)
security_level: advanced
allowed_tools: ["fs_read", "fs_write", "terminal", "browser", "ssh"]
access_paths: ["/"]
"#;
        fs::write(advanced_harness, yaml)?;
    }

    let orch_dir = app_data_dir.join("templates/orchestration");
    let default_orch = orch_dir.join("balanced.yaml");
    if !default_orch.exists() {
        let yaml = r#"name: Balanced
description: Planner → Executor → Critic 기본 루프
max_iterations: 5
routing_threshold: 0.85
default_model: "llama-3.1-8b-q4"
"#;
        fs::write(default_orch, yaml)?;
    }

    Ok(())
}

pub fn load_persona(app: &AppHandle, name: &str) -> Result<PersonaConfig> {
    let app_data_dir = ensure_app_data_dir(app).map_err(|e| anyhow::anyhow!(e))?;
    let path = app_data_dir.join(format!("templates/personas/{}.yaml", name));
    if !path.exists() {
        return Err(anyhow::anyhow!("Persona template '{}' not found", name));
    }
    let content = fs::read_to_string(path)?;
    let config: PersonaConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

pub fn load_harness(app: &AppHandle, name: &str) -> Result<HarnessConfig> {
    let app_data_dir = ensure_app_data_dir(app).map_err(|e| anyhow::anyhow!(e))?;
    let path = app_data_dir.join(format!("templates/harness/{}.yaml", name));
    let content = fs::read_to_string(path)?;
    let config: HarnessConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

pub fn load_orchestration(app: &AppHandle, name: &str) -> Result<OrchestrationConfig> {
    let app_data_dir = ensure_app_data_dir(app).map_err(|e| anyhow::anyhow!(e))?;
    let path = app_data_dir.join(format!("templates/orchestration/{}.yaml", name));
    let content = fs::read_to_string(path)?;
    let config: OrchestrationConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

#[tauri::command]
pub async fn list_templates(app: AppHandle, category: String) -> Result<Vec<String>, String> {
    let app_data_dir = ensure_app_data_dir(&app)?;
    let dir_path = app_data_dir.join(format!("templates/{}", category));
    
    let mut templates = Vec::new();
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    templates.push(stem.to_string());
                }
            }
        }
    }
    Ok(templates)
}

#[tauri::command]
pub async fn get_template_content(app: AppHandle, category: String, name: String) -> Result<String, String> {
    let app_data_dir = ensure_app_data_dir(&app)?;
    let path = app_data_dir.join(format!("templates/{}/{}.yaml", category, name));
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_template_content(app: AppHandle, category: String, name: String, content: String) -> Result<(), String> {
    let app_data_dir = ensure_app_data_dir(&app)?;
    let path = app_data_dir.join(format!("templates/{}/{}.yaml", category, name));
    fs::write(path, content).map_err(|e| e.to_string())
}
