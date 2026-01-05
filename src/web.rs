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
// WebSocket Support (TODO: Future Implementation)
// =============================================================================

// WebSocket functionality planned for future versions.
//
// Proposed API:
//
// ```duck
// -- Connect to a WebSocket server
// quack [let ws be ws-connect("wss://echo.websocket.org")]
//
// -- Send a message
// quack [ws-send(ws, "Hello, server!")]
//
// -- Receive a message (blocking)
// quack [let message be ws-receive(ws)]
//
// -- Close the connection
// quack [ws-close(ws)]
//
// -- Event-based handling (callback style)
// quack [ws-on-message(ws, [msg] -> [
//   quack [print f"Received: {msg}"]
// ])]
//
// quack [ws-on-error(ws, [err] -> [
//   quack [print f"Error: {err}"]
// ])]
//
// quack [ws-on-close(ws, [x] -> [
//   quack [print "Connection closed"]
// ])]
// ```
//
// Implementation notes:
// - Will require `tungstenite` or `tokio-tungstenite` crate
// - Need to handle async nature of WebSockets
// - Consider whether to use blocking or async runtime
// - WebSocket connections should be stored as a new Value type

/// Placeholder for WebSocket connect (not yet implemented)
pub fn builtin_ws_connect(_args: Vec<Value>) -> Result<Value, String> {
    Err("WebSocket support coming soon! The goose is still learning to swim in async waters.".to_string())
}

/// Placeholder for WebSocket send (not yet implemented)
pub fn builtin_ws_send(_args: Vec<Value>) -> Result<Value, String> {
    Err("WebSocket support coming soon! The goose is still learning to swim in async waters.".to_string())
}

/// Placeholder for WebSocket receive (not yet implemented)
pub fn builtin_ws_receive(_args: Vec<Value>) -> Result<Value, String> {
    Err("WebSocket support coming soon! The goose is still learning to swim in async waters.".to_string())
}

/// Placeholder for WebSocket close (not yet implemented)
pub fn builtin_ws_close(_args: Vec<Value>) -> Result<Value, String> {
    Err("WebSocket support coming soon! The goose is still learning to swim in async waters.".to_string())
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
    fn test_websocket_placeholder() {
        let result = builtin_ws_connect(vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("coming soon"));
    }
}
