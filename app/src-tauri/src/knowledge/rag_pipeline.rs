use anyhow::Result;
use std::sync::Arc;
use futures::future::join_all;

use crate::knowledge::query_expander::QueryExpander;
use crate::knowledge::bm25_search::{Bm25Searcher, SearchResult};
use crate::knowledge::vector_search::VectorSearcher;
use crate::knowledge::provider_chain::ProviderChain;
use crate::knowledge::reranker::CrossEncoderReranker;
use crate::knowledge::citation::{CitationEngine, CitedDocument};

pub struct RagPipeline {
    query_expander: Arc<QueryExpander>,
    bm25: Arc<Bm25Searcher>,
    vector: Arc<VectorSearcher>,
    provider: Arc<ProviderChain>,
    reranker: Arc<CrossEncoderReranker>,
    citation: Arc<CitationEngine>,
}

impl RagPipeline {
    pub fn new(
        query_expander: Arc<QueryExpander>,
        bm25: Arc<Bm25Searcher>,
        vector: Arc<VectorSearcher>,
        provider: Arc<ProviderChain>,
        reranker: Arc<CrossEncoderReranker>,
        citation: Arc<CitationEngine>,
    ) -> Self {
        Self {
            query_expander,
            bm25,
            vector,
            provider,
            reranker,
            citation,
        }
    }

    pub async fn process_query(&self, user_query: &str) -> Result<(String, Vec<CitedDocument>)> {
        // 1. Query Expansion
        let expanded_queries = self.query_expander.expand_query(user_query).await?;
        
        // 2. Parallel Search
        let mut search_futures = Vec::new();
        
        for q in &expanded_queries {
            // BM25
            let bm25_arc = Arc::clone(&self.bm25);
            let q_clone = q.clone();
            search_futures.push(tokio::spawn(async move {
                bm25_arc.search(&q_clone, 5).unwrap_or_default()
            }));

            // Vector (Assuming mock embedding here)
            let vector_arc = Arc::clone(&self.vector);
            search_futures.push(tokio::spawn(async move {
                vector_arc.search(&[0.0; 768], 5).await.unwrap_or_default()
            }));

            // Provider (External Web)
            let provider_arc = Arc::clone(&self.provider);
            let q_clone2 = q.clone();
            search_futures.push(tokio::spawn(async move {
                provider_arc.search_web(&q_clone2, 5).await.unwrap_or_default()
            }));
        }

        let search_results = join_all(search_futures).await;
        
        // 3. Deduplication & Merging
        let mut merged_results: Vec<SearchResult> = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        for res in search_results {
            if let Ok(docs) = res {
                for doc in docs {
                    if !seen_ids.contains(&doc.id) {
                        seen_ids.insert(doc.id.clone());
                        merged_results.push(doc);
                    }
                }
            }
        }

        // 4. Re-ranking
        let mut reranked = self.reranker.rerank(user_query, merged_results)?;
        
        // 5. Context Budgeting (e.g. limit to top 10)
        reranked.truncate(10);

        // 6. Citation Assignment
        let cited_docs = self.citation.assign_citations(reranked)?;
        let context_block = self.citation.build_context_block(&cited_docs);

        Ok((context_block, cited_docs))
    }
}
