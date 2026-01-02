# Suggestions for Duck-lang

A living document of ideas, improvements, and wild dreams for the future of Duck. Some are practical, some are ambitious, all are quack-worthy.

---

## Quick Wins

Small changes that pack a punch. Weekend projects or rainy afternoon vibes.

### `honk` as an alias for `print`

Because why not? The goose should be able to honk.

```duck
quack [honk "Hello, World!"]  -- Same as print, more on-brand
```

### `waddle` - Random delay

A themed version of `sleep` that adds a bit of randomness (like a real duck waddling).

```duck
quack [waddle 1000]  -- Sleeps 800-1200ms randomly
```

### `unless` keyword

The inverse of `if`. Reads more naturally for negative conditions.

```duck
quack [unless is-banned then
  quack [grant-access()]
]
```

### `until` loop

The inverse of `while`. Loop until something becomes true.

```duck
quack [until found do
  quack [search-next()]
]
```

### Built-in `pond` namespace for constants

Math and common constants, duck-themed.

```duck
quack [print pond.pi]        -- 3.14159...
quack [print pond.e]         -- 2.71828...
quack [print pond.golden]    -- 1.61803... (golden ratio)
quack [print pond.dozen]     -- 12 (why not)
```

### `quack?` - Optional execution

Try to run a block, return `nil` on error instead of crashing. Like a soft quack.

```duck
quack? [let data be json-parse(maybe-invalid-json)]
quack [if data == nil then
  quack [print "Parse failed, but we're okay"]
]
```

### Color output helpers

```duck
quack [print-red "Error!"]
quack [print-green "Success!"]
quack [print-yellow "Warning..."]
quack [print-bold "Important"]
```

### `feathers` - Pretty print for debugging

Auto-formats structs and lists nicely.

```duck
quack [feathers my-complex-struct]
-- Prints with indentation, colors, type annotations
```

---

## A Little Work

A few evenings of focused work. Meaningful improvements.

### Pipe operator `|>`

Chain operations left-to-right instead of nested parentheses.

```duck
-- Instead of:
quack [let result be uppercase(trim(read-file("data.txt")))]

-- Write:
quack [let result be "data.txt" |> read-file |> trim |> uppercase]
```

### `match` expression

Pattern matching for cleaner branching.

```duck
quack [match command with
  "ping" -> quack [send "pong"]
  "help" -> quack [show-help()]
  "quit" -> quack [exit()]
  _ -> quack [send "Unknown command"]
]
```

### `flock` - Simple parallelism

Run multiple operations concurrently, wait for all to complete.

```duck
quack [let results be flock(
  http-get("https://api1.com"),
  http-get("https://api2.com"),
  http-get("https://api3.com")
)]
```

### Destructuring assignment

Unpack structs and lists in one go.

```duck
quack [let [first, second, third] be my-list]
quack [let {name, age} be my-duck-struct]
```

### Range syntax

Literal syntax for ranges instead of `range()` function.

```duck
quack [for each [i] in 1..10 do
  quack [print i]
]

quack [let letters be "a".."z"]  -- List of letters
```

### `molt` - Clear variable from scope

Explicitly free a variable (useful in long-running scripts).

```duck
quack [let huge-data be load-big-file()]
quack [process(huge-data)]
quack [molt huge-data]  -- Gone, memory freed
```

### REPL improvements

- Command history (up arrow)
- Multi-line input (detect incomplete blocks)
- `.save` to export session to a file
- `.load` to import a file into session

### `attempt` improvements

Named error types and multiple rescue clauses.

```duck
quack [attempt
  quack [risky-operation()]
rescue NetworkError as e
  quack [print f"Network issue: {e}"]
rescue ParseError as e
  quack [print f"Bad data: {e}"]
rescue
  quack [print "Something else went wrong"]
]
```

### `defer` statement

Run something when the current block exits (like Go).

```duck
quack [define process-file taking [path] as
  quack [let file be open-file(path)]
  quack [defer close-file(file)]  -- Runs when function exits

  quack [do-stuff-with(file)]
  -- file automatically closed even if we return early or error
]
```

---

## Big Features

Week-long projects. Game changers.

### Duck Package Registry

A central registry for Duck libraries. Like npm, but with more waterfowl.

```bash
goose publish              # Publish your library
goose search "discord"     # Find packages
goose install pond/http    # Install from registry
```

Website at `pond.duck-lang.org` with search, docs, download counts.

### WebSocket support

For real-time applications and proper Discord bots.

```duck
quack [let ws be websocket-connect("wss://gateway.discord.gg")]

quack [websocket-on-message ws [msg] ->
  quack [print f"Received: {msg}"]
]

quack [websocket-send ws "{\"op\": 1}"]
```

### Built-in SQLite

Simple database for persistent storage without external dependencies.

```duck
quack [let db be db-open("app.db")]
quack [db-exec db "CREATE TABLE users (id INTEGER, name TEXT)"]
quack [db-exec db "INSERT INTO users VALUES (1, 'Gerald')"]

quack [let users be db-query db "SELECT * FROM users"]
quack [for each [row] in users do
  quack [print row.name]
]
```

### Simple web server

Serve HTTP without external libraries.

```duck
quack [migrate "std:web" as web]

quack [web.route "/" [req] ->
  quack [return web.html("<h1>Welcome to Duck!</h1>")]
]

quack [web.route "/api/quack" [req] ->
  quack [return web.json({message: "Quack!"})]
]

quack [web.listen 8080]
```

