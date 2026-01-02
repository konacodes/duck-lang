# Suggestions for Duck-lang

A living document of ideas, improvements, and wild dreams for the future of Duck. Some are practical, some are ambitious, all are quack-worthy.

---

## Quick Wins

Small changes that pack a punch. Weekend projects or rainy afternoon vibes.

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

### Library: `konacodes/dice`

Random utilities for games and decision-making.

```duck
quack [migrate "git+konacodes/dice" as dice]

quack [print dice.roll(6)]           -- 1-6
quack [print dice.roll(20)]          -- D&D d20
quack [print dice.flip()]            -- "heads" or "tails"
quack [print dice.pick(my-list)]     -- Random element
quack [print dice.shuffle(my-list)]  -- Shuffled copy
quack [print dice.chance(0.3)]       -- true 30% of the time
```

### Library: `konacodes/color`

Terminal styling made easy.

```duck
quack [migrate "git+konacodes/color" as c]

quack [print c.red("Error!")]
quack [print c.bold(c.green("Success!"))]
quack [print c.dim("subtle text")]
quack [print c.rainbow("party time")]  -- Each letter different color
```

### Library: `konacodes/dotenv`

Load environment from `.env` files.

```duck
quack [migrate "git+konacodes/dotenv"]
quack [dotenv-load()]  -- Loads .env into environment

quack [let token be env("DISCORD_TOKEN")]
```

### Library: `konacodes/uuid`

Generate unique IDs.

```duck
quack [migrate "git+konacodes/uuid" as uuid]

quack [print uuid.v4()]      -- "550e8400-e29b-41d4-a716-446655440000"
quack [print uuid.short()]   -- "x7Hk9pQ" (URL-safe short ID)
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

### Library: `konacodes/cli`

Build beautiful command-line interfaces.

```duck
quack [migrate "git+konacodes/cli" as cli]

quack [cli.spinner "Loading..." [done] ->
  quack [do-slow-thing()]
  quack [done()]
]

quack [let choice be cli.select("Pick one:" list("Option A", "Option B", "Option C"))]
quack [let confirmed be cli.confirm("Are you sure?")]
quack [cli.progress-bar 0 100 current-value]
```

### Library: `konacodes/csv`

Parse and generate CSV files.

```duck
quack [migrate "git+konacodes/csv" as csv]

quack [let data be csv.parse(read-file("data.csv"))]
quack [for each [row] in data do
  quack [print row.name]  -- Access by header name
]

quack [let output be csv.stringify(my-list-of-structs)]
quack [let unused be write-file("output.csv", output)]
```

### Library: `konacodes/cron`

Schedule recurring tasks.

```duck
quack [migrate "git+konacodes/cron" as cron]

quack [cron.every "5 minutes" [] ->
  quack [print "This runs every 5 minutes"]
]

quack [cron.daily "09:00" [] ->
  quack [send-morning-report()]
]

quack [cron.start()]  -- Blocks and runs scheduler
```

### Library: `konacodes/validate`

Data validation with helpful errors.

```duck
quack [migrate "git+konacodes/validate" as v]

quack [let schema be v.object({
  name: v.string().min(1).max(50),
  email: v.string().email(),
  age: v.number().min(0).max(150)
})]

quack [let result be v.check(schema, user-input)]
quack [if result.valid then
  quack [save-user(user-input)]
otherwise
  quack [print result.errors]
]
```

### Library: `konacodes/cache`

Simple in-memory caching with TTL.

```duck
quack [migrate "git+konacodes/cache" as cache]

quack [cache.set "user:123" user-data 3600]  -- TTL in seconds
quack [let user be cache.get "user:123"]

