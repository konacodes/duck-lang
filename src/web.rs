// Web primitives for Duck language
// HTTP client, WebSocket support, and related utilities

use crate::values::Value;
use std::collections::HashMap;

// =============================================================================
// JSON Support
// =============================================================================

/// Convert a serde_json::Value to a Duck Value
fn json_to_value(json: serde_json::Value) -> Result<Value, String> {
    match json {
        serde_json::Value::Null => Ok(Value::Null),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(b)),
        serde_json::Value::Number(n) => Ok(Value::Number(n.as_f64().unwrap_or(0.0))),
        serde_json::Value::String(s) => Ok(Value::String(s)),
        serde_json::Value::Array(arr) => {
            let items: Result<Vec<_>, _> = arr.into_iter().map(json_to_value).collect();
            Ok(Value::new_list(items?))
        }
        serde_json::Value::Object(obj) => {
            let mut fields = HashMap::new();
            for (k, v) in obj {
                fields.insert(k, json_to_value(v)?);
            }
            Ok(Value::new_struct("object".to_string(), fields))
        }
    }
}

/// Convert a Duck Value to serde_json::Value
fn value_to_json(value: &Value) -> Result<serde_json::Value, String> {
    match value {
        Value::Null => Ok(serde_json::Value::Null),
        Value::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
        Value::Number(n) => serde_json::Number::from_f64(*n)
            .map(serde_json::Value::Number)
            .ok_or_else(|| "Cannot convert number to JSON".to_string()),
        Value::String(s) => Ok(serde_json::Value::String(s.clone())),
        Value::List(items) => {
            let arr: Result<Vec<_>, _> = items.borrow().iter().map(value_to_json).collect();
            Ok(serde_json::Value::Array(arr?))
        }
        Value::Struct { fields, .. } => {
            let mut obj = serde_json::Map::new();
            for (k, v) in fields.borrow().iter() {
                obj.insert(k.clone(), value_to_json(v)?);
            }
            Ok(serde_json::Value::Object(obj))
        }
        other => Err(format!("Cannot convert {} to JSON", other.type_name())),
    }
}

/// Parse a JSON string into a Duck value
pub fn builtin_json_parse(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::String(s)) => {
            let parsed: serde_json::Value =
                serde_json::from_str(s).map_err(|e| format!("JSON parse error: {}", e))?;
            json_to_value(parsed)
        }
        Some(other) => Err(format!(
            "json-parse() expects a string, got {}",
            other.type_name()
        )),
        None => Err("json-parse() requires 1 argument".to_string()),
    }
}

/// Convert a Duck value to a JSON string
pub fn builtin_json_stringify(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(value) => {
            let json = value_to_json(value)?;
            let s = serde_json::to_string(&json)
                .map_err(|e| format!("JSON stringify error: {}", e))?;
            Ok(Value::String(s))
        }
        None => Err("json-stringify() requires 1 argument".to_string()),
    }
}

// =============================================================================
// HTTP Client
// =============================================================================

/// Parse headers from a list of key-value pairs
fn parse_headers(header_list: &Value) -> Result<Vec<(String, String)>, String> {
    match header_list {
        Value::List(items) => {
            let borrowed = items.borrow();
            let mut headers = Vec::new();
            let mut iter = borrowed.iter();

            while let Some(key) = iter.next() {
                match key {
                    Value::String(k) => {
                        if let Some(val) = iter.next() {
                            match val {
                                Value::String(v) => headers.push((k.clone(), v.clone())),
                                other => {
                                    return Err(format!(
                                        "Header value must be string, got {}",
                                        other.type_name()
                                    ))
                                }
                            }
                        } else {
                            return Err(
                                "Headers list must have even number of elements (key, value pairs)"
                                    .to_string(),
                            );
                        }
                    }
                    other => {
                        return Err(format!(
                            "Header key must be string, got {}",
                            other.type_name()
                        ))
                    }
                }
            }
            Ok(headers)
        }
        _ => Err("Headers must be a list".to_string()),
    }
}

