use crate::types::{PiiDetection, PiiType, TrainingData};

pub struct DemoData;

impl DemoData {
    pub fn new() -> Self {
        Self
    }
    
    pub fn get_training_data(&self) -> Vec<TrainingData> {
        vec![
            // Email examples
            TrainingData {
                text: "Contact us at john.doe@example.com for support".to_string(),
                pii_annotations: vec![
                    PiiDetection {
                        pii_type: PiiType::Email,
                        confidence: 0.95,
                        start_pos: 13,
                        end_pos: 33,
                        value: "john.doe@example.com".to_string(),
                    }
                ],
            },
            TrainingData {
                text: "My email is alice.smith@gmail.com".to_string(),
                pii_annotations: vec![
                    PiiDetection {
                        pii_type: PiiType::Email,
                        confidence: 0.95,
                        start_pos: 12,
                        end_pos: 32,
                        value: "alice.smith@gmail.com".to_string(),
                    }
                ],
            },
            
            // Phone number examples
            TrainingData {
                text: "Call me at 555-123-4567".to_string(),
                pii_annotations: vec![
                    PiiDetection {
                        pii_type: PiiType::PhoneNumber,
                        confidence: 0.9,
                        start_pos: 12,
                        end_pos: 25,
                        value: "555-123-4567".to_string(),
                    }
                ],
            },
            TrainingData {
                text: "Phone: (555) 987-6543".to_string(),
                pii_annotations: vec![
                    PiiDetection {
                        pii_type: PiiType::PhoneNumber,
                        confidence: 0.9,
                        start_pos: 7,
                        end_pos: 22,
                        value: "(555) 987-6543".to_string(),
                    }
                ],
            },
            
            // SSN examples
            TrainingData {
                text: "SSN: 123-45-6789".to_string(),
                pii_annotations: vec![
                    PiiDetection {
                        pii_type: PiiType::SocialSecurityNumber,
                        confidence: 0.95,
                        start_pos: 5,
                        end_pos: 18,
                        value: "123-45-6789".to_string(),
                    }
                ],
            },
            
            // Credit card examples
            TrainingData {
                text: "Card: 1234-5678-9012-3456".to_string(),
                pii_annotations: vec![
                    PiiDetection {
                        pii_type: PiiType::CreditCardNumber,
                        confidence: 0.9,
                        start_pos: 6,
                        end_pos: 27,
                        value: "1234-5678-9012-3456".to_string(),
                    }
                ],
            },
            
            // IP address examples
            TrainingData {
                text: "Server IP: 192.168.1.100".to_string(),
                pii_annotations: vec![
                    PiiDetection {
                        pii_type: PiiType::IpAddress,
                        confidence: 0.9,
                        start_pos: 12,
                        end_pos: 24,
                        value: "192.168.1.100".to_string(),
                    }
                ],
            },
            
            // Mixed PII examples
            TrainingData {
                text: "Contact John Doe at john.doe@company.com or 555-123-4567".to_string(),
                pii_annotations: vec![
                    PiiDetection {
                        pii_type: PiiType::Name,
                        confidence: 0.8,
                        start_pos: 8,
                        end_pos: 17,
                        value: "John Doe".to_string(),
                    },
                    PiiDetection {
                        pii_type: PiiType::Email,
                        confidence: 0.95,
                        start_pos: 25,
                        end_pos: 45,
                        value: "john.doe@company.com".to_string(),
                    },
                    PiiDetection {
                        pii_type: PiiType::PhoneNumber,
                        confidence: 0.9,
                        start_pos: 49,
                        end_pos: 62,
                        value: "555-123-4567".to_string(),
                    }
                ],
            },
            
            // Clean text examples
            TrainingData {
                text: "This is a clean text with no PII".to_string(),
                pii_annotations: vec![],
            },
            TrainingData {
                text: "General information about our services".to_string(),
                pii_annotations: vec![],
            },
        ]
    }
    
    pub fn get_demo_texts(&self) -> Vec<String> {
        vec![
            "Contact Alice Johnson at alice.johnson@company.com or call 555-987-6543 for support".to_string(),
            "Customer ID: 12345, SSN: 456-78-9012, Email: customer@example.com".to_string(),
            "Payment details: Card 4111-1111-1111-1111, Exp: 12/25".to_string(),
            "Server configuration: IP 10.0.0.1, Admin: admin@server.local".to_string(),
            "This is a clean message with no personal information".to_string(),
            "Meeting notes: Discuss project timeline with team@company.com".to_string(),
        ]
    }
} 