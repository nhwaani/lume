# Lume Development Dockerfile

FROM rust:latest

# Install system dependencies for WGPU, X11, and Vulkan
RUN apt-get update && apt-get install -y \
    pkg-config \
    libx11-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libgl1-mesa-dev \
    libasound2-dev \
    vulkan-tools \
    mesa-vulkan-drivers \
    cmake \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Create a non-root user to avoid permission issues with mounted volumes
RUN useradd -m lume-user
USER lume-user

# Set environment variables for the shell
ENV SHELL=/bin/bash
