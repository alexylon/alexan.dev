# syntax=docker/dockerfile:1.7

# =========
# Builder: compiles the Dioxus (Rust) site to static assets
# =========
FROM --platform=$BUILDPLATFORM rust:1.81-alpine AS builder

WORKDIR /app

# System deps needed by rustup/cargo and the dioxus CLI
RUN apk add --no-cache bash curl git nodejs npm build-base openssl-dev

# wasm target + dioxus CLI
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked dioxus-cli

# Cache deps first
COPY Cargo.toml Cargo.lock Dioxus.toml ./
COPY src ./src
COPY assets ./assets

# Build the static bundle (WASM + assets)
# This produces .../target/dx/<project>/release/web/public
RUN dx bundle --release

# Collect the generated public directory into a stable path for the next stage
RUN mkdir -p /out && \
    pubdir="$(find target/dx -type d -path '*/release/web/public' | head -n1)" && \
    test -n "$pubdir" && \
    cp -R "$pubdir"/. /out/

# =========
# Runtime: serve static files with nginx on port 7777
# =========
FROM nginx:stable-alpine

# Nginx config to listen on 7777 and serve /usr/share/nginx/html
RUN rm -f /etc/nginx/conf.d/default.conf
RUN printf '%s\n' \
  'server {' \
  '  listen 7777;' \
  '  listen [::]:7777;' \
  '  server_name _;' \
  '  root /usr/share/nginx/html;' \
  '  index index.html;' \
  '  location / {' \
  '    try_files $uri $uri/ /index.html;' \
  '  }' \
  '  # Basic security/headers (optional)' \
  '  add_header X-Content-Type-Options nosniff;' \
  '  add_header X-Frame-Options SAMEORIGIN;' \
  '  add_header X-XSS-Protection "1; mode=block";' \
  '}' \
  > /etc/nginx/conf.d/site_7777.conf

# Copy build output
COPY --from=builder /out/ /usr/share/nginx/html/

EXPOSE 7777

HEALTHCHECK --interval=30s --timeout=5s --retries=3 CMD wget -qO- http://127.0.0.1:7777/ >/dev/null 2>&1 || exit 1