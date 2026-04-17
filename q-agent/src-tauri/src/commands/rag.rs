use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb_types::{RecordId, SurrealValue};
use std::sync::atomic::Ordering;
use crate::CancellationState;
use tauri::State;
use futures_util::future::join_all;

#[derive(Serialize, Deserialize, Debug, SurrealValue)]
pub struct Source {
    pub id: Option<RecordId>,
    pub project_id: Option<String>,
    pub title: String,
    pub summary: String,
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug, SurrealValue)]
pub struct Chunk {
    pub source_id: RecordId,
    pub project_id: Option<String>,
    pub content: String,
    pub embedding: Vec<f32>,
}

#[derive(Serialize)]
struct EmbedRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

#[derive(Serialize)]
struct GenerationRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerationResponse {
    response: String,
}

async fn generate_summary(text: &str) -> Result<(String, String), String> {
    let client = reqwest::Client::new();
    let prompt = format!(
        "You are an expert summarizer. Given the following document text, provide a JSON response with exactly two string fields: 'title' and 'summary'. Do NOT output markdown, ONLY valid JSON.\n\nTEXT:\n{:.2000}",
        text
    );

    let payload = GenerationRequest {
        model: "qwen3.5:4b".to_string(), // Currently hardcoded to local fallback model
        prompt,
        stream: false,
    };

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json_res: GenerationResponse = res.json().await.map_err(|e| e.to_string())?;

    #[derive(Deserialize)]
    struct Extracted {
        title: String,
        summary: String,
    }

    // Try parsing the json output. In case qwen hallucinates some markdown blocks around JSON.
    let clean_json = json_res.response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
        .to_string();

    let parsed: Extracted = match serde_json::from_str(&clean_json) {
        Ok(p) => p,
        Err(_) => {
            if let (Some(start), Some(end)) = (clean_json.find('{'), clean_json.rfind('}')) {
                serde_json::from_str(&clean_json[start..=end]).unwrap_or_else(|_| Extracted {
                    title: "Untitled Document".into(),
                    summary: "Summary could not be automatically generated.".into(),
                })
            } else {
                Extracted {
                    title: "Untitled Document".into(),
                    summary: "Summary could not be automatically generated.".into(),
                }
            }
        }
    };

    Ok((parsed.title, parsed.summary))
}

async fn get_embeddings(chunks: &[String]) -> Result<Vec<Vec<f32>>, String> {
    let client = reqwest::Client::new();
    let payload = EmbedRequest {
        model: "nomic-embed-text".to_string(),
        input: chunks.to_vec(),
    };

    let res = client
        .post("http://localhost:11434/api/embed")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json_res: EmbedResponse = res.json().await.map_err(|e| e.to_string())?;
    Ok(json_res.embeddings)
}

fn simple_chunker(text: &str, chunk_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let words = text.split_whitespace();

    for word in words {
        if current_chunk.len() + word.len() > chunk_size && !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
            current_chunk.clear();
        }
        current_chunk.push_str(word);
        current_chunk.push(' ');
    }
    if !current_chunk.is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }
    chunks
}

#[tauri::command]
pub async fn add_source(
    db: State<'_, Surreal<Db>>,
    cancel: State<'_, CancellationState>,
    text: String,
    format: String,
    project_id: Option<String>,
) -> Result<Source, String> {
    // Reset cancellation flag
    cancel.0.store(false, Ordering::SeqCst);

    // 1. Generate Title and Summary using Qwen (with fallback)
    if cancel.0.load(Ordering::SeqCst) { return Err("Cancelled".into()); }
    let (title, summary) = match generate_summary(&text).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Summary generation failed: {}. Using fallback.", e);
            ("Untitled Document".into(), "Summary not available.".into())
        }
    };

    // 2. Add Source to DB
    let source_record: Option<Source> = db
        .create("source")
        .content(Source {
            id: None,
            project_id: project_id.clone(),
            title: title.clone(),
            summary: summary.clone(),
            format,
        })
        .await
        .map_err(|e| e.to_string())?;

    let source = source_record.ok_or("Failed to create Source in DB")?;
    let source_id = source.id.clone().unwrap();

    // 3. Chunk the document (approx 500 chars for a simple tokenizer fallback)
    let chunks_texts = simple_chunker(&text, 500);

    // 4. Determine embeddings in batches if necessary, but Ollama supports array.
    // For large documents, we should chunk the array being sent to Ollama.
    // Simplifying: send chunks batches concurrently
    let chunk_batches: Vec<_> = chunks_texts.chunks(100).map(|c| c.to_vec()).collect();
    let mut futures = Vec::new();
    
    for batch in chunk_batches {
        if cancel.0.load(Ordering::SeqCst) {
            return Err("Cancelled during embedding process".into());
        }
        futures.push(async move {
            let embeddings = get_embeddings(&batch).await?;
            Ok::<_, String>((batch, embeddings))
        });
    }

    let results = join_all(futures).await;

    let mut success_count = 0;
    for res in results {
        match res {
            Ok((batch, embeddings)) => {
                for (i, content) in batch.into_iter().enumerate() {
                    let chunk_record = Chunk {
                        source_id: source_id.clone(),
                        project_id: project_id.clone(),
                        content,
                        embedding: embeddings[i].clone(),
                    };
                    let save_res: Result<Option<Chunk>, _> = db
                        .create("chunk")
                        .content(chunk_record)
                        .await;
                    
                    if save_res.is_ok() {
                        success_count += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("Batch embedding failed: {}", e);
                // Continue to next batch instead of failing everything
            }
        }
    }

    if success_count == 0 && !chunks_texts.is_empty() {
        return Err("Failed to process any document chunks".into());
    }

    Ok(source)
}

#[tauri::command]
pub async fn get_sources(db: tauri::State<'_, Surreal<Db>>, project_id: Option<String>) -> Result<Vec<Source>, String> {
    let query = if let Some(pid) = project_id {
        format!("SELECT * FROM source WHERE project_id = '{}' ORDER BY created_at DESC", pid)
    } else {
        "SELECT * FROM source WHERE project_id = NONE ORDER BY created_at DESC".to_string()
    };

    let mut res = db
        .query(&query)
        .await
        .map_err(|e| e.to_string())?;
        
    let sources: Vec<Source> = res.take(0).map_err(|e| e.to_string())?;
    Ok(sources)
}
