use crate::types::{ChatMessage, ChatSession, RiskLevel};
use chrono::Utc;
use uuid::Uuid;

pub struct ChatbotDemoData;

impl ChatbotDemoData {
    pub fn new() -> Self {
        Self
    }
    
    pub fn get_chat_scenarios(&self) -> Vec<ChatSession> {
        vec![
            // Scenario 1: Customer Support with PII
            self.create_customer_support_scenario(),
            
            // Scenario 2: Healthcare Chat with Sensitive Data
            self.create_healthcare_scenario(),
            
            // Scenario 3: Banking Chat with Financial Data
            self.create_banking_scenario(),
            
            // Scenario 4: Clean Conversation
            self.create_clean_scenario(),
        ]
    }
    
    fn create_customer_support_scenario(&self) -> ChatSession {
        let session_id = Uuid::new_v4().to_string();
        let user_id = "user_123".to_string();
        
        let messages = vec![
            ChatMessage {
                user_id: user_id.clone(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "Hi, I need help with my account".to_string(),
                is_user_message: true,
            },
            ChatMessage {
                user_id: "bot_001".to_string(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "Hello! I'd be happy to help. Can you provide your email address?".to_string(),
                is_user_message: false,
            },
            ChatMessage {
                user_id: user_id.clone(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "Sure, my email is john.doe@gmail.com and my phone is 555-123-4567".to_string(),
                is_user_message: true,
            },
        ];
        
        ChatSession {
            session_id,
            user_id,
            messages,
            compliance_violations: vec![],
            risk_level: RiskLevel::Medium,
        }
    }
    
    fn create_healthcare_scenario(&self) -> ChatSession {
        let session_id = Uuid::new_v4().to_string();
        let user_id = "user_456".to_string();
        
        let messages = vec![
            ChatMessage {
                user_id: user_id.clone(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "I need to schedule an appointment".to_string(),
                is_user_message: true,
            },
            ChatMessage {
                user_id: "bot_002".to_string(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "I can help you schedule an appointment. What's your name and date of birth?".to_string(),
                is_user_message: false,
            },
            ChatMessage {
                user_id: user_id.clone(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "My name is Jane Smith, DOB 05/15/1985, SSN 123-45-6789".to_string(),
                is_user_message: true,
            },
        ];
        
        ChatSession {
            session_id,
            user_id,
            messages,
            compliance_violations: vec![],
            risk_level: RiskLevel::Critical,
        }
    }
    
    fn create_banking_scenario(&self) -> ChatSession {
        let session_id = Uuid::new_v4().to_string();
        let user_id = "user_789".to_string();
        
        let messages = vec![
            ChatMessage {
                user_id: user_id.clone(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "I need to update my payment method".to_string(),
                is_user_message: true,
            },
            ChatMessage {
                user_id: "bot_003".to_string(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "I can help you update your payment method. What's your card number?".to_string(),
                is_user_message: false,
            },
            ChatMessage {
                user_id: user_id.clone(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "My card number is 4111-1111-1111-1111, exp 12/25".to_string(),
                is_user_message: true,
            },
        ];
        
        ChatSession {
            session_id,
            user_id,
            messages,
            compliance_violations: vec![],
            risk_level: RiskLevel::High,
        }
    }
    
    fn create_clean_scenario(&self) -> ChatSession {
        let session_id = Uuid::new_v4().to_string();
        let user_id = "user_999".to_string();
        
        let messages = vec![
            ChatMessage {
                user_id: user_id.clone(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "What's the weather like today?".to_string(),
                is_user_message: true,
            },
            ChatMessage {
                user_id: "bot_004".to_string(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "I can check the weather for you. What city are you in?".to_string(),
                is_user_message: false,
            },
            ChatMessage {
                user_id: user_id.clone(),
                session_id: session_id.clone(),
                message_id: Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                content: "I'm in San Francisco".to_string(),
                is_user_message: true,
            },
        ];
        
        ChatSession {
            session_id,
            user_id,
            messages,
            compliance_violations: vec![],
            risk_level: RiskLevel::Safe,
        }
    }
} 