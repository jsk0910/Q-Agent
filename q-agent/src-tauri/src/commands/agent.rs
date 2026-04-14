use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use regex::Regex;
use tauri::State;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb_types::SurrealValue;

// ─── DuckDuckGo Web Search ────────────────────────────────────────────────
pub async fn duckduckgo_search(query: &str) -> Result<String, String> {
    let client = Client::new();
    let res = client
        .get("https://html.duckduckgo.com/html/")
        .query(&[("q", query)])
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .send()
        .await
        .map_err(|e| format!("DuckDuckGo request failed: {}", e))?;
        
    let html = res.text().await.map_err(|e| e.to_string())?;
    let document = Html::parse_document(&html);
    let result_sel = Selector::parse(".result__body").unwrap();
    let title_sel = Selector::parse(".result__title").unwrap();
    let snippet_sel = Selector::parse(".result__snippet").unwrap();
    
    let mut results = String::new();
    
    for (i, element) in document.select(&result_sel).take(4).enumerate() {
        let title = element.select(&title_sel).next().map(|el| el.text().collect::<Vec<_>>().join(" ")).unwrap_or_default();
        let snippet = element.select(&snippet_sel).next().map(|el| el.text().collect::<Vec<_>>().join(" ")).unwrap_or_default();
        
        results.push_str(&format!("[Web Source {}] {}\nSummary: {}\n\n", i + 1, title.trim(), snippet.trim()));
    }
    
    if results.is_empty() {
        return Ok("No relevant web search results found.".to_string());
    }
    
    Ok(results)
}

// ─── Query Routing ────────────────────────────────────────────────────────
#[derive(Serialize)]
struct GenerationRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f64,
}

#[derive(Deserialize)]
struct GenerationResponse {
    response: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RouteWeights {
    pub local_db: f32,
    pub web_search: f32,
    pub internal: f32,
}

impl Default for RouteWeights {
    fn default() -> Self {
        Self {
            local_db: 0.0,
            web_search: 0.0,
            internal: 1.0,
        }
    }
}

pub async fn route_query(prompt: &str) -> RouteWeights {
    let system = "You are a Query Router for an AI Assistant. Given the user's query, assign weights (0.0 to 1.0) to three sources: 
1. 'local_db': for queries about uploaded files, documents, or personal context (e.g., 'in the pdf', 'summarize my source').
2. 'web_search': for current events, recent news, or live facts (e.g., 'today's weather', 'latest news').
3. 'internal': for coding, general facts, chit-chat, or reasoning.

Output strictly valid JSON with no markdown and no other text.
Example format:
{\"local_db\": 0.8, \"web_search\": 0.0, \"internal\": 0.2}";

    let client = Client::new();
    let payload = GenerationRequest {
        model: "qwen3.5:4b".into(),
        prompt: prompt.to_string(),
        system: system.to_string(),
        stream: false,
        options: OllamaOptions { temperature: 0.1 },
    };

    let Ok(res) = client.post("http://localhost:11434/api/generate").json(&payload).send().await else {
        return RouteWeights::default();
    };

    let Ok(json_res) = res.json::<GenerationResponse>().await else {
        return RouteWeights::default();
    };

    // Try normal JSON parsing
    let clean = json_res.response.replace("```json", "").replace("```", "");
    if let Ok(weights) = serde_json::from_str::<RouteWeights>(&clean) {
        return weights;
    }

    // Fallback: Regex extraction
    let mut weights = RouteWeights::default();
    if let Ok(re) = Regex::new(r#""local_db"\s*:\s*([\d\.]+)"#) {
        if let Some(caps) = re.captures(&clean) {
            weights.local_db = caps.get(1).unwrap().as_str().parse().unwrap_or(0.0);
        }
    }
    if let Ok(re) = Regex::new(r#""web_search"\s*:\s*([\d\.]+)"#) {
        if let Some(caps) = re.captures(&clean) {
            weights.web_search = caps.get(1).unwrap().as_str().parse().unwrap_or(0.0);
        }
    }
    if let Ok(re) = Regex::new(r#""internal"\s*:\s*([\d\.]+)"#) {
        if let Some(caps) = re.captures(&clean) {
            weights.internal = caps.get(1).unwrap().as_str().parse().unwrap_or(1.0);
        }
    }
    
    weights
}

// ─── Context Fetcher (The Hybrid RAG) ──────────────────────────────────
// We also need local embed fetch to query SurrealDB


#[tauri::command]
pub async fn intercept_and_route(
    db: State<'_, Surreal<Db>>,
    prompt: String,
) -> Result<String, String> {
    // 1. Route the query
    let weights = route_query(&prompt).await;
    let mut context_blocks = String::new();

    // 2. Fetch Web Context if weight is high enough
    if weights.web_search > 0.4 {
        if let Ok(web_ctx) = duckduckgo_search(&prompt).await {
            context_blocks.push_str("--- WEB SEARCH RESULTS ---\n");
            context_blocks.push_str(&web_ctx);
            context_blocks.push_str("\n");
        }
    }

    // 3. Fetch Local DB Context if weight is high enough
    if weights.local_db > 0.3 {
        // Embed the prompt
        let client = Client::new();
        // Nomic-embed-text embedding
        #[derive(Serialize)]
        struct EmbedReq { model: String, input: Vec<String> }
        #[derive(Deserialize)]
        struct EmbedRes { embeddings: Vec<Vec<f32>> }

        let embed_res = client.post("http://localhost:11434/api/embed")
            .json(&EmbedReq {
                model: "nomic-embed-text".into(),
                input: vec![prompt.clone()],
            })
            .send().await;

        if let Ok(res) = embed_res {
            if let Ok(embed_json) = res.json::<EmbedRes>().await {
                if let Some(vector) = embed_json.embeddings.first() {
                    // Query SurrealDB
                    let query = format!(
                        "SELECT source_id.title as source_title, content, vector::distance::cosine(embedding, $vec) AS dist FROM chunk WHERE embedding <|4|> $vec ORDER BY dist ASC;"
                    );
                    if let Ok(mut db_res) = db.query(query).bind(("vec", vector.clone())).await {
                        #[derive(Deserialize, Debug, SurrealValue)]
                        struct RAGResult { source_title: String, content: String, dist: f32 }
                        
                        if let Ok(results) = db_res.take::<Vec<RAGResult>>(0) {
                            if !results.is_empty() {
                                context_blocks.push_str("--- LOCAL KNOWLEDGE BASE SOURCES ---\n");
                                for (i, chunk) in results.iter().enumerate() {
                                    context_blocks.push_str(&format!("[Source {}] (from: {})\n{}\n\n", i + 1, chunk.source_title, chunk.content));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(context_blocks)
}
