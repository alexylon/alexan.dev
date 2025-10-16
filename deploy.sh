#!/usr/bin/env bash
set -euo pipefail

# Image and container names
IMAGE="innoxius/alexo:latest"
CONTAINER="alexo"

# Detect the correct Docker platform for Raspberry Pi
arch="$(uname -m)"
case "$arch" in
  aarch64)   PLATFORM="linux/arm64/v8" ;;
  armv7l)    PLATFORM="linux/arm/v7" ;;
  armv6l)    PLATFORM="linux/arm/v6" ;;
  *)         echo "Warning: unrecognized arch '$arch', defaulting to linux/amd64"; PLATFORM="linux/amd64" ;;
esac

# Ensure buildx is ready
if ! docker buildx inspect builder >/dev/null 2>&1; then
  docker buildx create --name builder --use >/dev/null
else
  docker buildx use builder >/dev/null
fi

# Build for the detected platform and load locally
docker buildx build \
  --platform "${PLATFORM}" \
  -t "${IMAGE}" \
  --load \
  .

# Stop/remove previous container if exists
if docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER}\$"; then
  docker rm -f "${CONTAINER}" >/dev/null
fi

# Run on port 7777
docker run -d \
  --name "${CONTAINER}" \
  --restart unless-stopped \
  -p 7777:7777 \
  "${IMAGE}"

echo "Deployed ${CONTAINER} from ${IMAGE} on http://localhost:7777"