### Debug mode

Step-through debugging with breakpoints.

```bash
goose debug myfile.duck
```

```
[debug] Stopped at line 15
[debug] > print x
42
[debug] > step
[debug] Stopped at line 16
[debug] > continue
```

### Standard library expansion

Core libraries that ship with Duck:

- `std:web` - HTTP server
- `std:db` - SQLite wrapper
- `std:crypto` - Hashing, encryption
- `std:time` - Date/time handling
- `std:test` - Unit testing framework
- `std:cli` - Argument parsing, colors, spinners

### `duck fmt`

Auto-formatter for Duck code.

```bash
goose fmt myfile.duck        # Format one file
goose fmt .                  # Format all .duck files
goose fmt --check myfile.duck  # Check without modifying
```

### Compile to standalone binary

Bundle a Duck program into a single executable.

```bash
goose build myapp.duck -o myapp
./myapp  # Runs without needing goose installed
```

---

## Might Require Overhauls

Ambitious ideas that could reshape the language. Proceed with caution.

### Optional type hints

Add type annotations without making them required.

```duck
quack [define add taking [a: number, b: number] -> number as
  quack [return a + b]
]

quack [let name: string be "Gerald"]
```

Benefits:
- Better error messages
- Editor autocomplete
- Catch bugs earlier
- Still optional (gradual typing)

### Async/await

Non-blocking I/O for performance.

```duck
quack [define fetch-all taking [urls] as
  quack [let results be list()]
  quack [for each [url] in urls do
    quack [let data be await http-get(url)]
    quack [results push data]
  ]
  quack [return results]
]

quack [let data be await fetch-all(my-urls)]
```

### Macros

User-defined syntax extensions.

```duck
quack [macro log! [expr] as
  quack [print f"[DEBUG] {stringify(expr)} = {expr}"]
]

quack [let x be 42]
quack [log! x * 2]  -- Prints: [DEBUG] x * 2 = 84
```

### Bytecode compilation

Compile to bytecode for faster execution.

```bash
goose compile myfile.duck -o myfile.duckc  # Compile
goose run myfile.duckc                       # Run compiled
```

Benefits:
- Faster startup
- Better performance
- Smaller distribution size
- Harder to reverse-engineer (if that matters)

### Module system overhaul

Proper namespacing, circular dependency handling, lazy loading.

```duck
quack [migrate "std:http" exposing [get, post]]  -- Selective import
quack [migrate "./utils" hiding [internal-fn]]    -- Hide internals
```

### Duck Language Server (LSP)

Full IDE support for VS Code, Neovim, etc.

- Syntax highlighting
- Error checking as you type
- Autocomplete for functions and fields
- Go to definition
- Hover for documentation
- Rename symbol

---

## Wild Ideas (Just for Fun)

Things that probably shouldn't exist but would be hilarious.

### `quackquackquack` for priority execution

The more quacks, the higher priority. Just because.

```duck
quackquackquack [print "I run first!"]
quack [print "I run second"]
```

### Easter egg: `goose run --chaos`

Randomly skips 10% of quacked blocks. For testing "fault tolerance" (chaos engineering, duck-style).

### `pond.duck` - Simulation mode

A built-in duck pond simulator you can access from any program.

```duck
quack [migrate "std:pond"]
quack [pond.add-duck "Gerald"]
quack [pond.add-duck "Waddles"]
quack [pond.simulate 100]  -- 100 ticks of duck pond life
```

### Goose gets sassier over time

The more you use the REPL, the more personality the goose develops.

```
duck> quack [print "hello"]
hello
   Not bad.

-- 100 commands later --

duck> quack [print "hello"]
hello
   You again? Still writing hello world? We've been over this 47 times.
```

### Achievement system

```
🏆 First Quack - Run your first program
🏆 Centurion - Write a 100-line program
🏆 Library Author - Publish a package
🏆 Goose Whisperer - Get a 10/10 rating
🏆 Chaos Duck - Run with --chaos and survive
```

---

## Consolidation Ideas

Things we could simplify or merge.

### Merge `repeat N times` into `for`

```duck
-- Current:
quack [repeat 5 times ...]

-- Could become:
quack [for 5 times ...]
```

### Unify list/struct access

Both use `.` for fields and `at` for indices. Consider making `at` work on structs too (by field name as string).

```duck
quack [print my-struct at "name"]  -- Same as my-struct.name
```

### Simplify HTTP response

Instead of accessing `.status`, `.body`, `.headers` separately:

```duck
quack [let {status, body, headers} be http-get(url)]
```

---

## Performance Ideas

### Lazy evaluation for lists

Don't compute list elements until accessed.

```duck
quack [let huge be range(0, 1000000)]  -- Instant, not computed yet
quack [let first-ten be take(huge, 10)]  -- Only computes 10
```

### String interning

Cache string values to reduce memory for repeated strings.

### Tail call optimization

Enable infinite recursion for tail-recursive functions.

```duck
quack [define countdown taking [n] as
  quack [if n == 0 then
    quack [print "Done!"]
  otherwise
    quack [print n]
    quack [return countdown(n - 1)]  -- Tail call, no stack growth
  ]
]
```

---

## What's Next?

Vote with your commits. Pick something from Quick Wins and just do it. The best suggestion is the one that gets implemented.

Remember: Duck is a fun language. If an idea makes you smile, it's probably worth exploring.

*Happy quacking!* 🦆
