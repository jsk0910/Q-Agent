use anyhow::Result;
use crate::knowledge::bm25_search::SearchResult;

#[derive(Debug, Clone)]
pub struct CitedDocument {
    pub id: String,
    pub content: String,
    pub source: String,
    pub citation_id: usize, // e.g., 1 for [1]
    pub confidence_stars: usize, // 1 to 5 stars
}

pub struct CitationEngine;

impl CitationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn assign_citations(&self, documents: Vec<SearchResult>) -> Result<Vec<CitedDocument>> {
        let mut cited_docs = Vec::new();

        for (index, doc) in documents.into_iter().enumerate() {
            let citation_id = index + 1;
            
            // Calculate confidence stars (1 to 5) based on score and source
            // E.g., if score is > 0.8, 5 stars, etc.
            let stars = if doc.score > 0.9 {
                5
            } else if doc.score > 0.7 {
                4
            } else if doc.score > 0.5 {
                3
            } else if doc.score > 0.3 {
                2
            } else {
                1
            };

            cited_docs.push(CitedDocument {
                id: doc.id,
                content: doc.content,
                source: doc.source,
                citation_id,
                confidence_stars: stars,
            });
        }

        Ok(cited_docs)
    }

    /// Formats the context block to be injected into the LLM prompt
    pub fn build_context_block(&self, cited_docs: &[CitedDocument]) -> String {
        let mut block = String::new();
        block.push_str("Use the following sources to answer the query. Cite them using [N] notation.\n\n");
        for doc in cited_docs {
            block.push_str(&format!("[{}] (Source: {}, Confidence: {}/5 stars)\n{}\n\n", 
                doc.citation_id, doc.source, doc.confidence_stars, doc.content));
        }
        block
    }
}
