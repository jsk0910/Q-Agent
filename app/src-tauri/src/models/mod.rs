pub mod manager;
pub mod entities;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequest {
    pub prompt: String,
    pub max_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    pub token: String,
}

#[async_trait::async_trait]
pub trait ModelRunner: Send + Sync {
    async fn generate_stream(
        &self,
        req: ModelRequest,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<StreamEvent>>>;
    async fn generate(&self, req: ModelRequest) -> Result<String>;
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    fn supports_vision(&self) -> bool {
        false
    }
    fn context_window(&self) -> usize;
    fn model_id(&self) -> &str;
}

pub struct MockLlamaRunner;

#[async_trait::async_trait]
impl ModelRunner for MockLlamaRunner {
    async fn generate_stream(
        &self,
        req: ModelRequest,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<StreamEvent>>> {
        let (tx, rx) = tokio::sync::mpsc::channel(32);

        let prompt_len = req.prompt.len();
        tokio::spawn(async move {
            let tokens = vec![
                "Hello".to_string(),
                " from".to_string(),
                " Mock".to_string(),
                " Llama".to_string(),
                " Engine!".to_string(),
                format!(" (Prompt len: {})", prompt_len),
            ];

            for token in tokens {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                if tx.send(Ok(StreamEvent { token })).await.is_err() {
                    break;
                }
            }
        });

        Ok(rx)
    }

    async fn generate(&self, _req: ModelRequest) -> Result<String> {
        Ok("Mock generation response".to_string())
    }

    async fn embed(&self, _text: &str) -> Result<Vec<f32>> {
        Ok(vec![0.1, 0.2, 0.3])
    }

    fn context_window(&self) -> usize {
        8192
    }

    fn model_id(&self) -> &str {
        "mock-llama-3b"
    }
}
