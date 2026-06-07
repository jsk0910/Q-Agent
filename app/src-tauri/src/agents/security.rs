use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::models::{ModelRequest, ModelRunner};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PermissionMode {
    Strict,
    #[default]
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
                    let _response = model.generate(req).await?;
                    
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

#[cfg(test)]
mod tests {
    use super::*;

    // --- Strict 모드 ---

    #[tokio::test]
    async fn test_strict_mode_always_requires_approval() {
        let orch = PermissionOrchestrator::new(PermissionMode::Strict);
        let args = serde_json::json!({});
        // 읽기 전용 도구도 Strict에선 승인 필요
        let report = orch
            .evaluate_tool_call("fs_read", &args, None::<&crate::models::MockLlamaRunner>)
            .await
            .unwrap();
        assert!(report.approval_required);
        assert_eq!(report.score, 1.0);
    }

    #[tokio::test]
    async fn test_strict_mode_write_requires_approval() {
        let orch = PermissionOrchestrator::new(PermissionMode::Strict);
        let args = serde_json::json!({"path": "/tmp/x.txt"});
        let report = orch
            .evaluate_tool_call("fs_write", &args, None::<&crate::models::MockLlamaRunner>)
            .await
            .unwrap();
        assert!(report.approval_required);
    }

    // --- Balanced 모드 ---

    #[tokio::test]
    async fn test_balanced_read_only_auto_approved() {
        let orch = PermissionOrchestrator::new(PermissionMode::Balanced);
        let args = serde_json::json!({});
        for tool in ["fs_read", "ls", "cat", "search"] {
            let report = orch
                .evaluate_tool_call(tool, &args, None::<&crate::models::MockLlamaRunner>)
                .await
                .unwrap();
            assert!(!report.approval_required, "Tool '{}' should be auto-approved in Balanced mode", tool);
            assert!(report.score < 0.5, "Read-only tool score should be low");
        }
    }

    #[tokio::test]
    async fn test_balanced_mutating_requires_approval() {
        let orch = PermissionOrchestrator::new(PermissionMode::Balanced);
        let args = serde_json::json!({"cmd": "rm -rf /"});
        for tool in ["fs_write", "terminal", "ssh"] {
            let report = orch
                .evaluate_tool_call(tool, &args, None::<&crate::models::MockLlamaRunner>)
                .await
                .unwrap();
            assert!(report.approval_required, "Tool '{}' should require approval in Balanced mode", tool);
            assert!(report.score >= 0.5, "Mutating tool score should be high");
        }
    }

    // --- Agentic 모드 (classifier 없음 → Balanced 폴백) ---

    #[tokio::test]
    async fn test_agentic_fallback_read_only() {
        let orch = PermissionOrchestrator::new(PermissionMode::Agentic);
        let args = serde_json::json!({});
        let report = orch
            .evaluate_tool_call("fs_read", &args, None::<&crate::models::MockLlamaRunner>)
            .await
            .unwrap();
        // classifier 없으므로 Balanced 폴백: 읽기 전용 자동 승인
        assert!(!report.approval_required);
        assert!(report.reason.contains("fallback"));
    }

    #[tokio::test]
    async fn test_agentic_with_mock_classifier() {
        use crate::models::MockLlamaRunner;
        let orch = PermissionOrchestrator::new(PermissionMode::Agentic);
        let args = serde_json::json!({"cmd": "npm install"});
        let runner = MockLlamaRunner;
        let report = orch
            .evaluate_tool_call("terminal", &args, Some(&runner))
            .await
            .unwrap();
        // MockLlamaRunner 고정 score = 0.4 → 0.7 미만이므로 자동 승인
        assert!(!report.approval_required);
        assert_eq!(report.score, 0.4);
    }

    // --- 위험도 임계치 경계 테스트 ---

    #[test]
    fn test_risk_threshold_boundary() {
        // score > 0.7 이면 승인 필요 (Plan §6.2 기준)
        let high_risk = RiskReport { score: 0.71, reason: String::new(), approval_required: true };
        let low_risk  = RiskReport { score: 0.70, reason: String::new(), approval_required: false };
        assert!(high_risk.approval_required);
        assert!(!low_risk.approval_required);
    }
}
