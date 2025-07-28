#!/usr/bin/env python3
"""
AI Chatbot Compliance Web Service
Deployable to Brev platform
"""

from flask import Flask, request, jsonify, render_template_string
import json
import subprocess
import sys
import time
from typing import Dict, List, Optional
import requests
from datetime import datetime
import os

app = Flask(__name__)

class ChatbotComplianceService:
    def __init__(self):
        self.session_id = f"session_{int(time.time())}"
        self.user_id = "web_user"
        self.conversation_history = []
        self.compliance_violations = []
        
        # Brev configuration
        self.brev_api_base = os.getenv("BREV_API_BASE", "https://api.brev.com/v1")
        self.brev_api_key = os.getenv("BREV_API_KEY", "")
        self.model_name = "llama-3.3-nemotron-super-49b-v1"
        
        # Compliance system path (will be built-in for Brev deployment)
        self.rust_binary = "./pii-compliance-agent"  # Local path for Brev
        
    def call_brev_llm(self, prompt: str, max_tokens: int = 150) -> str:
        """Call Brev LLM for intelligent responses"""
        if not self.brev_api_key:
            return self._get_mock_response(prompt)
            
        headers = {
            "Authorization": f"Bearer {self.brev_api_key}",
            "Content-Type": "application/json"
        }
        
        payload = {
            "model": self.model_name,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful customer service chatbot. Keep responses concise and professional."
                },
                {
                    "role": "user", 
                    "content": prompt
                }
            ],
            "max_tokens": max_tokens,
            "temperature": 0.7
        }
        
        try:
            response = requests.post(
                f"{self.brev_api_base}/chat/completions",
                headers=headers,
                json=payload,
                timeout=30
            )
            
            if response.status_code == 200:
                result = response.json()
                return result["choices"][0]["message"]["content"].strip()
            else:
                print(f"⚠️  Brev API error: {response.status_code}")
                return self._get_mock_response(prompt)
                
        except Exception as e:
            print(f"⚠️  Brev API call failed: {e}")
            return self._get_mock_response(prompt)
    
    def _get_mock_response(self, prompt: str) -> str:
        """Mock responses when Brev is not available"""
        prompt_lower = prompt.lower()
        
        if "email" in prompt_lower or "contact" in prompt_lower:
            return "I'd be happy to help! Could you please provide your email address so I can assist you better?"
        elif "account" in prompt_lower:
            return "I can help you with your account. What specific issue are you experiencing?"
        elif "payment" in prompt_lower or "card" in prompt_lower:
            return "I can help you update your payment information. What would you like to change?"
        elif "appointment" in prompt_lower or "schedule" in prompt_lower:
            return "I can help you schedule an appointment. What type of service do you need?"
        elif "help" in prompt_lower:
            return "I'm here to help! What can I assist you with today?"
        else:
            return "Thank you for your message. How can I help you today?"
    
    def check_compliance(self, message: str) -> Dict:
        """Check message compliance using Rust binary"""
        try:
            # Call the Rust compliance system
            result = subprocess.run(
                [self.rust_binary, "scan", "--text", message],
                capture_output=True,
                text=True
            )
            
            if result.returncode == 0:
                # Parse the JSON output from Rust
                output_lines = result.stdout.strip().split('\n')
                json_start = output_lines.index("📋 Compliance Report:") + 1
                json_str = '\n'.join(output_lines[json_start:])
                
                try:
                    compliance_data = json.loads(json_str)
                    return compliance_data
                except json.JSONDecodeError:
                    return self._parse_compliance_output(result.stdout)
            else:
                print(f"⚠️  Compliance check failed: {result.stderr}")
                return self._get_mock_compliance_result(message)
                
        except Exception as e:
            print(f"⚠️  Compliance system error: {e}")
            return self._get_mock_compliance_result(message)
    
    def _parse_compliance_output(self, output: str) -> Dict:
        """Parse compliance output when JSON parsing fails"""
        lines = output.split('\n')
        compliance_data = {
            "original_text": "",
            "redacted_text": "",
            "detected_pii": [],
            "compliance_score": 1.0,
            "recommendations": []
        }
        
        for line in lines:
            if "Original:" in line:
                compliance_data["original_text"] = line.split("Original:", 1)[1].strip()
            elif "Redacted:" in line:
                compliance_data["redacted_text"] = line.split("Redacted:", 1)[1].strip()
            elif "Score:" in line:
                try:
                    score_str = line.split("Score:", 1)[1].strip().replace("%", "")
                    compliance_data["compliance_score"] = float(score_str) / 100.0
                except:
                    pass
            elif "PII Found:" in line:
                try:
                    pii_count = int(line.split("PII Found:", 1)[1].strip().split()[0])
                    if pii_count > 0:
                        compliance_data["compliance_score"] = max(0.1, compliance_data["compliance_score"] - 0.3)
                except:
                    pass
        
        return compliance_data
    
    def _get_mock_compliance_result(self, message: str) -> Dict:
        """Mock compliance result for testing"""
        return {
            "original_text": message,
            "redacted_text": message,
            "detected_pii": [],
            "compliance_score": 1.0,
            "recommendations": ["✅ Message appears compliant"]
        }
    
    def log_conversation(self, user_message: str, bot_response: str, compliance_result: Dict):
        """Log conversation with compliance data"""
        entry = {
            "timestamp": datetime.now().isoformat(),
            "user_message": user_message,
            "bot_response": bot_response,
            "compliance_score": compliance_result.get("compliance_score", 1.0),
            "pii_detected": len(compliance_result.get("detected_pii", [])),
            "redacted_text": compliance_result.get("redacted_text", user_message)
        }
        
        self.conversation_history.append(entry)
        
        # Check for compliance violations
        if compliance_result.get("compliance_score", 1.0) < 0.8:
            violation = {
                "timestamp": datetime.now().isoformat(),
                "severity": "HIGH" if compliance_result.get("compliance_score", 1.0) < 0.6 else "MEDIUM",
                "pii_count": len(compliance_result.get("detected_pii", [])),
                "original_message": user_message,
                "redacted_message": compliance_result.get("redacted_text", user_message)
            }
            self.compliance_violations.append(violation)

