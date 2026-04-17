use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatOptions {
    pub temperature: f64,
}

#[async_trait]
pub trait ModelRunner: Send + Sync {
    async fn chat(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
        options: ChatOptions,
    ) -> Result<String, Box<dyn Error + Send + Sync>>;

    async fn list_models(&self) -> Result<Vec<String>, Box<dyn Error + Send + Sync>>;
    
    // Preparation for "separated execution" - could pull model blobs/weights
    async fn pull_model(&self, model: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct OllamaRunner {
    base_url: String,
    client: reqwest::Client,
}

impl OllamaRunner {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl ModelRunner for OllamaRunner {
    async fn chat(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
        options: ChatOptions,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        #[derive(Serialize)]
        struct OllamaRequest {
            model: String,
            messages: Vec<ChatMessage>,
            stream: bool,
            options: OllamaOptions,
        }
        #[derive(Serialize)]
        struct OllamaOptions {
            temperature: f64,
        }
        #[derive(Deserialize)]
        struct OllamaResponse {
            message: Option<ChatMessage>,
        }

        let payload = OllamaRequest {
            model: model.to_string(),
            messages,
            stream: false, // We'll handle streaming differently in commands if needed, or expand trait
            options: OllamaOptions {
                temperature: options.temperature,
            },
        };

        let res = self.client
            .post(format!("{}/api/chat", self.base_url))
            .json(&payload)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!("Ollama error: {}", res.status()).into());
        }

        let data: OllamaResponse = res.json().await?;
        Ok(data.message.map(|m| m.content).unwrap_or_default())
    }

    async fn list_models(&self) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        #[derive(Deserialize)]
        struct ModelInfo { name: String }
        #[derive(Deserialize)]
        struct ModelsResponse { models: Vec<ModelInfo> }

        let res = self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?;

        let body: ModelsResponse = res.json().await?;
        Ok(body.models.into_iter().map(|m| m.name).collect())
    }

    async fn pull_model(&self, _model: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Implement in refactored chat.rs for better streaming handle, 
        // or add a progress callback to this trait eventually.
        Ok(())
    }
}
