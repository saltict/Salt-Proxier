#!/bin/bash

# Script to build and publish Docker image

set -e

DOCKER_USERNAME="saltict"
IMAGE_NAME="salt-proxier"
TAG="${1:-latest}"
FULL_IMAGE="${DOCKER_USERNAME}/${IMAGE_NAME}:${TAG}"

echo "🐳 Building Docker image: ${FULL_IMAGE}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Build the Docker image
docker build -f docker/Dockerfile -t ${FULL_IMAGE} .

if [ $? -ne 0 ]; then
    echo "❌ Docker build failed!"
    exit 1
fi

echo ""
echo "✅ Docker image built successfully!"
echo ""
echo "📤 Pushing to Docker Hub..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Push to Docker Hub
docker push ${FULL_IMAGE}

if [ $? -ne 0 ]; then
    echo "❌ Docker push failed!"
    echo "💡 Tip: Make sure you're logged in with: docker login -u ${DOCKER_USERNAME}"
    exit 1
fi

echo ""
echo "✅ Successfully published!"
echo ""
echo "📦 Image: ${FULL_IMAGE}"
echo "🔗 Pull command: docker pull ${FULL_IMAGE}"
echo ""
echo "🚀 Quick start:"
echo "   docker run -d -p 3000:3000 ${FULL_IMAGE} --port 3000"
echo ""

# Also tag and push as latest if a specific version was provided
if [ "$TAG" != "latest" ]; then
    echo "🏷️  Tagging as latest..."
    docker tag ${FULL_IMAGE} ${DOCKER_USERNAME}/${IMAGE_NAME}:latest
    docker push ${DOCKER_USERNAME}/${IMAGE_NAME}:latest
    echo "✅ Also published as ${DOCKER_USERNAME}/${IMAGE_NAME}:latest"
fi
