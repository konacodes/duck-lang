# CLAUDE.md - Duck Language Guide for AI Assistants

This document provides essential context for AI assistants working on the Duck programming language codebase.

## Project Overview

Duck is a humorous programming language where every code block must be preceded by `quack` to be executed. The interpreter is named "Goose" and provides snarky, duck-themed commentary on your code.

**Key Concept**: Code blocks wrapped in `[...]` only execute if preceded by `quack`. The goose refuses to run unquacked blocks.

```duck
quack [print "This will run"]      -- Works!
[print "This will be ignored"]     -- Goose refuses
quack quack [print "A"] [print "B"] -- Multiple quacks authorize multiple blocks
```

## Codebase Structure

```
duck-lang/
├── Cargo.toml          # Project manifest - binary is named "goose"
├── src/
│   ├── main.rs         # CLI entry point (run, check, repl commands)
│   ├── lexer.rs        # Tokenization with string interpolation support
│   ├── parser.rs       # Recursive descent parser with quack tracking
│   ├── ast.rs          # AST node definitions (expressions, statements, blocks)
│   ├── interpreter.rs  # Tree-walking interpreter with lexical scoping
│   ├── values.rs       # Runtime value types (Number, String, List, Struct, etc.)
│   ├── builtins.rs     # Built-in functions (print, input, len, range, etc.)
│   └── goose.rs        # Snarky error messages and code rating system
└── examples/           # Example Duck programs
    ├── hello.duck      # Basic hello world with variables
    ├── fizzbuzz.duck   # Classic FizzBuzz implementation
    ├── countdown.duck  # Countdown with conditional messages
    └── pond.duck       # Comprehensive demo: structs, functions, lambdas
```

## Architecture

### Compilation Pipeline

```
Source (.duck) → Lexer → Tokens → Parser → AST (Blocks) → Interpreter → Output
```

1. **Lexer** (`lexer.rs`): Tokenizes source into `Token` structs with kind, lexeme, line, column
2. **Parser** (`parser.rs`): Builds AST with `Block` nodes that track `was_quacked` status
3. **Interpreter** (`interpreter.rs`): Executes blocks only if `was_quacked == true`

### Key Data Structures

- **Block**: Wraps a `Statement` with `was_quacked: bool` and `line: usize`
- **Value**: Runtime values - `Number(f64)`, `String`, `Boolean`, `List`, `Struct`, `Function`, `Lambda`, `Null`
- **Environment**: Lexical scope chain using `HashMap<String, Value>` with optional parent reference

### The Quack System

The parser tracks pending quacks:
- Each `quack` token increments `quack_count`
- Each `[...]` block consumes one quack (sets `was_quacked = true`)
- Unquacked blocks are parsed but skipped at runtime with snarky messages

## Duck Language Syntax

### Variables and Assignment
```duck
quack [let x be 42]           -- Declaration with 'be'
quack [x becomes x + 1]       -- Assignment with 'becomes'
```

### Strings and Interpolation
```duck
quack [let name be "Duck"]
quack [print "Hello, {name}!"]  -- Interpolation with {expr}
```

### Control Flow
```duck
quack [if condition then
  quack [print "true branch"]
otherwise
  quack [print "false branch"]
]

quack [while x > 0 do
  quack [print x]
  quack [x becomes x - 1]
]

quack [repeat 5 times
  quack [print "quack!"]
]

quack [for each [item] in list do
  quack [print item]
]
```

### Functions
```duck
quack [define greet taking [name] as
  quack [print "Hello, {name}!"]
]
quack [greet "World"]
```

### Lambdas
```duck
quack [let double be [x] -> x * 2]
quack [let result be [double 21]]
```

### Structs
```duck
quack [struct duck with [name, age, quackiness]]
quack [let d be duck("Waddles", 3, 100)]
quack [print d.name]
```

### Lists
```duck
quack [let items be list(1, 2, 3)]
quack [items push 4]
quack [print items at 0]       -- Indexing with 'at'
quack [print items length]     -- Length access
```

