#!/bin/bash

# AI Chatbot Compliance - Brev Deployment Script
# For NVIDIA AI Agent Hackathon

set -e

echo "🚀 Deploying AI Chatbot Compliance to Brev..."

# Check if Brev CLI is installed
if ! command -v brev &> /dev/null; then
    echo "❌ Brev CLI not found. Please install it first:"
    echo "   curl -fsSL https://brev.com/install.sh | sh"
    exit 1
fi

# Check if logged in to Brev
if ! brev auth status &> /dev/null; then
    echo "❌ Not logged in to Brev. Please run: brev auth login"
    exit 1
fi

# Check environment variables
if [ -z "$BREV_API_KEY" ]; then
    echo "⚠️  BREV_API_KEY not set. You can set it with:"
    echo "   export BREV_API_KEY=your_api_key_here"
    echo "   Or set it in the Brev dashboard after deployment."
fi

# Build the Rust binary locally first
echo "🔨 Building Rust binary..."
cd ..
cargo build --release
cp target/release/pii-compliance-agent python/

# Copy source files for Docker build
echo "📁 Preparing files for deployment..."
cp -r src python/
cp Cargo.toml python/
cp Cargo.lock python/

# Deploy to Brev
echo "🚀 Deploying to Brev..."
cd python
brev deploy

echo "✅ Deployment complete!"
echo ""
echo "🌐 Your application should be available at:"
echo "   https://your-app-name.brev.com"
echo ""
echo "📊 To monitor your deployment:"
echo "   brev logs"
echo ""
echo "🔧 To update environment variables:"
echo "   brev env set BREV_API_KEY=your_key_here"
echo ""
echo "🎉 Ready for the hackathon judges!" 