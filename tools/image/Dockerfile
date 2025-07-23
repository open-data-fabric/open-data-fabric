# Use official Ubuntu base image
FROM ubuntu:22.04

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Update and install system dependencies
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    curl \
    ca-certificates \
    gnupg \
    build-essential \
    make \
    software-properties-common

# Install Node.js and tools
RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash - && \
    apt-get install -y nodejs

RUN npm install -g markdown-toc

# Install Rust (via rustup)
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Add Rust to PATH (for login and non-login shells)
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /workspace

# Default command
CMD ["bash"]
