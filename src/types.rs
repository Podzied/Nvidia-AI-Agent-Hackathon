use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiDetection {
    pub pii_type: PiiType,
    pub confidence: f32,
    pub start_pos: usize,
    pub end_pos: usize,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PiiType {
    Email,
    PhoneNumber,
    SocialSecurityNumber,
    CreditCardNumber,
    IpAddress,
    DateOfBirth,
    Address,
    Name,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComplianceResult {
    pub original_text: String,
    pub redacted_text: String,
    pub detected_pii: Vec<PiiDetection>,
    pub compliance_score: f32,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingData {
    pub text: String,
    pub pii_annotations: Vec<PiiDetection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub agent_id: String,
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    PiiScanRequest,
    PiiScanResult,
    ComplianceEnforcementRequest,
    ComplianceEnforcementResult,
    LlmReasoningResult,
    Error,
}

#[derive(Debug, Clone)]
pub struct AgentContext {
    pub config: HashMap<String, String>,
    pub shared_memory: HashMap<String, serde_json::Value>,
}

impl Default for AgentContext {
    fn default() -> Self {
        Self {
            config: HashMap::new(),
            shared_memory: HashMap::new(),
        }
    }
} 