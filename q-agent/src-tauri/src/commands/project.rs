use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb_types::{RecordId, SurrealValue};

#[derive(Serialize, Deserialize, Debug, SurrealValue)]
pub struct Project {
    pub id: Option<RecordId>,
    pub name: String,
    pub description: String,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, SurrealValue)]
pub struct Guideline {
    pub id: Option<RecordId>,
    pub project_id: Option<String>,
    pub keywords: Vec<String>,
    pub content: String,
    pub created_at: Option<String>,
}

#[tauri::command]
pub async fn create_project(
    db: tauri::State<'_, Surreal<Db>>,
    name: String,
    description: String,
) -> Result<Project, String> {
    println!("Creating project: {} - {}", name, description);
    let result: Result<Option<Project>, _> = db
        .create("project")
        .content(Project {
            id: None,
            name,
            description,
            created_at: None,
        })
        .await;

    match result {
        Ok(Some(p)) => {
            println!("Created successfully: {:?}", p);
            Ok(p)
        }
        Ok(None) => {
            println!("Created successfully but returned None");
            Err("Failed to create project (returned None)".to_string())
        }
        Err(e) => {
            println!("DB Create Error: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn list_projects(
    db: tauri::State<'_, Surreal<Db>>,
) -> Result<Vec<Project>, String> {
    let mut res = db
        .query("SELECT * FROM project ORDER BY created_at DESC")
        .await
        .map_err(|e| e.to_string())?;

    let projects: Vec<Project> = res.take(0).map_err(|e| e.to_string())?;
    Ok(projects)
}

#[tauri::command]
pub async fn add_guideline(
    db: tauri::State<'_, Surreal<Db>>,
    project_id: Option<String>,
    keywords: Vec<String>,
    content: String,
) -> Result<Guideline, String> {
    let guideline_record: Option<Guideline> = db
        .create("guideline")
        .content(Guideline {
            id: None,
            project_id,
            keywords,
            content,
            created_at: None,
        })
        .await
        .map_err(|e| e.to_string())?;

    guideline_record.ok_or_else(|| "Failed to add guideline".to_string())
}

#[tauri::command]
pub async fn get_guidelines(
    db: tauri::State<'_, Surreal<Db>>,
    project_id: Option<String>,
) -> Result<Vec<Guideline>, String> {
    let query = if let Some(pid) = project_id {
        format!("SELECT * FROM guideline WHERE project_id = '{}'", pid)
    } else {
        "SELECT * FROM guideline WHERE project_id = NONE".to_string()
    };

    let mut res = db
        .query(&query)
        .await
        .map_err(|e| e.to_string())?;

    let guidelines: Vec<Guideline> = res.take(0).map_err(|e| e.to_string())?;
    Ok(guidelines)
}

#[tauri::command]
pub async fn seed_default_guidelines(db: tauri::State<'_, Surreal<Db>>) -> Result<(), String> {
    let defaults = vec![
        (
            vec!["rust", "cargo", "rs", "tauri"],
            "Write highly idiomatic Rust code. Prefer Result and Option for error handling. Avoid panicking unless absolutely necessary. Use async/await efficiently."
        ),
        (
            vec!["svelte", "rune", "props", "svelte 5", "ts", "javascript"],
            "Use Svelte 5 Runes natively ($state, $derived, $props). Keep UI state management isolated. Use clean TypeScript for prop typing. Prefer CSS variables for styling."
        ),
        (
            vec!["surrealdb", "sql", "db", "query"],
            "Write SurrealQL efficiently using the Rust surrealdb driver. Handle SurrealValue conversions safely."
        )
    ];

    for (keywords, content) in defaults {
        // Try creating these globally without a specific project ID for default fallbacks
        let _: Option<Guideline> = db
            .create("guideline")
            .content(Guideline {
                id: None,
                project_id: None, // Global defaults
                keywords: keywords.iter().map(|s| s.to_string()).collect(),
                content: content.to_string(),
                created_at: None,
            })
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
