use crate::agents::{Agent, PiiScannerAgent, ComplianceEnforcerAgent, LlmReasonerAgent};
use crate::types::{AgentContext, AgentMessage, ComplianceResult, MessageType, PiiDetection};
use anyhow::Result;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AgentCoordinator {
    context: Arc<Mutex<AgentContext>>,
    coordinator_id: String,
}

impl AgentCoordinator {
    pub fn new() -> Self {
        Self {
            context: Arc::new(Mutex::new(AgentContext::default())),
            coordinator_id: "coordinator-001".to_string(),
        }
    }
    
    pub async fn run_compliance_pipeline(
        &self,
        text: &str,
        scanner: PiiScannerAgent,
        enforcer: ComplianceEnforcerAgent,
        reasoner: LlmReasonerAgent,
    ) -> Result<ComplianceResult> {
        let mut context = self.context.lock().await;
        
        // Step 1: Scan for PII
        let scan_message = AgentMessage {
            agent_id: self.coordinator_id.clone(),
            message_type: MessageType::PiiScanRequest,
            payload: json!({
                "text": text,
                "scan_id": uuid::Uuid::new_v4().to_string(),
            }),
            timestamp: chrono::Utc::now(),
        };
        
        let scan_result = scanner.process(scan_message, &mut context).await?;
        
        // Extract detections from scan result
        let detections: Vec<PiiDetection> = serde_json::from_value(
            scan_result.payload["detections"].clone()
        ).unwrap_or_default();
        
        // Step 2: Apply compliance enforcement
        let enforcement_message = AgentMessage {
            agent_id: self.coordinator_id.clone(),
            message_type: MessageType::ComplianceEnforcementRequest,
            payload: json!({
                "text": text,
                "detections": detections,
                "enforcement_id": uuid::Uuid::new_v4().to_string(),
            }),
            timestamp: chrono::Utc::now(),
        };
        
        let enforcement_result = enforcer.process(enforcement_message, &mut context).await?;
        
        // Extract compliance result
        let redacted_text = enforcement_result.payload["redacted_text"].as_str().unwrap_or(text);
        let compliance_score = enforcement_result.payload["compliance_score"].as_f64().unwrap_or(0.0) as f32;
        let recommendations: Vec<String> = serde_json::from_value(
            enforcement_result.payload["recommendations"].clone()
        ).unwrap_or_default();
        
        let final_detections: Vec<PiiDetection> = serde_json::from_value(
            enforcement_result.payload["detected_pii"].clone()
        ).unwrap_or_default();
        
        let compliance_result = ComplianceResult {
            original_text: text.to_string(),
            redacted_text: redacted_text.to_string(),
            detected_pii: final_detections,
            compliance_score,
            recommendations,
        };
        
        // Step 3: Get LLM reasoning
        let reasoning_message = AgentMessage {
            agent_id: self.coordinator_id.clone(),
            message_type: MessageType::ComplianceEnforcementResult,
            payload: json!({
                "compliance_result": compliance_result,
                "reasoning_id": uuid::Uuid::new_v4().to_string(),
            }),
            timestamp: chrono::Utc::now(),
        };
        
        let reasoning_result = reasoner.process(reasoning_message, &mut context).await?;
        
        // Extract LLM explanation
        let llm_explanation = reasoning_result.payload["llm_explanation"].as_str().unwrap_or("");
        println!("🤖 LLM Reasoning: {}", llm_explanation);
        
        Ok(compliance_result)
    }
    
    pub async fn run_swarm_workflow(
        &self,
        agents: Vec<Box<dyn Agent>>,
        initial_message: AgentMessage,
    ) -> Result<Vec<AgentMessage>> {
        let mut context = self.context.lock().await;
        let mut messages = vec![initial_message];
        let mut results = vec![];
        
        for agent in agents {
            if let Some(message) = messages.pop() {
                match agent.process(message, &mut context).await {
                    Ok(result) => {
                        results.push(result.clone());
                        messages.push(result);
                    }
                    Err(e) => {
                        tracing::error!("Agent {} failed: {}", agent.agent_id(), e);
                        break;
                    }
                }
            }
        }
        
        Ok(results)
    }
} 