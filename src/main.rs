use clap::{Parser, Subcommand};
use pii_compliance_agent::{
    agents::{pii_scanner::PiiScannerAgent, compliance_enforcer::ComplianceEnforcerAgent, llm_reasoner::LlmReasonerAgent},
    coordinator::AgentCoordinator,
    models::pii_classifier::PiiClassifier,
    utils::demo_data::DemoData,
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
    }
    
    Ok(())
} 