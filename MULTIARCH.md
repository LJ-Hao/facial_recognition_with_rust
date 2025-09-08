# Multi-Architecture Docker Build Guide

This project now supports building Docker images for both x86_64 (amd64) and ARM64 architectures using Docker Buildx.

## Prerequisites

1. Docker with Buildx support (Docker Desktop or Docker Engine with Buildx plugin)
2. QEMU emulators for cross-platform builds (usually installed automatically with Docker Desktop)

## Building for Both Architectures

To build the Docker images for both x86_64 and ARM64, run:

```bash
./build_multiarch.sh
```

This script will:
1. Create a Docker Buildx builder instance if one doesn't exist
2. Build the image for linux/amd64 (x86_64)
3. Build the image for linux/arm64 (ARM64)

The built images will be available in your local Docker registry with the tag `facial_recognition:latest`.

## Manual Build Commands

If you prefer to build manually, you can use these commands:

```bash
# Create and use a new builder instance
docker buildx create --name multiarch-builder --use

# Build for both platforms
docker buildx build --platform linux/amd64,linux/arm64 -t facial_recognition:latest .
```

## Pushing to Registry

To push the multi-arch image to a container registry:

```bash
docker buildx build --platform linux/amd64,linux/arm64 -t your-registry/facial_recognition:latest --push .
```

## Testing Images

To test the images on different architectures, you can run:

```bash
# For x86_64
docker run --rm facial_recognition:latest --help

# For ARM64
docker run --rm --platform linux/arm64 facial_recognition:latest --help
```

The Dockerfile has been modified to support cross-compilation:
1. It installs the necessary cross-compilation tools for ARM64
2. It uses conditional logic to build for the correct target platform
3. It copies the correct binary based on the target platform

This setup ensures that the application can be built and run on both x86_64 and ARM64 systems.