/// Build HTTP response struct
fn build_http_response(status: u16, body: String, headers: Vec<(String, String)>) -> Value {
    let mut fields = HashMap::new();
    fields.insert("status".to_string(), Value::Number(status as f64));
    fields.insert("body".to_string(), Value::String(body));

    // Convert headers to list of key-value pairs
    let header_values: Vec<Value> = headers
        .into_iter()
        .flat_map(|(k, v)| vec![Value::String(k), Value::String(v)])
        .collect();
    fields.insert("headers".to_string(), Value::new_list(header_values));

    Value::new_struct("response".to_string(), fields)
}

/// HTTP GET request
pub fn builtin_http_get(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("http-get() requires at least 1 argument (url)".to_string());
    }

    let url = match &args[0] {
        Value::String(u) => u.clone(),
        other => {
            return Err(format!(
                "http-get() expects a URL string, got {}",
                other.type_name()
            ))
        }
    };

    // Optional headers
    let headers = if args.len() > 1 {
        parse_headers(&args[1])?
    } else {
        Vec::new()
    };

    let client = reqwest::blocking::Client::new();
    let mut request = client.get(&url);

    for (key, value) in &headers {
        request = request.header(key.as_str(), value.as_str());
    }

    let response = request
        .send()
        .map_err(|e| format!("HTTP GET error: {}", e))?;

    let status = response.status().as_u16();
    let resp_headers: Vec<(String, String)> = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let body = response
        .text()
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(build_http_response(status, body, resp_headers))
}

/// HTTP POST request
pub fn builtin_http_post(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("http-post() requires at least 2 arguments (url, body)".to_string());
    }

    let url = match &args[0] {
        Value::String(u) => u.clone(),
        other => {
            return Err(format!(
                "http-post() expects a URL string, got {}",
                other.type_name()
            ))
        }
    };

    let body = match &args[1] {
        Value::String(b) => b.clone(),
        other => {
            return Err(format!(
                "http-post() expects a body string, got {}",
                other.type_name()
            ))
        }
    };

    // Optional headers
    let headers = if args.len() > 2 {
        parse_headers(&args[2])?
    } else {
        Vec::new()
    };

    let client = reqwest::blocking::Client::new();
    let mut request = client.post(&url).body(body);

    for (key, value) in &headers {
        request = request.header(key.as_str(), value.as_str());
    }

    let response = request
        .send()
        .map_err(|e| format!("HTTP POST error: {}", e))?;

    let status = response.status().as_u16();
    let resp_headers: Vec<(String, String)> = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let resp_body = response
        .text()
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(build_http_response(status, resp_body, resp_headers))
}

// =============================================================================
// Base64 Encoding
// =============================================================================

/// Encode a string to base64
pub fn builtin_base64_encode(args: Vec<Value>) -> Result<Value, String> {
    use base64::Engine;
    match args.first() {
        Some(Value::String(s)) => {
            let encoded = base64::engine::general_purpose::STANDARD.encode(s.as_bytes());
            Ok(Value::String(encoded))
        }
        Some(other) => Err(format!(
            "base64-encode() expects a string, got {}",
            other.type_name()
        )),
        None => Err("base64-encode() requires 1 argument".to_string()),
    }
}

/// Decode a base64 string
pub fn builtin_base64_decode(args: Vec<Value>) -> Result<Value, String> {
    use base64::Engine;
    match args.first() {
        Some(Value::String(s)) => {
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(s)
                .map_err(|e| format!("Base64 decode error: {}", e))?;
            let text = String::from_utf8(decoded)
                .map_err(|e| format!("Invalid UTF-8 after decode: {}", e))?;
            Ok(Value::String(text))
        }
        Some(other) => Err(format!(
            "base64-decode() expects a string, got {}",
            other.type_name()
        )),
        None => Err("base64-decode() requires 1 argument".to_string()),
    }
}

// =============================================================================
// WebSocket Support
// =============================================================================

use std::net::TcpStream;
use std::sync::Mutex;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;

// Global WebSocket connection store
// Each connection is identified by a unique numeric ID
lazy_static::lazy_static! {
    static ref WS_CONNECTIONS: Mutex<HashMap<u64, WebSocket<MaybeTlsStream<TcpStream>>>> =
        Mutex::new(HashMap::new());
    static ref WS_NEXT_ID: Mutex<u64> = Mutex::new(1);
}

