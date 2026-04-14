mod commands;
pub mod db;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
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
            commands::chat::list_models,
            commands::rag::add_source,
            commands::rag::get_sources,
            commands::agent::intercept_and_route,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
