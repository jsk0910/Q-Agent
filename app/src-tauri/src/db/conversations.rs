use anyhow::Result;
use super::Database;
use crate::models::entities::{Conversation, Message};

impl Database {
    pub async fn create_conversation(&self, conv: Conversation) -> Result<Conversation> {
        let created: Conversation = self.client
            .create(("conversation", conv.id.id.to_raw()))
            .content(conv)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to create conversation"))?;
        Ok(created)
    }

    pub async fn list_conversations(&self, project_id: Option<&str>) -> Result<Vec<Conversation>> {
        let mut query = "SELECT * FROM conversation".to_string();
        if let Some(pid) = project_id {
            query.push_str(&format!(" WHERE project_id = project:{}", pid));
        }
        query.push_str(" ORDER BY created_at DESC");
        
        let mut response = self.client.query(query).await?;
        let convs: Vec<Conversation> = response.take(0)?;
        Ok(convs)
    }

    pub async fn save_message(&self, msg: Message) -> Result<Message> {
        let created: Message = self.client
            .create(("message", msg.id.id.to_raw()))
            .content(msg)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to save message"))?;
        Ok(created)
    }

    pub async fn list_messages(&self, conversation_id: &str) -> Result<Vec<Message>> {
        let mut response = self.client
            .query("SELECT * FROM message WHERE conversation_id = type::thing('conversation', $id) ORDER BY created_at ASC")
            .bind(("id", conversation_id.to_string()))
            .await?;
        let msgs: Vec<Message> = response.take(0)?;
        Ok(msgs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::entities::{Conversation, Message};
    use chrono::Utc;
    use surrealdb::sql::Thing;

    async fn setup_db() -> Database {
        Database::init("memory").await.expect("In-memory DB should init")
    }

    // --- Conversation CRUD ---

    #[tokio::test]
    async fn test_create_and_list_conversation() {
        let db = setup_db().await;
        let conv = Conversation {
            id: Thing::from(("conversation", "conv-1")),
            project_id: Some(Thing::from(("project", "proj-1"))),
            title: "Test Conversation".into(),
            mode: "chat".into(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        };
        let created = db.create_conversation(conv.clone()).await.unwrap();
        assert_eq!(created.id.id.to_raw(), "conv-1");
        assert_eq!(created.title, "Test Conversation");

        let all = db.list_conversations(None).await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id.id.to_raw(), "conv-1");
    }

    #[tokio::test]
    async fn test_list_conversations_filter_by_project() {
        let db = setup_db().await;
        for (id, pid) in [("c1", "p1"), ("c2", "p2"), ("c3", "p1")] {
            db.create_conversation(Conversation {
                id: Thing::from(("conversation", id)),
                project_id: Some(Thing::from(("project", pid))),
                title: format!("conv {}", id),
                mode: "chat".into(),
                created_at: Utc::now().into(),
                updated_at: Utc::now().into(),
            }).await.unwrap();
        }
        let p1_convs = db.list_conversations(Some("p1")).await.unwrap();
        // 프로젝트 p1에 속한 대화만 반환되어야 함
        assert_eq!(p1_convs.len(), 2);
    }

    // --- Message CRUD ---

    #[tokio::test]
    async fn test_save_and_list_messages() {
        let db = setup_db().await;
        // 대화 먼저 생성
        db.create_conversation(Conversation {
            id: Thing::from(("conversation", "conv-msg")),
            project_id: None,
            title: "msg test".into(),
            mode: "chat".into(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        }).await.unwrap();

        // 메시지 저장
        for (i, role) in ["user", "assistant", "user"].iter().enumerate() {
            db.save_message(Message {
                id: Thing::from(("message", format!("msg-{}", i).as_str())),
                conversation_id: Thing::from(("conversation", "conv-msg")),
                role: role.to_string(),
                content: format!("message {}", i),
                citations: vec![],
                thought_trace: vec![],
                artifact_ids: vec![],
                model_used: None,
                tokens_used: None,
                duration_ms: None,
                created_at: Utc::now().into(),
            }).await.unwrap();
        }

        let msgs = db.list_messages("conv-msg").await.unwrap();
        assert_eq!(msgs.len(), 3);
        // 첫 메시지는 user
        assert_eq!(msgs[0].role, "user");
    }

    #[tokio::test]
    async fn test_list_messages_empty_conversation() {
        let db = setup_db().await;
        // 존재하지 않는 대화 ID로 조회 → 빈 벡터 반환
        let msgs = db.list_messages("nonexistent").await.unwrap();
        assert!(msgs.is_empty());
    }
}
