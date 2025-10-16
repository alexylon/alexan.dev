# syntax=docker/dockerfile:1

# Runtime only: serve prebuilt static site
FROM nginx:stable-alpine

# Nginx on port 7777 with SPA fallback
RUN rm -f /etc/nginx/conf.d/default.conf && \
    printf '%s\n' \
    'server {' \
    '  listen 7777;' \
    '  listen [::]:7777;' \
    '  server_name _;' \
    '  root /usr/share/nginx/html;' \
    '  index index.html;' \
    '  location / {' \
    '    try_files $uri $uri/ /index.html;' \
    '  }' \
    '  add_header X-Content-Type-Options nosniff;' \
    '  add_header X-Frame-Options SAMEORIGIN;' \
    '  add_header X-XSS-Protection "1; mode=block";' \
    '}' \
    > /etc/nginx/conf.d/site_7777.conf

# Copy the prebuilt static files staged by deploy.sh
COPY site_public/ /usr/share/nginx/html/

EXPOSE 7777

HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
  CMD wget -qO- http://127.0.0.1:7777/ >/dev/null 2>&1 || exit 1
  