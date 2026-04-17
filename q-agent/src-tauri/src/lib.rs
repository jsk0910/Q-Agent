mod commands;
pub mod db;
pub mod models;
pub mod harness;

use tauri::Manager;
use std::sync::atomic::AtomicBool;

pub struct CancellationState(pub AtomicBool);
impl Default for CancellationState {
    fn default() -> Self {
        Self(AtomicBool::new(false))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            
            // Register Model Runner
            app.manage(models::runner::OllamaRunner::new("http://localhost:11434"));
            app.manage(CancellationState::default());

            tauri::async_runtime::spawn(async move {
                match db::init_db(&handle).await {
                    Ok(db) => {
                        handle.manage(db);
                        println!("SurrealDB initialized successfully.");
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize DB: {}", e);
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::chat::chat_with_ollama,
            commands::chat::stop_generation,
            commands::chat::list_models,
            commands::chat::evaluate_response,
            commands::rag::add_source,
            commands::rag::get_sources,
            commands::agent::intercept_and_route,
            commands::profile::save_profile,
            commands::profile::load_profiles,
            commands::chat::pull_model,
            commands::project::create_project,
            commands::project::list_projects,
            commands::project::add_guideline,
            commands::project::get_guidelines,
            commands::project::seed_default_guidelines,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
