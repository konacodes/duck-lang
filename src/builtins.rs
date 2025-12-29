// Built-in functions for Duck language

use crate::values::Value;
use std::io::{self, Write};

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
}
