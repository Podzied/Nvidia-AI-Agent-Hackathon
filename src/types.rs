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

// New: Chatbot-specific types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatMessage {
    pub user_id: String,
    pub session_id: String,
    pub message_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub content: String,
    pub is_user_message: bool, // true for user, false for bot
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub session_id: String,
    pub user_id: String,
    pub messages: Vec<ChatMessage>,
    pub compliance_violations: Vec<ComplianceViolation>,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub violation_type: ViolationType,
    pub severity: Severity,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub pii_detected: Vec<PiiDetection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ViolationType {
    PiiExposure,
    DataRetention,
    ConsentMissing,
    UnauthorizedAccess,
    DataMinimization,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RiskLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
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
    ChatMessageReceived,
    ChatSessionUpdate,
    ComplianceViolationAlert,
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