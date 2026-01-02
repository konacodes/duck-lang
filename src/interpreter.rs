// Interpreter - executes Duck programs
// Only executes blocks that were properly "quacked"

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;

use crate::ast::{AssignTarget, BinaryOp, Block, Expr, Literal, Pattern, Statement, StringPart, UnaryOp};
use crate::lexer;
use crate::parser;
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

/// Default instruction limit (10 million instructions)
const DEFAULT_INSTRUCTION_LIMIT: usize = 10_000_000;

/// The interpreter
pub struct Interpreter {
    /// Global environment
    env: Rc<RefCell<Environment>>,
    /// Execution statistics
    pub stats: ExecutionStats,
    /// Instruction counter for infinite loop protection
    instruction_count: usize,
    /// Maximum instructions allowed (None = unlimited)
    max_instructions: Option<usize>,
    /// Files already imported (to prevent circular imports)
    imported_files: HashSet<PathBuf>,
}

impl Interpreter {
    /// Create a new interpreter with math constants pre-defined
    pub fn new() -> Self {
        Self::with_args(vec![])
    }

    /// Create a new interpreter with command-line arguments
    pub fn with_args(args: Vec<String>) -> Self {
        let env = Rc::new(RefCell::new(Environment::new()));

        // Pre-define math constants
        env.borrow_mut().define("PI".to_string(), Value::Number(std::f64::consts::PI));
        env.borrow_mut().define("E".to_string(), Value::Number(std::f64::consts::E));
        env.borrow_mut().define("TAU".to_string(), Value::Number(std::f64::consts::TAU));

        // Pre-define command-line arguments as quack-args
        let args_values: Vec<Value> = args.into_iter().map(Value::String).collect();
        env.borrow_mut().define("quack-args".to_string(), Value::new_list(args_values));

        Interpreter {
            env,
            stats: ExecutionStats::default(),
            instruction_count: 0,
            max_instructions: Some(DEFAULT_INSTRUCTION_LIMIT),
            imported_files: HashSet::new(),
        }
    }

    /// Set the maximum instruction limit (None for unlimited)
    pub fn set_instruction_limit(&mut self, limit: Option<usize>) {
        self.max_instructions = limit;
    }

    /// Check and increment instruction counter
    fn check_instruction_limit(&mut self) -> Result<(), String> {
        self.instruction_count += 1;
        if let Some(max) = self.max_instructions {
            if self.instruction_count > max {
                return Err(format!(
                    "Execution limit exceeded ({} instructions) - the goose suspects an infinite loop",
                    max
                ));
            }
        }
        Ok(())
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
        // Check instruction limit for infinite loop protection
        self.check_instruction_limit()?;

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

            Statement::Honk { condition, message } => {
                let cond_val = self.evaluate(condition, line)?;
                if !cond_val.is_truthy() {
                    let msg = if let Some(msg_expr) = message {
                        let msg_val = self.evaluate(msg_expr, line)?;
                        format!("{}", msg_val)
                    } else {
                        String::new()
                    };
                    return Err(goose::honk_failure(line, &msg));
                }
                Ok(ControlFlow::None)
            }

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

            Statement::Attempt { try_block, rescue_var, rescue_block } => {
                // Try to execute the try_block
                let result = self.execute_statements(try_block, line);

                match result {
                    Ok(flow) => Ok(flow),
                    Err(error_msg) => {
                        // Error occurred, execute rescue block with error bound to rescue_var
                        let child_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&self.env))));
                        child_env.borrow_mut().define(rescue_var.clone(), Value::String(error_msg));
                        let old_env = std::mem::replace(&mut self.env, child_env);

                        let rescue_result = self.execute_statements(rescue_block, line);

