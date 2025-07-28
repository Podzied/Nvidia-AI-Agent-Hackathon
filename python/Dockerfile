# Multi-stage build for Rust + Python deployment
FROM rust:1.75 as rust-builder

# Install Python for the build stage
RUN apt-get update && apt-get install -y python3 python3-pip

# Copy Rust source code
WORKDIR /app
COPY ../src ./src
COPY ../Cargo.toml ../Cargo.lock ./

# Build Rust binary
RUN cargo build --release
RUN cp target/release/pii-compliance-agent ./pii-compliance-agent

# Python runtime stage
FROM python:3.11-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Python requirements and install
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy Rust binary from builder stage
COPY --from=rust-builder /app/pii-compliance-agent ./pii-compliance-agent

# Copy Python application
COPY app.py .
COPY chatbot_demo.py .
COPY langchain_reasoner.py .
COPY langchain_reasoner_mock.py .

# Make Rust binary executable
RUN chmod +x ./pii-compliance-agent

# Expose port
EXPOSE 8080

# Set environment variables
ENV PORT=8080
ENV PYTHONUNBUFFERED=1

# Run the application
CMD ["python", "app.py"] 