/// Get the next unique WebSocket connection ID
fn next_ws_id() -> u64 {
    let mut id = WS_NEXT_ID.lock().unwrap();
    let current = *id;
    *id += 1;
    current
}

/// Connect to a WebSocket server
/// Usage: ws-connect("wss://example.com/socket")
/// Returns: A connection handle (number) to use with other ws-* functions
pub fn builtin_ws_connect(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("ws-connect() requires 1 argument (url)".to_string());
    }

    let url = match &args[0] {
        Value::String(u) => u.clone(),
        other => {
            return Err(format!(
                "ws-connect() expects a URL string, got {}",
                other.type_name()
            ))
        }
    };

    // Connect to the WebSocket server
    let (socket, _response) = connect(&url)
        .map_err(|e| format!("WebSocket connection failed: {}", e))?;

    // Store the connection and return the ID
    let id = next_ws_id();
    WS_CONNECTIONS.lock().unwrap().insert(id, socket);

    // Return a struct with the connection ID and URL for debugging
    let mut fields = HashMap::new();
    fields.insert("id".to_string(), Value::Number(id as f64));
    fields.insert("url".to_string(), Value::String(url));
    fields.insert("connected".to_string(), Value::Boolean(true));

    Ok(Value::new_struct("websocket".to_string(), fields))
}

/// Send a message through a WebSocket connection
/// Usage: ws-send(ws, "message")
/// Returns: true on success, false on failure
pub fn builtin_ws_send(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("ws-send() requires 2 arguments (connection, message)".to_string());
    }

    // Get the connection ID from the websocket struct
    let conn_id = match &args[0] {
        Value::Struct { fields, .. } => {
            let borrowed = fields.borrow();
            match borrowed.get("id") {
                Some(Value::Number(n)) => *n as u64,
                _ => return Err("Invalid WebSocket connection object".to_string()),
            }
        }
        Value::Number(n) => *n as u64,
        other => {
            return Err(format!(
                "ws-send() expects a WebSocket connection, got {}",
                other.type_name()
            ))
        }
    };

    let message = match &args[1] {
        Value::String(s) => s.clone(),
        other => {
            return Err(format!(
                "ws-send() expects a string message, got {}",
                other.type_name()
            ))
        }
    };

    // Get the connection and send
    let mut connections = WS_CONNECTIONS.lock().unwrap();
    match connections.get_mut(&conn_id) {
        Some(socket) => {
            socket
                .send(Message::Text(message))
                .map_err(|e| format!("WebSocket send failed: {}", e))?;
            Ok(Value::Boolean(true))
        }
        None => Err(format!("WebSocket connection {} not found or closed", conn_id)),
    }
}

/// Receive a message from a WebSocket connection (blocking)
/// Usage: ws-receive(ws)
/// Returns: The received message as a string, or nil if connection closed
pub fn builtin_ws_receive(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("ws-receive() requires 1 argument (connection)".to_string());
    }

    // Get the connection ID
    let conn_id = match &args[0] {
        Value::Struct { fields, .. } => {
            let borrowed = fields.borrow();
            match borrowed.get("id") {
                Some(Value::Number(n)) => *n as u64,
                _ => return Err("Invalid WebSocket connection object".to_string()),
            }
        }
        Value::Number(n) => *n as u64,
        other => {
            return Err(format!(
                "ws-receive() expects a WebSocket connection, got {}",
                other.type_name()
            ))
        }
    };

    // Get the connection and receive
    let mut connections = WS_CONNECTIONS.lock().unwrap();
    match connections.get_mut(&conn_id) {
        Some(socket) => {
            match socket.read() {
                Ok(msg) => {
                    match msg {
                        Message::Text(text) => Ok(Value::String(text)),
                        Message::Binary(data) => {
                            // Return binary data as base64 encoded string
                            use base64::Engine;
                            let encoded = base64::engine::general_purpose::STANDARD.encode(&data);
                            let mut fields = HashMap::new();
                            fields.insert("type".to_string(), Value::String("binary".to_string()));
                            fields.insert("data".to_string(), Value::String(encoded));
                            Ok(Value::new_struct("ws-message".to_string(), fields))
                        }
                        Message::Ping(_) | Message::Pong(_) => {
                            // Handle ping/pong internally, try to read next message
                            drop(connections);
                            builtin_ws_receive(args)
                        }
                        Message::Close(_) => {
                            Ok(Value::Null)
                        }
                        Message::Frame(_) => {
                            // Raw frame, skip and read next
                            drop(connections);
                            builtin_ws_receive(args)
                        }
                    }
                }
                Err(e) => {
                    // Connection error or closed
                    Err(format!("WebSocket receive failed: {}", e))
                }
            }
        }
        None => Err(format!("WebSocket connection {} not found or closed", conn_id)),
    }
}