                        self.env = old_env;
                        rescue_result
                    }
                }
            }

            Statement::Migrate { path, alias } => {
                self.execute_migrate(path, alias.as_ref(), line)?;
                Ok(ControlFlow::None)
            }
        }
    }

    /// Execute a migrate statement - import code from another Duck file
    fn execute_migrate(&mut self, path: &str, alias: Option<&String>, _line: usize) -> Result<(), String> {
        // Check if this is a git library reference (git+user/repo)
        let file_path = if path.starts_with("git+") {
            self.resolve_git_library(path)?
        } else {
            PathBuf::from(path)
        };

        // Get canonical path to handle duplicates properly
        let canonical_path = file_path.canonicalize().map_err(|e| {
            format!("The flock couldn't find '{}': {} - maybe they flew south?", path, e)
        })?;

        // Check for circular imports
        if self.imported_files.contains(&canonical_path) {
            // Already imported, skip silently (this is fine for circular deps)
            return Ok(());
        }

        // Mark as imported
        self.imported_files.insert(canonical_path.clone());

        // Read the file
        let source = std::fs::read_to_string(&canonical_path).map_err(|e| {
            format!("The goose couldn't read '{}': {}", path, e)
        })?;

        // Lex and parse
        let tokens = lexer::lex(&source).map_err(|e| {
            format!("Syntax error in '{}': {}", path, e)
        })?;
        let mut parser = parser::Parser::new(tokens);
        let blocks = parser.parse().map_err(|errors| {
            format!("Parse error in '{}': {}", path, errors.join(", "))
        })?;

        // Execute the blocks and collect definitions
        if let Some(namespace) = alias {
            // With alias: execute in a child environment, then create a struct-like namespace
            let child_env = Rc::new(RefCell::new(Environment::with_parent(Rc::clone(&self.env))));
            let old_env = std::mem::replace(&mut self.env, child_env);

            // Execute all blocks
            for block in &blocks {
                if block.was_quacked {
                    if let Err(e) = self.execute_block(block) {
                        self.env = old_env;
                        return Err(e);
                    }
                }
            }

            // Collect all definitions from the child environment
            let child_values = self.env.borrow().values.clone();
            self.env = old_env;

            // Create a namespace struct with all the definitions
            let namespace_struct = Value::new_struct(namespace.clone(), child_values);
            self.env.borrow_mut().define(namespace.clone(), namespace_struct);

            println!("The flock has arrived from '{}' as {}!", path, namespace);
        } else {
            // Without alias: execute directly in current scope (definitions become globals)
            for block in &blocks {
                if block.was_quacked {
                    self.execute_block(block)?;
                }
            }
            println!("The flock has arrived from '{}'!", path);
        }

        Ok(())
    }

    /// Resolve a git+ library path to an actual file path
    /// Format: git+user/repo[@version]
    fn resolve_git_library(&self, path: &str) -> Result<PathBuf, String> {
        // Strip the "git+" prefix
        let lib_ref = path.strip_prefix("git+").unwrap_or(path);

        // Parse optional version: user/repo@version or user/repo
        let (lib_path, version) = if let Some(at_pos) = lib_ref.find('@') {
            (&lib_ref[..at_pos], &lib_ref[at_pos + 1..])
        } else {
            (lib_ref, "main")
        };

        // Parse user/repo
        let parts: Vec<&str> = lib_path.split('/').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid library reference '{}'. Expected format: git+user/repo[@version]",
                path
            ));
        }

        let user = parts[0];
        let repo = parts[1];

        // Find the library in ~/.duck/libs/user/repo/version/
        let home = dirs::home_dir().ok_or("Cannot determine home directory")?;
        let libs_dir = home.join(".duck").join("libs");
        let lib_dir = libs_dir.join(user).join(repo).join(version);

        if !lib_dir.exists() {
            return Err(format!(
                "Library '{}/{}@{}' not installed.\n\
                 Run: goose install {}/{} {}",
                user, repo, version, user, repo, version
            ));
        }

        // Look for metadata.dm to find the entry point
        let metadata_path = lib_dir.join("metadata.dm");
        let entry_file = if metadata_path.exists() {
            // Parse metadata.dm to find [point to] section
            let metadata = std::fs::read_to_string(&metadata_path)
                .map_err(|e| format!("Failed to read metadata.dm: {}", e))?;

            let mut entry_point = None;
            let mut in_point_to = false;

            for line in metadata.lines() {
                let line = line.trim();
                if line == "[point to]" {
                    in_point_to = true;
                    continue;
                }
                if line.starts_with('[') && line.ends_with(']') {
                    in_point_to = false;
                    continue;
                }
                if in_point_to && !line.is_empty() && !line.starts_with("//") {
                    // This is the entry point path
                    let entry = line.trim_start_matches("./");
                    entry_point = Some(entry.to_string());
                    break;
                }
            }

            entry_point.unwrap_or_else(|| "lib.duck".to_string())
        } else {
            // Default to lib.duck
            "lib.duck".to_string()
        };

        let full_path = lib_dir.join(&entry_file);
        if !full_path.exists() {
            return Err(format!(
                "Library entry file '{}' not found in {}/{}@{}",
                entry_file, user, repo, version
            ));
        }

        Ok(full_path)
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
                // Handle higher-order functions that need interpreter access
                match name.as_str() {
                    "map" => self.builtin_map(args, line),
                    "filter" => self.builtin_filter(args, line),
                    "fold" => self.builtin_fold(args, line),
                    "find" => self.builtin_find(args, line),
                    "any" => self.builtin_any(args, line),
                    "all" => self.builtin_all(args, line),
                    _ => builtins::call_builtin(&name, args)
                        .map_err(|e| goose::error(ErrorKind::InvalidOperation(e), line, ""))
                }
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

    /// Helper to call a function/lambda with given arguments
    fn call_callable(&mut self, callable: Value, args: Vec<Value>, line: usize) -> Result<Value, String> {
        self.call_function(callable, args, line)
    }

    /// Built-in map: apply function to each element
    fn builtin_map(&mut self, args: Vec<Value>, line: usize) -> Result<Value, String> {
        if args.len() != 2 {
            return Err(goose::error(
                ErrorKind::ArgumentMismatch { expected: 2, got: args.len() },
                line,
                "map(list, function)",
            ));
        }

        let list = match &args[0] {
            Value::List(items) => items.borrow().clone(),
            other => return Err(goose::error(
                ErrorKind::TypeError { expected: "list".to_string(), got: other.type_name().to_string() },
                line,
                "in map() first argument",
            )),
        };

        let func = args[1].clone();
        let mut results = Vec::new();

        for item in list {
            let result = self.call_callable(func.clone(), vec![item], line)?;
            results.push(result);
        }

        Ok(Value::new_list(results))
    }

    /// Built-in filter: keep elements that satisfy predicate
    fn builtin_filter(&mut self, args: Vec<Value>, line: usize) -> Result<Value, String> {
        if args.len() != 2 {
            return Err(goose::error(
                ErrorKind::ArgumentMismatch { expected: 2, got: args.len() },
                line,
                "filter(list, predicate)",
            ));
        }

        let list = match &args[0] {
            Value::List(items) => items.borrow().clone(),
            other => return Err(goose::error(
                ErrorKind::TypeError { expected: "list".to_string(), got: other.type_name().to_string() },
                line,
                "in filter() first argument",
            )),
        };

        let func = args[1].clone();
        let mut results = Vec::new();

        for item in list {
            let result = self.call_callable(func.clone(), vec![item.clone()], line)?;
            if result.is_truthy() {
                results.push(item);
            }
        }

        Ok(Value::new_list(results))
    }

    /// Built-in fold: reduce list to single value
    fn builtin_fold(&mut self, args: Vec<Value>, line: usize) -> Result<Value, String> {
        if args.len() != 3 {
            return Err(goose::error(
                ErrorKind::ArgumentMismatch { expected: 3, got: args.len() },
                line,
                "fold(list, initial, function)",
            ));
        }

        let list = match &args[0] {
            Value::List(items) => items.borrow().clone(),
            other => return Err(goose::error(
                ErrorKind::TypeError { expected: "list".to_string(), got: other.type_name().to_string() },
                line,
                "in fold() first argument",
            )),
        };

        let mut accumulator = args[1].clone();
        let func = args[2].clone();

        for item in list {
            accumulator = self.call_callable(func.clone(), vec![accumulator, item], line)?;
        }

        Ok(accumulator)
    }

    /// Built-in find: find first element matching predicate
    fn builtin_find(&mut self, args: Vec<Value>, line: usize) -> Result<Value, String> {
        if args.len() != 2 {
            return Err(goose::error(
                ErrorKind::ArgumentMismatch { expected: 2, got: args.len() },
                line,
                "find(list, predicate)",
            ));
        }

        let list = match &args[0] {
            Value::List(items) => items.borrow().clone(),
            other => return Err(goose::error(
                ErrorKind::TypeError { expected: "list".to_string(), got: other.type_name().to_string() },
                line,
                "in find() first argument",
            )),
        };

        let func = args[1].clone();

        for item in list {
            let result = self.call_callable(func.clone(), vec![item.clone()], line)?;
            if result.is_truthy() {
                return Ok(item);
            }
        }

        Ok(Value::Null)
    }

    /// Built-in any: check if any element satisfies predicate
    fn builtin_any(&mut self, args: Vec<Value>, line: usize) -> Result<Value, String> {
        if args.len() != 2 {
            return Err(goose::error(
                ErrorKind::ArgumentMismatch { expected: 2, got: args.len() },
                line,
                "any(list, predicate)",
            ));
        }

        let list = match &args[0] {
            Value::List(items) => items.borrow().clone(),
            other => return Err(goose::error(
                ErrorKind::TypeError { expected: "list".to_string(), got: other.type_name().to_string() },
                line,
                "in any() first argument",
            )),
        };

        let func = args[1].clone();

        for item in list {
            let result = self.call_callable(func.clone(), vec![item], line)?;
            if result.is_truthy() {
                return Ok(Value::Boolean(true));
            }
        }

        Ok(Value::Boolean(false))
    }

    /// Built-in all: check if all elements satisfy predicate
    fn builtin_all(&mut self, args: Vec<Value>, line: usize) -> Result<Value, String> {
        if args.len() != 2 {
            return Err(goose::error(
                ErrorKind::ArgumentMismatch { expected: 2, got: args.len() },
                line,
                "all(list, predicate)",
            ));
        }

        let list = match &args[0] {
            Value::List(items) => items.borrow().clone(),
            other => return Err(goose::error(
                ErrorKind::TypeError { expected: "list".to_string(), got: other.type_name().to_string() },
                line,
                "in all() first argument",
            )),
        };

        let func = args[1].clone();

        for item in list {
            let result = self.call_callable(func.clone(), vec![item], line)?;
            if !result.is_truthy() {
                return Ok(Value::Boolean(false));
            }
        }

        Ok(Value::Boolean(true))
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
