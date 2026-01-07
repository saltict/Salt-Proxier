#!/bin/bash

# Script to build Linux binary using Docker

set -e

echo "Building Linux binary using Docker..."

# Build with Docker
docker build -f docker/Dockerfile --target builder -t salt-proxier-builder .

# Create a temporary container to extract the binary
CONTAINER_ID=$(docker create salt-proxier-builder)

# Create output directory
mkdir -p target/linux-release

# Copy the binary from the container
docker cp $CONTAINER_ID:/app/target/release/salt-proxier target/linux-release/salt-proxier

# Clean up the container
docker rm $CONTAINER_ID

echo "✅ Build complete!"
echo "Binary location: target/linux-release/salt-proxier"
echo ""
echo "To test the binary (requires Docker):"
echo "  docker run --rm -p 3000:3000 salt-proxier-builder /app/target/release/salt-proxier --port 3000"
