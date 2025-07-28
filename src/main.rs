use clap::{Parser, Subcommand};
use pii_compliance_agent::{
    agents::{pii_scanner::PiiScannerAgent, compliance_enforcer::ComplianceEnforcerAgent, llm_reasoner::LlmReasonerAgent, chatbot_compliance::ChatbotComplianceAgent},
    coordinator::AgentCoordinator,
    models::pii_classifier::PiiClassifier,
    utils::{demo_data::DemoData, chatbot_demo::ChatbotDemoData},
};

#[derive(Parser)]
#[command(name = "pii-compliance-agent")]
#[command(about = "AI Agent for PII Compliance and Security")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan text for PII and apply compliance rules
    Scan {
        /// Input text to scan
        #[arg(short, long)]
        text: String,
        
        /// Output file for results (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Train the PII classifier with demo data
    Train {
        /// Number of training epochs
        #[arg(short, long, default_value = "100")]
        epochs: usize,
    },
    /// Run a demo with sample data
    Demo,
    /// Run chatbot compliance demo
    ChatbotDemo,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Scan { text, output } => {
            println!("🔍 Scanning text for PII...");
            
            // Initialize coordinator and agents
            let coordinator = AgentCoordinator::new();
            let scanner = PiiScannerAgent::new();
            let enforcer = ComplianceEnforcerAgent::new();
            let reasoner = LlmReasonerAgent::new();
            
            // Run the compliance pipeline
            let result = coordinator.run_compliance_pipeline(&text, scanner, enforcer, reasoner).await?;
            
            // Output results
            if let Some(output_path) = output {
                std::fs::write(&output_path, serde_json::to_string_pretty(&result)?)?;
                println!("✅ Results saved to {}", &output_path);
            } else {
                println!("📋 Compliance Report:");
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
        }
        
        Commands::Train { epochs } => {
            println!("🧠 Training PII classifier...");
            
            let mut classifier = PiiClassifier::new();
            let demo_data = DemoData::new();
            
            classifier.train(&demo_data.get_training_data(), epochs).await?;
            classifier.save("models/pii_classifier.json")?;
            
            println!("✅ Training completed! Model saved to models/pii_classifier.json");
        }
        
        Commands::Demo => {
            println!("🎯 Running PII Compliance Demo...");
            
            let demo_data = DemoData::new();
            let sample_texts = demo_data.get_demo_texts();
            
            let coordinator = AgentCoordinator::new();
            let scanner = PiiScannerAgent::new();
            let enforcer = ComplianceEnforcerAgent::new();
            let reasoner = LlmReasonerAgent::new();
            
            for (i, text) in sample_texts.iter().enumerate() {
                println!("\n--- Demo {} ---", i + 1);
                println!("Input: {}", text);
                
                let result = coordinator.run_compliance_pipeline(text, scanner.clone(), enforcer.clone(), reasoner.clone()).await?;
                println!("Output: {}", result.redacted_text);
                println!("PII Found: {:?}", result.detected_pii);
            }
        }
        
        Commands::ChatbotDemo => {
            println!("🤖 Running Chatbot Compliance Demo...");
            
            let chatbot_demo = ChatbotDemoData::new();
            let scenarios = chatbot_demo.get_chat_scenarios();
            let compliance_agent = ChatbotComplianceAgent::new();
            
            for (i, scenario) in scenarios.iter().enumerate() {
                println!("\n=== Chatbot Scenario {} ===", i + 1);
                println!("Session ID: {}", scenario.session_id);
                println!("User ID: {}", scenario.user_id);
                println!("Risk Level: {:?}", scenario.risk_level);
                
                // Process each message in the conversation
                for (msg_idx, message) in scenario.messages.iter().enumerate() {
                    println!("\n--- Message {} ---", msg_idx + 1);
                    println!("{}: {}", if message.is_user_message { "User" } else { "Bot" }, message.content);
                    
                    if message.is_user_message {
                        // Process user message for compliance
                        let compliance_result = compliance_agent.process_chat_message(message.clone()).await?;
                        
                        println!("🔍 Compliance Check:");
                        println!("  Original: {}", compliance_result.original_text);
                        println!("  Redacted: {}", compliance_result.redacted_text);
                        println!("  Score: {:.1}%", compliance_result.compliance_score * 100.0);
                        println!("  PII Found: {} items", compliance_result.detected_pii.len());
                        
                        for pii in &compliance_result.detected_pii {
                            println!("    - {:?}: {} (confidence: {:.1})", 
                                   pii.pii_type, pii.value, pii.confidence);
                        }
                        
                        for rec in &compliance_result.recommendations {
                            println!("  💡 {}", rec);
                        }
                    }
                }
                
                println!("\n📊 Session Summary:");
                println!("  Total Messages: {}", scenario.messages.len());
                println!("  User Messages: {}", scenario.messages.iter().filter(|m| m.is_user_message).count());
                println!("  Risk Level: {:?}", scenario.risk_level);
            }
        }
    }
    
    Ok(())
} 