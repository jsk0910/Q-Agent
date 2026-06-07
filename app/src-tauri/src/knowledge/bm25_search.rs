use anyhow::Result;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, TEXT, STORED, Value};
use tantivy::{doc, Index, IndexReader};

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub content: String,
    pub score: f32,
    pub source: String,
}

pub struct Bm25Searcher {
    index: Index,
    reader: IndexReader,
}

impl Bm25Searcher {
    pub fn new() -> Result<Self> {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("id", TEXT | STORED);
        schema_builder.add_text_field("content", TEXT | STORED);
        let schema = schema_builder.build();

        // Use RamDirectory for in-memory indexing, or MmapDirectory for persistent
        let index = Index::create_in_ram(schema.clone());
        let reader = index.reader()?;

        Ok(Self { index, reader })
    }

    pub fn add_document(&self, id: &str, content: &str) -> Result<()> {
        let mut index_writer = self.index.writer(50_000_000)?;
        let schema = self.index.schema();
        let id_field = schema.get_field("id").unwrap();
        let content_field = schema.get_field("content").unwrap();

        index_writer.add_document(doc!(
            id_field => id,
            content_field => content
        ))?;
        index_writer.commit()?;
        Ok(())
    }

    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let searcher = self.reader.searcher();
        let schema = self.index.schema();
        let content_field = schema.get_field("content").unwrap();
        let id_field = schema.get_field("id").unwrap();

        let query_parser = QueryParser::for_index(&self.index, vec![content_field]);
        let query = query_parser.parse_query(query_str)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let retrieved_doc: tantivy::TantivyDocument = searcher.doc(doc_address)?;
            let id = retrieved_doc.get_first(id_field).and_then(|f| f.as_str()).unwrap_or("").to_string();
            let content = retrieved_doc.get_first(content_field).and_then(|f| f.as_str()).unwrap_or("").to_string();
            results.push(SearchResult {
                id,
                content,
                score,
                source: "local_bm25".to_string(),
            });
        }

        Ok(results)
    }
}
