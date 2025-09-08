# Use an official Rust base image
FROM --platform=$BUILDPLATFORM rust:1.83 AS builder

# Install cross-compilation tools
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/facial_recognition

# Copy the Cargo files
COPY Cargo.toml Cargo.lock ./

# Create a dummy src/lib.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies only, so they are cached if Cargo.toml/Cargo.lock don't change
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY src ./src

# Build the application for the target platform
RUN case $TARGETPLATFORM in \
    "linux/amd64") cargo build --release ;; \
    "linux/arm64") cargo build --release --target aarch64-unknown-linux-gnu ;; \
    esac

# Use a minimal base image for the runtime
FROM --platform=$TARGETPLATFORM debian:bookworm-slim

# Install ca-certificates for HTTPS requests (if needed in the future)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a default database directory
RUN mkdir /database

# Copy the binary from the builder stage
RUN case $TARGETPLATFORM in \
    "linux/amd64") cp /usr/src/facial_recognition/target/release/facial_recognition /usr/local/bin/facial_recognition ;; \
    "linux/arm64") cp /usr/src/facial_recognition/target/aarch64-unknown-linux-gnu/release/facial_recognition /usr/local/bin/facial_recognition ;; \
    esac

# Command to run the application
ENTRYPOINT ["facial_recognition"]