use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use crate::{
    coordinator::AgentCoordinator,
    agents::{pii_scanner::PiiScannerAgent, compliance_enforcer::ComplianceEnforcerAgent, llm_reasoner::LlmReasonerAgent},
};

#[derive(Deserialize)]
pub struct ScanRequest {
    pub text: String,
}

#[derive(Serialize)]
pub struct ScanResponse {
    pub text: String,
    pub pii_detected: Vec<PiiDetectionResponse>,
    pub compliance_score: f32,
    pub redacted_text: String,
    pub recommendations: Vec<String>,
    pub processing_time: u64,
}

#[derive(Serialize)]
pub struct PiiDetectionResponse {
    pub type_: String,
    pub value: String,
    pub confidence: f32,
    pub position: PositionResponse,
}

#[derive(Serialize)]
pub struct PositionResponse {
    pub start: usize,
    pub end: usize,
}

pub async fn scan_text(req: web::Json<ScanRequest>) -> Result<HttpResponse, actix_web::Error> {
    let start_time = std::time::Instant::now();
    
    println!("🔍 Received scan request for text: {}", req.text);
    
    // Initialize coordinator and agents
    let coordinator = AgentCoordinator::new();
    let scanner = PiiScannerAgent::new();
    let enforcer = ComplianceEnforcerAgent::new();
    let reasoner = LlmReasonerAgent::new();
    
    // Run the compliance pipeline
    println!("🔄 Starting compliance pipeline...");
    let result = coordinator.run_compliance_pipeline(&req.text, scanner, enforcer, reasoner)
        .await
        .map_err(|e| {
            println!("❌ Error in compliance pipeline: {:?}", e);
            actix_web::error::ErrorInternalServerError(format!("Compliance pipeline error: {:?}", e))
        })?;
    
    let processing_time = start_time.elapsed().as_millis() as u64;
    
    println!("✅ Scan completed in {}ms", processing_time);
    
    // Convert to response format
    let pii_detected = result.detected_pii.into_iter().map(|pii| PiiDetectionResponse {
        type_: match pii.pii_type {
            crate::types::PiiType::Email => "email".to_string(),
            crate::types::PiiType::PhoneNumber => "phone".to_string(),
            crate::types::PiiType::SocialSecurityNumber => "ssn".to_string(),
            crate::types::PiiType::CreditCardNumber => "credit_card".to_string(),
            crate::types::PiiType::IpAddress => "ip_address".to_string(),
            crate::types::PiiType::DateOfBirth => "dob".to_string(),
            crate::types::PiiType::Address => "address".to_string(),
            crate::types::PiiType::Name => "name".to_string(),
            crate::types::PiiType::Unknown => "unknown".to_string(),
        },
        value: pii.value,
        confidence: pii.confidence,
        position: PositionResponse {
            start: pii.start_pos,
            end: pii.end_pos,
        },
    }).collect();
    
    let response = ScanResponse {
        text: result.original_text,
        pii_detected,
        compliance_score: result.compliance_score,
        redacted_text: result.redacted_text,
        recommendations: result.recommendations,
        processing_time,
    };
    
    println!("📊 Found {} PII items, compliance score: {:.1}%", 
             response.pii_detected.len(), response.compliance_score * 100.0);
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn test_endpoint() -> Result<HttpResponse, actix_web::Error> {
    println!("🧪 Test endpoint called");
    
    // Test basic agent creation
    let scanner = PiiScannerAgent::new();
    let enforcer = ComplianceEnforcerAgent::new();
    let reasoner = LlmReasonerAgent::new();
    
    println!("✅ Agents created successfully");
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "test_passed",
        "agents_created": true,
        "message": "Basic agent creation test passed"
    })))
}

pub async fn health_check() -> Result<HttpResponse, actix_web::Error> {
    println!("🏥 Health check requested");
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "PII Compliance Agent",
        "version": "1.0.0"
    })))
}

pub async fn start_web_server() -> std::io::Result<()> {
    println!("🚀 Starting PII Compliance Agent web server on http://localhost:8000");
    println!("📡 Available endpoints:");
    println!("   POST /api/scan - Scan text for PII");
    println!("   GET  /health   - Health check");
    println!("   GET  /test     - Test endpoint");
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .wrap(cors)
            .route("/api/scan", web::post().to(scan_text))
            .route("/health", web::get().to(health_check))
            .route("/test", web::get().to(test_endpoint))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
} 