### Comments
```duck
-- This is a comment (double dash)
```

## Development Workflows

### Build Commands
```bash
cargo build              # Debug build
cargo build --release    # Release build (binary at target/release/goose)
cargo test               # Run all tests
cargo test lexer         # Run tests matching "lexer"
cargo clippy             # Linting
cargo fmt                # Format code
```

### Running Duck Programs
```bash
./target/debug/goose run examples/hello.duck   # Run a file
./target/debug/goose check examples/hello.duck # Check for missing quacks
./target/debug/goose repl                      # Interactive mode
```

### Test Structure
Each module has inline tests with `#[cfg(test)]`:
- `lexer.rs`: Token generation tests
- `parser.rs`: AST construction tests
- `interpreter.rs`: Execution tests
- `builtins.rs`: Built-in function tests
- `values.rs`: Value equality and display tests
- `goose.rs`: Message generation tests

## Key Conventions

### Code Style
- Rust 2021 edition
- Standard Rust formatting (`cargo fmt`)
- Prefer `Result<T, String>` for errors with descriptive messages
- Use `#[derive(Debug, Clone, PartialEq)]` on AST nodes

### Error Messages
All errors go through `goose.rs` for consistent theming:
```rust
goose::error(ErrorKind::TypeError { expected, got }, line, context)
goose::refusal(line, block_preview)  // For unquacked blocks
```

### Adding New Features

**New Built-in Function**:
1. Add name to `is_builtin()` match in `builtins.rs`
2. Add handler to `call_builtin()` match
3. Implement `builtin_yourfunc(args: Vec<Value>) -> Result<Value, String>`
4. Add tests

**New Statement Type**:
1. Add variant to `Statement` enum in `ast.rs`
2. Add parsing in `parse_statement()` in `parser.rs`
3. Add execution in `execute_statement()` in `interpreter.rs`
4. Add tests

**New Expression Type**:
1. Add variant to `Expr` enum in `ast.rs`
2. Add parsing at appropriate precedence level in `parser.rs`
3. Add evaluation in `evaluate()` in `interpreter.rs`
4. Add tests

### Token Types
Key tokens defined in `lexer.rs`:
- `Quack` - the essential `quack` keyword
- `LeftBracket`/`RightBracket` - `[` and `]` for blocks
- `Be`/`Becomes` - variable declaration and assignment
- `Define`/`Taking`/`As` - function definition
- `If`/`Then`/`Otherwise` - conditionals
- `While`/`Do`, `Repeat`/`Times`, `For`/`Each`/`In` - loops

### Built-in Functions
Available builtins: `print`, `input`, `random`, `floor`, `ceil`, `abs`, `type-of`, `len`, `push`, `pop`, `string`, `number`, `sqrt`, `pow`, `min`, `max`, `range`

## Code Rating System

The goose rates code 1-10 based on:
- Quack ratio (quacked blocks / total blocks) - up to 7 points
- Functions defined (bonus)
- Structs defined (bonus)
- Loops executed (bonus)
- Penalty for unquacked blocks

## Common Gotchas

1. **Every block needs a quack** - even inside functions and loops
2. **Use `be` for declaration, `becomes` for assignment** - not `=`
3. **String interpolation uses `{expr}`** - escape with `\{`
4. **Comments are `--`** - not `//` or `#`
5. **Function calls can use `[func arg1 arg2]`** - not just `func(arg1, arg2)`
6. **Lists use `at` for indexing** - `list at 0` not `list[0]`
7. **Identifiers can contain hyphens** - `my-variable` is valid

## Testing Quick Reference

```bash
# Run specific test
cargo test test_quack_keyword

# Run tests with output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored
```

## Performance Notes

- Interpreter is tree-walking (not bytecode compiled)
- Values use `Rc<RefCell<...>>` for mutable reference types (lists, structs)
- Closures capture variables by value at definition time
