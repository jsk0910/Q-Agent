// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub mod db;
pub mod models;
pub mod system;
pub mod templates;
pub mod agents;

use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            // 앱 데이터 디렉토리 초기화
            if let Err(e) = system::ensure_app_data_dir(_app.handle()) {
                tracing::error!("Failed to initialize app data directory: {}", e);
            }
            
            // 기본 템플릿 복사
            if let Err(e) = templates::manager::init_default_templates(_app.handle()) {
                tracing::error!("Failed to initialize default templates: {}", e);
            }

            tauri::async_runtime::block_on(async {
                let _db = db::Database::init("memory")
                    .await
                    .expect("Failed to init database");
            });

            // Alt+Space 단축키 등록
            let shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
            _app.global_shortcut()
                .on_shortcut(shortcut, |app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            system::get_system_specs,
            system::get_app_data_path,
            models::manager::get_available_models,
            models::manager::download_model,
            templates::manager::list_templates,
            templates::manager::get_template_content,
            templates::manager::save_template_content
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use crate::models::{MockLlamaRunner, ModelRequest, ModelRunner};

    #[tokio::test]
    async fn test_db_init() {
        let db = Database::init("memory")
            .await
            .expect("DB should initialize");
        // We just ensure it didn't panic or error out during init (which runs the schema.surql)
    }

    #[tokio::test]
    async fn test_mock_llama_runner() {
        let runner = MockLlamaRunner;
        let req = ModelRequest {
            prompt: "Test prompt".to_string(),
            max_tokens: 50,
        };
        let mut rx = runner
            .generate_stream(req)
            .await
            .expect("Stream should start");

        let mut tokens = Vec::new();
        while let Some(Ok(event)) = rx.recv().await {
            tokens.push(event.token);
        }

        assert!(!tokens.is_empty());
        assert_eq!(tokens[0], "Hello");
    }

    #[tokio::test]
    async fn test_langgraph_workflow() {
        use crate::agents::state::{AgentState, AgentStatus, Message};
        use crate::agents::graph::WorkflowGraph;

        let graph = WorkflowGraph::default();
        let state = AgentState {
            task_id: "test-task-1".into(),
            max_iterations: 3,
            token_budget: 8192,
            messages: vec![Message { role: "user".into(), content: "Write a hello world script".into() }],
            ..Default::default()
        };

        let db = crate::db::Database::init("memory").await.unwrap();
        let result = graph.run(state, &db).await.expect("Graph should run successfully");
        assert_eq!(result.status, AgentStatus::Finished);
        assert!(result.plan.is_some());
        assert!(result.final_answer.is_some());
        assert_eq!(result.iteration, 2); // First fail, second pass
    }
}
