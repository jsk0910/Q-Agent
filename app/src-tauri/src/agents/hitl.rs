use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitlRequestPayload {
    pub id: String,
    pub action_type: String,
    pub description: String,
    pub command: Option<String>,
    pub path: Option<String>,
    pub risk_score: Option<f32>,
}

#[derive(Default, Clone)]
pub struct HitlManager {
    requests: Arc<Mutex<HashMap<String, oneshot::Sender<bool>>>>,
}

impl HitlManager {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_request(&self, id: String, sender: oneshot::Sender<bool>) {
        let mut map = self.requests.lock().await;
        map.insert(id, sender);
    }

    pub async fn resolve_request(&self, id: &str, approved: bool) -> Result<(), String> {
        let mut map = self.requests.lock().await;
        if let Some(sender) = map.remove(id) {
            let _ = sender.send(approved);
            Ok(())
        } else {
            Err(format!("HITL request id '{}' not found.", id))
        }
    }
}
