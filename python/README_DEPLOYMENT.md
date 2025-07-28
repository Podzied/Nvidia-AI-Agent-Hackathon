# 🚀 Deploying to Brev Platform

This guide shows you how to deploy your AI Chatbot Compliance system to NVIDIA's Brev platform for the hackathon.

## 📋 Prerequisites

1. **Brev Account**: Sign up at [brev.com](https://brev.com)
2. **Brev CLI**: Install the Brev CLI
3. **API Key**: Get your Brev API key from the dashboard

## 🔧 Setup Steps

### 1. Install Brev CLI
```bash
curl -fsSL https://brev.com/install.sh | sh
```

### 2. Login to Brev
```bash
brev auth login
```

### 3. Set Environment Variables
```bash
export BREV_API_KEY=your_api_key_here
```

## 🚀 Quick Deployment

### Option 1: Automated Deployment
```bash
cd python
./deploy.sh
```

### Option 2: Manual Deployment
```bash
# Build Rust binary
cd ..
cargo build --release
cp target/release/pii-compliance-agent python/

# Copy source files
cp -r src python/
cp Cargo.toml python/
cp Cargo.lock python/

# Deploy to Brev
cd python
brev deploy
```

## 🌐 What Gets Deployed

### **Web Application Features:**
- ✅ **Interactive Chat Interface**: Real-time messaging
- ✅ **PII Detection**: Automatic scanning of messages
- ✅ **Compliance Scoring**: Real-time compliance assessment
- ✅ **Brev LLM Integration**: Intelligent responses
- ✅ **Session Tracking**: Conversation history
- ✅ **Violation Logging**: Compliance violation tracking

### **Technical Stack:**
- **Backend**: Flask (Python)
- **PII Detection**: Rust binary
- **LLM**: NVIDIA Brev (`llama-3.3-nemotron-super-49b-v1`)
- **Frontend**: HTML/CSS/JavaScript
- **Deployment**: Docker + Brev

## 🎯 Demo Scenarios

The web interface includes demo buttons for:

1. **Account Help**: `"Hi, I need help with my account"`
2. **Email PII**: `"My email is john.doe@company.com"`
3. **SSN PII**: `"My SSN is 123-45-6789"`
4. **Clean Message**: `"What is the weather today?"`

## 📊 Monitoring

### View Logs
```bash
brev logs
```

### Check Status
```bash
curl https://your-app.brev.com/status
```

### View Violations
```bash
curl https://your-app.brev.com/violations
```

## 🔧 Configuration

### Environment Variables
- `BREV_API_KEY`: Your Brev API key
- `BREV_API_BASE`: Brev API base URL (default: https://api.brev.com/v1)
- `PORT`: Application port (default: 8080)

### Update Environment Variables
```bash
brev env set BREV_API_KEY=your_new_key
```

## 🏆 Hackathon Judging Tips

### **What Judges Will See:**
1. **Professional Web Interface**: Clean, modern UI
2. **Real-time PII Detection**: Instant compliance checking
3. **Intelligent Responses**: Powered by NVIDIA's 49B parameter LLM
4. **Compliance Scoring**: Visual compliance indicators
5. **Session Management**: Conversation tracking
6. **Violation Logging**: Detailed compliance reports

### **Key Talking Points:**
- **NVIDIA Integration**: Using NVIDIA's own platform
- **Multi-language Architecture**: Rust for performance, Python for flexibility
- **Real-time Processing**: Sub-second PII detection
- **Scalable Design**: Docker containerization
- **Production Ready**: Error handling, logging, monitoring

### **Demo Flow:**
1. **Show the Interface**: "This is our web-based chatbot"
2. **Demo PII Detection**: Click "Email PII" button
3. **Show Compliance**: Point out the compliance score
4. **Explain Architecture**: "Rust handles PII detection, Python handles the web interface"
5. **Show Brev Integration**: "We're using NVIDIA's 49B parameter model"
6. **Show Scalability**: "This can handle thousands of concurrent users"

## 🐛 Troubleshooting

### Common Issues:

**1. Build Failures**
```bash
# Check Rust compilation
cargo build --release

# Check Docker build
docker build -t chatbot-compliance .
```

**2. API Key Issues**
```bash
# Verify API key
curl -H "Authorization: Bearer $BREV_API_KEY" \
     https://api.brev.com/v1/models
```

**3. Port Issues**
```bash
# Check if port is available
netstat -an | grep 8080
```

**4. Memory Issues**
```bash
# Check resource usage
brev logs --follow
```

## 📈 Performance Optimization

### **For High Traffic:**
- Increase CPU/memory in `brev.yaml`
- Add Redis for session storage
- Implement connection pooling
- Add CDN for static assets

### **For Better Compliance:**
- Add more PII patterns
- Implement ML-based detection
- Add regulatory compliance rules
- Implement audit logging

## 🎉 Success!

Once deployed, your application will be available at:
```
https://your-app-name.brev.com
```

**Perfect for hackathon judging!** 🏆

---

*Built for NVIDIA AI Agent Hackathon - World's Shortest Hackathon* 🚀 