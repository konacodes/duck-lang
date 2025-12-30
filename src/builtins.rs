// Built-in functions for Duck language

use crate::values::Value;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::fs;
use std::path::Path;

/// Check if a function name is a built-in function
pub fn is_builtin(name: &str) -> bool {
    matches!(
        name,
        "print"
            | "input"
            | "random"
            | "floor"
            | "ceil"
            | "abs"
            | "type-of"
            | "len"
            | "push"
            | "pop"
            | "string"
            | "number"
            | "sqrt"
            | "pow"
            | "min"
            | "max"
            | "range"
            // Phase 1: String/list operations
            | "reverse"
            | "sort"
            | "join"
            | "split"
            | "trim"
            | "uppercase"
            | "lowercase"
            | "contains"
            | "sleep"
            | "keys"
            | "values"
            // Phase 2: File I/O
            | "read-file"
            | "write-file"
            | "append-file"
            | "file-exists"
    )
}

/// Call a built-in function with the given arguments
pub fn call_builtin(name: &str, args: Vec<Value>) -> Result<Value, String> {
    match name {
        "print" => builtin_print(args),
        "input" => builtin_input(args),
        "random" => builtin_random(args),
        "floor" => builtin_floor(args),
        "ceil" => builtin_ceil(args),
        "abs" => builtin_abs(args),
        "type-of" => builtin_type_of(args),
        "len" => builtin_len(args),
        "push" => builtin_push(args),
        "pop" => builtin_pop(args),
        "string" => builtin_string(args),
        "number" => builtin_number(args),
        "sqrt" => builtin_sqrt(args),
        "pow" => builtin_pow(args),
        "min" => builtin_min(args),
        "max" => builtin_max(args),
        "range" => builtin_range(args),
        // Phase 1: String/list operations
        "reverse" => builtin_reverse(args),
        "sort" => builtin_sort(args),
        "join" => builtin_join(args),
        "split" => builtin_split(args),
        "trim" => builtin_trim(args),
        "uppercase" => builtin_uppercase(args),
        "lowercase" => builtin_lowercase(args),
        "contains" => builtin_contains(args),
        "sleep" => builtin_sleep(args),
        "keys" => builtin_keys(args),
        "values" => builtin_values(args),
        // Phase 2: File I/O
        "read-file" => builtin_read_file(args),
        "write-file" => builtin_write_file(args),
        "append-file" => builtin_append_file(args),
        "file-exists" => builtin_file_exists(args),
        _ => Err(format!("Unknown builtin: {}", name)),
    }
}

/// Print all arguments space-separated, then a newline
fn builtin_print(args: Vec<Value>) -> Result<Value, String> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    println!();
    io::stdout().flush().ok();
    Ok(Value::Null)
}

/// Read a line from stdin. Optional prompt argument
fn builtin_input(args: Vec<Value>) -> Result<Value, String> {
    if let Some(prompt) = args.first() {
        print!("{}", prompt);
        io::stdout().flush().ok();
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed = input.trim_end_matches('\n').trim_end_matches('\r');
            Ok(Value::String(trimmed.to_string()))
        }
        Err(e) => Err(format!("Failed to read input: {}", e)),
    }
}

/// Return a pseudo-random f64 between 0.0 and 1.0
fn builtin_random(_args: Vec<Value>) -> Result<Value, String> {
    // Simple pseudo-random using time-based seed
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let nanos = duration.subsec_nanos() as f64;
    let rand = (nanos / 1_000_000_000.0).fract();
    Ok(Value::Number(rand))
}

/// Return the floor of a number
fn builtin_floor(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.floor())),
        Some(other) => Err(format!("floor() expects a number, got {}", other.type_name())),
        None => Err("floor() requires 1 argument".to_string()),
    }
}

/// Return the ceiling of a number
fn builtin_ceil(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.ceil())),
        Some(other) => Err(format!("ceil() expects a number, got {}", other.type_name())),
        None => Err("ceil() requires 1 argument".to_string()),
    }
}

/// Return the absolute value of a number
fn builtin_abs(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.abs())),
        Some(other) => Err(format!("abs() expects a number, got {}", other.type_name())),
        None => Err("abs() requires 1 argument".to_string()),
    }
}

/// Return the type of a value as a string
fn builtin_type_of(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(value) => Ok(Value::String(value.type_name().to_string())),
        None => Err("type-of() requires 1 argument".to_string()),
    }
}

