use crate::types::{AgentContext, AgentMessage, ComplianceResult, MessageType};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use std::process::Command;

#[derive(Clone)]
pub struct LlmReasonerAgent {
    agent_id: String,
    python_script_path: String,
}

impl LlmReasonerAgent {
    pub fn new() -> Self {
        Self {
            agent_id: "llm-reasoner-001".to_string(),
            python_script_path: "langchain_reasoner_mock.py".to_string(),
        }
    }
    
    pub fn with_script_path(script_path: String) -> Self {
        Self {
            agent_id: "llm-reasoner-001".to_string(),
            python_script_path: script_path,
        }
    }
    
    pub fn call_langchain_reasoner(&self, compliance_result: &ComplianceResult) -> Result<String> {
        // Serialize compliance result to JSON
        let input_json = serde_json::to_string(compliance_result)?;
        
        // Call Python script
        let mut child = Command::new("python")
            .arg(&self.python_script_path)
            .env("MOCK_MODE", "true") // Use mock mode for now
            .current_dir("python")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        
        // Write input to stdin
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(input_json.as_bytes())?;
        }
        
        let output = child.wait_with_output()?;
        
        if output.status.success() {
            let response: serde_json::Value = serde_json::from_slice(&output.stdout)?;
            Ok(response["explanation"].as_str().unwrap_or("No explanation provided").to_string())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Python script failed: {}", error_msg))
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
                
                // Get LLM explanation
                let explanation = self.call_langchain_reasoner(&compliance_result)?;
                
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