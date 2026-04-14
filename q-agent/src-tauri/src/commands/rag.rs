use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb_types::{RecordId, SurrealValue};

#[derive(Serialize, Deserialize, Debug, SurrealValue)]
pub struct Source {
    pub id: Option<RecordId>,
    pub title: String,
    pub summary: String,
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug, SurrealValue)]
pub struct Chunk {
    pub source_id: RecordId,
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
    let clean_json = json_res.response.replace("```json", "").replace("```", "").trim().to_string();
    let parsed: Extracted = serde_json::from_str(&clean_json).unwrap_or_else(|_| Extracted {
        title: "Untitled Document".into(),
        summary: "Summary could not be automatically generated.".into(),
    });

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
    db: tauri::State<'_, Surreal<Db>>,
    text: String,
    format: String,
) -> Result<Source, String> {
    
    // 1. Generate Title and Summary using Qwen
    let (title, summary) = generate_summary(&text).await?;

    // 2. Add Source to DB
    let source_record: Option<Source> = db
        .create("source")
        .content(Source {
            id: None,
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
    // Simplifying: send chunks 50 at a time
    for chunk_batch in chunks_texts.chunks(50) {
        let embeddings = get_embeddings(chunk_batch).await?;
        
        for (i, content) in chunk_batch.iter().enumerate() {
            let chunk_record = Chunk {
                source_id: source_id.clone(),
                content: content.clone(),
                embedding: embeddings[i].clone(),
            };
            let _: Option<Chunk> = db
                .create("chunk")
                .content(chunk_record)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(source)
}

#[tauri::command]
pub async fn get_sources(db: tauri::State<'_, Surreal<Db>>) -> Result<Vec<Source>, String> {
    let mut res = db
        .query("SELECT * FROM source ORDER BY created_at DESC")
        .await
        .map_err(|e| e.to_string())?;
        
    let sources: Vec<Source> = res.take(0).map_err(|e| e.to_string())?;
    Ok(sources)
}
