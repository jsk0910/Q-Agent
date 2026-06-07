use super::{ModelRequest, ModelRunner, StreamEvent};
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: usize,
    stream: bool,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Option<MessageData>,
    delta: Option<MessageData>,
}

#[derive(Deserialize)]
struct MessageData {
    content: Option<String>,
}

#[derive(Serialize)]
struct EmbeddingRequest<'a> {
    model: &'a str,
    input: &'a str,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

pub struct RealLlamaRunner {
    pub base_url: String,
    pub model_name: String,
    pub client: Client,
}

impl RealLlamaRunner {
    pub fn new(base_url: String, model_name: String) -> Self {
        Self {
            base_url,
            model_name,
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl ModelRunner for RealLlamaRunner {
    async fn generate_stream(
        &self,
        req: ModelRequest,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<StreamEvent>>> {
        let (tx, rx) = tokio::sync::mpsc::channel(32);
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let client = self.client.clone();
        
        let request_body = ChatCompletionRequest {
            model: self.model_name.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: req.prompt.clone(),
            }],
            max_tokens: req.max_tokens,
            stream: true,
        };

        tokio::spawn(async move {
            let res = match client.post(&url).json(&request_body).send().await {
                Ok(r) => r,
                Err(e) => {
                    let _ = tx.send(Err(anyhow::anyhow!("Request failed: {}", e))).await;
                    return;
                }
            };

            if !res.status().is_success() {
                let status = res.status();
                let text = res.text().await.unwrap_or_default();
                let _ = tx.send(Err(anyhow::anyhow!("API Error {}: {}", status, text))).await;
                return;
            }

            use futures::StreamExt;
            let mut stream = res.bytes_stream();
            while let Some(chunk_res) = stream.next().await {
                match chunk_res {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line["data: ".len()..];
                                if data == "[DONE]" {
                                    return;
                                }
                                if let Ok(parsed) = serde_json::from_str::<ChatCompletionResponse>(data) {
                                    if let Some(choice) = parsed.choices.first() {
                                        if let Some(delta) = &choice.delta {
                                            if let Some(content) = &delta.content {
                                                if tx.send(Ok(StreamEvent { token: content.clone() })).await.is_err() {
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Err(e) => {
                        let _ = tx.send(Err(anyhow::anyhow!("Stream error: {}", e))).await;
                        return;
                    }
                }
            }
        });

        Ok(rx)
    }

    async fn generate(&self, req: ModelRequest) -> Result<String> {
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let request_body = ChatCompletionRequest {
            model: self.model_name.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: req.prompt,
            }],
            max_tokens: req.max_tokens,
            stream: false,
        };

        let res = self.client.post(&url).json(&request_body).send().await?;
        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("API Error {}: {}", status, text));
        }

        let parsed: ChatCompletionResponse = res.json().await?;
        if let Some(choice) = parsed.choices.first() {
            if let Some(msg) = &choice.message {
                if let Some(content) = &msg.content {
                    return Ok(content.clone());
                }
            }
        }
        Ok(String::new())
    }

    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/embeddings", self.base_url.trim_end_matches('/'));
        let request_body = EmbeddingRequest {
            model: "nomic-embed-text",
            input: text,
        };

        let res = self.client.post(&url).json(&request_body).send().await?;
        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Embed API Error {}: {}", status, text));
        }

        let parsed: EmbeddingResponse = res.json().await?;
        if let Some(data) = parsed.data.first() {
            return Ok(data.embedding.clone());
        }
        Ok(Vec::new())
    }

    fn context_window(&self) -> usize {
        8192
    }

    fn model_id(&self) -> &str {
        &self.model_name
    }
}
