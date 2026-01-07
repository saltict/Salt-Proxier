# Docker Deployment Guide

This directory contains Docker configuration files for Salt-Proxier.

## Quick Start

### 1. Setup Environment

Copy the example environment file:

```bash
cd docker
cp .env.example .env
```

Edit `.env` file with your configuration:

```bash
# Example configuration
PORT=3000
PROXY=username:password@proxy.example.com:8080
CORS=*
BEARER_TOKEN=my-secret-token-123
ALLOW_HEADERS=authorization,content-type,x-api-key
STRIP_PREFIXES=kalshi,poly
RUST_LOG=info
```

### 2. Deploy with Docker Compose

```bash
# Start the service
docker-compose up -d

# View logs
docker-compose logs -f

# Stop the service
docker-compose down

# Restart the service
docker-compose restart
```

### 3. Deploy without Docker Compose

```bash
# Build the image
docker build -f docker/Dockerfile -t salt-proxier .

# Run the container
docker run -d \
  --name salt-proxier \
  -p 3000:3000 \
  salt-proxier \
  --port 3000 \
  --cors "*"

# With all options
docker run -d \
  --name salt-proxier \
  -p 3000:3000 \
  salt-proxier \
  --port 3000 \
  --proxy user:pass@proxy.com:8080 \
  --cors "https://myapp.com" \
  --bearer-token my-secret-token \
  --allow-headers "authorization,content-type,x-api-key" \
  --strip-prefixes "kalshi,poly"
```

## Configuration Options

All options can be configured via environment variables in `.env` file:

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `PORT` | Host port mapping | 3000 | `8080` |
| `PROXY` | Proxy server config | None | `user:pass@proxy.com:8080` |
| `CORS` | CORS allowed origins | * | `https://myapp.com` |
| `BEARER_TOKEN` | Auth token | None | `secret-token-123` || `ALLOW_HEADERS` | Headers to forward to target | None | `authorization,content-type` || `RUST_LOG` | Log level | info | `debug` |

## Health Check

The container includes a health check that verifies the service is running:

```bash
# Check health status
docker-compose ps

# Manual health check
curl http://localhost:3000/health
```

## Logs

```bash
# View logs
docker-compose logs -f salt-proxier

# View last 100 lines
docker-compose logs --tail 100 salt-proxier
```

## Update & Rebuild

```bash
# Pull latest code
git pull

# Rebuild and restart
docker-compose up -d --build
```

## Production Deployment

For production, consider:

1. **Use specific image tags** instead of `latest`
2. **Set strong bearer token**
3. **Configure specific CORS origins**
4. **Use reverse proxy** (nginx/traefik) for SSL/TLS
5. **Set up monitoring** and log aggregation
6. **Regular updates** and security patches

### Example Production Setup

```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  salt-proxier:
    image: konistudio/salt-proxier:v1.0.0
    restart: always
    ports:
      - "127.0.0.1:3000:3000"
    environment:
      - RUST_LOG=warn
    command: >
      --port 3000
      --proxy ${PROXY}
      --cors https://myapp.com
      --bearer-token ${BEARER_TOKEN}
      --allow-headers "authorization,content-type"
      --strip-prefixes "kalshi,poly"
```

## Troubleshooting

### Container won't start

```bash
# Check logs
docker-compose logs salt-proxier

# Check container status
docker ps -a
```

### Port already in use

```bash
# Change PORT in .env file
PORT=8080

# Restart
docker-compose up -d
```

### Image build fails

```bash
# Clean build
docker-compose build --no-cache
```
