use anyhow::Result;
use super::Database;
use crate::models::entities::{Conversation, Message};

impl Database {
    pub async fn create_conversation(&self, conv: Conversation) -> Result<Conversation> {
        let created: Conversation = self.client
            .create(("conversation", conv.id.clone()))
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
            .create(("message", msg.id.clone()))
            .content(msg)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to save message"))?;
        Ok(created)
    }

    pub async fn list_messages(&self, conversation_id: &str) -> Result<Vec<Message>> {
        let mut response = self.client
            .query("SELECT * FROM message WHERE conversation_id = conversation:$id ORDER BY created_at ASC")
            .bind(("id", conversation_id))
            .await?;
        let msgs: Vec<Message> = response.take(0)?;
        Ok(msgs)
    }
}