/// Close a WebSocket connection
/// Usage: ws-close(ws)
/// Returns: true on success
pub fn builtin_ws_close(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("ws-close() requires 1 argument (connection)".to_string());
    }

    // Get the connection ID
    let conn_id = match &args[0] {
        Value::Struct { fields, .. } => {
            let borrowed = fields.borrow();
            match borrowed.get("id") {
                Some(Value::Number(n)) => *n as u64,
                _ => return Err("Invalid WebSocket connection object".to_string()),
            }
        }
        Value::Number(n) => *n as u64,
        other => {
            return Err(format!(
                "ws-close() expects a WebSocket connection, got {}",
                other.type_name()
            ))
        }
    };

    // Remove and close the connection
    let mut connections = WS_CONNECTIONS.lock().unwrap();
    match connections.remove(&conn_id) {
        Some(mut socket) => {
            let _ = socket.close(None); // Ignore close errors
            Ok(Value::Boolean(true))
        }
        None => {
            // Already closed, that's fine
            Ok(Value::Boolean(true))
        }
    }
}

/// Check if a WebSocket connection is still open
/// Usage: ws-connected(ws)
/// Returns: true if connected, false otherwise
pub fn builtin_ws_connected(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("ws-connected() requires 1 argument (connection)".to_string());
    }

    // Get the connection ID
    let conn_id = match &args[0] {
        Value::Struct { fields, .. } => {
            let borrowed = fields.borrow();
            match borrowed.get("id") {
                Some(Value::Number(n)) => *n as u64,
                _ => return Err("Invalid WebSocket connection object".to_string()),
            }
        }
        Value::Number(n) => *n as u64,
        other => {
            return Err(format!(
                "ws-connected() expects a WebSocket connection, got {}",
                other.type_name()
            ))
        }
    };

    let connections = WS_CONNECTIONS.lock().unwrap();
    Ok(Value::Boolean(connections.contains_key(&conn_id)))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parse_simple() {
        let result = builtin_json_parse(vec![Value::String(r#"{"name": "duck"}"#.to_string())]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_stringify() {
        let mut fields = HashMap::new();
        fields.insert("name".to_string(), Value::String("duck".to_string()));
        let value = Value::new_struct("test".to_string(), fields);

        let result = builtin_json_stringify(vec![value]);
        assert!(result.is_ok());
        if let Ok(Value::String(s)) = result {
            assert!(s.contains("duck"));
        }
    }

    #[test]
    fn test_base64_encode() {
        let result = builtin_base64_encode(vec![Value::String("hello".to_string())]);
        assert!(matches!(result, Ok(Value::String(s)) if s == "aGVsbG8="));
    }

    #[test]
    fn test_base64_decode() {
        let result = builtin_base64_decode(vec![Value::String("aGVsbG8=".to_string())]);
        assert!(matches!(result, Ok(Value::String(s)) if s == "hello"));
    }

    #[test]
    fn test_websocket_connect_requires_url() {
        let result = builtin_ws_connect(vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires 1 argument"));
    }

    #[test]
    fn test_websocket_send_requires_args() {
        let result = builtin_ws_send(vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires 2 arguments"));
    }

    #[test]
    fn test_websocket_receive_requires_connection() {
        let result = builtin_ws_receive(vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires 1 argument"));
    }

    #[test]
    fn test_websocket_close_requires_connection() {
        let result = builtin_ws_close(vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires 1 argument"));
    }
}
