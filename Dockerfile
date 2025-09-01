# Use an official Rust base image
FROM rust:1.81 as builder

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

# Build the application
RUN cargo build --release

# Use a minimal base image for the runtime
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS requests (if needed in the future)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a default database directory
RUN mkdir /database

# Copy the binary from the builder stage
COPY --from=builder /usr/src/facial_recognition/target/release/facial_recognition /usr/local/bin/facial_recognition

# Command to run the application
ENTRYPOINT ["facial_recognition"]