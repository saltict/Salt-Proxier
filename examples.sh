#!/bin/bash

# Example usage of salt-proxier

# 1. Start the server with allowed headers (in one terminal)
echo "Start server with:"
echo "cargo run -- --port 3000 --allow-headers \"authorization,content-type,x-api-key,user-agent,accept\" --strip-prefixes \"kalshi,poly\""
echo ""

# 2. Test with a simple request
echo "=== Example 1: Simple GET request ==="
curl -X GET http://localhost:3000/users \
  -H "Host: jsonplaceholder.typicode.com" \
  -v

echo -e "\n\n=== Example 2: POST request with custom headers ==="
curl -X POST http://localhost:3000/posts \
  -H "Host: jsonplaceholder.typicode.com" \
  -H "Content-Type: application/json" \
  -d '{"title": "Test Post", "body": "This is a test", "userId": 1}' \
  -v

echo -e "\n\n=== Example 3: Request with Authorization header ==="
curl -X GET http://localhost:3000/api/v1/data \
  -H "Host: api.example.com" \
  -H "Authorization: Bearer token123" \
  -H "X-API-Key: secret-key" \
  -v

echo -e "\n\n=== Example 4: Request with multiple allowed headers ==="
curl -X POST http://localhost:3000/api/users \
  -H "Host: jsonplaceholder.typicode.com" \
  -H "Authorization: Bearer token123" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: secret-key" \
  -d '{"name": "John Doe"}' \
  -v

echo -e "\n\n=== Example 5: Using with proxy ==="
echo "Start server with proxy:"
echo "cargo run -- --port 3000 --proxy username:password@proxy.example.com:8080 --allow-headers \"authorization,content-type\""

echo -e "\n\n=== Example 6: Test path prefix stripping ==="
echo "Start server with:"
echo "cargo run -- --port 3000 --strip-prefixes \"kalshi,poly\" --allow-headers \"content-type\""
echo ""
echo "Test requests:"
curl -X GET http://localhost:3000/kalshi/markets \
  -H "Host: httpbin.org" \
  -v
echo ""
curl -X GET http://localhost:3000/poly/events \
  -H "Host: httpbin.org" \
  -v

echo -e "\n\n=== Example 7: Test with custom user agent ==="
curl -X GET http://localhost:3000/headers \
  -H "Host: httpbin.org" \
  -H "User-Agent: CustomAgent/1.0" \
  -H "Accept: application/json" \
  -v
