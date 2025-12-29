// Runtime value types for Duck language

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ast::{Block, Expr};

/// Environment snapshot for closures - captures variables at function definition time
#[derive(Debug, Clone)]
pub struct Closure {
    /// Captured variables from the enclosing scope
    pub captured: Rc<RefCell<HashMap<String, Value>>>,
}

impl Closure {
    /// Create a new empty closure
    pub fn new() -> Self {
        Closure {
            captured: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Create a closure from a map of captured variables
    pub fn from_map(vars: HashMap<String, Value>) -> Self {
        Closure {
            captured: Rc::new(RefCell::new(vars)),
        }
    }

    /// Get a value from the closure
    pub fn get(&self, name: &str) -> Option<Value> {
        self.captured.borrow().get(name).cloned()
    }

    /// Set a value in the closure
    pub fn set(&self, name: String, value: Value) {
        self.captured.borrow_mut().insert(name, value);
    }
}

impl Default for Closure {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Closure {
    fn eq(&self, other: &Self) -> bool {
        // Closures are equal if they point to the same allocation
        Rc::ptr_eq(&self.captured, &other.captured)
    }
}

/// Runtime values in Duck language
#[derive(Debug, Clone)]
pub enum Value {
    /// A floating-point number (all numbers in Duck are f64)
    Number(f64),

    /// A UTF-8 string
    String(String),

    /// A boolean value
    Boolean(bool),

    /// A list of values (mutable, reference-counted)
    List(Rc<RefCell<Vec<Value>>>),

    /// A struct instance with named fields (mutable, reference-counted)
    Struct {
        name: String,
        fields: Rc<RefCell<HashMap<String, Value>>>,
    },

    /// A user-defined function
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Block>,
        closure: Closure,
    },

    /// A lambda/anonymous function (expression-bodied)
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
        closure: Closure,
    },

    /// A built-in function (identified by name)
    BuiltinFunction(String),

    /// A struct type definition (not an instance, but the type itself)
    StructType {
        name: String,
        fields: Vec<String>,
    },

    /// The null value
    Null,
}

impl Value {
    /// Get the type name of this value as a string
    pub fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::List(_) => "list",
            Value::Struct { name, .. } => name,
            Value::Function { .. } => "function",
            Value::Lambda { .. } => "lambda",
            Value::BuiltinFunction(_) => "builtin",
            Value::StructType { name, .. } => name,
            Value::Null => "null",
        }
    }

    /// Determine if this value is truthy
    /// In Duck: false, null, 0, "", and empty lists are falsy; everything else is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(list) => !list.borrow().is_empty(),
            // Functions, structs, and struct types are always truthy
            Value::Function { .. } => true,
            Value::Lambda { .. } => true,
            Value::BuiltinFunction(_) => true,
            Value::Struct { .. } => true,
            Value::StructType { .. } => true,
        }
    }

    /// Check if this value is null
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Try to get this value as a number
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to get this value as a string reference
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get this value as a boolean
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Try to get this value as a list
    pub fn as_list(&self) -> Option<Rc<RefCell<Vec<Value>>>> {
        match self {
            Value::List(list) => Some(Rc::clone(list)),
            _ => None,
        }
    }

    /// Create a new list value
    pub fn new_list(values: Vec<Value>) -> Value {
        Value::List(Rc::new(RefCell::new(values)))
    }

    /// Create a new struct instance
    pub fn new_struct(name: String, fields: HashMap<String, Value>) -> Value {
        Value::Struct {
            name,
            fields: Rc::new(RefCell::new(fields)),
        }
    }

    /// Create a new function value
    pub fn new_function(
        name: String,
        params: Vec<String>,
        body: Vec<Block>,
        closure: Closure,
    ) -> Value {
        Value::Function {
            name,
            params,
            body,
            closure,
        }
    }

    /// Create a new lambda value
    pub fn new_lambda(params: Vec<String>, body: Expr, closure: Closure) -> Value {
        Value::Lambda {
            params,
            body: Box::new(body),
            closure,
        }
    }

    /// Deep clone a value, creating new Rc/RefCell wrappers for mutable types
    pub fn deep_clone(&self) -> Value {
        match self {
            Value::List(list) => {
                let cloned: Vec<Value> = list.borrow().iter().map(|v| v.deep_clone()).collect();
                Value::List(Rc::new(RefCell::new(cloned)))
            }
            Value::Struct { name, fields } => {
                let cloned: HashMap<String, Value> = fields
                    .borrow()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.deep_clone()))
                    .collect();
                Value::Struct {
                    name: name.clone(),
                    fields: Rc::new(RefCell::new(cloned)),
                }
            }
            // For other types, regular clone is fine
            other => other.clone(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => {
                // Format integers without decimal point
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(list) => {
                let items = list.borrow();
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    // Strings in lists should be quoted
                    if let Value::String(s) = item {
                        write!(f, "\"{}\"", s)?;
                    } else {
                        write!(f, "{}", item)?;
                    }
                }
                write!(f, "]")
            }
            Value::Struct { name, fields } => {
                let field_map = fields.borrow();
                write!(f, "{} {{ ", name)?;
                for (i, (key, value)) in field_map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, " }}")
            }
            Value::Function { name, params, .. } => {
                write!(f, "<function {}({})>", name, params.join(", "))
            }
            Value::Lambda { params, .. } => {
                write!(f, "<lambda ({})>", params.join(", "))
            }
            Value::BuiltinFunction(name) => write!(f, "<builtin {}>", name),
            Value::StructType { name, fields } => {
                write!(f, "<struct {} {{ {} }}>", name, fields.join(", "))
            }
            Value::Null => write!(f, "null"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                // Handle NaN equality (NaN != NaN in IEEE 754, but we want structural equality)
                if a.is_nan() && b.is_nan() {
                    true
                } else {
                    a == b
                }
            }
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::List(a), Value::List(b)) => {
                // Compare by reference first (fast path)
                if Rc::ptr_eq(a, b) {
                    return true;
                }
                // Compare by value
                let a_borrowed = a.borrow();
                let b_borrowed = b.borrow();
                a_borrowed.len() == b_borrowed.len()
                    && a_borrowed
                        .iter()
                        .zip(b_borrowed.iter())
                        .all(|(x, y)| x == y)
            }
            (
                Value::Struct {
                    name: n1,
                    fields: f1,
                },
                Value::Struct {
                    name: n2,
                    fields: f2,
                },
            ) => {
                if n1 != n2 {
                    return false;
                }
                // Compare by reference first (fast path)
                if Rc::ptr_eq(f1, f2) {
                    return true;
                }
                // Compare by value
                let f1_borrowed = f1.borrow();
                let f2_borrowed = f2.borrow();
                if f1_borrowed.len() != f2_borrowed.len() {
                    return false;
                }
                f1_borrowed
                    .iter()
                    .all(|(k, v)| f2_borrowed.get(k).is_some_and(|v2| v == v2))
            }
            (
                Value::Function {
                    name: n1,
                    params: p1,
                    ..
                },
                Value::Function {
                    name: n2,
                    params: p2,
                    ..
                },
            ) => {
                // Functions are equal if they have the same name and parameters
                n1 == n2 && p1 == p2
            }
            (Value::Lambda { params: p1, .. }, Value::Lambda { params: p2, .. }) => {
                // Lambdas with same parameter lists are considered equal
                p1 == p2
            }
            (Value::BuiltinFunction(a), Value::BuiltinFunction(b)) => a == b,
            (
                Value::StructType {
                    name: n1,
                    fields: f1,
                },
                Value::StructType {
                    name: n2,
                    fields: f2,
                },
            ) => n1 == n2 && f1 == f2,
            (Value::Null, Value::Null) => true,
            // Different types are never equal
            _ => false,
        }
    }
}