/// Return the length of a list or string
fn builtin_len(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::List(items)) => Ok(Value::Number(items.borrow().len() as f64)),
        Some(Value::String(s)) => Ok(Value::Number(s.chars().count() as f64)),
        Some(other) => Err(format!(
            "len() expects a list or string, got {}",
            other.type_name()
        )),
        None => Err("len() requires 1 argument".to_string()),
    }
}

/// Push an item to a list (mutates the list)
fn builtin_push(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("push() requires 2 arguments, got {}", args.len()));
    }

    match &args[0] {
        Value::List(items) => {
            items.borrow_mut().push(args[1].clone());
            Ok(Value::Null)
        }
        other => Err(format!(
            "push() expects a list as first argument, got {}",
            other.type_name()
        )),
    }
}

/// Pop an item from a list (mutates the list, returns popped item)
fn builtin_pop(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::List(items)) => {
            items
                .borrow_mut()
                .pop()
                .ok_or_else(|| "pop() called on empty list".to_string())
        }
        Some(other) => Err(format!("pop() expects a list, got {}", other.type_name())),
        None => Err("pop() requires 1 argument".to_string()),
    }
}

/// Convert a value to a string
fn builtin_string(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(value) => Ok(Value::String(format!("{}", value))),
        None => Err("string() requires 1 argument".to_string()),
    }
}

/// Convert a value to a number
fn builtin_number(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::String(s)) => s
            .parse::<f64>()
            .map(Value::Number)
            .map_err(|_| format!("Cannot convert '{}' to number", s)),
        Some(Value::Number(n)) => Ok(Value::Number(*n)),
        Some(Value::Boolean(b)) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
        Some(other) => Err(format!("number() cannot convert {}", other.type_name())),
        None => Err("number() requires 1 argument".to_string()),
    }
}

/// Return the square root of a number
fn builtin_sqrt(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::Number(n)) => {
            if *n < 0.0 {
                Err("sqrt() called with negative number".to_string())
            } else {
                Ok(Value::Number(n.sqrt()))
            }
        }
        Some(other) => Err(format!("sqrt() expects a number, got {}", other.type_name())),
        None => Err("sqrt() requires 1 argument".to_string()),
    }
}

/// Return base raised to the power of exponent
fn builtin_pow(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("pow() requires 2 arguments, got {}", args.len()));
    }

    match (&args[0], &args[1]) {
        (Value::Number(base), Value::Number(exp)) => Ok(Value::Number(base.powf(*exp))),
        (Value::Number(_), other) => Err(format!(
            "pow() expects numbers, got {}",
            other.type_name()
        )),
        (other, _) => Err(format!("pow() expects numbers, got {}", other.type_name())),
    }
}

/// Return the minimum of the given numbers
fn builtin_min(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("min() requires at least 1 argument".to_string());
    }

    let mut min_val = match &args[0] {
        Value::Number(n) => *n,
        other => {
            return Err(format!("min() expects numbers, got {}", other.type_name()));
        }
    };

    for arg in args.iter().skip(1) {
        match arg {
            Value::Number(n) => {
                if *n < min_val {
                    min_val = *n;
                }
            }
            other => {
                return Err(format!("min() expects numbers, got {}", other.type_name()));
            }
        }
    }

    Ok(Value::Number(min_val))
}

/// Return the maximum of the given numbers
fn builtin_max(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("max() requires at least 1 argument".to_string());
    }

    let mut max_val = match &args[0] {
        Value::Number(n) => *n,
        other => {
            return Err(format!("max() expects numbers, got {}", other.type_name()));
        }
    };

    for arg in args.iter().skip(1) {
        match arg {
            Value::Number(n) => {
                if *n > max_val {
                    max_val = *n;
                }
            }
            other => {
                return Err(format!("max() expects numbers, got {}", other.type_name()));
            }
        }
    }

    Ok(Value::Number(max_val))
}

/// Create a range of numbers from start to end (exclusive)
fn builtin_range(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("range() requires 2 arguments, got {}", args.len()));
    }

    match (&args[0], &args[1]) {
        (Value::Number(start), Value::Number(end)) => {
            let s = *start as i64;
            let e = *end as i64;
            let items: Vec<Value> = (s..e).map(|i| Value::Number(i as f64)).collect();
            Ok(Value::new_list(items))
        }
        (Value::Number(_), other) => Err(format!(
            "range() expects numbers, got {}",
            other.type_name()
        )),
        (other, _) => Err(format!("range() expects numbers, got {}", other.type_name())),
    }
}

// =============================================================================
// Phase 1: String/List Operations
// =============================================================================

