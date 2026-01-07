# Salt Proxier

A Rust-based HTTP proxy server that forwards requests through a configured proxy with custom header transformations.

## Features

- 🚀 Fast and lightweight Rust implementation
- 🔄 Forward requests through a configured proxy
- 🎯 Dynamic target host configuration via headers
- 🔧 Custom header transformation (Salt-* headers)
- 📡 Support for all HTTP methods

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
docker build -t salt-proxier .

# Run with Docker
docker run --rm -p 3000:3000 salt-proxier --port 3000
```

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

## How It Works

The server acts as an intermediary that:

1. Receives requests with special `Salt-*` headers
2. Extracts the target host from `Salt-Host` header
3. Transforms `Salt-*` headers by removing the `Salt-` prefix
4. Forwards the request through the configured proxy (if any)
5. Returns the response to the client

### Header Transformation

- `Salt-Host: api.example.com` → Target host for the request
- `Salt-Authorization: Bearer token123` → `Authorization: Bearer token123`
- `Salt-Content-Type: application/json` → `Content-Type: application/json`
- `Salt-X-Custom-Header: value` → `X-Custom-Header: value`

### Example Request

```bash
curl -X POST http://localhost:3000/api/users \
  -H "Salt-Host: api.example.com" \
  -H "Salt-Authorization: Bearer token123" \
  -H "Salt-Content-Type: application/json" \
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
