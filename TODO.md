# Duck Language Feature Roadmap

This document outlines planned features for the Duck programming language, organized by category and priority.

---

## Table of Contents

1. [Quick Wins](#quick-wins)
2. [Fun/Thematic Features](#funthematic-features)
3. [Practical Features](#practical-features)
4. [Advanced Features](#advanced-features)

---

## Quick Wins

These are simple additions that can be implemented quickly with minimal changes.

### New Built-in Functions

#### `reverse`
Reverse a list or string.

**Signature:** `reverse(value) -> value`

**Examples:**
```duck
quack [let backwards be [reverse list(1, 2, 3)]]  -- [3, 2, 1]
quack [let flipped be [reverse "hello"]]          -- "olleh"
```

**Implementation:**
- Add to `is_builtin()` match in `builtins.rs`
- Handle both `Value::List` and `Value::String`
- For lists: clone and reverse
- For strings: collect chars, reverse, collect back

---

#### `sort`
Sort a list of numbers or strings.

**Signature:** `sort(list) -> list`

**Examples:**
```duck
quack [let ordered be [sort list(3, 1, 2)]]       -- [1, 2, 3]
quack [let alpha be [sort list("c", "a", "b")]]   -- ["a", "b", "c"]
```

**Implementation:**
- Add to `builtins.rs`
- Clone the list, sort in place, return new list
- Error if list contains mixed types or non-comparable values

---

#### `join`
Join a list of strings with a separator.

**Signature:** `join(list, separator) -> string`

**Examples:**
```duck
quack [let csv be [join list("a", "b", "c") ","]]  -- "a,b,c"
quack [let words be [join list("hello", "world") " "]]  -- "hello world"
```

**Implementation:**
- Add to `builtins.rs`
- Convert each element to string, join with separator

---

#### `split`
Split a string by a separator.

**Signature:** `split(string, separator) -> list`

**Examples:**
```duck
quack [let parts be [split "a,b,c" ","]]          -- ["a", "b", "c"]
quack [let words be [split "hello world" " "]]   -- ["hello", "world"]
```

**Implementation:**
- Add to `builtins.rs`
- Use Rust's `str::split()`, collect into `Value::List`

---

#### `uppercase` / `lowercase`
Convert string case.

**Signature:** `uppercase(string) -> string`, `lowercase(string) -> string`

**Examples:**
```duck
quack [let loud be [uppercase "hello"]]    -- "HELLO"
quack [let quiet be [lowercase "HELLO"]]   -- "hello"
```

**Implementation:**
- Add to `builtins.rs`
- Use Rust's `str::to_uppercase()` / `str::to_lowercase()`

---

#### `contains`
Check if a list contains a value or a string contains a substring.

**Signature:** `contains(collection, value) -> boolean`

**Examples:**
```duck
quack [let has-it be [contains list(1, 2, 3) 2]]   -- true
quack [let found be [contains "hello" "ell"]]       -- true
```

**Implementation:**
- Add to `builtins.rs`
- For lists: iterate and check equality
- For strings: use Rust's `str::contains()`

---

#### `trim`
Remove leading and trailing whitespace from a string.

**Signature:** `trim(string) -> string`

**Examples:**
```duck
quack [let clean be [trim "  hello  "]]  -- "hello"
```

**Implementation:**
- Add to `builtins.rs`
- Use Rust's `str::trim()`

---

#### `keys` / `values`
Get keys or values from a struct.

**Signature:** `keys(struct) -> list`, `values(struct) -> list`

**Examples:**
```duck
quack [struct person with [name, age]]
quack [let p be person("Alice", 30)]
quack [print [keys p]]    -- ["name", "age"]
quack [print [values p]]  -- ["Alice", 30]
```

**Implementation:**
- Add to `builtins.rs`
- Extract from struct's field HashMap

---

### Math Constants

Add predefined constants to the global environment.

| Constant | Value | Description |
|----------|-------|-------------|
| `PI` | 3.141592653589793 | Pi |
| `E` | 2.718281828459045 | Euler's number |
| `TAU` | 6.283185307179586 | 2 * Pi |

**Implementation:**
- In `Interpreter::new()`, pre-define these in the global environment
- Or add them as special identifiers in `evaluate()` for `Expr::Identifier`

---

### `sleep`
Pause execution for a specified duration.

**Signature:** `sleep(milliseconds)`

**Examples:**
```duck
quack [print "Starting..."]
quack [sleep 1000]
quack [print "One second later!"]
```

**Implementation:**
- Add to `builtins.rs`
- Use `std::thread::sleep(Duration::from_millis(n))`
- Goose could print a sleepy message: "*yawns* Taking a quick nap..."

---

## Fun/Thematic Features

These features embrace the duck/goose theme for a more entertaining experience.

### `honk` - Assertions

A thematic assertion statement. If the condition is false, the goose honks angrily and crashes.

**Syntax:**
```duck
quack [honk condition]
quack [honk condition "optional message"]
```

**Examples:**
```duck
quack [let x be 5]
quack [honk x > 0]                        -- Passes silently
quack [honk x < 0]                        -- HONK! Assertion failed!
quack [honk x < 0 "x must be negative"]   -- HONK! x must be negative
```

**Error Messages (random selection):**
- "HONK! Assertion failed at line {line}. The goose is NOT happy."
- "HONK HONK HONK! Your assumption was wrong at line {line}!"
- "*aggressive honking* Line {line}: That condition is FALSE!"
- "The goose has inspected your assertion at line {line}. It is LIES."

**Implementation:**
1. Add `Honk` token to `lexer.rs`
2. Add `Statement::Honk { condition: Expr, message: Option<String> }` to `ast.rs`
3. Parse in `parser.rs` after `quack [honk ...]`
4. Execute in `interpreter.rs`: evaluate condition, if falsy, return error via `goose::honk_failure()`
5. Add honk messages to `goose.rs`

---

### `waddle` - Slow/Dramatic Iteration

A loop that executes with deliberate pauses, for dramatic effect.

**Syntax:**
```duck
quack [waddle count times
  quack [...]
]

quack [waddle through list do
  quack [...]
]
```

**Examples:**
```duck
quack [waddle 5 times
  quack [print "Step {i}..."]  -- Prints with 500ms delay between each
]
```

**Implementation:**
1. Add `Waddle` token to `lexer.rs`
2. Add `Statement::Waddle` variants to `ast.rs`
3. Parse similar to `repeat` and `for each`
4. In interpreter, add `thread::sleep()` between iterations
5. Goose commentary between steps: "*waddles pensively*", "*considers next step*"

---

### `migrate` - Module Imports

Import code from other Duck files using migration terminology.

**Syntax:**
```duck
quack [migrate "path/to/file.duck"]
quack [migrate "utils.duck" as utils]
```

**Examples:**
```duck
quack [migrate "math-helpers.duck"]       -- Import all into current scope
quack [migrate "utils.duck" as u]         -- Import with namespace prefix
quack [u.helper-function 42]
```

**Implementation:**
1. Add `Migrate` token to `lexer.rs`
2. Add `Statement::Migrate { path: String, alias: Option<String> }` to `ast.rs`
3. Parse in `parser.rs`
4. In interpreter:
   - Read and parse the target file
   - Execute in a new environment
   - Copy defined functions/structs to current scope (or namespaced)
5. Handle circular imports with a "seen files" set
6. Goose commentary: "The flock has arrived from {path}!"

---

### `nest` - Namespaces

Group related definitions in a namespace.

**Syntax:**
```duck
quack [nest name with
  quack [define func taking [...] as ...]
  quack [struct thing with [...]]
]
```

**Examples:**
```duck
quack [nest math with
  quack [define square taking [x] as
    quack [return x * x]
  ]
  quack [define cube taking [x] as
    quack [return x * x * x]
  ]
]

quack [print [math.square 5]]  -- 25
quack [print [math.cube 3]]    -- 27
```

**Implementation:**
1. Add `Nest` token to `lexer.rs`
2. Add `Statement::Nest { name: String, body: Vec<Statement> }` to `ast.rs`
3. In interpreter, create a struct-like value that holds the namespace's bindings
4. Field access (`math.square`) resolves to the function

---

### Enhanced Goose Reactions

Add special goose reactions for specific code patterns.

| Pattern | Goose Reaction |
|---------|----------------|
| Infinite loop detected | "You've sent me on a wild goose chase!" |
| Empty program | "You gave me nothing. I am an empty goose." |
| 100% quack ratio | "*proud honk* Perfect quacking discipline!" |
| Variable named `goose` | "Oh, you're naming things after me? I'm flattered." |
| Very long function | "This function is longer than my wingspan." |
| Deeply nested code | "I'm going to need a deeper pond for this." |

**Implementation:**
- Add detection logic in `interpreter.rs` or `goose.rs`
- Trigger special messages based on patterns

---

## Practical Features

These features make Duck more useful for real programs.

### Higher-Order List Functions

#### `map`
Apply a function to each element.

**Signature:** `map(list, function) -> list`

**Examples:**
```duck
quack [let doubled be [map list(1, 2, 3) [x] -> x * 2]]  -- [2, 4, 6]

quack [define add-one taking [x] as
  quack [return x + 1]
]
quack [let incremented be [map list(1, 2, 3) add-one]]   -- [2, 3, 4]
```

**Implementation:**
- Add to `builtins.rs`
- Accept a `Value::Lambda` or `Value::Function` as second argument
- Iterate list, call function on each element, collect results

---

#### `filter`
Keep elements that satisfy a predicate.

**Signature:** `filter(list, predicate) -> list`

**Examples:**
```duck
quack [let evens be [filter list(1, 2, 3, 4) [x] -> x % 2 == 0]]  -- [2, 4]
```

**Implementation:**
- Add to `builtins.rs`
- Call predicate on each element, keep those returning truthy values

---

#### `fold` / `reduce`
Reduce a list to a single value.

**Signature:** `fold(list, initial, function) -> value`

**Examples:**
```duck
quack [let sum be [fold list(1, 2, 3) 0 [acc, x] -> acc + x]]     -- 6
quack [let product be [fold list(1, 2, 3, 4) 1 [a, b] -> a * b]]  -- 24
```

**Implementation:**
- Add to `builtins.rs`
- Start with initial value, apply function with accumulator and each element

---

#### `find`
Find the first element matching a predicate.

**Signature:** `find(list, predicate) -> value or null`

**Examples:**
```duck
quack [let first-even be [find list(1, 3, 4, 5) [x] -> x % 2 == 0]]  -- 4
quack [let not-found be [find list(1, 3, 5) [x] -> x % 2 == 0]]     -- null
```

---

#### `any` / `all`
Check if any/all elements satisfy a predicate.

**Signature:** `any(list, predicate) -> boolean`, `all(list, predicate) -> boolean`

**Examples:**
```duck
quack [let has-even be [any list(1, 2, 3) [x] -> x % 2 == 0]]  -- true
quack [let all-pos be [all list(1, 2, 3) [x] -> x > 0]]        -- true
```

---

### Dictionary/Map Type

A key-value collection type.

**Syntax:**
```duck
quack [let scores be dict("alice", 100, "bob", 85)]
quack [let empty be dict()]
```

**Operations:**
```duck
quack [print scores.alice]              -- 100 (dot access)
quack [print [scores get "alice"]]      -- 100 (method access)
quack [scores set "charlie" 90]         -- Add/update key
quack [let has-it be [scores has "bob"]] -- true
quack [scores remove "bob"]             -- Remove key
quack [let ks be [keys scores]]         -- ["alice", "charlie"]
quack [let vs be [values scores]]       -- [100, 90]
```

**Implementation:**
1. Add `Value::Dict(Rc<RefCell<HashMap<String, Value>>>)` to `values.rs`
2. Add `dict` builtin to create dictionaries
3. Add dict-specific builtins: `get`, `set`, `has`, `remove`
4. Support dot notation for string keys in interpreter

---

### File I/O

Read and write files from Duck programs.

#### `read-file`
Read entire file contents as a string.

**Signature:** `read-file(path) -> string`

**Examples:**
```duck
quack [let content be [read-file "data.txt"]]
quack [print content]
```

**Error handling:**
- File not found: "The goose searched everywhere but couldn't find '{path}'"
- Permission denied: "The goose is not allowed to look at '{path}'"

---

#### `write-file`
Write a string to a file (creates or overwrites).

**Signature:** `write-file(path, content) -> null`

**Examples:**
```duck
quack [write-file "output.txt" "Hello, file!"]
```

---

#### `append-file`
Append to an existing file.

**Signature:** `append-file(path, content) -> null`

**Examples:**
```duck
quack [append-file "log.txt" "New log entry\n"]
```

---

#### `file-exists`
Check if a file exists.

**Signature:** `file-exists(path) -> boolean`

**Examples:**
```duck
quack [if [file-exists "config.duck"] then
  quack [migrate "config.duck"]
]
```

---

### Error Handling with `attempt`/`rescue`

Handle runtime errors gracefully.

**Syntax:**
```duck
quack [attempt
  quack [...]  -- Code that might fail
rescue error-var
  quack [...]  -- Error handling code
]
```

**Examples:**
```duck
quack [attempt
  quack [let data be [read-file "missing.txt"]]
  quack [print data]
rescue err
  quack [print "Failed to read file: {err}"]
  quack [let data be "default"]
]
```

**Implementation:**
1. Add `Attempt`/`Rescue` tokens to `lexer.rs`
2. Add `Statement::Attempt { try_block, rescue_var, rescue_block }` to `ast.rs`
3. In interpreter, catch errors from try_block, bind to rescue_var, execute rescue_block

---

### Pattern Matching Improvements

Enhanced patterns for match statements.

#### Range Patterns
```duck
quack [match score with
  [when 90..100 then quack [print "A"]]
  [when 80..89 then quack [print "B"]]
  [when 70..79 then quack [print "C"]]
  [when _ then quack [print "F"]]
]
```

#### List Destructuring
```duck
quack [match my-list with
  [when [] then quack [print "empty"]]
  [when [first] then quack [print "one element: {first}"]]
  [when [first, second, ...rest] then
    quack [print "first two: {first}, {second}"]
  ]
]
```

#### Guard Clauses
```duck
quack [match value with
  [when n if n > 0 then quack [print "positive"]]
  [when n if n < 0 then quack [print "negative"]]
  [when _ then quack [print "zero"]]
]
```

**Implementation:**
1. Extend `Pattern` enum in `ast.rs` with `Range`, `ListDestructure`, guards
2. Update parser for new pattern syntax
3. Update `match_pattern()` in interpreter

---

### Command-Line Arguments

Access arguments passed to the Duck program.

**Built-in variable:** `quack-args`

**Examples:**
```bash
$ goose run greet.duck Alice Bob
```

```duck
-- greet.duck
quack [let args be quack-args]
quack [print "Number of args: {args length}"]

quack [for each [name] in args do
  quack [print "Hello, {name}!"]
]
```

**Implementation:**
- Capture `std::env::args()` in `main.rs`
- Pass to interpreter, pre-define as `quack-args` in global environment

---

## Advanced Features

These are more complex features for future consideration.

### List Comprehensions

Concise list creation syntax.

**Syntax:**
```duck
quack [let squares be [for x in range(1, 10) yield x * x]]
quack [let evens be [for x in range(1, 20) if x % 2 == 0 yield x]]
```

**Implementation:**
- New expression type in AST
- Parse `for ... in ... yield ...` with optional `if`
- Desugar to map/filter or implement directly

---

### Async/Concurrent Features

Run operations concurrently.

**Syntax:**
```duck
quack [let results be [flock
  quack [fetch "url1"]
  quack [fetch "url2"]
  quack [fetch "url3"]
]]
-- All three run concurrently, results collected in list
```

**Implementation:**
- Would require significant runtime changes
- Consider using Rust's async/await under the hood
- "Flock" terminology fits the theme!

---

### REPL Enhancements

Improve the interactive experience.

| Feature | Description |
|---------|-------------|
| History | Arrow keys to navigate previous commands |
| Tab completion | Complete variable/function names |
| Multi-line input | Continue input across lines |
| `:help` command | Show available commands |
| `:clear` command | Clear the screen |
| `:save` command | Save session to file |
| `:load` command | Load and execute a file |

**Implementation:**
- Use `rustyline` crate for readline functionality
- Add REPL-specific commands (prefixed with `:`)

---

### Type Hints (Optional)

Optional type annotations for documentation and potential future type checking.

**Syntax:**
```duck
quack [define add taking [x: number, y: number] -> number as
  quack [return x + y]
]

quack [let name: string be "Duck"]
```

**Implementation:**
- Parse but ignore for now (documentation only)
- Could enable runtime type checking as an option
- Future: static type checker as separate tool

---

## Implementation Priority

### Phase 1: Quick Wins (1-2 days each)
1. [ ] `reverse`, `sort`, `join`, `split`, `trim`
2. [ ] `uppercase`, `lowercase`, `contains`
3. [ ] Math constants (PI, E, TAU)
4. [ ] `sleep`
5. [ ] `honk` assertions

### Phase 2: Core Improvements (3-5 days each)
1. [ ] `map`, `filter`, `fold`
2. [ ] `find`, `any`, `all`
3. [ ] File I/O (`read-file`, `write-file`)
4. [ ] Command-line arguments
5. [ ] `attempt`/`rescue` error handling

### Phase 3: Major Features (1-2 weeks each)
1. [ ] Dictionary type
2. [ ] `migrate` module system
3. [ ] Pattern matching improvements
4. [ ] `nest` namespaces
5. [ ] `waddle` dramatic iteration

### Phase 4: Advanced (Future)
1. [ ] List comprehensions
2. [ ] REPL enhancements
3. [ ] Async/concurrent features
4. [ ] Optional type hints

---

## Contributing

When implementing a feature:

1. **Read CLAUDE.md** for coding conventions
2. **Start with tests** - add test cases first
3. **Follow the pattern** - look at similar existing features
4. **Update docs** - add to README and examples
5. **Keep the theme** - maintain goose personality in messages

---

*Honk! Let's make Duck the most entertaining language to program in!* 🦆
