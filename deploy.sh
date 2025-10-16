#!/usr/bin/env bash
# Deploy alexan.dev as a static site served by nginx on port 7777
# Assumes you build on the host (dx bundle) and Dockerfile copies site_public/

# -------------------------
# Config (edit as needed)
# -------------------------
IMAGE="innoxius/alexo:latest"
CONTAINER="alexo"
HOST_PORT=7777
CONTAINER_PORT=7777
STAGE_DIR="site_public"

# -------------------------
# Pretty output helpers
# -------------------------
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status()   { echo -e "${BLUE}[INFO]${NC} $1"; }
print_success()  { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
print_warning()  { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_error()    { echo -e "${RED}[ERROR]${NC} $1"; }

set -euo pipefail

print_status "Starting deployment for ${CONTAINER} using image ${IMAGE}..."

# -------------------------
# Check Docker availability
# -------------------------
if ! command -v docker >/dev/null 2>&1; then
  print_error "Docker CLI not found in PATH."
  exit 1
fi
if ! docker info >/dev/null 2>&1; then
  print_error "Docker daemon is not running. Please start Docker and retry."
  exit 1
fi
print_success "Docker is available."

# -------------------------
# Bundle the site on host
# -------------------------
if ! command -v dx >/dev/null 2>&1; then
  print_error "dioxus CLI 'dx' not found. Install with: cargo install dioxus-cli"
  exit 1
fi

print_status "Bundling Dioxus site (dx bundle --release)..."
if dx bundle --release; then
  print_success "Bundle completed."
else
  print_error "dx bundle failed."
  exit 1
fi

# Locate Dioxus public output
PUBDIR="$(find target/dx -type d -path '*/release/web/public' 2>/dev/null | head -n1 || true)"
if [[ -z "${PUBDIR}" ]]; then
  print_error "Could not find bundled public dir under target/dx/*/release/web/public"
  exit 1
fi
print_status "Found public dir: ${PUBDIR}"

# Stage into predictable build context folder
print_status "Staging public files to ./${STAGE_DIR} ..."
rm -rf "${STAGE_DIR}"
mkdir -p "${STAGE_DIR}"
cp -R "${PUBDIR}/." "${STAGE_DIR}/"
print_success "Staged into ${STAGE_DIR}"

# -------------------------
# Remove existing container
# -------------------------
print_status "Checking for existing '${CONTAINER}' container..."
if docker ps -a --format '{{.Names}}' | grep -qx "${CONTAINER}"; then
  print_status "Stopping existing container..."
  docker stop "${CONTAINER}" >/dev/null 2>&1 || true
  print_status "Removing existing container..."
  docker rm "${CONTAINER}" >/dev/null 2>&1 || true
  print_success "Existing container removed."
else
  print_status "No existing '${CONTAINER}' container found."
fi

# -------------------------
# Remove existing image tag
# -------------------------
print_status "Checking for existing '${IMAGE}' image..."
if docker images --format '{{.Repository}}:{{.Tag}}' | grep -qx "${IMAGE}"; then
  print_status "Removing existing image '${IMAGE}'..."
  docker rmi "${IMAGE}" >/dev/null 2>&1 || true
  print_success "Previous image removed."
else
  print_status "No existing '${IMAGE}' image found."
fi

# -------------------------
# Build new image
# -------------------------
print_status "Building new Docker image '${IMAGE}'..."
if docker build -q -t "${IMAGE}" . >/dev/null; then
  print_success "Docker image built successfully."
else
  print_error "Failed to build Docker image."
  exit 1
fi

# -------------------------
# Run new container
# -------------------------
print_status "Starting new '${CONTAINER}' container on port ${HOST_PORT}..."
if docker run -d \
    -p "${HOST_PORT}:${CONTAINER_PORT}" \
    --name "${CONTAINER}" \
    --restart unless-stopped \
    "${IMAGE}" >/dev/null; then
  print_success "Container started."
  print_success "Site is now available at http://localhost:${HOST_PORT}"
else
  print_error "Failed to start container."
  exit 1
fi

# -------------------------
# Post-deploy status
# -------------------------
echo ""
print_status "Container Status:"
docker ps --filter "name=${CONTAINER}" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

print_status "Deployment completed successfully."
