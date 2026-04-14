use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use std::time::{SystemTime, UNIX_EPOCH};

/// Main HTTP handler – dispatches based on path
#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    let path = req.path();

    let response = match path {
        "/health"       => handle_health(),
        "/echo"         => handle_echo(req),
        "/info"         => handle_info(),
        _               => handle_not_found(path),
    };

    Ok(response)
}

/// GET /health – liveness probe
fn handle_health() -> Response {
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(r#"{"status":"ok","runtime":"wasm"}"#)
        .build()
}

/// POST /echo – echo back the request body
fn handle_echo(req: Request) -> Response {
    let body = req.body();
    let body_str = std::str::from_utf8(body).unwrap_or("(invalid utf-8)");

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let response_body = format!(
        r#"{{"echo":{},"timestamp":{}}}"#,
        body_str, timestamp
    );

    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(response_body)
        .build()
}

/// GET /info – returns module info
fn handle_info() -> Response {
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(r#"{
  "name": "wasm-microservice",
  "version": "1.0.0",
  "runtime": "wasmtime",
  "framework": "fermyon-spin",
  "language": "rust",
  "description": "Demo WASM HTTP microservice for cloud-native workshop"
}"#)
        .build()
}

/// 404 handler
fn handle_not_found(path: &str) -> Response {
    Response::builder()
        .status(404)
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"error":"not found","path":"{}"}}"#, path))
        .build()
}
