use crate::types::{
    AgentContext, AgentMessage, ChatMessage, ChatSession, ComplianceResult, ComplianceViolation,
    MessageType, PiiDetection, RiskLevel, Severity, ViolationType,
};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ChatbotComplianceAgent {
    agent_id: String,
    active_sessions: Arc<Mutex<HashMap<String, ChatSession>>>,
    compliance_rules: HashMap<String, Severity>,
}

impl ChatbotComplianceAgent {
    pub fn new() -> Self {
        let mut compliance_rules = HashMap::new();
        compliance_rules.insert("SSN".to_string(), Severity::Critical);
        compliance_rules.insert("CreditCard".to_string(), Severity::High);
        compliance_rules.insert("Email".to_string(), Severity::Medium);
        compliance_rules.insert("Phone".to_string(), Severity::Medium);
        compliance_rules.insert("Address".to_string(), Severity::Low);
        
        Self {
            agent_id: "chatbot-compliance-001".to_string(),
            active_sessions: Arc::new(Mutex::new(HashMap::new())),
            compliance_rules,
        }
    }
    
    pub async fn process_chat_message(&self, message: ChatMessage) -> Result<ComplianceResult> {
        // Check for PII in the message
        let pii_detections = self.detect_pii_in_message(&message.content).await?;
        
        // Create compliance result
        let redacted_content = self.redact_pii(&message.content, &pii_detections);
        let compliance_score = self.calculate_compliance_score(&pii_detections);
        let recommendations = self.generate_recommendations(&pii_detections);
        
        let compliance_result = ComplianceResult {
            original_text: message.content.clone(),
            redacted_text: redacted_content,
            detected_pii: pii_detections.clone(),
            compliance_score,
            recommendations,
        };
        
        // Update session with compliance info
        self.update_session_compliance(&message, &compliance_result).await?;
        
        Ok(compliance_result)
    }
    
    async fn detect_pii_in_message(&self, content: &str) -> Result<Vec<PiiDetection>> {
        // Use the existing PII detection logic
        // This would integrate with your PiiScannerAgent
        let mut detections = vec![];
        
        // Simple regex-based detection for demo
        let email_regex = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b")?;
        let phone_regex = regex::Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b")?;
        let ssn_regex = regex::Regex::new(r"\b\d{3}-\d{2}-\d{4}\b")?;
        let cc_regex = regex::Regex::new(r"\b\d{4}[- ]?\d{4}[- ]?\d{4}[- ]?\d{4}\b")?;
        
        for mat in email_regex.find_iter(content) {
            detections.push(PiiDetection {
                pii_type: crate::types::PiiType::Email,
                confidence: 0.9,
                start_pos: mat.start(),
                end_pos: mat.end(),
                value: mat.as_str().to_string(),
            });
        }
        
        for mat in phone_regex.find_iter(content) {
            detections.push(PiiDetection {
                pii_type: crate::types::PiiType::PhoneNumber,
                confidence: 0.9,
                start_pos: mat.start(),
                end_pos: mat.end(),
                value: mat.as_str().to_string(),
            });
        }
        
        for mat in ssn_regex.find_iter(content) {
            detections.push(PiiDetection {
                pii_type: crate::types::PiiType::SocialSecurityNumber,
                confidence: 0.95,
                start_pos: mat.start(),
                end_pos: mat.end(),
                value: mat.as_str().to_string(),
            });
        }
        
        for mat in cc_regex.find_iter(content) {
            detections.push(PiiDetection {
                pii_type: crate::types::PiiType::CreditCardNumber,
                confidence: 0.9,
                start_pos: mat.start(),
                end_pos: mat.end(),
                value: mat.as_str().to_string(),
            });
        }
        
        Ok(detections)
    }
    
    fn redact_pii(&self, content: &str, detections: &[PiiDetection]) -> String {
        let mut redacted = content.to_string();
        let mut offset = 0;
        
        let mut sorted_detections = detections.to_vec();
        sorted_detections.sort_by_key(|d| d.start_pos);
        
        for detection in sorted_detections {
            let start = detection.start_pos + offset;
            let end = detection.end_pos + offset;
            
            if start < redacted.len() && end <= redacted.len() {
                let replacement = match detection.pii_type {
                    crate::types::PiiType::Email => "[EMAIL]",
                    crate::types::PiiType::PhoneNumber => "[PHONE]",
                    crate::types::PiiType::SocialSecurityNumber => "[SSN]",
                    crate::types::PiiType::CreditCardNumber => "[CC]",
                    _ => "[REDACTED]",
                };
                
                redacted.replace_range(start..end, replacement);
                let new_offset = offset as i32 + replacement.len() as i32 - (end - start) as i32;
                offset = new_offset.max(0) as usize;
            }
        }
        
        redacted
    }
    