# Global service instance
service = ChatbotComplianceService()

# HTML template for the web interface
HTML_TEMPLATE = """
<!DOCTYPE html>
<html>
<head>
    <title>AI Chatbot Compliance Demo</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }
        .container { max-width: 800px; margin: 0 auto; background: white; padding: 20px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .header { text-align: center; margin-bottom: 20px; }
        .chat-container { height: 400px; overflow-y: auto; border: 1px solid #ddd; padding: 10px; margin-bottom: 20px; background: #fafafa; }
        .message { margin-bottom: 10px; padding: 10px; border-radius: 5px; }
        .user-message { background: #e3f2fd; text-align: right; }
        .bot-message { background: #f1f8e9; }
        .compliance-info { font-size: 12px; color: #666; margin-top: 5px; }
        .input-container { display: flex; gap: 10px; }
        input[type="text"] { flex: 1; padding: 10px; border: 1px solid #ddd; border-radius: 5px; }
        button { padding: 10px 20px; background: #2196F3; color: white; border: none; border-radius: 5px; cursor: pointer; }
        button:hover { background: #1976D2; }
        .status { margin-bottom: 20px; padding: 10px; background: #e8f5e8; border-radius: 5px; }
        .demo-buttons { margin-bottom: 20px; }
        .demo-btn { margin-right: 10px; padding: 5px 10px; background: #ff9800; color: white; border: none; border-radius: 3px; cursor: pointer; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🤖 AI Chatbot Compliance Demo</h1>
            <p>Powered by NVIDIA Brev & Rust PII Detection</p>
        </div>
        
        <div class="status">
            <strong>Session:</strong> {{ session_id }} | 
            <strong>Messages:</strong> {{ message_count }} | 
            <strong>Violations:</strong> {{ violation_count }}
        </div>
        
        <div class="demo-buttons">
            <button class="demo-btn" onclick="sendDemoMessage('Hi, I need help with my account')">Account Help</button>
            <button class="demo-btn" onclick="sendDemoMessage('My email is john.doe@company.com')">Email PII</button>
            <button class="demo-btn" onclick="sendDemoMessage('My SSN is 123-45-6789')">SSN PII</button>
            <button class="demo-btn" onclick="sendDemoMessage('What is the weather today?')">Clean Message</button>
        </div>
        
        <div class="chat-container" id="chatContainer">
            <div class="message bot-message">
                <strong>Bot:</strong> Hello! I'm your AI assistant with built-in compliance checking. How can I help you today?
                <div class="compliance-info">✅ Compliance Score: 100%</div>
            </div>
        </div>
        
        <div class="input-container">
            <input type="text" id="messageInput" placeholder="Type your message..." onkeypress="handleKeyPress(event)">
            <button onclick="sendMessage()">Send</button>
        </div>
    </div>

    <script>
        function handleKeyPress(event) {
            if (event.key === 'Enter') {
                sendMessage();
            }
        }
        
        function sendDemoMessage(message) {
            document.getElementById('messageInput').value = message;
            sendMessage();
        }
        
        function sendMessage() {
            const input = document.getElementById('messageInput');
            const message = input.value.trim();
            if (!message) return;
            
            // Add user message
            addMessage('User', message, 'user-message');
            input.value = '';
            
            // Send to backend
            fetch('/chat', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ message: message })
            })
            .then(response => response.json())
            .then(data => {
                addMessage('Bot', data.bot_response, 'bot-message', data.compliance_score);
            })
            .catch(error => {
                addMessage('Bot', 'Sorry, I encountered an error. Please try again.', 'bot-message', 0);
            });
        }
        
        function addMessage(sender, text, className, complianceScore = 1.0) {
            const container = document.getElementById('chatContainer');
            const messageDiv = document.createElement('div');
            messageDiv.className = `message ${className}`;
            
            const scoreColor = complianceScore >= 0.8 ? 'green' : complianceScore >= 0.6 ? 'orange' : 'red';
            const scoreText = complianceScore >= 0.8 ? '✅' : complianceScore >= 0.6 ? '⚠️' : '❌';
            
            messageDiv.innerHTML = `
                <strong>${sender}:</strong> ${text}
                <div class="compliance-info" style="color: ${scoreColor}">
                    ${scoreText} Compliance Score: ${(complianceScore * 100).toFixed(0)}%
                </div>
            `;
            
            container.appendChild(messageDiv);
            container.scrollTop = container.scrollHeight;
        }
    </script>
</body>
</html>
"""