quack [if user == nil then
  quack [let user be fetch-from-db("123")]
  quack [cache.set "user:123" user 3600]
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

### Library: `konacodes/web`

Serve HTTP with a simple web framework.

```duck
quack [migrate "git+konacodes/web" as web]

quack [web.route "/" [req] ->
  quack [return web.html("<h1>Welcome to Duck!</h1>")]
]

quack [web.route "/api/quack" [req] ->
  quack [return web.json({message: "Quack!"})]
]

quack [web.route "/users/:id" [req] ->
  quack [let user be get-user(req.params.id)]
  quack [return web.json(user)]
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

### Library: `konacodes/telegram`

Telegram bot library (like the Discord one).

```duck
quack [migrate "git+konacodes/telegram" as tg]

quack [let bot be tg.bot(env("TELEGRAM_TOKEN"))]

quack [tg.on-message bot [msg] ->
  quack [if msg.text == "/start" then
    quack [tg.reply msg "Welcome! Quack!"]
  ]
]

quack [tg.start-polling bot]
```

### Library: `konacodes/openai`

Talk to AI APIs (OpenAI, Claude, etc).

```duck
quack [migrate "git+konacodes/openai" as ai]

quack [let client be ai.client(env("OPENAI_API_KEY"))]

quack [let response be ai.chat client list(
  ai.system("You are a helpful duck."),
  ai.user("What's the meaning of quack?")
)]

quack [print response.content]
```

### Library: `konacodes/websocket`

WebSocket client for real-time connections.

```duck
quack [migrate "git+konacodes/websocket" as ws]

quack [let socket be ws.connect("wss://example.com/socket")]

quack [ws.on-message socket [msg] ->
  quack [print f"Received: {msg}"]
]

quack [ws.on-close socket [] ->
  quack [print "Connection closed"]
]

quack [ws.send socket "Hello!"]
```

### Library: `konacodes/sqlite`

Embedded database.

```duck
quack [migrate "git+konacodes/sqlite" as db]

quack [let conn be db.open("app.db")]
quack [db.exec conn "CREATE TABLE IF NOT EXISTS ducks (name TEXT, age INTEGER)"]
quack [db.exec conn "INSERT INTO ducks VALUES (?, ?)" list("Gerald", 5)]

quack [let ducks be db.query conn "SELECT * FROM ducks WHERE age > ?" list(3)]
quack [for each [duck] in ducks do
  quack [print duck.name]
]
```

### Library: `konacodes/test`

Unit testing framework that builds on `honk` assertions.

While `honk` is great for inline assertions that crash on failure, a test library lets you run multiple tests, collect results, and get nice output. Uses `honk` under the hood but catches failures gracefully.

```duck
quack [migrate "git+konacodes/test" as test]

quack [test.describe "Math operations" [] ->
  quack [test.it "adds numbers correctly" [] ->
    quack [honk 1 + 1 == 2]          -- Use honk directly
    quack [honk 5 + 5 == 10]
  ]

  quack [test.it "handles negative numbers" [] ->
    quack [honk -1 + 1 == 0]
  ]

  quack [test.it "works with messages" [] ->
    quack [honk 2 * 2 == 4 "basic multiplication"]
  ]
]

quack [test.run()]
-- Output:
-- Math operations
--   ✓ adds numbers correctly
--   ✓ handles negative numbers
--   ✓ works with messages
-- 3 passing, 0 failing
```

### Library: `konacodes/html`

HTML templating and generation.

```duck
quack [migrate "git+konacodes/html" as h]

quack [let page be h.doc(
  h.head(
    h.title("My Page")
  ),
  h.body(
    h.h1("Welcome!"),
    h.p("This is a paragraph."),
    h.ul(
      map(items, [item] -> h.li(item))
    )
  )
)]

quack [print h.render(page)]
```

### `goose fmt`

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
quack [migrate "git+konacodes/http" exposing [get, post]]  -- Selective import
quack [migrate "./utils" hiding [internal-fn]]              -- Hide internals
quack [migrate "git+konacodes/big-lib" lazy]                -- Load on first use
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

### Library: `konacodes/pond` - Simulation mode

A duck pond life simulator you can run from any program.

```duck
quack [migrate "git+konacodes/pond" as pond]

quack [pond.add-duck "Gerald"]
quack [pond.add-duck "Waddles"]
quack [pond.add-bread 10 20]  -- x, y coordinates
quack [pond.simulate 100]     -- 100 ticks of duck pond life
-- Watch the ducks waddle around, find bread, and interact
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

### Library: `konacodes/quackspeak`

Translate text to duck sounds and back.

```duck
quack [migrate "git+konacodes/quackspeak" as qs]

quack [print qs.encode("Hello")]     -- "QUACK quack QUACK quack quack"
quack [print qs.decode("QUACK quack QUACK quack quack")]  -- "Hello"

-- Encrypted duck communication
quack [let secret be qs.encode("The bread is at midnight")]
```

### Library: `konacodes/ascii`

ASCII art generators and utilities.

```duck
quack [migrate "git+konacodes/ascii" as art]

quack [print art.banner("DUCK LANG")]
-- ____  _   _  ____ _  __  _        _    _   _  ____
-- |  _ \| | | |/ ___| |/ / | |      / \  | \ | |/ ___|
-- | | | | | | | |   | ' /  | |     / _ \ |  \| | |  _
-- | |_| | |_| | |___| . \  | |___ / ___ \| |\  | |_| |
-- |____/ \___/ \____|_|\_\ |_____/_/   \_\_| \_|\____|

quack [print art.box("Important message here")]
quack [print art.duck()]  -- Prints an ASCII duck
```

### Library: `konacodes/game`

Simple terminal game engine.

```duck
quack [migrate "git+konacodes/game" as game]

quack [let screen be game.screen(80, 24)]
quack [let player be game.sprite("🦆", 10, 10)]

quack [game.on-key "w" [] -> quack [player.move-up()]]
quack [game.on-key "s" [] -> quack [player.move-down()]]
quack [game.on-key "a" [] -> quack [player.move-left()]]
quack [game.on-key "d" [] -> quack [player.move-right()]]

quack [game.loop screen 60 [] ->  -- 60 FPS
  quack [screen.clear()]
  quack [screen.draw(player)]
  quack [screen.render()]
]
```

### Library: `konacodes/sound`

Play sounds and music.

```duck
quack [migrate "git+konacodes/sound" as sound]

quack [sound.beep()]                      -- System beep
quack [sound.play("quack.wav")]           -- Play audio file
quack [sound.say("Hello, I am a duck")]   -- Text-to-speech

quack [sound.tone(440, 500)]  -- 440Hz for 500ms (A note)
```

### Library: `konacodes/fortune`

Random wisdom and nonsense.

```duck
quack [migrate "git+konacodes/fortune" as fortune]

quack [print fortune.wisdom()]     -- Random inspirational quote
quack [print fortune.joke()]       -- Random programming joke
quack [print fortune.duck-fact()]  -- Random fact about ducks
quack [print fortune.excuse()]     -- Random excuse for broken code
-- "The code works on my machine because mercury is in retrograde"
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
