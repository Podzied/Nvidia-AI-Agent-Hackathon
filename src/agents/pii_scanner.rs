use crate::models::PiiClassifier;
use crate::types::{AgentContext, AgentMessage, MessageType};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait Agent: Send + Sync {
    async fn process(&self, message: AgentMessage, context: &mut AgentContext) -> Result<AgentMessage>;
    fn agent_id(&self) -> &str;
}

#[derive(Clone)]
pub struct PiiScannerAgent {
    classifier: Arc<Mutex<PiiClassifier>>,
    agent_id: String,
}

impl PiiScannerAgent {
    pub fn new() -> Self {
        Self {
            classifier: Arc::new(Mutex::new(PiiClassifier::new())),
            agent_id: "pii-scanner-001".to_string(),
        }
    }
    
    pub fn with_classifier(classifier: PiiClassifier) -> Self {
        Self {
            classifier: Arc::new(Mutex::new(classifier)),
            agent_id: "pii-scanner-001".to_string(),
        }
    }
}

#[async_trait]
impl Agent for PiiScannerAgent {
    async fn process(&self, message: AgentMessage, _context: &mut AgentContext) -> Result<AgentMessage> {
        match message.message_type {
            MessageType::PiiScanRequest => {
                let text = message.payload["text"].as_str().unwrap_or("");
                let classifier = self.classifier.lock().await;
                let detections = classifier.detect_pii(text);
                
                let result = AgentMessage {
                    agent_id: self.agent_id.clone(),
                    message_type: MessageType::PiiScanResult,
                    payload: json!({
                        "detections": detections,
                        "text": text,
                        "scan_timestamp": chrono::Utc::now(),
                    }),
                    timestamp: chrono::Utc::now(),
                };
                
                Ok(result)
            }
            _ => {
                Err(anyhow::anyhow!("Unsupported message type for PII scanner"))
            }
        }
    }
    
    fn agent_id(&self) -> &str {
        &self.agent_id
    }
}

impl Default for PiiScannerAgent {
    fn default() -> Self {
        Self::new()
    }
} 