// Implement Eq for Value (since we've defined PartialEq)
impl Eq for Value {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_type_names() {
        assert_eq!(Value::Number(42.0).type_name(), "number");
        assert_eq!(Value::String("hello".to_string()).type_name(), "string");
        assert_eq!(Value::Boolean(true).type_name(), "boolean");
        assert_eq!(Value::new_list(vec![]).type_name(), "list");
        assert_eq!(Value::Null.type_name(), "null");
    }

    #[test]
    fn test_value_truthiness() {
        // Truthy values
        assert!(Value::Boolean(true).is_truthy());
        assert!(Value::Number(1.0).is_truthy());
        assert!(Value::Number(-1.0).is_truthy());
        assert!(Value::String("hello".to_string()).is_truthy());
        assert!(Value::new_list(vec![Value::Number(1.0)]).is_truthy());

        // Falsy values
        assert!(!Value::Boolean(false).is_truthy());
        assert!(!Value::Null.is_truthy());
        assert!(!Value::Number(0.0).is_truthy());
        assert!(!Value::String("".to_string()).is_truthy());
        assert!(!Value::new_list(vec![]).is_truthy());
    }

    #[test]
    fn test_value_equality() {
        // Numbers
        assert_eq!(Value::Number(42.0), Value::Number(42.0));
        assert_ne!(Value::Number(42.0), Value::Number(43.0));

        // Strings
        assert_eq!(
            Value::String("hello".to_string()),
            Value::String("hello".to_string())
        );
        assert_ne!(
            Value::String("hello".to_string()),
            Value::String("world".to_string())
        );

        // Booleans
        assert_eq!(Value::Boolean(true), Value::Boolean(true));
        assert_ne!(Value::Boolean(true), Value::Boolean(false));

        // Null
        assert_eq!(Value::Null, Value::Null);

        // Lists
        let list1 = Value::new_list(vec![Value::Number(1.0), Value::Number(2.0)]);
        let list2 = Value::new_list(vec![Value::Number(1.0), Value::Number(2.0)]);
        let list3 = Value::new_list(vec![Value::Number(1.0), Value::Number(3.0)]);
        assert_eq!(list1, list2);
        assert_ne!(list1, list3);

        // Different types
        assert_ne!(Value::Number(1.0), Value::String("1".to_string()));
        assert_ne!(Value::Boolean(false), Value::Null);
    }

    #[test]
    fn test_value_display() {
        assert_eq!(format!("{}", Value::Number(42.0)), "42");
        assert_eq!(format!("{}", Value::Number(3.14)), "3.14");
        assert_eq!(format!("{}", Value::String("hello".to_string())), "hello");
        assert_eq!(format!("{}", Value::Boolean(true)), "true");
        assert_eq!(format!("{}", Value::Null), "null");

        let list = Value::new_list(vec![Value::Number(1.0), Value::Number(2.0)]);
        assert_eq!(format!("{}", list), "[1, 2]");
    }

    #[test]
    fn test_list_mutability() {
        let list = Value::new_list(vec![Value::Number(1.0)]);
        if let Value::List(inner) = &list {
            inner.borrow_mut().push(Value::Number(2.0));
            assert_eq!(inner.borrow().len(), 2);
        }
    }

    #[test]
    fn test_deep_clone() {
        let original = Value::new_list(vec![Value::Number(1.0), Value::Number(2.0)]);
        let cloned = original.deep_clone();

        // Modify original
        if let Value::List(inner) = &original {
            inner.borrow_mut().push(Value::Number(3.0));
        }

        // Check that clone is unaffected
        if let Value::List(inner) = &cloned {
            assert_eq!(inner.borrow().len(), 2);
        }
    }
}
