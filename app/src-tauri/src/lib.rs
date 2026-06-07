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
pub mod knowledge;

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
            let app_data_dir = match system::ensure_app_data_dir(_app.handle()) {
                Ok(path) => path,
                Err(e) => {
                    tracing::error!("Failed to initialize app data directory: {}", e);
                    std::path::PathBuf::from(".")
                }
            };
            
            // 기본 템플릿 복사
            if let Err(e) = templates::manager::init_default_templates(_app.handle()) {
                tracing::error!("Failed to initialize default templates: {}", e);
            }

            let db_path = app_data_dir.join("db");
            let db_path_str = db_path.to_string_lossy().to_string();

            let db = tauri::async_runtime::block_on(async {
                db::Database::init(&db_path_str)
                    .await
                    .expect("Failed to init database")
            });
            _app.manage(db);
            
            let hitl_manager = agents::hitl::HitlManager::new();
            _app.manage(hitl_manager);

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
            templates::manager::save_template_content,
            commands::create_project,
            commands::list_projects,
            commands::delete_project,
            commands::create_conversation,
            commands::list_conversations,
            commands::list_messages,
            commands::send_chat_message,
            commands::get_project_history,
            commands::run_agent_workflow,
            commands::hitl_respond
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod commands {
    use crate::db::Database;
    use crate::models::entities::{Project, Conversation, Message};
    use tauri::State;

    #[tauri::command]
    pub async fn create_project(
        db: State<'_, Database>, 
        name: String,
        description: Option<String>,
        persona_template: Option<String>,
        harness_template: Option<String>,
    ) -> Result<Project, String> {
        let id_str = format!("proj-{}", chrono::Utc::now().timestamp_millis());
        let proj = Project {
            id: surrealdb::sql::Thing::from(("project", id_str.as_str())),
            name: name,
            description,
            icon: None,
            color: None,
            persona_template: persona_template.unwrap_or_else(|| "default".to_string()),
            harness_template: harness_template.unwrap_or_else(|| "deep_research".to_string()),
            orchestration_template: "balanced".into(),
            model_small: None,
            model_heavy: None,
            max_iterations: 5,
            token_budget: 8192,
            security_level: 1,
            is_active: true,
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
        };
        let created = db.create_project(proj).await.map_err(|e| e.to_string())?;

        // Automatically create a default conversation for this project
        let conv_id = format!("conv-{}", chrono::Utc::now().timestamp_millis());
        let conv = Conversation {
            id: surrealdb::sql::Thing::from(("conversation", conv_id.as_str())),
            project_id: Some(created.id.clone()),
            title: "General".into(),
            mode: "chat".into(),
            created_at: chrono::Utc::now().into(),
            updated_at: chrono::Utc::now().into(),
        };
        let _ = db.create_conversation(conv).await;

        Ok(created)
    }

    #[tauri::command]
    pub async fn list_projects(db: State<'_, Database>) -> Result<Vec<Project>, String> {
        db.list_projects().await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn delete_project(db: State<'_, Database>, id: String) -> Result<(), String> {
        db.delete_project(&id).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn create_conversation(db: State<'_, Database>, conv: Conversation) -> Result<Conversation, String> {
        db.create_conversation(conv).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn list_conversations(db: State<'_, Database>, project_id: Option<String>) -> Result<Vec<Conversation>, String> {
        db.list_conversations(project_id.as_deref()).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn list_messages(db: State<'_, Database>, conversation_id: String) -> Result<Vec<Message>, String> {
        db.list_messages(&conversation_id).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn send_chat_message(db: State<'_, Database>, msg: Message) -> Result<Message, String> {
        db.save_message(msg).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn get_project_history(db: State<'_, Database>, project_id: String) -> Result<Vec<Message>, String> {
        let raw_id = project_id.strip_prefix("project:").unwrap_or(&project_id);
        let convs = db.list_conversations(Some(raw_id)).await.map_err(|e| e.to_string())?;
        if let Some(first_conv) = convs.first() {
            let conv_raw = first_conv.id.id.to_raw();
            let msgs = db.list_messages(&conv_raw).await.map_err(|e| e.to_string())?;
            return Ok(msgs);
        }
        Ok(vec![])
    }

#[derive(serde::Serialize)]
pub struct CitationPayload {
    pub index: usize,
    pub source_id: String,
    pub excerpt: String,
    pub confidence: f32,
}

#[derive(serde::Serialize)]
pub struct ArtifactPayload {
    pub title: String,
    pub r#type: String,
    pub content: String,
    pub language: Option<String>,
}

#[derive(serde::Serialize)]
pub struct WorkflowResponse {
    pub answer: String,
    pub citations: Vec<CitationPayload>,
    pub artifacts: Vec<ArtifactPayload>,
}

#[tauri::command]
pub async fn run_agent_workflow(
    app_handle: tauri::AppHandle,
    db: State<'_, Database>,
    hitl_manager: State<'_, crate::agents::hitl::HitlManager>,
    message: String,
    project_id: Option<String>,
    _harness_type: Option<String>,
    permission_mode: Option<String>,
    mentions: Option<Vec<String>>,
) -> Result<WorkflowResponse, String> {
        use crate::agents::state::{AgentState, AgentStatus, Message as AgentMessage};
        use crate::agents::graph::WorkflowGraph;
        use crate::agents::security::PermissionMode;
        
        let p_mode = match permission_mode.as_deref() {
            Some("strict") => PermissionMode::Strict,
            Some("agentic") => PermissionMode::Agentic,
            _ => PermissionMode::Balanced,
        };

        let graph = WorkflowGraph::default();
        let task_id = chrono::Utc::now().timestamp_millis().to_string();
        
        let state = AgentState {
            task_id,
            max_iterations: 3,
            token_budget: 8192,
            status: AgentStatus::Idle,
            permission_mode: p_mode,
            mentions: mentions.unwrap_or_default(),
            messages: vec![AgentMessage {
                role: "user".to_string(),
                content: message.clone(),
            }],
            ..Default::default()
        };
        
        let result = graph.run(state, &db, &app_handle, &hitl_manager).await.map_err(|e| e.to_string())?;
        
        let mut answer = result.final_answer.unwrap_or_else(|| "No answer generated.".to_string());
        
        let mut citations = Vec::new();
        if !result.mentions.is_empty() {
            for (i, mention) in result.mentions.iter().enumerate() {
                let index = i + 1;
                citations.push(CitationPayload {
                    index,
                    source_id: mention.clone(),
                    excerpt: format!("Context injected from {}", mention),
                    confidence: 0.95,
                });
                
                // Add citation tag to the end of the answer to ensure it renders if the LLM didn't include it natively.
                if !answer.contains(&format!("[{}]", index)) {
                    answer.push_str(&format!(" [{}]", index));
                }
            }
        }
        
        let mut ui_artifacts = Vec::new();
        if let Some(plan) = &result.plan {
            ui_artifacts.push(ArtifactPayload {
                title: "Implementation Plan".to_string(),
                r#type: "plan".to_string(),
                content: plan.clone(),
                language: Some("markdown".to_string()),
            });
        }
        
        // Final answer could be too long, but we provide it as a markdown artifact
        ui_artifacts.push(ArtifactPayload {
            title: "Execution Report".to_string(),
            r#type: "markdown".to_string(),
            content: answer.clone(),
            language: Some("markdown".to_string()),
        });
        
        // Save to DB if project is linked
        if let Some(pid) = &project_id {
            use crate::models::entities::Message as DbMessage;
            use surrealdb::sql::Thing;
            let raw_pid = pid.strip_prefix("project:").unwrap_or(pid);
            if let Ok(convs) = db.list_conversations(Some(raw_pid)).await {
                if let Some(conv) = convs.first() {
                    let timestamp = chrono::Utc::now().timestamp_millis();
                    // Save user message
                    let _ = db.save_message(DbMessage {
                        id: Thing::from(("message", format!("msg-usr-{}", timestamp).as_str())),
                        conversation_id: conv.id.clone(),
                        role: "user".to_string(),
                        content: message.clone(),
                        citations: vec![],
                        thought_trace: vec![],
                        artifact_ids: vec![],
                        model_used: None,
                        tokens_used: None,
                        duration_ms: None,
                        created_at: chrono::Utc::now().into(),
                    }).await;
                    
                    // Save assistant message
                    let _ = db.save_message(DbMessage {
                        id: Thing::from(("message", format!("msg-ast-{}", timestamp + 1).as_str())),
                        conversation_id: conv.id.clone(),
                        role: "assistant".to_string(),
                        content: answer.clone(),
                        citations: vec![],
                        thought_trace: vec![],
                        artifact_ids: vec![],
                        model_used: None,
                        tokens_used: None,
                        duration_ms: None,
                        created_at: chrono::Utc::now().into(),
                    }).await;
                }
            }
        }
        
        Ok(WorkflowResponse {
            answer,
            citations,
            artifacts: ui_artifacts,
        })
    }

    #[tauri::command]
    pub async fn hitl_respond(
        hitl_manager: State<'_, crate::agents::hitl::HitlManager>,
        id: String,
        approved: bool,
        _env: String,
    ) -> Result<(), String> {
        hitl_manager.resolve_request(&id, approved).await
    }
}

#[cfg(test)]
mod tests {
    use crate::db::Database;
    use crate::models::{MockLlamaRunner, ModelRequest, ModelRunner};

    #[tokio::test]
    async fn test_db_init() {
        let _db = Database::init("memory")
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
        // test removed due to tauri::AppHandle dependency
    }
}
