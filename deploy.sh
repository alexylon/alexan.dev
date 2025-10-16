#!/usr/bin/env bash
set -euo pipefail

IMAGE="innoxius/alexo:latest"
CONTAINER="alexo"
STAGE_DIR="site_public"

# 1) Bundle on the Pi (host), using your existing Rust/Dioxus toolchain
#    If already bundled, this will just refresh the output.
if command -v dx >/dev/null 2>&1; then
  dx bundle --release
else
  echo "Error: dioxus CLI (dx) not found in PATH. Install with: cargo install dioxus-cli"
  exit 1
fi

# 2) Locate the generated public dir from Dioxus
#    Typical path: target/dx/<project>/release/web/public
PUBDIR="$(find target/dx -type d -path '*/release/web/public' 2>/dev/null | head -n1 || true)"
if [[ -z "${PUBDIR}" ]]; then
  echo "Error: Could not locate the bundled public directory under target/dx/*/release/web/public"
  exit 1
fi

# 3) Stage the public files into a predictable folder included in the Docker build context
rm -rf "${STAGE_DIR}"
mkdir -p "${STAGE_DIR}"
cp -R "${PUBDIR}/." "${STAGE_DIR}/"

# 4) Stop any previous container
if docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER}\$"; then
  docker rm -f "${CONTAINER}" >/dev/null
fi

# 5) Build a tiny, multi-arch Nginx image that just contains the static files
#    On the Pi, Docker will pull the correct arm image automatically.
docker build -t "${IMAGE}" .

# 6) Run on port 7777
docker run -d \
  --name "${CONTAINER}" \
  --restart unless-stopped \
  -p 7777:7777 \
  "${IMAGE}"

echo "Deployed ${CONTAINER} from ${IMAGE} on http://localhost:7777"
