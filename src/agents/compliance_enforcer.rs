use crate::types::{AgentContext, AgentMessage, MessageType, PiiDetection, PiiType};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ComplianceEnforcerAgent {
    agent_id: String,
    redaction_rules: HashMap<PiiType, String>,
    masking_patterns: HashMap<PiiType, String>,
}

impl ComplianceEnforcerAgent {
    pub fn new() -> Self {
        let mut redaction_rules = HashMap::new();
        redaction_rules.insert(PiiType::Email, "[EMAIL]".to_string());
        redaction_rules.insert(PiiType::PhoneNumber, "[PHONE]".to_string());
        redaction_rules.insert(PiiType::SocialSecurityNumber, "[SSN]".to_string());
        redaction_rules.insert(PiiType::CreditCardNumber, "[CC]".to_string());
        redaction_rules.insert(PiiType::IpAddress, "[IP]".to_string());
        redaction_rules.insert(PiiType::DateOfBirth, "[DOB]".to_string());
        redaction_rules.insert(PiiType::Address, "[ADDRESS]".to_string());
        redaction_rules.insert(PiiType::Name, "[NAME]".to_string());
        redaction_rules.insert(PiiType::Unknown, "[PII]".to_string());
        
        let mut masking_patterns = HashMap::new();
        masking_patterns.insert(PiiType::Email, "***@***.***".to_string());
        masking_patterns.insert(PiiType::PhoneNumber, "***-***-****".to_string());
        masking_patterns.insert(PiiType::SocialSecurityNumber, "***-**-****".to_string());
        masking_patterns.insert(PiiType::CreditCardNumber, "****-****-****-****".to_string());
        masking_patterns.insert(PiiType::IpAddress, "***.***.***.***".to_string());
        masking_patterns.insert(PiiType::DateOfBirth, "**/**/****".to_string());
        masking_patterns.insert(PiiType::Address, "[ADDRESS REDACTED]".to_string());
        masking_patterns.insert(PiiType::Name, "*** ***".to_string());
        masking_patterns.insert(PiiType::Unknown, "[REDACTED]".to_string());
        
        Self {
            agent_id: "compliance-enforcer-001".to_string(),
            redaction_rules,
            masking_patterns,
        }
    }
    
    fn apply_redaction(&self, text: &str, detections: &[PiiDetection]) -> String {
        let mut redacted_text = text.to_string();
        let mut offset = 0;
        
        // Sort detections by start position to handle overlapping
        let mut sorted_detections = detections.to_vec();
        sorted_detections.sort_by_key(|d| d.start_pos);
        
        for detection in sorted_detections {
            let start = detection.start_pos + offset;
            let end = detection.end_pos + offset;
            
            if start < redacted_text.len() && end <= redacted_text.len() {
                let replacement = self.redaction_rules.get(&detection.pii_type)
                    .unwrap_or(&"[REDACTED]".to_string())
                    .clone();
                
                redacted_text.replace_range(start..end, &replacement);
                let new_offset = offset as i32 + replacement.len() as i32 - (end - start) as i32;
                offset = new_offset.max(0) as usize;
            }
        }
        
        redacted_text
    }
    
    fn calculate_compliance_score(&self, detections: &[PiiDetection]) -> f32 {
        if detections.is_empty() {
            return 1.0; // Perfect compliance if no PII found
        }
        
        // Calculate score based on detection confidence and number of PII items
        let total_confidence: f32 = detections.iter().map(|d| d.confidence).sum();
        let avg_confidence = total_confidence / detections.len() as f32;
        
        // Penalize for high number of PII detections
        let pii_penalty = (detections.len() as f32 * 0.1).min(0.5);
        
        (avg_confidence - pii_penalty).max(0.0)
    }
    
    fn generate_recommendations(&self, detections: &[PiiDetection]) -> Vec<String> {
        let mut recommendations = vec![];
        
        if detections.is_empty() {
            recommendations.push("✅ No PII detected - text is compliant".to_string());
            return recommendations;
        }
        
        // Group detections by type
        let mut pii_counts: HashMap<PiiType, usize> = HashMap::new();
        for detection in detections {
            *pii_counts.entry(detection.pii_type.clone()).or_insert(0) += 1;
        }
        
        for (pii_type, count) in pii_counts {
            recommendations.push(format!(
                "⚠️  Found {} {} items - consider redaction or masking",
                count,
                format!("{:?}", pii_type).to_lowercase()
            ));
        }
        
        recommendations.push("🔒 Apply appropriate data protection measures".to_string());
        recommendations.push("📋 Review data handling policies".to_string());
        
        recommendations
    }
}

#[async_trait]
impl crate::agents::Agent for ComplianceEnforcerAgent {
    async fn process(&self, message: AgentMessage, _context: &mut AgentContext) -> Result<AgentMessage> {
        match message.message_type {
            MessageType::ComplianceEnforcementRequest => {
                let text = message.payload["text"].as_str().unwrap_or("");
                let detections: Vec<PiiDetection> = serde_json::from_value(
                    message.payload["detections"].clone()
                ).unwrap_or_default();
                
                let redacted_text = self.apply_redaction(text, &detections);
                let compliance_score = self.calculate_compliance_score(&detections);
                let recommendations = self.generate_recommendations(&detections);
                
                let result = AgentMessage {
                    agent_id: self.agent_id.clone(),
                    message_type: MessageType::ComplianceEnforcementResult,
                    payload: json!({
                        "original_text": text,
                        "redacted_text": redacted_text,
                        "detected_pii": detections,
                        "compliance_score": compliance_score,
                        "recommendations": recommendations,
                        "enforcement_timestamp": chrono::Utc::now(),
                    }),
                    timestamp: chrono::Utc::now(),
                };
                
                Ok(result)
            }
            _ => {
                Err(anyhow::anyhow!("Unsupported message type for compliance enforcer"))
            }
        }
    }
    
    fn agent_id(&self) -> &str {
        &self.agent_id
    }
}

impl Default for ComplianceEnforcerAgent {
    fn default() -> Self {
        Self::new()
    }
} 