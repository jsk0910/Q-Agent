use surrealdb::engine::local::{Db, SurrealKv};
use surrealdb::Surreal;
use tauri::Manager;

pub async fn init_db(app_handle: &tauri::AppHandle) -> Result<Surreal<Db>, String> {
    // Note: requires tauri-plugin-fs or similar? No, AppDir is available via app_handle.path() in v2.
    // wait, in tauri v2 it's `app_handle.path().app_data_dir()`
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    
    let db_path = app_dir.join("q-agent.db");
    
    let db = Surreal::new::<SurrealKv>(db_path.to_str().unwrap())
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;
        
    db.use_ns("qagent").use_db("knowledge").await
        .map_err(|e| format!("Failed to use namespace/db: {}", e))?;
        
    // Define Schema
    // nomic-embed-text usually uses 768 dimensions
    let queries = "
        DEFINE TABLE source SCHEMAFULL;
        DEFINE FIELD title ON source TYPE string;
        DEFINE FIELD summary ON source TYPE string;
        DEFINE FIELD format ON source TYPE string;
        DEFINE FIELD created_at ON source TYPE datetime DEFAULT time::now();
        
        DEFINE TABLE chunk SCHEMAFULL;
        DEFINE FIELD source_id ON chunk TYPE record<source>;
        DEFINE FIELD content ON chunk TYPE string;
        DEFINE FIELD embedding ON chunk TYPE array<float>;
        DEFINE FIELD project_id ON chunk TYPE option<string>;
        
        DEFINE TABLE project SCHEMAFULL;
        DEFINE FIELD name ON project TYPE string;
        DEFINE FIELD description ON project TYPE string;
        DEFINE FIELD created_at ON project TYPE datetime DEFAULT time::now();

        DEFINE TABLE guideline SCHEMAFULL;
        DEFINE FIELD project_id ON guideline TYPE option<string>;
        DEFINE FIELD keywords ON guideline TYPE array<string>;
        DEFINE FIELD content ON guideline TYPE string;
        DEFINE FIELD created_at ON guideline TYPE datetime DEFAULT time::now();

        DEFINE INDEX chunk_embedding_idx ON chunk FIELDS embedding HNSW DIMENSION 768 DIST COSINE;
    ";
    
    db.query(queries).await.map_err(|e| format!("Failed to initialize schema: {}", e))?;
    
    Ok(db)
}