/// Reverse a list or string
fn builtin_reverse(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::List(items)) => {
            let mut reversed: Vec<Value> = items.borrow().clone();
            reversed.reverse();
            Ok(Value::new_list(reversed))
        }
        Some(Value::String(s)) => {
            let reversed: String = s.chars().rev().collect();
            Ok(Value::String(reversed))
        }
        Some(other) => Err(format!(
            "reverse() expects a list or string, got {}",
            other.type_name()
        )),
        None => Err("reverse() requires 1 argument".to_string()),
    }
}

/// Sort a list of numbers or strings
fn builtin_sort(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::List(items)) => {
            let borrowed = items.borrow();
            if borrowed.is_empty() {
                return Ok(Value::new_list(vec![]));
            }

            // Check if all numbers or all strings
            let first = &borrowed[0];
            let mut sorted: Vec<Value> = borrowed.clone();

            match first {
                Value::Number(_) => {
                    // Verify all are numbers
                    for v in &sorted {
                        if !matches!(v, Value::Number(_)) {
                            return Err("sort() cannot sort mixed types".to_string());
                        }
                    }
                    sorted.sort_by(|a, b| {
                        if let (Value::Number(na), Value::Number(nb)) = (a, b) {
                            na.partial_cmp(nb).unwrap_or(std::cmp::Ordering::Equal)
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    });
                }
                Value::String(_) => {
                    // Verify all are strings
                    for v in &sorted {
                        if !matches!(v, Value::String(_)) {
                            return Err("sort() cannot sort mixed types".to_string());
                        }
                    }
                    sorted.sort_by(|a, b| {
                        if let (Value::String(sa), Value::String(sb)) = (a, b) {
                            sa.cmp(sb)
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    });
                }
                other => {
                    return Err(format!(
                        "sort() can only sort numbers or strings, got {}",
                        other.type_name()
                    ));
                }
            }

            Ok(Value::new_list(sorted))
        }
        Some(other) => Err(format!("sort() expects a list, got {}", other.type_name())),
        None => Err("sort() requires 1 argument".to_string()),
    }
}

/// Join a list of values with a separator
fn builtin_join(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("join() requires 2 arguments, got {}", args.len()));
    }

    match (&args[0], &args[1]) {
        (Value::List(items), Value::String(sep)) => {
            let strings: Vec<String> = items.borrow().iter().map(|v| format!("{}", v)).collect();
            Ok(Value::String(strings.join(sep)))
        }
        (Value::List(_), other) => Err(format!(
            "join() expects a string separator, got {}",
            other.type_name()
        )),
        (other, _) => Err(format!(
            "join() expects a list as first argument, got {}",
            other.type_name()
        )),
    }
}

/// Split a string by a separator
fn builtin_split(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("split() requires 2 arguments, got {}", args.len()));
    }

    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(sep)) => {
            let parts: Vec<Value> = s.split(sep.as_str()).map(|p| Value::String(p.to_string())).collect();
            Ok(Value::new_list(parts))
        }
        (Value::String(_), other) => Err(format!(
            "split() expects a string separator, got {}",
            other.type_name()
        )),
        (other, _) => Err(format!(
            "split() expects a string as first argument, got {}",
            other.type_name()
        )),
    }
}

/// Trim whitespace from a string
fn builtin_trim(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::String(s)) => Ok(Value::String(s.trim().to_string())),
        Some(other) => Err(format!("trim() expects a string, got {}", other.type_name())),
        None => Err("trim() requires 1 argument".to_string()),
    }
}

/// Convert string to uppercase
fn builtin_uppercase(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::String(s)) => Ok(Value::String(s.to_uppercase())),
        Some(other) => Err(format!(
            "uppercase() expects a string, got {}",
            other.type_name()
        )),
        None => Err("uppercase() requires 1 argument".to_string()),
    }
}

/// Convert string to lowercase
fn builtin_lowercase(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::String(s)) => Ok(Value::String(s.to_lowercase())),
        Some(other) => Err(format!(
            "lowercase() expects a string, got {}",
            other.type_name()
        )),
        None => Err("lowercase() requires 1 argument".to_string()),
    }
}

