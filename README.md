# Salt Proxier

A Rust-based HTTP proxy server that forwards requests through a configured proxy with flexible header forwarding.

## Features

- 🚀 Fast and lightweight Rust implementation
- 🔄 Forward requests through a configured proxy
- 🎯 Dynamic target host configuration via Host header
- 🔧 Configurable header forwarding via allow-headers option
- 📡 Support for all HTTP methods
- 🔐 Optional bearer token authentication

## Installation

### Build from Source (macOS/Linux with Rust)

```bash
cargo build --release
```

### Build for Linux (using Docker on macOS)

```bash
./build-linux.sh
```

The Linux binary will be available at `target/linux-release/salt-proxier`

### Using Docker

```bash
# Build Docker image
docker build -f docker/Dockerfile -t salt-proxier .

# Run with Docker
docker run --rm -p 3000:3000 salt-proxier --port 3000
```

### Using Docker Compose (Recommended)

```bash
# Quick start
cd docker
cp .env.example .env
# Edit .env with your configuration
docker-compose up -d

# View logs
docker-compose logs -f

# Stop
docker-compose down
```

See [docker/README.md](docker/README.md) for detailed Docker deployment guide.

## Usage

### Basic Usage

Start the server on default port 3000:

```bash
cargo run
```

### With Custom Port

```bash
cargo run -- --port 8080
```

### With Proxy Configuration

```bash
# With authentication
cargo run -- --port 8080 --proxy username:password@proxy.example.com:8080

# Without authentication
cargo run -- --port 8080 --proxy proxy.example.com:8080
```

### With Bearer Token Authentication

```bash
# Enable bearer token authentication
cargo run -- --port 8080 --bearer-token your-secret-token

# With proxy and bearer token
cargo run -- --port 8080 \
  --proxy user:pass@proxy.com:8080 \
  --bearer-token your-secret-token
```

### With CORS Configuration

```bash
# Allow all origins (default)
cargo run -- --port 8080 --cors "*"

# Allow specific origin
cargo run -- --port 8080 --cors "https://myapp.com"
```

### With Custom Header Forwarding

```bash
# Forward specific headers to target server
cargo run -- --port 8080 --allow-headers "authorization,content-type,x-api-key"

# Combine with other options
cargo run -- --port 8080 \
  --proxy user:pass@proxy.com:8080 \
  --bearer-token your-secret-token \
  --allow-headers "authorization,content-type"
```

### With Path Prefix Stripping

```bash
# Strip specific prefixes from paths before forwarding
cargo run -- --port 8080 --strip-prefixes "meta,poly,api"

# Request to localhost:8080/meta/markets will be forwarded as /markets
# Request to localhost:8080/poly/events will be forwarded as /events

# Combine with other options
cargo run -- --port 8080 \
  --strip-prefixes "meta,poly" \
  --allow-headers "authorization,content-type" \
  --bearer-token your-secret-token
```

## How It Works

The server acts as an intermediary that:

1. Receives requests with standard HTTP headers
2. Extracts the target host from `Host` header
3. Strips configured path prefixes (if `--strip-prefixes` is set)
4. Forwards only the headers specified in `--allow-headers` option
5. Proxies the request through the configured proxy (if any)
6. Returns the response to the client

### Path Prefix Stripping

When `--strip-prefixes` is configured, the proxy will remove matching prefixes from the request path:

**Example with `--strip-prefixes "meta,poly"`:**
- Request: `localhost:3000/meta/markets/123` → Forwarded: `/markets/123`
- Request: `localhost:3000/poly/events` → Forwarded: `/events`
- Request: `localhost:3000/other/path` → Forwarded: `/other/path` (no match, unchanged)

**Note:** Only the first matching prefix is stripped.

### Header Forwarding

Only headers specified in the `--allow-headers` option are forwarded to the target server:

- `Host: api.example.com` → Determines the target host for the request
- `Authorization: Bearer token123` → Forwarded if `authorization` is in allow-headers
- `Content-Type: application/json` → Forwarded if `content-type` is in allow-headers
- `X-API-Key: secret` → Forwarded if `x-api-key` is in allow-headers

**Note:** Header names in `--allow-headers` are case-insensitive.

### Example Request

**Start server with allowed headers:**
```bash
cargo run -- --port 3000 --allow-headers "authorization,content-type,x-api-key"
```

**Send request:**
```bash
curl -X POST http://localhost:3000/api/users \
  -H "Host: api.example.com" \
  -H "Authorization: Bearer token123" \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe"}'
```

This will proxy the request to:
```
POST https://api.example.com/api/users
Authorization: Bearer token123
Content-Type: application/json

{"name": "John Doe"}
```

## CLI Options

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `--port` | Port to listen on | 3000 | `--port 8080` |
| `--proxy` | Proxy configuration | None | `--proxy user:pass@proxy.com:8080` |
| `--cors` | CORS allowed origins | * (all) | `--cors https://myapp.com` |
| `--bearer-token` | Bearer token for auth | None | `--bearer-token secret123` || `--allow-headers` | Headers to forward to target (comma-separated) | None | `--allow-headers "authorization,content-type"` |
## Development

### Run in development mode

```bash
cargo run -- --port 3000
```

### Run with logs

```bash
RUST_LOG=info cargo run -- --port 3000
```

### Build for production

```bash
cargo build --release
./target/release/salt-proxier --port 8080
```

## License

MIT
