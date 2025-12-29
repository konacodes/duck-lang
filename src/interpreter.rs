// Interpreter - executes Duck programs
// Only executes blocks that were properly "quacked"

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{AssignTarget, BinaryOp, Block, Expr, Literal, Pattern, Statement, StringPart, UnaryOp};
use crate::builtins;
use crate::goose::{self, ErrorKind, ExecutionStats};
use crate::values::{Closure, Value};

/// Control flow signals for statements
#[derive(Debug)]
pub enum ControlFlow {
    /// Normal execution
    None,
    /// Return from a function
    Return(Value),
    /// Break from a loop
    Break,
    /// Continue to next iteration
    Continue,
}

/// Environment for variable storage with lexical scoping
#[derive(Debug, Clone)]
pub struct Environment {
    /// Variables in this scope
    values: HashMap<String, Value>,
    /// Parent scope (if any)
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// Create a new global environment
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            parent: None,
        }
    }

    /// Create a child environment
    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// Define a new variable in this scope
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    /// Get a variable, searching up the scope chain
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(ref parent) = self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }

    /// Assign to an existing variable in any scope
    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            true
        } else if let Some(ref parent) = self.parent {
            parent.borrow_mut().assign(name, value)
        } else {
            false
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

/// The interpreter
pub struct Interpreter {
    /// Global environment
    env: Rc<RefCell<Environment>>,
    /// Execution statistics
    pub stats: ExecutionStats,
}

impl Interpreter {
    /// Create a new interpreter
    pub fn new() -> Self {
        Interpreter {
            env: Rc::new(RefCell::new(Environment::new())),
            stats: ExecutionStats::default(),
        }
    }

    /// Run a complete program (list of blocks)
    pub fn run(&mut self, blocks: Vec<Block>) -> Result<(), String> {
        self.stats.total_blocks = blocks.len();

        for block in blocks {
            if block.was_quacked {
                self.stats.quacked_blocks += 1;
                self.execute_block(&block)?;
            } else {
                self.stats.unquacked_blocks += 1;
                // Report the skipped block with a sarcastic message
                let msg = goose::refusal(block.line, "");
                eprintln!("{}", msg);
            }
        }

        Ok(())
    }

    /// Run a single block (for REPL use)
    /// Returns the value of the last expression if it was an expression statement
    pub fn run_block(&mut self, block: Block) -> Result<Option<Value>, String> {
        self.stats.total_blocks += 1;

        if block.was_quacked {
            self.stats.quacked_blocks += 1;

            // For expression statements, we want to return the value
            match &block.statement {
                Statement::Expression(expr) => {
                    let value = self.evaluate(expr, block.line)?;
                    Ok(Some(value))
                }
                _ => {
                    self.execute_block(&block)?;
                    Ok(None)
                }
            }
        } else {
            self.stats.unquacked_blocks += 1;
            let msg = goose::refusal(block.line, "");
            eprintln!("{}", msg);
            Ok(None)
        }
    }

    /// Get a reference to the execution stats
    pub fn stats(&self) -> &ExecutionStats {
        &self.stats
    }

    /// Execute a single block
    fn execute_block(&mut self, block: &Block) -> Result<ControlFlow, String> {
        self.execute_statement(&block.statement, block.line)
    }

    /// Execute a statement
    fn execute_statement(&mut self, stmt: &Statement, line: usize) -> Result<ControlFlow, String> {
        match stmt {
            Statement::Let { name, value } => {
                let val = self.evaluate(value, line)?;
                self.env.borrow_mut().define(name.clone(), val);
                Ok(ControlFlow::None)
            }

            Statement::Assign { target, value } => {
                let val = self.evaluate(value, line)?;
                self.assign_to_target(target, val, line)?;
                Ok(ControlFlow::None)
            }

            Statement::Expression(expr) => {
                self.evaluate(expr, line)?;
                Ok(ControlFlow::None)
            }

            Statement::Print(expr) => {
                let value = self.evaluate(expr, line)?;
                println!("{}", value);
                Ok(ControlFlow::None)
            }

            Statement::Block(stmts) => {
                let child_env = Environment::with_parent(Rc::clone(&self.env));
                let old_env = std::mem::replace(&mut self.env, Rc::new(RefCell::new(child_env)));

                let result = self.execute_statements(stmts, line);

                self.env = old_env;
                result
            }

            Statement::FunctionDef { name, params, body } => {
                self.stats.functions_defined += 1;
                let closure = self.create_closure();
                let func = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: self.statements_to_blocks(body, line),
                    closure,
                };
                self.env.borrow_mut().define(name.clone(), func);
                Ok(ControlFlow::None)
            }

            Statement::If { condition, then_block, otherwise_block } => {
                let cond_value = self.evaluate(condition, line)?;
                if cond_value.is_truthy() {
                    self.execute_statements(then_block, line)
                } else if let Some(else_stmts) = otherwise_block {
                    self.execute_statements(else_stmts, line)
                } else {
                    Ok(ControlFlow::None)
                }
            }

            Statement::Match { value, arms } => {
                let val = self.evaluate(value, line)?;
                for arm in arms {
                    if let Some(bindings) = self.match_pattern(&arm.pattern, &val) {
                        // Create new scope with pattern bindings
                        let child_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&self.env))));
                        for (name, binding_value) in bindings {
                            child_env.borrow_mut().define(name, binding_value);
                        }
                        let old_env = std::mem::replace(&mut self.env, child_env);

                        let result = if let Some(ref body) = arm.body {
                            self.execute_statements(body, line)
                        } else {
                            Ok(ControlFlow::None)
                        };

                        self.env = old_env;
                        return result;
                    }
                }
                // No arm matched - this is fine, just continue
                Ok(ControlFlow::None)
            }

            Statement::Repeat { count, body } => {
                self.stats.loops_executed += 1;
                let count_val = self.evaluate(count, line)?;
                let n = match count_val {
                    Value::Number(n) => n as i64,
                    _ => {
                        return Err(goose::error(
                            ErrorKind::TypeError {
                                expected: "number".to_string(),
                                got: count_val.type_name().to_string(),
                            },
                            line,
                            "in repeat count",
                        ));
                    }
                };

                for _ in 0..n {
                    match self.execute_statements(body, line)? {
                        ControlFlow::Break => break,
                        ControlFlow::Continue => continue,
                        ControlFlow::Return(v) => return Ok(ControlFlow::Return(v)),
                        ControlFlow::None => {}
                    }
                }

                Ok(ControlFlow::None)
            }

            Statement::While { condition, body } => {
                self.stats.loops_executed += 1;
                while self.evaluate(condition, line)?.is_truthy() {
                    match self.execute_statements(body, line)? {
                        ControlFlow::Break => break,
                        ControlFlow::Continue => continue,
                        ControlFlow::Return(v) => return Ok(ControlFlow::Return(v)),
                        ControlFlow::None => {}
                    }
                }
                Ok(ControlFlow::None)
            }

            Statement::ForEach { variable, iterable, body } => {
                self.stats.loops_executed += 1;
                let collection = self.evaluate(iterable, line)?;

                match collection {
                    Value::List(items) => {
                        let items_borrowed = items.borrow().clone();
                        for item in items_borrowed {
                            let child_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&self.env))));
                            child_env.borrow_mut().define(variable.clone(), item);
                            let old_env = std::mem::replace(&mut self.env, child_env);

                            match self.execute_statements(body, line)? {
                                ControlFlow::Break => {
                                    self.env = old_env;
                                    break;
                                }
                                ControlFlow::Continue => {
                                    self.env = old_env;
                                    continue;
                                }
                                ControlFlow::Return(v) => {
                                    self.env = old_env;
                                    return Ok(ControlFlow::Return(v));
                                }
                                ControlFlow::None => {}
                            }

                            self.env = old_env;
                        }
                    }
                    Value::String(s) => {
                        for c in s.chars() {
                            let child_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&self.env))));
                            child_env.borrow_mut().define(variable.clone(), Value::String(c.to_string()));
                            let old_env = std::mem::replace(&mut self.env, child_env);

                            match self.execute_statements(body, line)? {
                                ControlFlow::Break => {
                                    self.env = old_env;
                                    break;
                                }
                                ControlFlow::Continue => {
                                    self.env = old_env;
                                    continue;
                                }
                                ControlFlow::Return(v) => {
                                    self.env = old_env;
                                    return Ok(ControlFlow::Return(v));
                                }
                                ControlFlow::None => {}
                            }

                            self.env = old_env;
                        }
                    }
                    _ => {
                        return Err(goose::error(
                            ErrorKind::TypeError {
                                expected: "list or string".to_string(),
                                got: collection.type_name().to_string(),
                            },
                            line,
                            "in for-each iterable",
                        ));
                    }
                }

                Ok(ControlFlow::None)
            }

            Statement::StructDef { name, fields } => {
                self.stats.structs_defined += 1;
                let struct_type = Value::StructType {
                    name: name.clone(),
                    fields: fields.clone(),
                };
                self.env.borrow_mut().define(name.clone(), struct_type);
                Ok(ControlFlow::None)
            }

            Statement::Return(value_opt) => {
                let val = if let Some(expr) = value_opt {
                    self.evaluate(expr, line)?
                } else {
                    Value::Null
                };
                Ok(ControlFlow::Return(val))
            }

            Statement::Break => Ok(ControlFlow::Break),

            Statement::Continue => Ok(ControlFlow::Continue),

            Statement::Push { list, value } => {
                let list_val = self.evaluate(list, line)?;
                let item = self.evaluate(value, line)?;

                match list_val {
                    Value::List(items) => {
                        items.borrow_mut().push(item);
                        Ok(ControlFlow::None)
                    }
                    _ => Err(goose::error(
                        ErrorKind::TypeError {
                            expected: "list".to_string(),
                            got: list_val.type_name().to_string(),
                        },
                        line,
                        "in push statement",
                    )),
                }
            }
        }
    }

    /// Execute multiple statements
    fn execute_statements(&mut self, stmts: &[Statement], line: usize) -> Result<ControlFlow, String> {
        for stmt in stmts {
            match self.execute_statement(stmt, line)? {
                ControlFlow::None => {}
                other => return Ok(other),
            }
        }
        Ok(ControlFlow::None)
    }

    /// Convert statements to blocks (for function body storage)
    fn statements_to_blocks(&self, stmts: &[Statement], line: usize) -> Vec<Block> {
        stmts
            .iter()
            .map(|s| Block {
                statement: s.clone(),
                was_quacked: true,
                line,
            })
            .collect()
    }

    /// Create a closure capturing the current environment
    fn create_closure(&self) -> Closure {
        // For simplicity, capture all variables in current scope
        let vars = self.env.borrow().values.clone();
        Closure::from_map(vars)
    }

    /// Assign a value to an assignment target
    fn assign_to_target(&mut self, target: &AssignTarget, value: Value, line: usize) -> Result<(), String> {
        match target {
            AssignTarget::Variable(name) => {
                if !self.env.borrow_mut().assign(name, value.clone()) {
                    // Variable doesn't exist yet, define it
                    self.env.borrow_mut().define(name.clone(), value);
                }
                Ok(())
            }
            AssignTarget::Field { object, field } => {
                let obj_val = self.evaluate(object, line)?;
                match obj_val {
                    Value::Struct { fields, .. } => {
                        fields.borrow_mut().insert(field.clone(), value);
                        Ok(())
                    }
                    _ => Err(goose::error(
                        ErrorKind::InvalidFieldAccess {
                            type_name: obj_val.type_name().to_string(),
                            field: field.clone(),
                        },
                        line,
                        "",
                    )),
                }
            }
            AssignTarget::Index { object, index } => {
                let obj_val = self.evaluate(object, line)?;
                let idx_val = self.evaluate(index, line)?;

                match (&obj_val, &idx_val) {
                    (Value::List(items), Value::Number(n)) => {
                        let idx = *n as i64;
                        let mut items_mut = items.borrow_mut();
                        let len = items_mut.len();
                        let actual_idx = if idx < 0 {
                            (len as i64 + idx) as usize
                        } else {
                            idx as usize
                        };

                        if actual_idx >= len {
                            return Err(goose::error(
                                ErrorKind::IndexOutOfBounds { index: idx, len },
                                line,
                                "",
                            ));
                        }

                        items_mut[actual_idx] = value;
                        Ok(())
                    }
                    (Value::List(_), _) => Err(goose::error(
                        ErrorKind::TypeError {
                            expected: "number".to_string(),
                            got: idx_val.type_name().to_string(),
                        },
                        line,
                        "in index",
                    )),
                    _ => Err(goose::error(
                        ErrorKind::TypeError {
                            expected: "list".to_string(),
                            got: obj_val.type_name().to_string(),
                        },
                        line,
                        "for indexing",
                    )),
                }
            }
        }
    }

    /// Match a value against a pattern, returning bindings if successful
    fn match_pattern(&self, pattern: &Pattern, value: &Value) -> Option<HashMap<String, Value>> {
        match pattern {
            Pattern::Wildcard => Some(HashMap::new()),

            Pattern::Variable(name) => {
                let mut bindings = HashMap::new();
                bindings.insert(name.clone(), value.clone());
                Some(bindings)
            }

            Pattern::Literal(lit) => {
                let matches = match (lit, value) {
                    (Literal::Int(a), Value::Number(b)) => *a == *b as i64,
                    (Literal::Float(a), Value::Number(b)) => *a == *b,
                    (Literal::String(a), Value::String(b)) => a == b,
                    (Literal::Bool(a), Value::Boolean(b)) => *a == *b,
                    (Literal::Nil, Value::Null) => true,
                    _ => false,
                };
                if matches { Some(HashMap::new()) } else { None }
            }

            Pattern::List(patterns) => {
                if let Value::List(items) = value {
                    let items_borrowed = items.borrow();
                    if patterns.len() != items_borrowed.len() {
                        return None;
                    }
                    let mut all_bindings = HashMap::new();
                    for (pat, val) in patterns.iter().zip(items_borrowed.iter()) {
                        let bindings = self.match_pattern(pat, val)?;
                        all_bindings.extend(bindings);
                    }
                    Some(all_bindings)
                } else {
                    None
                }
            }

            Pattern::Struct { name, fields } => {
                if let Value::Struct { name: struct_name, fields: struct_fields } = value {
                    if name != struct_name {
                        return None;
                    }
                    let fields_borrowed = struct_fields.borrow();
                    let mut all_bindings = HashMap::new();
                    for (field_name, field_pattern) in fields {
                        if let Some(field_value) = fields_borrowed.get(field_name) {
                            let bindings = self.match_pattern(field_pattern, field_value)?;
                            all_bindings.extend(bindings);
                        } else {
                            return None;
                        }
                    }
                    Some(all_bindings)
                } else {
                    None
                }
            }
        }
    }

    /// Evaluate an expression
    fn evaluate(&mut self, expr: &Expr, line: usize) -> Result<Value, String> {
        match expr {
            Expr::Literal(lit) => Ok(self.literal_to_value(lit)),

            Expr::Identifier(name) => {
                // Check for builtin first
                if builtins::is_builtin(name) {
                    return Ok(Value::BuiltinFunction(name.clone()));
                }

                self.env.borrow().get(name).ok_or_else(|| {
                    goose::error(ErrorKind::UnknownVariable(name.clone()), line, "")
                })
            }

            Expr::Binary { left, operator, right } => {
                let lhs = self.evaluate(left, line)?;
                let rhs = self.evaluate(right, line)?;
                self.apply_binary_op(operator, lhs, rhs, line)
            }

            Expr::Unary { operator, operand } => {
                let val = self.evaluate(operand, line)?;
                self.apply_unary_op(operator, val, line)
            }

            Expr::Call { callee, arguments } => {
                let func = self.evaluate(callee, line)?;
                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate(arg, line)?);
                }
                self.call_function(func, args, line)
            }

            Expr::FieldAccess { object, field } => {
                let obj = self.evaluate(object, line)?;
                match obj {
                    Value::Struct { fields, name } => {
                        fields.borrow().get(field).cloned().ok_or_else(|| {
                            goose::error(
                                ErrorKind::InvalidFieldAccess {
                                    type_name: name,
                                    field: field.clone(),
                                },
                                line,
                                "",
                            )
                        })
                    }
                    _ => Err(goose::error(
                        ErrorKind::InvalidFieldAccess {
                            type_name: obj.type_name().to_string(),
                            field: field.clone(),
                        },
                        line,
                        "",
                    )),
                }
            }

            Expr::Index { object, index } => {
                let obj = self.evaluate(object, line)?;
                let idx = self.evaluate(index, line)?;

                match (&obj, &idx) {
                    (Value::List(items), Value::Number(n)) => {
                        let i = *n as i64;
                        let items_borrowed = items.borrow();
                        let len = items_borrowed.len();
                        let actual_idx = if i < 0 {
                            (len as i64 + i) as usize
                        } else {
                            i as usize
                        };

                        items_borrowed.get(actual_idx).cloned().ok_or_else(|| {
                            goose::error(
                                ErrorKind::IndexOutOfBounds { index: i, len },
                                line,
                                "",
                            )
                        })
                    }
                    (Value::String(s), Value::Number(n)) => {
                        let i = *n as i64;
                        let len = s.chars().count();
                        let actual_idx = if i < 0 {
                            (len as i64 + i) as usize
                        } else {
                            i as usize
                        };

                        s.chars().nth(actual_idx)
                            .map(|c| Value::String(c.to_string()))
                            .ok_or_else(|| {
                                goose::error(
                                    ErrorKind::IndexOutOfBounds { index: i, len },
                                    line,
                                    "",
                                )
                            })
                    }
                    (Value::List(_), _) => Err(goose::error(
                        ErrorKind::TypeError {
                            expected: "number".to_string(),
                            got: idx.type_name().to_string(),
                        },
                        line,
                        "in index",
                    )),
                    _ => Err(goose::error(
                        ErrorKind::TypeError {
                            expected: "list or string".to_string(),
                            got: obj.type_name().to_string(),
                        },
                        line,
                        "for indexing",
                    )),
                }
            }

            Expr::List(elements) => {
                let mut items = Vec::new();
                for elem in elements {
                    items.push(self.evaluate(elem, line)?);
                }
                Ok(Value::new_list(items))
            }

            Expr::Lambda { params, body } => {
                let closure = self.create_closure();
                Ok(Value::new_lambda(params.clone(), (**body).clone(), closure))
            }

            Expr::StructInit { name, fields } => {
                // Check if struct type is defined
                let struct_type = self.env.borrow().get(name);
                let expected_fields = match struct_type {
                    Some(Value::StructType { fields: f, .. }) => f,
                    _ => {
                        return Err(goose::error(
                            ErrorKind::UnknownVariable(name.clone()),
                            line,
                            "struct type not defined",
                        ));
                    }
                };

                // Evaluate field values
                let mut field_values = HashMap::new();
                for (field_name, field_expr) in fields {
                    let value = self.evaluate(field_expr, line)?;
                    field_values.insert(field_name.clone(), value);
                }

                // Check that all expected fields are provided
                for expected in &expected_fields {
                    if !field_values.contains_key(expected) {
                        return Err(format!(
                            "Missing field '{}' in struct '{}' at line {}",
                            expected, name, line
                        ));
                    }
                }

                Ok(Value::new_struct(name.clone(), field_values))
            }

            Expr::Ternary { condition, then_expr, else_expr } => {
                let cond = self.evaluate(condition, line)?;
                if cond.is_truthy() {
                    self.evaluate(then_expr, line)
                } else {
                    self.evaluate(else_expr, line)
                }
            }

            Expr::Range { start, end, inclusive } => {
                let start_val = self.evaluate(start, line)?;
                let end_val = self.evaluate(end, line)?;

                match (&start_val, &end_val) {
                    (Value::Number(s), Value::Number(e)) => {
                        let mut items = Vec::new();
                        let s_int = *s as i64;
                        let e_int = *e as i64;
                        let final_end = if *inclusive { e_int + 1 } else { e_int };
                        for i in s_int..final_end {
                            items.push(Value::Number(i as f64));
                        }
                        Ok(Value::new_list(items))
                    }
                    _ => Err(goose::error(
                        ErrorKind::TypeError {
                            expected: "numbers".to_string(),
                            got: format!("{} and {}", start_val.type_name(), end_val.type_name()),
                        },
                        line,
                        "in range",
                    )),
                }
            }

            Expr::StringInterpolation(parts) => {
                let mut result = String::new();
                for part in parts {
                    match part {
                        StringPart::Literal(s) => result.push_str(s),
                        StringPart::Expr(e) => {
                            let val = self.evaluate(e, line)?;
                            result.push_str(&format!("{}", val));
                        }
                    }
                }
                Ok(Value::String(result))
            }

            Expr::Match { value, arms } => {
                let val = self.evaluate(value, line)?;
                for arm in arms {
                    if let Some(bindings) = self.match_pattern(&arm.pattern, &val) {
                        // Create scope with bindings
                        let child_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&self.env))));
                        for (name, binding_val) in bindings {
                            child_env.borrow_mut().define(name, binding_val);
                        }
                        let old_env = std::mem::replace(&mut self.env, child_env);

                        let result = if let Some(ref expr) = arm.expression {
                            self.evaluate(expr, line)
                        } else {
                            Ok(Value::Null)
                        };

                        self.env = old_env;
                        return result;
                    }
                }
                Ok(Value::Null)
            }
        }
    }

    /// Convert a literal to a value
    fn literal_to_value(&self, lit: &Literal) -> Value {
        match lit {
            Literal::Int(n) => Value::Number(*n as f64),
            Literal::Float(n) => Value::Number(*n),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Bool(b) => Value::Boolean(*b),
            Literal::Nil => Value::Null,
        }
    }

    /// Apply a binary operator
    fn apply_binary_op(&self, op: &BinaryOp, lhs: Value, rhs: Value, line: usize) -> Result<Value, String> {
        match op {
            BinaryOp::Add => match (&lhs, &rhs) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("{} + {}", lhs.type_name(), rhs.type_name())),
                    line,
                    "",
                )),
            },

            BinaryOp::Sub => match (&lhs, &rhs) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("{} - {}", lhs.type_name(), rhs.type_name())),
                    line,
                    "",
                )),
            },

            BinaryOp::Mul => match (&lhs, &rhs) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                (Value::String(s), Value::Number(n)) | (Value::Number(n), Value::String(s)) => {
                    Ok(Value::String(s.repeat(*n as usize)))
                }
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("{} * {}", lhs.type_name(), rhs.type_name())),
                    line,
                    "",
                )),
            },

            BinaryOp::Div => match (&lhs, &rhs) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(goose::error(ErrorKind::DivisionByZero, line, ""))
                    } else {
                        Ok(Value::Number(a / b))
                    }
                }
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("{} / {}", lhs.type_name(), rhs.type_name())),
                    line,
                    "",
                )),
            },

            BinaryOp::Mod => match (&lhs, &rhs) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(goose::error(ErrorKind::DivisionByZero, line, ""))
                    } else {
                        Ok(Value::Number(a % b))
                    }
                }
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("{} % {}", lhs.type_name(), rhs.type_name())),
                    line,
                    "",
                )),
            },

            BinaryOp::Pow => match (&lhs, &rhs) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(*b))),
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("{} ** {}", lhs.type_name(), rhs.type_name())),
                    line,
                    "",
                )),
            },

            BinaryOp::Eq => Ok(Value::Boolean(self.values_equal(&lhs, &rhs))),
            BinaryOp::NotEq => Ok(Value::Boolean(!self.values_equal(&lhs, &rhs))),

            BinaryOp::Lt => match (&lhs, &rhs) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a < b)),
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("{} < {}", lhs.type_name(), rhs.type_name())),
                    line,
                    "",
                )),
            },

            BinaryOp::LtEq => match (&lhs, &rhs) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a <= b)),
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("{} <= {}", lhs.type_name(), rhs.type_name())),
                    line,
                    "",
                )),
            },

            BinaryOp::Gt => match (&lhs, &rhs) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a > b)),
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("{} > {}", lhs.type_name(), rhs.type_name())),
                    line,
                    "",
                )),
            },

            BinaryOp::GtEq => match (&lhs, &rhs) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
                (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a >= b)),
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("{} >= {}", lhs.type_name(), rhs.type_name())),
                    line,
                    "",
                )),
            },

            BinaryOp::And => Ok(Value::Boolean(lhs.is_truthy() && rhs.is_truthy())),
            BinaryOp::Or => Ok(Value::Boolean(lhs.is_truthy() || rhs.is_truthy())),

            BinaryOp::Concat => match (&lhs, &rhs) {
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::List(a), Value::List(b)) => {
                    let mut new_list = a.borrow().clone();
                    new_list.extend(b.borrow().iter().cloned());
                    Ok(Value::new_list(new_list))
                }
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("{} ++ {}", lhs.type_name(), rhs.type_name())),
                    line,
                    "",
                )),
            },
        }
    }

    /// Check if two values are equal
    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        a == b
    }

    /// Apply a unary operator
    fn apply_unary_op(&self, op: &UnaryOp, val: Value, line: usize) -> Result<Value, String> {
        match op {
            UnaryOp::Neg => match val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(goose::error(
                    ErrorKind::InvalidOperation(format!("-{}", val.type_name())),
                    line,
                    "",
                )),
            },
            UnaryOp::Not => Ok(Value::Boolean(!val.is_truthy())),
        }
    }

    /// Call a function or builtin
    fn call_function(&mut self, func: Value, args: Vec<Value>, line: usize) -> Result<Value, String> {
        match func {
            Value::BuiltinFunction(name) => {
                builtins::call_builtin(&name, args)
                    .map_err(|e| goose::error(ErrorKind::InvalidOperation(e), line, ""))
            }

            Value::Function { name, params, body, closure } => {
                if args.len() != params.len() {
                    return Err(goose::error(
                        ErrorKind::ArgumentMismatch {
                            expected: params.len(),
                            got: args.len(),
                        },
                        line,
                        &format!("in call to '{}'", name),
                    ));
                }

                // Create new environment for function call
                let func_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&self.env))));

                // Bind parameters
                for (param, arg) in params.iter().zip(args) {
                    func_env.borrow_mut().define(param.clone(), arg);
                }

                // Bind closure variables
                for (name, value) in closure.captured.borrow().iter() {
                    if func_env.borrow().get(name).is_none() {
                        func_env.borrow_mut().define(name.clone(), value.clone());
                    }
                }

                let old_env = std::mem::replace(&mut self.env, func_env);

                // Execute function body
                let mut result = Value::Null;
                for block in &body {
                    match self.execute_block(block)? {
                        ControlFlow::Return(v) => {
                            result = v;
                            break;
                        }
                        ControlFlow::Break | ControlFlow::Continue => {
                            return Err(format!(
                                "Unexpected break/continue outside loop at line {}",
                                line
                            ));
                        }
                        ControlFlow::None => {}
                    }
                }

                self.env = old_env;
                Ok(result)
            }

            Value::Lambda { params, body, closure } => {
                if args.len() != params.len() {
                    return Err(goose::error(
                        ErrorKind::ArgumentMismatch {
                            expected: params.len(),
                            got: args.len(),
                        },
                        line,
                        "in lambda call",
                    ));
                }

                // Create environment for lambda
                let lambda_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&self.env))));

                // Bind parameters
                for (param, arg) in params.iter().zip(args) {
                    lambda_env.borrow_mut().define(param.clone(), arg);
                }

                // Bind closure variables
                for (name, value) in closure.captured.borrow().iter() {
                    if lambda_env.borrow().get(name).is_none() {
                        lambda_env.borrow_mut().define(name.clone(), value.clone());
                    }
                }

                let old_env = std::mem::replace(&mut self.env, lambda_env);

                // Evaluate lambda body
                let result = self.evaluate(&body, line)?;

                self.env = old_env;
                Ok(result)
            }

            Value::StructType { name, fields } => {
                // Struct instantiation via function call syntax
                if args.len() != fields.len() {
                    return Err(goose::error(
                        ErrorKind::ArgumentMismatch {
                            expected: fields.len(),
                            got: args.len(),
                        },
                        line,
                        &format!("in struct '{}' constructor", name),
                    ));
                }

                let mut field_values = HashMap::new();
                for (field_name, arg) in fields.iter().zip(args) {
                    field_values.insert(field_name.clone(), arg);
                }

                Ok(Value::new_struct(name, field_values))
            }

            _ => Err(goose::error(
                ErrorKind::InvalidOperation(format!("cannot call {}", func.type_name())),
                line,
                "",
            )),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::Parser;

    fn run_source(source: &str) -> Result<(), String> {
        let tokens = lex(source).map_err(|e| e)?;
        let mut parser = Parser::new(tokens);
        let blocks = parser.parse().map_err(|e| e.join("\n"))?;
        let mut interpreter = Interpreter::new();
        interpreter.run(blocks)
    }

    #[test]
    fn test_let_statement() {
        let result = run_source("quack [let x be 42]");
        assert!(result.is_ok());
    }

    #[test]
    fn test_arithmetic() {
        let result = run_source("quack [let x be 10 + 5 * 2]");
        assert!(result.is_ok());
    }

    #[test]
    fn test_unquacked_block_skipped() {
        // This should run without error but skip the unquacked block
        let result = run_source("[let x be 42]");
        assert!(result.is_ok());
    }

    #[test]
    fn test_multi_quack() {
        let result = run_source("quack quack [let x be 1] [let y be 2]");
        assert!(result.is_ok());
    }
}
