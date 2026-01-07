use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, HeaderName, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    Router,
};
use clap::Parser;
use reqwest::Client;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(long, default_value = "3000")]
    port: u16,

    /// Proxy configuration (format: username:password@host:port or host:port)
    #[arg(long)]
    proxy: Option<String>,

    /// CORS allowed origins (default: * for all origins)
    #[arg(long, default_value = "*")]
    cors: String,
}

#[derive(Clone)]
struct AppState {
    client: Client,
}

struct ProxyConfig {
    username: Option<String>,
    password: Option<String>,
    host: String,
    port: u16,
}

fn parse_proxy(proxy_str: &str) -> Result<ProxyConfig, String> {
    // Format: username:password@host:port or host:port
    let (auth_part, host_part) = if let Some(at_pos) = proxy_str.rfind('@') {
        let auth = &proxy_str[..at_pos];
        let host = &proxy_str[at_pos + 1..];
        (Some(auth), host)
    } else {
        (None, proxy_str)
    };

    // Parse authentication if present
    let (username, password) = if let Some(auth) = auth_part {
        if let Some(colon_pos) = auth.find(':') {
            let user = auth[..colon_pos].to_string();
            let pass = auth[colon_pos + 1..].to_string();
            (Some(user), Some(pass))
        } else {
            return Err("Invalid auth format, expected username:password".to_string());
        }
    } else {
        (None, None)
    };

    // Parse host and port
    let (host, port) = if let Some(colon_pos) = host_part.rfind(':') {
        let h = host_part[..colon_pos].to_string();
        let p = host_part[colon_pos + 1..]
            .parse::<u16>()
            .map_err(|_| "Invalid port number".to_string())?;
        (h, p)
    } else {
        return Err("Port is required, expected host:port".to_string());
    };

    Ok(ProxyConfig {
        username,
        password,
        host,
        port,
    })
}

