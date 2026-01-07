#!/bin/bash

# Example usage of salt-proxier

# 1. Start the server (in one terminal)
# cargo run -- --port 3000

# 2. Test with a simple request
echo "=== Example 1: Simple GET request ==="
curl -X GET http://localhost:3000/users \
  -H "Salt-Host: jsonplaceholder.typicode.com" \
  -v

echo -e "\n\n=== Example 2: POST request with custom headers ==="
curl -X POST http://localhost:3000/posts \
  -H "Salt-Host: jsonplaceholder.typicode.com" \
  -H "Salt-Content-Type: application/json" \
  -d '{"title": "Test Post", "body": "This is a test", "userId": 1}' \
  -v

echo -e "\n\n=== Example 3: Request with Authorization header ==="
curl -X GET http://localhost:3000/api/v1/data \
  -H "Salt-Host: api.example.com" \
  -H "Salt-Authorization: Bearer token123" \
  -H "Salt-X-Custom-Header: custom-value" \
  -v

echo -e "\n\n=== Example 4: Using with proxy ==="
echo "Start server with proxy:"
echo "cargo run -- --port 3000 --proxy username:password@proxy.example.com:8080"

echo -e "\n\n=== Example 5: Test multiple headers transformation ==="
curl -X GET http://localhost:3000/test \
  -H "Salt-Host: httpbin.org" \
  -H "Salt-User-Agent: CustomAgent/1.0" \
  -H "Salt-Accept: application/json" \
  -H "Salt-X-Test-Header: test-value" \
  -v
