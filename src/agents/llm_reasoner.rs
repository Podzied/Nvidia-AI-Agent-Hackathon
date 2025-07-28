use crate::types::{AgentContext, AgentMessage, ComplianceResult, MessageType};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;

#[derive(Clone)]
pub struct LlmReasonerAgent {
    agent_id: String,
}

impl LlmReasonerAgent {
    pub fn new() -> Self {
        Self {
            agent_id: "llm-reasoner-001".to_string(),
        }
    }
    
    pub fn generate_explanation(&self, compliance_result: &ComplianceResult) -> String {
        let pii_count = compliance_result.detected_pii.len();
        let compliance_score = compliance_result.compliance_score;
        
        if pii_count == 0 {
            "✅ No PII detected in the text. This content appears to be safe for sharing.".to_string()
        } else if compliance_score >= 0.8 {
            format!("⚠️ Found {} PII item(s) but with good compliance score ({:.1}%). Consider reviewing the detected information.", 
                   pii_count, compliance_score * 100.0)
        } else {
            format!("🚨 Found {} PII item(s) with low compliance score ({:.1}%). Immediate action recommended to protect sensitive information.", 
                   pii_count, compliance_score * 100.0)
        }
    }
}

#[async_trait]
impl super::Agent for LlmReasonerAgent {
    async fn process(&self, message: AgentMessage, _context: &mut AgentContext) -> Result<AgentMessage> {
        match message.message_type {
            MessageType::ComplianceEnforcementResult => {
                // Extract compliance result from the message
                let compliance_result: ComplianceResult = serde_json::from_value(
                    message.payload["compliance_result"].clone()
                ).unwrap_or_default();
                
                // Generate explanation
                let explanation = self.generate_explanation(&compliance_result);
                
                let result = AgentMessage {
                    agent_id: self.agent_id.clone(),
                    message_type: MessageType::LlmReasoningResult,
                    payload: json!({
                        "compliance_result": compliance_result,
                        "llm_explanation": explanation,
                        "reasoning_timestamp": chrono::Utc::now(),
                    }),
                    timestamp: chrono::Utc::now(),
                };
                
                Ok(result)
            }
            _ => {
                Err(anyhow::anyhow!("Unsupported message type for LLM reasoner"))
            }
        }
    }
    
    fn agent_id(&self) -> &str {
        &self.agent_id
    }
}

impl Default for LlmReasonerAgent {
    fn default() -> Self {
        Self::new()
    }
} 