#[tokio::main]
async fn main() {
    // Initialize tracing with default level INFO if RUST_LOG is not set
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();

    let args = Args::parse();
    
    info!("Starting salt-proxier server...");
    info!("Port: {}", args.port);

    // Create HTTP client with optional proxy
    let client = match &args.proxy {
        Some(proxy_str) => {
            match parse_proxy(proxy_str) {
                Ok(config) => {
                    let proxy_url = format!("http://{}:{}", config.host, config.port);
                    
                    if let (Some(username), Some(password)) = (&config.username, &config.password) {
                        info!(
                            "Configuring proxy with authentication: {}:***@{}:{}", 
                            username, config.host, config.port
                        );
                    } else {
                        info!("Configuring proxy: {}:{}", config.host, config.port);
                    }

                    match reqwest::Proxy::all(&proxy_url) {
                        Ok(mut proxy) => {
                            // Add basic authentication if credentials provided
                            if let (Some(username), Some(password)) = (config.username, config.password) {
                                proxy = proxy.basic_auth(&username, &password);
                            }
                            
                            match Client::builder().proxy(proxy).build() {
                                Ok(client) => {
                                    info!("Proxy configured successfully");
                                    client
                                }
                                Err(e) => {
                                    error!("Failed to build client with proxy: {}", e);
                                    Client::new()
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to create proxy: {}", e);
                            Client::new()
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to parse proxy configuration: {}", e);
                    error!("Expected format: username:password@host:port or host:port");
                    Client::new()
                }
            }
        }
        None => {
            info!("No proxy configured");
            Client::new()
        }
    };

    let state = AppState { client };

    // Configure CORS
    let cors = if args.cors == "*" {
        info!("CORS: Allowing all origins");
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        info!("CORS: Allowing origin: {}", args.cors);
        CorsLayer::new()
            .allow_origin(args.cors.parse::<axum::http::HeaderValue>().unwrap())
            .allow_methods(Any)
            .allow_headers(Any)
    };

    // Build router
    let app = Router::new()
        .fallback(handler)
        .layer(cors)
        .with_state(state);

    // Run server
    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    info!("Server listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    State(state): State<AppState>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Body,
) -> Result<Response, AppError> {
    info!("Received {} request to {}", method, uri.path());
    
    // Extract Salt-Host header
    let target_host = headers
        .get("Salt-Host")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            error!("Missing Salt-Host header");
            AppError::MissingHeader("Salt-Host".to_string())
        })?;

    info!("Target host from Salt-Host header: {}", target_host);

    // Construct target URL
    let target_url = if target_host.starts_with("http://") || target_host.starts_with("https://") {
        format!("{}{}", target_host, uri.path())
    } else {
        format!("https://{}{}", target_host, uri.path())
    };

    // Add query string if present
    let target_url = if let Some(query) = uri.query() {
        format!("{}?{}", target_url, query)
    } else {
        target_url
    };

    info!("Proxying {} request to: {}", method, target_url);

    // Build new headers (extract Salt-* headers and forward them)
    let mut new_headers = reqwest::header::HeaderMap::new();
    for (key, value) in headers.iter() {
        let key_str = key.as_str();
        if key_str.starts_with("Salt-") && key_str != "Salt-Host" {
            // Remove "Salt-" prefix and add to new headers
            let new_key = &key_str[5..]; // Skip "Salt-"
            if let Ok(header_name) = reqwest::header::HeaderName::from_bytes(new_key.as_bytes()) {
                if let Ok(header_value) = reqwest::header::HeaderValue::from_bytes(value.as_bytes()) {
                    new_headers.insert(header_name, header_value);
                }
            }
        }
    }

    // Convert body to bytes
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to read request body: {}", e);
            return Err(AppError::BodyReadError);
        }
    };

    // Build and send request
    // Convert axum Method to reqwest Method
    let reqwest_method = match method.as_str() {
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        "PATCH" => reqwest::Method::PATCH,
        "HEAD" => reqwest::Method::HEAD,
        "OPTIONS" => reqwest::Method::OPTIONS,
        _ => reqwest::Method::GET,
    };
    
    let mut request = state.client.request(reqwest_method, &target_url);
    
    // Add headers
    for (key, value) in new_headers.iter() {
        request = request.header(key, value);
    }

    // Add body if not empty
    if !body_bytes.is_empty() {
        request = request.body(body_bytes.to_vec());
    }

    // Execute request
    let response = match request.send().await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to send request: {}", e);
            return Err(AppError::RequestFailed(e.to_string()));
        }
    };

    // Build response
    let status_code = response.status().as_u16();
    let resp_headers = response.headers().clone();
    let body_bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to read response body: {}", e);
            return Err(AppError::ResponseReadError);
        }
    };

    // Print response status
    // info!("Received response with status: {}", status_code);
    // Print response result
    // let rs_str = String::from_utf8_lossy(&body_bytes);
    // info!("Response body: {}", rs_str);

    // Create response with converted status code
    let mut resp = Response::builder().status(status_code);
    
    // Copy headers (convert from reqwest to axum types)
    // Skip headers that axum manages automatically
    let skip_headers = ["transfer-encoding", "content-length", "connection"];
    
    for (key, value) in resp_headers.iter() {
        let key_str = key.as_str();
        
        // Skip headers that conflict with axum's automatic handling
        if skip_headers.contains(&key_str.to_lowercase().as_str()) {
            continue;
        }
        
        if let Ok(header_name) = HeaderName::from_bytes(key_str.as_bytes()) {
            if let Ok(header_value) = axum::http::HeaderValue::from_bytes(value.as_bytes()) {
                resp = resp.header(header_name, header_value);
            }
        }
    }

    match resp.body(Body::from(body_bytes)) {
        Ok(response) => {
            info!("Successfully proxied request with status: {}", status_code);
            Ok(response)
        }
        Err(e) => {
            error!("Failed to build response: {}", e);
            Err(AppError::ResponseBuildError)
        }
    }
}

// Error handling
enum AppError {
    MissingHeader(String),
    BodyReadError,
    RequestFailed(String),
    ResponseReadError,
    ResponseBuildError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::MissingHeader(header) => {
                (StatusCode::BAD_REQUEST, format!("Missing required header: {}", header))
            }
            AppError::BodyReadError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read request body".to_string())
            }
            AppError::RequestFailed(err) => {
                (StatusCode::BAD_GATEWAY, format!("Failed to send request: {}", err))
            }
            AppError::ResponseReadError => {
                (StatusCode::BAD_GATEWAY, "Failed to read response body".to_string())
            }
            AppError::ResponseBuildError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to build response".to_string())
            }
        };

        (status, message).into_response()
    }
}
