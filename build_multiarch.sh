#!/bin/bash

# Exit on any error
set -e

# Define image name and tag
IMAGE_NAME="facial_recognition"
TAG="latest"

# Function to build for a specific platform
build_platform() {
    local platform=$1
    echo "Building for $platform..."
    
    # Build the Docker image for the specified platform
    docker buildx build \
        --platform $platform \
        -t $IMAGE_NAME:$TAG \
        --load \
        .
    
    echo "Build for $platform completed."
}

# Build for both x86_64 and arm64
build_platform "linux/amd64"
build_platform "linux/arm64"

echo "All builds completed successfully!"
