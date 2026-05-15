use anyhow::Result;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

pub mod checkpoint;

// Q-Agent 데이터베이스 연결 관리 구조체
#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Db>,
}

impl Database {
    /// 로컬 데이터베이스 초기화 (개발 중에는 메모리, 프로덕션은 파일 사용 가능)
    pub async fn init(path: &str) -> Result<Self> {
        let client = Surreal::new::<Mem>(()).await?;

        // 네임스페이스 및 데이터베이스 선택
        client.use_ns("qagent").use_db("core").await?;

        // 스키마 마이그레이션 적용
        let schema_query = include_str!("schema.surql");
        client.query(schema_query).await?;

        tracing::info!("SurrealDB initialized successfully at {}", path);

        Ok(Self { client })
    }
}