/// Check if a list contains a value or a string contains a substring
fn builtin_contains(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("contains() requires 2 arguments, got {}", args.len()));
    }

    match &args[0] {
        Value::List(items) => {
            let needle = &args[1];
            let found = items.borrow().iter().any(|item| item == needle);
            Ok(Value::Boolean(found))
        }
        Value::String(haystack) => {
            match &args[1] {
                Value::String(needle) => Ok(Value::Boolean(haystack.contains(needle.as_str()))),
                other => Err(format!(
                    "contains() expects a string needle for string search, got {}",
                    other.type_name()
                )),
            }
        }
        other => Err(format!(
            "contains() expects a list or string, got {}",
            other.type_name()
        )),
    }
}

/// Sleep for a specified number of milliseconds
fn builtin_sleep(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::Number(ms)) => {
            if *ms < 0.0 {
                return Err("sleep() requires a non-negative number".to_string());
            }
            thread::sleep(Duration::from_millis(*ms as u64));
            Ok(Value::Null)
        }
        Some(other) => Err(format!("sleep() expects a number, got {}", other.type_name())),
        None => Err("sleep() requires 1 argument".to_string()),
    }
}

/// Get keys from a struct
fn builtin_keys(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::Struct { fields, .. }) => {
            let keys: Vec<Value> = fields
                .borrow()
                .keys()
                .map(|k| Value::String(k.clone()))
                .collect();
            Ok(Value::new_list(keys))
        }
        Some(other) => Err(format!("keys() expects a struct, got {}", other.type_name())),
        None => Err("keys() requires 1 argument".to_string()),
    }
}

/// Get values from a struct
fn builtin_values(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::Struct { fields, .. }) => {
            let vals: Vec<Value> = fields.borrow().values().cloned().collect();
            Ok(Value::new_list(vals))
        }
        Some(other) => Err(format!(
            "values() expects a struct, got {}",
            other.type_name()
        )),
        None => Err("values() requires 1 argument".to_string()),
    }
}

// =============================================================================
// Phase 2: File I/O
// =============================================================================

/// Read entire file contents as a string
fn builtin_read_file(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::String(path)) => {
            fs::read_to_string(path).map(Value::String).map_err(|e| {
                if e.kind() == io::ErrorKind::NotFound {
                    format!("The goose searched everywhere but couldn't find '{}'", path)
                } else if e.kind() == io::ErrorKind::PermissionDenied {
                    format!("The goose is not allowed to look at '{}'", path)
                } else {
                    format!("Failed to read '{}': {}", path, e)
                }
            })
        }
        Some(other) => Err(format!(
            "read-file() expects a string path, got {}",
            other.type_name()
        )),
        None => Err("read-file() requires 1 argument".to_string()),
    }
}

/// Write a string to a file (creates or overwrites)
fn builtin_write_file(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("write-file() requires 2 arguments, got {}", args.len()));
    }

    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(content)) => {
            fs::write(path, content).map(|_| Value::Null).map_err(|e| {
                if e.kind() == io::ErrorKind::PermissionDenied {
                    format!("The goose is not allowed to write to '{}'", path)
                } else {
                    format!("Failed to write '{}': {}", path, e)
                }
            })
        }
        (Value::String(_), other) => Err(format!(
            "write-file() expects string content, got {}",
            other.type_name()
        )),
        (other, _) => Err(format!(
            "write-file() expects a string path, got {}",
            other.type_name()
        )),
    }
}

/// Append a string to a file
fn builtin_append_file(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("append-file() requires 2 arguments, got {}", args.len()));
    }

    match (&args[0], &args[1]) {
        (Value::String(path), Value::String(content)) => {
            use std::fs::OpenOptions;
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path);

            match file {
                Ok(mut f) => {
                    use std::io::Write;
                    f.write_all(content.as_bytes())
                        .map(|_| Value::Null)
                        .map_err(|e| format!("Failed to append to '{}': {}", path, e))
                }
                Err(e) => {
                    if e.kind() == io::ErrorKind::PermissionDenied {
                        Err(format!("The goose is not allowed to write to '{}'", path))
                    } else {
                        Err(format!("Failed to open '{}': {}", path, e))
                    }
                }
            }
        }
        (Value::String(_), other) => Err(format!(
            "append-file() expects string content, got {}",
            other.type_name()
        )),
        (other, _) => Err(format!(
            "append-file() expects a string path, got {}",
            other.type_name()
        )),
    }
}

