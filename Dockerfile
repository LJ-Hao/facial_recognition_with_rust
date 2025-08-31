# Use Debian bookworm as base image
FROM debian:bookworm

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    cmake \
    pkg-config \
    libclang-dev \
    libopencv-dev \
    libgtk-3-dev \
    wget \
    clang \
    libssl-dev \
    libprotobuf-dev \
    libprotoc-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Create app directory
WORKDIR /usr/src/facial_recognition

# Copy project files
COPY . .

# Build the application
RUN cargo build --release

# Create directories for models
RUN mkdir -p dnn_models

# Expose port for HTTP server
EXPOSE 8001

# Run the application
CMD ["./target/release/facial_recognition_system"]