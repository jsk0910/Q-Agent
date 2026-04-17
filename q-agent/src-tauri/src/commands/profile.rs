use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModeModelMap {
    pub default: String,
    pub academic: String,
    pub coding: String,
    pub finance: String,
    pub custom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub mode_models: ModeModelMap,
    pub theme: String,
    pub system_prompts: HashMap<String, String>,
    pub created_at: String,
}

#[tauri::command]
pub async fn save_profile(handle: AppHandle, profile: UserProfile) -> Result<(), String> {
    let app_dir = handle.path().app_data_dir().map_err(|e| e.to_string())?;
    
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    }
    
    let profile_path = app_dir.join("profiles.json");
    
    // Load existing profiles
    let mut profiles: Vec<UserProfile> = if profile_path.exists() {
        let content = fs::read_to_string(&profile_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };
    
    // Update or insert
    if let Some(pos) = profiles.iter().position(|p| p.id == profile.id) {
        profiles[pos] = profile;
    } else {
        profiles.push(profile);
    }
    
    let json = serde_json::to_string_pretty(&profiles).map_err(|e| e.to_string())?;
    fs::write(profile_path, json).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn load_profiles(handle: AppHandle) -> Result<Vec<UserProfile>, String> {
    let app_dir = handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let profile_path = app_dir.join("profiles.json");
    
    if !profile_path.exists() {
        return Ok(vec![]);
    }
    
    let content = fs::read_to_string(profile_path).map_err(|e| e.to_string())?;
    let profiles: Vec<UserProfile> = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(profiles)
}