/// Check if a file exists
fn builtin_file_exists(args: Vec<Value>) -> Result<Value, String> {
    match args.first() {
        Some(Value::String(path)) => Ok(Value::Boolean(Path::new(path).exists())),
        Some(other) => Err(format!(
            "file-exists() expects a string path, got {}",
            other.type_name()
        )),
        None => Err("file-exists() requires 1 argument".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_builtin() {
        assert!(is_builtin("print"));
        assert!(is_builtin("input"));
        assert!(is_builtin("random"));
        assert!(is_builtin("floor"));
        assert!(is_builtin("ceil"));
        assert!(is_builtin("abs"));
        assert!(is_builtin("type-of"));
        assert!(is_builtin("range"));
        assert!(!is_builtin("unknown"));
    }

    #[test]
    fn test_floor() {
        let result = builtin_floor(vec![Value::Number(3.7)]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 3.0));

        let result = builtin_floor(vec![Value::Number(-3.2)]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == -4.0));
    }

    #[test]
    fn test_ceil() {
        let result = builtin_ceil(vec![Value::Number(3.2)]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 4.0));

        let result = builtin_ceil(vec![Value::Number(-3.7)]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == -3.0));
    }

    #[test]
    fn test_abs() {
        let result = builtin_abs(vec![Value::Number(-5.0)]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 5.0));

        let result = builtin_abs(vec![Value::Number(5.0)]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 5.0));
    }

    #[test]
    fn test_type_of() {
        assert!(matches!(
            builtin_type_of(vec![Value::Number(1.0)]),
            Ok(Value::String(s)) if s == "number"
        ));
        assert!(matches!(
            builtin_type_of(vec![Value::String("hi".to_string())]),
            Ok(Value::String(s)) if s == "string"
        ));
        assert!(matches!(
            builtin_type_of(vec![Value::Boolean(true)]),
            Ok(Value::String(s)) if s == "boolean"
        ));
        assert!(matches!(
            builtin_type_of(vec![Value::Null]),
            Ok(Value::String(s)) if s == "null"
        ));
    }

    #[test]
    fn test_len() {
        let list = Value::new_list(vec![Value::Number(1.0), Value::Number(2.0)]);
        let result = builtin_len(vec![list]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 2.0));

        let result = builtin_len(vec![Value::String("hello".to_string())]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 5.0));
    }

    #[test]
    fn test_push_pop() {
        let list = Value::new_list(vec![Value::Number(1.0)]);

        // Push
        let _ = builtin_push(vec![list.clone(), Value::Number(2.0)]);
        let result = builtin_len(vec![list.clone()]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 2.0));

        // Pop
        let popped = builtin_pop(vec![list.clone()]).unwrap();
        assert!(matches!(popped, Value::Number(n) if n == 2.0));
        let result = builtin_len(vec![list]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 1.0));
    }

    #[test]
    fn test_sqrt() {
        let result = builtin_sqrt(vec![Value::Number(16.0)]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 4.0));

        let result = builtin_sqrt(vec![Value::Number(-1.0)]);
        assert!(result.is_err());
    }

    #[test]
    fn test_pow() {
        let result = builtin_pow(vec![Value::Number(2.0), Value::Number(3.0)]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 8.0));
    }

    #[test]
    fn test_min_max() {
        let result = builtin_min(vec![
            Value::Number(3.0),
            Value::Number(1.0),
            Value::Number(2.0),
        ]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 1.0));

        let result = builtin_max(vec![
            Value::Number(3.0),
            Value::Number(1.0),
            Value::Number(2.0),
        ]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 3.0));
    }

    #[test]
    fn test_range() {
        let result = builtin_range(vec![Value::Number(0.0), Value::Number(3.0)]).unwrap();
        if let Value::List(items) = result {
            let borrowed = items.borrow();
            assert_eq!(borrowed.len(), 3);
        } else {
            panic!("Expected list");
        }
    }

    #[test]
    fn test_string_conversion() {
        let result = builtin_string(vec![Value::Number(42.0)]);
        assert!(matches!(result, Ok(Value::String(s)) if s == "42"));

        let result = builtin_string(vec![Value::Boolean(true)]);
        assert!(matches!(result, Ok(Value::String(s)) if s == "true"));
    }

    #[test]
    fn test_number_conversion() {
        let result = builtin_number(vec![Value::String("42".to_string())]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 42.0));

        let result = builtin_number(vec![Value::Boolean(true)]);
        assert!(matches!(result, Ok(Value::Number(n)) if n == 1.0));
    }

    #[test]
    fn test_random() {
        let result = builtin_random(vec![]);
        match result {
            Ok(Value::Number(n)) => {
                assert!(n >= 0.0 && n < 1.0);
            }
            _ => panic!("Expected number"),
        }
    }

    // Phase 1 tests

    #[test]
    fn test_reverse() {
        // Reverse list
        let list = Value::new_list(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);
        let result = builtin_reverse(vec![list]).unwrap();
        if let Value::List(items) = result {
            let borrowed = items.borrow();
            assert!(matches!(&borrowed[0], Value::Number(n) if *n == 3.0));
            assert!(matches!(&borrowed[2], Value::Number(n) if *n == 1.0));
        } else {
            panic!("Expected list");
        }

        // Reverse string
        let result = builtin_reverse(vec![Value::String("hello".to_string())]).unwrap();
        assert!(matches!(result, Value::String(s) if s == "olleh"));
    }

    #[test]
    fn test_sort() {
        // Sort numbers
        let list = Value::new_list(vec![
            Value::Number(3.0),
            Value::Number(1.0),
            Value::Number(2.0),
        ]);
        let result = builtin_sort(vec![list]).unwrap();
        if let Value::List(items) = result {
            let borrowed = items.borrow();
            assert!(matches!(&borrowed[0], Value::Number(n) if *n == 1.0));
            assert!(matches!(&borrowed[1], Value::Number(n) if *n == 2.0));
            assert!(matches!(&borrowed[2], Value::Number(n) if *n == 3.0));
        } else {
            panic!("Expected list");
        }

        // Sort strings
        let list = Value::new_list(vec![
            Value::String("c".to_string()),
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]);
        let result = builtin_sort(vec![list]).unwrap();
        if let Value::List(items) = result {
            let borrowed = items.borrow();
            assert!(matches!(&borrowed[0], Value::String(s) if s == "a"));
            assert!(matches!(&borrowed[1], Value::String(s) if s == "b"));
            assert!(matches!(&borrowed[2], Value::String(s) if s == "c"));
        } else {
            panic!("Expected list");
        }
    }

    #[test]
    fn test_join() {
        let list = Value::new_list(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
            Value::String("c".to_string()),
        ]);
        let result = builtin_join(vec![list, Value::String(",".to_string())]).unwrap();
        assert!(matches!(result, Value::String(s) if s == "a,b,c"));
    }

    #[test]
    fn test_split() {
        let result = builtin_split(vec![
            Value::String("a,b,c".to_string()),
            Value::String(",".to_string()),
        ])
        .unwrap();
        if let Value::List(items) = result {
            let borrowed = items.borrow();
            assert_eq!(borrowed.len(), 3);
            assert!(matches!(&borrowed[0], Value::String(s) if s == "a"));
            assert!(matches!(&borrowed[1], Value::String(s) if s == "b"));
            assert!(matches!(&borrowed[2], Value::String(s) if s == "c"));
        } else {
            panic!("Expected list");
        }
    }

    #[test]
    fn test_trim() {
        let result = builtin_trim(vec![Value::String("  hello  ".to_string())]).unwrap();
        assert!(matches!(result, Value::String(s) if s == "hello"));
    }

    #[test]
    fn test_uppercase_lowercase() {
        let result = builtin_uppercase(vec![Value::String("hello".to_string())]).unwrap();
        assert!(matches!(result, Value::String(s) if s == "HELLO"));

        let result = builtin_lowercase(vec![Value::String("HELLO".to_string())]).unwrap();
        assert!(matches!(result, Value::String(s) if s == "hello"));
    }

    #[test]
    fn test_contains() {
        // List contains
        let list = Value::new_list(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);
        let result = builtin_contains(vec![list.clone(), Value::Number(2.0)]).unwrap();
        assert!(matches!(result, Value::Boolean(true)));

        let result = builtin_contains(vec![list, Value::Number(5.0)]).unwrap();
        assert!(matches!(result, Value::Boolean(false)));

        // String contains
        let result = builtin_contains(vec![
            Value::String("hello world".to_string()),
            Value::String("world".to_string()),
        ])
        .unwrap();
        assert!(matches!(result, Value::Boolean(true)));

        let result = builtin_contains(vec![
            Value::String("hello".to_string()),
            Value::String("xyz".to_string()),
        ])
        .unwrap();
        assert!(matches!(result, Value::Boolean(false)));
    }

    #[test]
    fn test_file_exists() {
        // Test with a file that definitely exists
        let result = builtin_file_exists(vec![Value::String("Cargo.toml".to_string())]).unwrap();
        assert!(matches!(result, Value::Boolean(true)));

        // Test with a file that doesn't exist
        let result =
            builtin_file_exists(vec![Value::String("nonexistent_file_12345.txt".to_string())])
                .unwrap();
        assert!(matches!(result, Value::Boolean(false)));
    }
}
