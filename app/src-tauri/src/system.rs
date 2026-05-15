use serde::{Deserialize, Serialize};
use sysinfo::System;
use tauri::{AppHandle, Manager};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemSpecs {
    pub total_memory_mb: u64,
    pub used_memory_mb: u64,
    pub cpu_cores: usize,
}

#[tauri::command]
pub async fn get_system_specs() -> Result<SystemSpecs, String> {
    // Note: instantiating System takes a bit of time, in production we might want to keep it in state
    let mut sys = System::new_all();
    sys.refresh_all();
    Ok(SystemSpecs {
        total_memory_mb: sys.total_memory() / 1024 / 1024,
        used_memory_mb: sys.used_memory() / 1024 / 1024,
        cpu_cores: sys.cpus().len(),
    })
}

/// 앱의 전용 데이터 경로를 반환하고, 필수 하위 폴더들을 생성합니다.
pub fn ensure_app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let path_resolver = app.path();
    let app_data_dir = path_resolver.app_data_dir()
        .map_err(|e| format!("Failed to get app_data_dir: {}", e))?;

    // 필요한 서브 디렉토리 목록
    let subdirs = ["models", "templates/personas", "templates/harness", "templates/orchestration"];
    
    for subdir in subdirs.iter() {
        let dir_path = app_data_dir.join(subdir);
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)
                .map_err(|e| format!("Failed to create dir {:?}: {}", dir_path, e))?;
        }
    }

    Ok(app_data_dir)
}

#[tauri::command]
pub async fn get_app_data_path(app: AppHandle) -> Result<String, String> {
    let path = ensure_app_data_dir(&app)?;
    Ok(path.to_string_lossy().to_string())
}
