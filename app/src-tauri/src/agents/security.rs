use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::models::{ModelRequest, ModelRunner};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionMode {
    Strict,
    Balanced,
    Agentic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskReport {
    pub score: f32,
    pub reason: String,
    pub approval_required: bool,
}

pub struct PermissionOrchestrator {
    mode: PermissionMode,
}

impl PermissionOrchestrator {
    pub fn new(mode: PermissionMode) -> Self {
        Self { mode }
    }

    /// 도구 호출의 위험도를 평가하고 승인 필요 여부를 결정합니다.
    pub async fn evaluate_tool_call(
        &self,
        tool_name: &str,
        args: &serde_json::Value,
        classifier_model: Option<&impl ModelRunner>,
    ) -> Result<RiskReport> {
        match self.mode {
            PermissionMode::Strict => Ok(RiskReport {
                score: 1.0,
                reason: "Strict mode: All tool calls require approval".to_string(),
                approval_required: true,
            }),
            PermissionMode::Balanced => {
                let is_read_only = matches!(tool_name, "fs_read" | "ls" | "cat" | "search");
                Ok(RiskReport {
                    score: if is_read_only { 0.1 } else { 0.8 },
                    reason: if is_read_only { "Read-only tool" } else { "Mutating tool" }.to_string(),
                    approval_required: !is_read_only,
                })
            }
            PermissionMode::Agentic => {
                if let Some(model) = classifier_model {
                    // Qwen 1.5B 등을 활용한 실제 위험도 분류 로직 (Mock)
                    let prompt = format!("Evaluate the risk of this tool call:\nTool: {}\nArgs: {}\nRisk Score (0.0-1.0):", tool_name, args);
                    let req = ModelRequest { prompt, max_tokens: 10 };
                    let response = model.generate(req).await?;
                    
                    // 정규식 등으로 점수 파싱 (Mock)
                    let score = 0.4; // 임시 고정값
                    Ok(RiskReport {
                        score,
                        reason: "Agentic risk classifier evaluation".to_string(),
                        approval_required: score > 0.7,
                    })
                } else {
                    // 분류 모델이 없을 경우 Balanced 모드로 폴백
                    let is_read_only = matches!(tool_name, "fs_read" | "ls" | "cat" | "search");
                    Ok(RiskReport {
                        score: if is_read_only { 0.1 } else { 0.8 },
                        reason: "Classifier missing, fallback to balanced logic".to_string(),
                        approval_required: !is_read_only,
                    })
                }
            }
        }
    }
}