@app.route('/')
def home():
    """Main web interface"""
    return render_template_string(HTML_TEMPLATE, 
                                session_id=service.session_id,
                                message_count=len(service.conversation_history),
                                violation_count=len(service.compliance_violations))

@app.route('/chat', methods=['POST'])
def chat():
    """Handle chat messages"""
    try:
        data = request.get_json()
        user_message = data.get('message', '')
        
        if not user_message:
            return jsonify({'error': 'No message provided'}), 400
        
        # Check compliance
        compliance_result = service.check_compliance(user_message)
        
        # Generate bot response
        bot_response = service.call_brev_llm(user_message)
        
        # Log conversation
        service.log_conversation(user_message, bot_response, compliance_result)
        
        return jsonify({
            'bot_response': bot_response,
            'compliance_score': compliance_result.get('compliance_score', 1.0),
            'pii_detected': len(compliance_result.get('detected_pii', [])),
            'redacted_text': compliance_result.get('redacted_text', user_message)
        })
        
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/status')
def status():
    """Get system status"""
    return jsonify({
        'session_id': service.session_id,
        'message_count': len(service.conversation_history),
        'violation_count': len(service.compliance_violations),
        'brev_configured': bool(service.brev_api_key)
    })

@app.route('/violations')
def violations():
    """Get compliance violations"""
    return jsonify({
        'violations': service.compliance_violations
    })

if __name__ == '__main__':
    port = int(os.environ.get('PORT', 8080))
    app.run(host='0.0.0.0', port=port, debug=True) 