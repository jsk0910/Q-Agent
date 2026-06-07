use anyhow::Result;

use std::sync::Arc;
use crate::models::{ModelRunner, ModelRequest};

#[derive(Clone)]
pub struct QueryExpander {
    model_runner: Arc<dyn ModelRunner>,
}

impl QueryExpander {
    pub fn new(model_runner: Arc<dyn ModelRunner>) -> Self {
        Self { model_runner }
    }

    /// Expands a single user query into 3-5 sub-queries for broader search context.
    pub async fn expand_query(&self, query: &str) -> Result<Vec<String>> {
        let prompt = format!(
            "You are an expert search query expander. Given the user's query, generate 3 to 5 alternative queries that cover different aspects, synonyms, or related concepts to help retrieve comprehensive information. 
Return ONLY the queries, one per line. Do not add numbers or bullet points.

User query: {}
Expanded queries:",
            query
        );

        let req = ModelRequest {
            prompt,
            max_tokens: 256,
        };

        let response = self.model_runner.generate(req).await?;
        let sub_queries: Vec<String> = response
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        // If generation fails or is empty, fallback to the original query
        if sub_queries.is_empty() {
            Ok(vec![query.to_string()])
        } else {
            Ok(sub_queries)
        }
    }
}
