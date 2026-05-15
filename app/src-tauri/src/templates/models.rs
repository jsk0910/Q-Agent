use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PersonaConfig {
    pub name: String,
    pub description: String,
    pub system_prompt: String,
    #[serde(default)]
    pub tone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HarnessConfig {
    pub name: String,
    pub description: String,
    pub security_level: String, // strict, standard, advanced
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub access_paths: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OrchestrationConfig {
    pub name: String,
    pub description: String,
    pub max_iterations: u32,
    pub routing_threshold: f32,
    pub default_model: String,
}
