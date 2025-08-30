# Use Rust official image as base
FROM rust:1.70

# Install system dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    pkg-config \
    libclang-dev \
    libopencv-dev \
    libgtk-3-dev \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/facial_recognition

# Copy project files
COPY . .

# Build the application
RUN cargo build --release

# Create directories for models
RUN mkdir -p dnn_models

# Expose port for MongoDB (if running in container)
EXPOSE 27017

# Run the application
CMD ["./target/release/facial_recognition_system"]