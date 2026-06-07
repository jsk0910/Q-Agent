pub mod manager;
pub mod entities;
pub mod llama;

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
pub use llama::RealLlamaRunner;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_generate_returns_string() {
        let runner = MockLlamaRunner;
        let req = ModelRequest { prompt: "hello".into(), max_tokens: 50 };
        let result = runner.generate(req).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn test_mock_embed_returns_vector() {
        let runner = MockLlamaRunner;
        let embedding = runner.embed("test text").await.unwrap();
        assert_eq!(embedding.len(), 3);
        assert_eq!(embedding[0], 0.1_f32);
    }

    #[test]
    fn test_mock_model_metadata() {
        let runner = MockLlamaRunner;
        assert_eq!(runner.model_id(), "mock-llama-3b");
        assert_eq!(runner.context_window(), 8192);
        assert!(!runner.supports_vision());
    }

    #[tokio::test]
    async fn test_mock_generate_stream_emits_tokens() {
        let runner = MockLlamaRunner;
        let req = ModelRequest { prompt: "stream test".into(), max_tokens: 100 };
        let mut rx = runner.generate_stream(req).await.unwrap();
        let mut tokens = Vec::new();
        while let Some(Ok(event)) = rx.recv().await {
            tokens.push(event.token);
        }
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0], "Hello");
        // 마지막 토큰은 프롬프트 길이 포함
        let last = tokens.last().unwrap();
        assert!(last.contains("Prompt len"));
    }

    #[tokio::test]
    async fn test_mock_stream_prompt_length_reflected() {
        let runner = MockLlamaRunner;
        let prompt = "a".repeat(42);
        let req = ModelRequest { prompt, max_tokens: 10 };
        let mut rx = runner.generate_stream(req).await.unwrap();
        let mut last_token = String::new();
        while let Some(Ok(event)) = rx.recv().await {
            last_token = event.token;
        }
        assert!(last_token.contains("42"), "Prompt length should be reflected in stream: {}", last_token);
    }

    #[tokio::test]
    async fn test_model_request_serde() {
        let req = ModelRequest { prompt: "test".into(), max_tokens: 128 };
        let json = serde_json::to_string(&req).unwrap();
        let restored: ModelRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.prompt, "test");
        assert_eq!(restored.max_tokens, 128);
    }
}
