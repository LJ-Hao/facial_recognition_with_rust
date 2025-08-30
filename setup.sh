#!/bin/bash

echo "Setting up Facial Recognition System development environment..."

# Create necessary directories
mkdir -p database photos

# Build Docker image
echo "Building Docker image..."
docker build -t facial_recognition_system .

echo "Setup complete!"
echo ""
echo "To run the facial recognition system:"
echo "  docker run -it --device=/dev/video0 facial_recognition_system"
echo ""
echo "To run the CLI tool:"
echo "  docker run -it facial_recognition_system facial-recognition-cli --help"