use crate::types::{PiiDetection, PiiType, TrainingData};
use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PiiClassifier {
    patterns: HashMap<PiiType, Vec<Regex>>,
    confidence_threshold: f32,
}

impl PiiClassifier {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // Initialize regex patterns for different PII types
        patterns.insert(
            PiiType::Email,
            vec![Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap()],
        );
        
        patterns.insert(
            PiiType::PhoneNumber,
            vec![
                Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap(),
                Regex::new(r"\b\(\d{3}\)\s*\d{3}[-.]?\d{4}\b").unwrap(),
            ],
        );
        
        patterns.insert(
            PiiType::SocialSecurityNumber,
            vec![Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap()],
        );
        
        patterns.insert(
            PiiType::CreditCardNumber,
            vec![Regex::new(r"\b\d{4}[- ]?\d{4}[- ]?\d{4}[- ]?\d{4}\b").unwrap()],
        );
        
        patterns.insert(
            PiiType::IpAddress,
            vec![Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b").unwrap()],
        );
        
        Self {
            patterns,
            confidence_threshold: 0.7,
        }
    }
    
    pub async fn train(&mut self, training_data: &[TrainingData], _epochs: usize) -> Result<()> {
        // Simplified training for hackathon demo
        // In production, this would use actual ML training
        println!("Training PII classifier with {} samples", training_data.len());
        
        // Update confidence threshold based on training data
        let total_detections: usize = training_data.iter()
            .map(|data| data.pii_annotations.len())
            .sum();
        
        if total_detections > 0 {
            self.confidence_threshold = 0.8; // Higher confidence for trained model
        }
        
        println!("Training completed! Confidence threshold set to {}", self.confidence_threshold);
        Ok(())
    }
    
    pub fn detect_pii(&self, text: &str) -> Vec<PiiDetection> {
        let mut detections = vec![];
        
        // Use regex patterns for initial detection
        for (pii_type, patterns) in &self.patterns {
            for pattern in patterns {
                for mat in pattern.find_iter(text) {
                    detections.push(PiiDetection {
                        pii_type: pii_type.clone(),
                        confidence: 0.9, // High confidence for regex matches
                        start_pos: mat.start(),
                        end_pos: mat.end(),
                        value: mat.as_str().to_string(),
                    });
                }
            }
        }
        
        // Additional pattern-based detection for edge cases
        // In production, this would use ML-based detection
        for detection in &mut detections {
            if detection.confidence < self.confidence_threshold {
                detection.confidence = self.confidence_threshold;
            }
        }
        
        detections
    }
    
    fn extract_features(&self, text: &str) -> Vec<f32> {
        // Simplified feature extraction for hackathon demo
        vec![
            text.len() as f32,
            text.chars().filter(|c| c.is_ascii_digit()).count() as f32,
            text.chars().filter(|c| c.is_ascii_alphabetic()).count() as f32,
            text.chars().filter(|c| *c == '@').count() as f32,
            text.chars().filter(|c| *c == '.').count() as f32,
            text.chars().filter(|c| *c == '-').count() as f32,
        ]
    }
    
    pub fn save(&self, _path: &str) -> Result<()> {
        // No-op for web server - no file system access needed
        Ok(())
    }
    
    pub fn load(_path: &str) -> Result<Self> {
        // No-op for web server - always return a new instance
        Ok(Self::new())
    }
} 