    fn calculate_compliance_score(&self, detections: &[PiiDetection]) -> f32 {
        if detections.is_empty() {
            return 1.0;
        }
        
        let total_confidence: f32 = detections.iter().map(|d| d.confidence).sum();
        let avg_confidence = total_confidence / detections.len() as f32;
        let pii_penalty = (detections.len() as f32 * 0.1).min(0.5);
        
        (avg_confidence - pii_penalty).max(0.0)
    }
    
    fn generate_recommendations(&self, detections: &[PiiDetection]) -> Vec<String> {
        let mut recommendations = vec![];
        
        if detections.is_empty() {
            recommendations.push("✅ Message is compliant".to_string());
            return recommendations;
        }
        
        for detection in detections {
            let severity = self.compliance_rules.get(&format!("{:?}", detection.pii_type))
                .unwrap_or(&Severity::Medium);
            
            recommendations.push(format!(
                "⚠️  {} detected (Severity: {:?}) - Consider redaction",
                format!("{:?}", detection.pii_type),
                severity
            ));
        }
        
        recommendations.push("🔒 Apply real-time redaction".to_string());
        recommendations.push("📋 Log compliance violation".to_string());
        
        recommendations
    }
    
    async fn update_session_compliance(&self, message: &ChatMessage, compliance_result: &ComplianceResult) -> Result<()> {
        let mut sessions = self.active_sessions.lock().await;
        
        let session = sessions.entry(message.session_id.clone()).or_insert_with(|| ChatSession {
            session_id: message.session_id.clone(),
            user_id: message.user_id.clone(),
            messages: vec![],
            compliance_violations: vec![],
            risk_level: RiskLevel::Safe,
        });
        
        // Add message to session
        session.messages.push(message.clone());
        
        // Check for compliance violations
        if !compliance_result.detected_pii.is_empty() {
            let violation = ComplianceViolation {
                violation_type: ViolationType::PiiExposure,
                severity: self.determine_violation_severity(&compliance_result.detected_pii),
                message: format!("PII detected in message: {}", message.content),
                timestamp: chrono::Utc::now(),
                pii_detected: compliance_result.detected_pii.clone(),
            };
            
            session.compliance_violations.push(violation);
            session.risk_level = self.calculate_session_risk_level(session);
        }
        
        Ok(())
    }
    
    fn determine_violation_severity(&self, detections: &[PiiDetection]) -> Severity {
        let mut max_severity = Severity::Low;
        
        for detection in detections {
            let severity = self.compliance_rules.get(&format!("{:?}", detection.pii_type))
                .unwrap_or(&Severity::Medium);
            
            max_severity = match (max_severity, severity) {
                (Severity::Critical, _) | (_, Severity::Critical) => Severity::Critical,
                (Severity::High, _) | (_, Severity::High) => Severity::High,
                (Severity::Medium, _) | (_, Severity::Medium) => Severity::Medium,
                _ => Severity::Low,
            };
        }
        
        max_severity
    }
    
    fn calculate_session_risk_level(&self, session: &ChatSession) -> RiskLevel {
        let critical_violations = session.compliance_violations.iter()
            .filter(|v| matches!(v.severity, Severity::Critical))
            .count();
        
        let high_violations = session.compliance_violations.iter()
            .filter(|v| matches!(v.severity, Severity::High))
            .count();
        
        match (critical_violations, high_violations) {
            (c, _) if c > 0 => RiskLevel::Critical,
            (_, h) if h > 2 => RiskLevel::High,
            (_, h) if h > 0 => RiskLevel::Medium,
            _ => RiskLevel::Low,
        }
    }
}

#[async_trait]
impl super::Agent for ChatbotComplianceAgent {
    async fn process(&self, message: AgentMessage, _context: &mut AgentContext) -> Result<AgentMessage> {
        match message.message_type {
            MessageType::ChatMessageReceived => {
                let chat_message: ChatMessage = serde_json::from_value(
                    message.payload["chat_message"].clone()
                ).unwrap_or_default();
                
                let compliance_result = self.process_chat_message(chat_message.clone()).await?;
                
                let result = AgentMessage {
                    agent_id: self.agent_id.clone(),
                    message_type: MessageType::ComplianceViolationAlert,
                    payload: json!({
                        "chat_message": chat_message,
                        "compliance_result": compliance_result,
                        "session_risk_level": self.active_sessions.lock().await
                            .get(&chat_message.session_id)
                            .map(|s| s.risk_level.clone()),
                        "timestamp": chrono::Utc::now(),
                    }),
                    timestamp: chrono::Utc::now(),
                };
                
                Ok(result)
            }
            _ => {
                Err(anyhow::anyhow!("Unsupported message type for chatbot compliance agent"))
            }
        }
    }
    
    fn agent_id(&self) -> &str {
        &self.agent_id
    }
}

impl Default for ChatbotComplianceAgent {
    fn default() -> Self {
        Self::new()
    }
} 