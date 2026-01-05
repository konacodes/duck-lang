# The Only Document You'll Ever Need

*A complete guide to Duck, the language where you have to ask permission to run code.*

---

## Table of Contents

1. [What Even Is This](#what-even-is-this)
2. [Getting the Goose](#getting-the-goose)
3. [The Quack Protocol](#the-quack-protocol)
4. [Storing Stuff](#storing-stuff)
5. [Strings (The Good Kind)](#strings-the-good-kind)
6. [Making Decisions](#making-decisions)
7. [Going in Circles](#going-in-circles)
8. [Functions (Reusable Quacks)](#functions-reusable-quacks)
9. [Lists (Duck Ponds)](#lists-duck-ponds)
10. [Structs (Duck Blueprints)](#structs-duck-blueprints)
11. [Higher-Order Shenanigans](#higher-order-shenanigans)
12. [Reading and Writing Files](#reading-and-writing-files)
13. [Talking to the Internet](#talking-to-the-internet)
14. [WebSockets (Persistent Connections)](#websockets-persistent-connections)
15. [Using Libraries](#using-libraries)
16. [When Things Go Wrong](#when-things-go-wrong)
17. [The Stuff You'll Mess Up](#the-stuff-youll-mess-up)
18. [All The Builtins](#all-the-builtins)

---

## What Even Is This

Duck is a programming language. Its interpreter is called Goose. They have a complicated relationship.

The core idea: **every code block must be preceded by `quack` to execute.** Without explicit authorization, the goose simply refuses to run your code. It's like `sudo` but with more feathers and passive aggression.

```duck
quack [print "This runs"]
[print "This does not"]  -- The goose stares at you judgmentally
```

Why? Honestly, it started as a joke. But it turns out that requiring explicit authorization for every statement has some interesting properties. You can "comment out" code by removing its quack. You're forced to think about what actually runs. And everything becomes slightly funnier.

Is Duck suitable for production? Almost certainly not. Is it fun? Absolutely. Will the goose rate your code at the end and make you feel bad? Yes.

---

## Getting the Goose

**Option A: The Easy Way**
```bash
curl -fsSL https://raw.githubusercontent.com/konacodes/duck-lang/master/install.sh | bash
```

**Option B: Build From Source (For the Brave)**
```bash
git clone https://github.com/konacodes/duck-lang
cd duck-lang
cargo build --release
```

The binary is called `goose`. Make sure it's in your PATH.

```bash
goose --version
```

If that prints something, you're golden. If not, add `~/.duck/bin` to your PATH and try again.

**Running Things**
```bash
goose run myfile.duck     # Run a file
goose check myfile.duck   # Check for missing quacks without running
goose repl                # Interactive mode for experimentation
```

---

## The Quack Protocol

Here's the deal. Code in Duck lives inside `[brackets]`. These are called blocks. Blocks don't run unless you say `quack` first.

```duck
quack [print "Hello"]  -- This executes
[print "Goodbye"]      -- The goose ignores this entirely
```

Think of `quack` as consent. The goose won't do anything without it.

**Multiple quacks authorize multiple blocks:**
```duck
quack quack quack [print "One"] [print "Two"] [print "Three"]
```

**Every block needs its own quack, even inside other structures:**
```duck
quack [if x > 5 then
  quack [print "x is big"]   -- Still needs quack!
otherwise
  quack [print "x is small"] -- This one too!
]
```

**Why this actually doesn't suck:**
- Want to disable a line? Remove its quack. No commenting out.
- Debugging becomes surgical. Enable exactly what you want to test.
- You can't accidentally execute code. Everything is intentional.

The goose notices unquacked blocks and will comment on them. Your final code rating suffers. The goose remembers everything.

---

## Storing Stuff

Variables use `let` and `be`. Assignment uses `becomes`. No equals signs here.

```duck
quack [let x be 42]
quack [let name be "Gerald"]
quack [let is-cool be true]
```

To change a variable:
```duck
quack [x becomes x + 1]
quack [name becomes "Waddles"]
```

**Why `be` and `becomes`?**

Because Duck reads like natural language. You're not assigning; you're declaring what something *is* or what it *becomes*.

**Types exist but you don't declare them:**
- Numbers: `42`, `3.14`, `-17`
- Strings: `"hello"`
- Booleans: `true`, `false`
- Lists: `list(1, 2, 3)`
- Structs: We'll get there
- Null: `nil`

Check a type with `type-of()`:
```duck
quack [print type-of(42)]       -- "number"
quack [print type-of("hello")]  -- "string"
quack [print type-of(nil)]      -- "null"
```

---

## Strings (The Good Kind)

Double quotes only. Single quotes are for air quotes and sarcasm, not code.

```duck
quack [let greeting be "Hello, World!"]
```

**String interpolation uses the `f` prefix:**
```duck
quack [let name be "Gerald"]
quack [let age be 5]
quack [print f"My name is {name} and I am {age} years old"]
-- Output: My name is Gerald and I am 5 years old
```

You can put any expression in the braces:
```duck
quack [print f"2 + 2 = {2 + 2}"]
quack [print f"Uppercase: {uppercase(name)}"]
```

Forget the `f` and you get literal braces:
```duck
quack [print "Hello, {name}"]  -- Prints: Hello, {name}
```

**String operations:**
```duck
quack [print len("hello")]           -- 5
quack [print uppercase("hello")]     -- HELLO
quack [print lowercase("HELLO")]     -- hello
quack [print trim("  hello  ")]      -- hello (no spaces)
quack [print contains("hello", "ell")]  -- true
quack [print reverse("hello")]       -- olleh
quack [print split("a,b,c", ",")]    -- ["a", "b", "c"]
```

**Accessing characters:**
```duck
quack [let s be "hello"]
quack [print s at 0]  -- h
quack [print s at 4]  -- o
```

---

## Making Decisions

Conditionals use `if`, `then`, and `otherwise`:

```duck
quack [let age be 18]

quack [if age >= 21 then
  quack [print "Have a drink"]
otherwise
  quack [print "Have some juice"]
]
```

No `otherwise` needed if you don't want one:
```duck
quack [if is-raining then
  quack [print "Bring an umbrella"]
]
```

**Chained conditions? Nest them:**
```duck
quack [if score >= 90 then
  quack [print "A"]
otherwise
  quack [if score >= 80 then
    quack [print "B"]
  otherwise
    quack [if score >= 70 then
      quack [print "C"]
    otherwise
      quack [print "F"]
    ]
  ]
]
```

Yeah, no `elsif`. The nesting has a certain brutalist charm.

**Comparison operators:**
| Operator | Meaning |
|----------|---------|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less or equal |
| `>=` | Greater or equal |

**Logical operators:**
| Operator | Meaning |
|----------|---------|
| `and` | Both must be true |
| `or` | Either can be true |
| `not` | Negation |

```duck
quack [if age >= 18 and has-id then
  quack [print "Access granted"]
]

quack [if not is-banned then
  quack [print "Welcome"]
]
```

---

## Going in Circles

**While loops:**
```duck
quack [let count be 5]

quack [while count > 0 do
  quack [print count]
  quack [count becomes count - 1]
]
quack [print "Blast off!"]
```

**Repeat N times:**
```duck
quack [repeat 3 times
  quack [print "Quack!"]
]
-- Output: Quack! Quack! Quack!
```

**For each loops:**
```duck
quack [let names be list("Alice", "Bob", "Charlie")]

quack [for each [name] in names do
  quack [print f"Hello, {name}!"]
]
```

Note the brackets around the loop variable: `[name]`. Yes, you need them.

**Loop over a range:**
```duck
quack [for each [i] in range(1, 6) do
  quack [print i]
]
-- Output: 1 2 3 4 5
```

`range(a, b)` gives you `a` up to but not including `b`.

**Breaking out:**
```duck
quack [while true do
  quack [if should-stop then
    quack [break]
  ]
  quack [do-stuff()]
]
```

**Skipping iterations:**
```duck
quack [for each [n] in range(1, 10) do
  quack [if n == 5 then
    quack [continue]
  ]
  quack [print n]  -- 5 is skipped
]
```

---

## Functions (Reusable Quacks)

Define functions with `define`, `taking`, and `as`:

```duck
quack [define greet taking [name] as
  quack [print f"Hello, {name}!"]
]

quack [greet("World")]  -- Hello, World!
```

**Multiple parameters:**
```duck
quack [define add taking [a, b] as
  quack [return a + b]
]

quack [let sum be add(5, 3)]
quack [print sum]  -- 8
```

**No parameters:**
```duck
quack [define say-hello taking [] as
  quack [print "Hello!"]
]

quack [say-hello()]
```

**Returning values:**
```duck
quack [define square taking [x] as
  quack [return x * x]
]
```

Functions without `return` give you `nil`. Don't forget it.

**Lambdas (anonymous functions):**
```duck
quack [let double be [x] -> x * 2]
quack [print double(5)]  -- 10
```

The syntax is `[params] -> expression`. Single expression only.

**Multi-line lambdas:**
```duck
quack [let process be [x] => [
  quack [let doubled be x * 2]
  quack [let result be doubled + 10]
  quack [return result]
]]
```

Use `=>` with brackets for multi-statement lambdas.

**Closures work:**
```duck
quack [let multiplier be 10]
quack [let multiply be [x] -> x * multiplier]
quack [print multiply(5)]  -- 50
```

---

## Lists (Duck Ponds)

Create lists with the `list()` function:

```duck
quack [let numbers be list(1, 2, 3, 4, 5)]
quack [let empty be list()]
quack [let mixed be list("hello", 42, true)]
```

**Access elements with `at`:**
```duck
quack [print numbers at 0]  -- 1
quack [print numbers at 2]  -- 3
```

Indices start at 0. Zero-indexed like sensible languages.

**Modify elements:**
```duck
quack [numbers at 0 becomes 100]
quack [print numbers]  -- [100, 2, 3, 4, 5]
```

**Add elements:**
```duck
quack [numbers push 6]
quack [print numbers]  -- [100, 2, 3, 4, 5, 6]
```

**Remove elements:**
```duck
quack [let last be pop(numbers)]
quack [print last]     -- 6
quack [print numbers]  -- [100, 2, 3, 4, 5]
```

**Get length:**
```duck
quack [print len(numbers)]     -- 5
quack [print numbers length]   -- Also 5
```

**Other operations:**
```duck
quack [print reverse(list(1, 2, 3))]      -- [3, 2, 1]
quack [print sort(list(3, 1, 4, 1, 5))]   -- [1, 1, 3, 4, 5]
quack [print join(list("a", "b"), "-")]   -- "a-b"
quack [print contains(list(1, 2, 3), 2)]  -- true
```

---

## Structs (Duck Blueprints)

Structs let you create custom data types with named fields.

**Define a struct:**
```duck
quack [struct duck with [name, age, quackiness]]
```

**Create instances:**
```duck
quack [let gerald be duck("Gerald", 5, 100)]
quack [let waddles be duck("Waddles", 3, 85)]
```

**Access fields with dots:**
```duck
quack [print gerald.name]        -- Gerald
quack [print gerald.quackiness]  -- 100
```

**Modify fields:**
```duck
quack [gerald.age becomes 6]
quack [gerald.quackiness becomes gerald.quackiness + 10]
```

**Nested structs:**
```duck
quack [struct pond with [name, depth, resident]]

quack [let central-park be pond("Central Park", 10, gerald)]
quack [print central-park.resident.name]  -- Gerald
```

**Inspect structs:**
```duck
quack [print keys(gerald)]    -- ["name", "age", "quackiness"]
quack [print values(gerald)]  -- ["Gerald", 5, 100]
```

---

## Higher-Order Shenanigans

Functions that take functions. Mind-bending but powerful.

**map - Transform every element:**
```duck
quack [let nums be list(1, 2, 3, 4)]
quack [let doubled be map(nums, [x] -> x * 2)]
quack [print doubled]  -- [2, 4, 6, 8]
```

**filter - Keep matching elements:**
```duck
quack [let nums be list(1, 2, 3, 4, 5, 6)]
quack [let evens be filter(nums, [x] -> x % 2 == 0)]
quack [print evens]  -- [2, 4, 6]
```

**fold - Reduce to single value:**
```duck
quack [let nums be list(1, 2, 3, 4, 5)]
quack [let sum be fold(nums, 0, [acc, x] -> acc + x)]
quack [print sum]  -- 15
```

**find - First matching element:**
```duck
quack [let first-big be find(list(1, 5, 10, 15), [x] -> x > 7)]
quack [print first-big]  -- 10
```

**any - Check if any match:**
```duck
quack [print any(list(1, 2, 3), [x] -> x > 2)]  -- true
```

**all - Check if all match:**
```duck
quack [print all(list(2, 4, 6), [x] -> x % 2 == 0)]  -- true
```

---

## Reading and Writing Files

**Read a file:**
```duck
quack [let content be read-file("data.txt")]
quack [print content]
```

**Write a file (overwrites):**
```duck
quack [write-file("output.txt", "Hello, file!")]
```

**Append to a file:**
```duck
quack [append-file("log.txt", "New log entry\n")]
```

**Check if file exists:**
```duck
quack [if file-exists("config.txt") then
  quack [print "Found config"]
]
```

Note: The goose only allows relative paths. No poking around `/etc/passwd`. Sorry, hackers.

---

## Talking to the Internet

Duck has built-in HTTP support. No libraries needed.

**GET request:**
```duck
quack [let response be http-get("https://api.example.com/data")]
quack [print response.status]  -- 200
quack [print response.body]    -- The response content
```

**GET with headers:**
```duck
quack [let headers be list("Authorization", "Bearer token123")]
quack [let response be http-get("https://api.example.com", headers)]
```

**POST request:**
```duck
quack [let body be "{\"message\": \"hello\"}"]
quack [let headers be list("Content-Type", "application/json")]
quack [let response be http-post("https://api.example.com", body, headers)]
```

**JSON parsing:**
```duck
quack [let data be json-parse(response.body)]
quack [print data.name]  -- Access parsed fields directly
```

**JSON stringify:**
```duck
quack [let json-string be json-stringify(my-struct)]
```

**Base64:**
```duck
quack [print base64-encode("Hello")]  -- SGVsbG8=
quack [print base64-decode("SGVsbG8=")]  -- Hello
```

**For cleaner HTTP, use the `quests` library:**
```duck
quack [migrate "git+konacodes/quests" as quest]

quack [let data be quest.get-json("https://api.example.com/users/1")]
quack [print data.name]
```

---

## WebSockets (Persistent Connections)

For real-time communication, Duck supports WebSockets.

**Connect:**
```duck
quack [let ws be ws-connect("wss://echo.websocket.org")]
quack [print f"Connected! ID: {ws.id}"]
```

**Send a message:**
```duck
quack [ws-send(ws, "Hello, server!")]
```

**Receive a message (blocking):**
```duck
quack [let message be ws-receive(ws)]
quack [print f"Received: {message}"]
```

**Check connection status:**
```duck
quack [if ws-connected(ws) then
  quack [print "Still connected"]
]
```

**Close connection:**
```duck
quack [ws-close(ws)]
```

**With the quests library:**
```duck
quack [migrate "git+konacodes/quests" as quest]

quack [let ws be quest.ws-open("wss://echo.websocket.org")]
quack [let response be quest.ws-request(ws, "ping")]  -- Send and receive
quack [quest.ws-disconnect(ws)]
```

---

## Using Libraries

Duck has a package system. Libraries live on GitHub.

**Install a library:**
```bash
goose install konacodes/quests v1.1.0
```

**Import in your code:**
```duck
-- With namespace (recommended)
quack [migrate "git+konacodes/quests@v1.1.0" as quest]
quack [quest.get("https://example.com")]

-- Without namespace (pollutes global scope)
quack [migrate "git+konacodes/quests"]
quack [get("https://example.com")]
```

**Official libraries:**
- `konacodes/quests` - Clean HTTP and WebSocket requests
- `konacodes/test` - Testing framework
- `konacodes/discord` - Discord bot library

**Make your own library:**

Create a repo with:
```
your-library/
├── metadata.dm   <- Required
├── lib.duck      <- Your code
└── README.md     <- Be nice
```

The `metadata.dm` file:
```dm
[about]
author: 'Your Name'
repo-url: 'https://github.com/you/your-library'
description: 'What it does'
version: 'v1.0.0'

[dependencies]
konacodes/quests v1.0.0

[point to]
./lib.duck
```

---

## When Things Go Wrong

**Try-catch with `attempt` and `rescue`:**
```duck
quack [attempt
  quack [let data be json-parse(bad-json)]
  quack [print data.name]
rescue err
  quack [print f"Something broke: {err}"]
]
```

The `err` variable contains the error message. Do with it what you will.

**Checking for nil:**
```duck
quack [let result be maybe-returns-nil()]
quack [if result == nil then
  quack [print "Got nothing"]
otherwise
  quack [print f"Got: {result}"]
]
```

---

## The Stuff You'll Mess Up

Learn from the collective suffering of everyone who came before you.

**"I forgot to quack"**
```duck
-- Wrong
[print "Hello"]

-- Right
quack [print "Hello"]
```

**"I used = instead of be/becomes"**
```duck
-- Wrong
quack [let x = 42]

-- Right
quack [let x be 42]
quack [x becomes 43]
```

**"I used [0] for indexing"**
```duck
-- Wrong
quack [print list at [0]]

-- Right
quack [print list at 0]
```

**"I used // for comments"**
```duck
-- Wrong
quack [print "Hi"] // comment

-- Right
quack [print "Hi"] -- comment
```

**"I used single quotes"**
```duck
-- Wrong
quack [let x be 'hello']

-- Right
quack [let x be "hello"]
```

**"I forgot the f in f-strings"**
```duck
-- Wrong (prints literal braces)
quack [print "Hello, {name}"]

-- Right
quack [print f"Hello, {name}"]
```

**"I forgot brackets around for-each variable"**
```duck
-- Wrong
quack [for each item in list do ...]

-- Right
quack [for each [item] in list do ...]
```

**"I forgot to return"**
```duck
-- Wrong (returns nil)
quack [define add taking [a, b] as
  quack [let result be a + b]
]

-- Right
quack [define add taking [a, b] as
  quack [return a + b]
]
```

**The Golden Rule:**
When something doesn't work, ask yourself:
1. Did I quack?
2. Did I use `be`/`becomes`?
3. Did I use `at` for indexing?
4. Did I use double quotes?
5. Did I use `--` for comments?

---

## All The Builtins

Everything that comes for free. No imports needed.

### I/O
| Function | Description |
|----------|-------------|
| `print(...)` | Print values to console |
| `input(prompt?)` | Read line from user |

### Type Conversion
| Function | Description |
|----------|-------------|
| `string(x)` | Convert to string |
| `number(x)` | Convert to number |
| `type-of(x)` | Get type name |

### Math
| Function | Description |
|----------|-------------|
| `abs(x)` | Absolute value |
| `floor(x)` | Round down |
| `ceil(x)` | Round up |
| `sqrt(x)` | Square root |
| `pow(x, y)` | x to the power of y |
| `min(...)` | Smallest value |
| `max(...)` | Largest value |
| `random()` | Random 0-1 |
| `range(a, b)` | List of numbers [a, b) |

### Strings
| Function | Description |
|----------|-------------|
| `len(s)` | Length |
| `uppercase(s)` | UPPERCASE |
| `lowercase(s)` | lowercase |
| `trim(s)` | Remove whitespace |
| `split(s, sep)` | Split into list |
| `contains(s, sub)` | Check for substring |
| `reverse(s)` | Reverse string |

### Lists
| Function | Description |
|----------|-------------|
| `list(...)` | Create list |
| `len(list)` | Length |
| `push(list, x)` | Add to end |
| `pop(list)` | Remove from end |
| `reverse(list)` | Reverse copy |
| `sort(list)` | Sorted copy |
| `join(list, sep)` | Join into string |
| `contains(list, x)` | Check membership |

### Higher-Order
| Function | Description |
|----------|-------------|
| `map(list, fn)` | Transform elements |
| `filter(list, fn)` | Keep matches |
| `fold(list, init, fn)` | Reduce to value |
| `find(list, fn)` | First match |
| `any(list, fn)` | Any match? |
| `all(list, fn)` | All match? |

### Structs
| Function | Description |
|----------|-------------|
| `keys(struct)` | Field names |
| `values(struct)` | Field values |

### Files
| Function | Description |
|----------|-------------|
| `read-file(path)` | Read file contents |
| `write-file(path, content)` | Write file |
| `append-file(path, content)` | Append to file |
| `file-exists(path)` | Check existence |

### Web
| Function | Description |
|----------|-------------|
| `http-get(url, headers?)` | GET request |
| `http-post(url, body, headers?)` | POST request |
| `json-parse(string)` | Parse JSON |
| `json-stringify(value)` | To JSON string |
| `base64-encode(string)` | Encode base64 |
| `base64-decode(string)` | Decode base64 |

### WebSocket
| Function | Description |
|----------|-------------|
| `ws-connect(url)` | Connect to WebSocket |
| `ws-send(conn, message)` | Send message |
| `ws-receive(conn)` | Receive message |
| `ws-close(conn)` | Close connection |
| `ws-connected(conn)` | Check if connected |

### System
| Function | Description |
|----------|-------------|
| `env(name)` | Get environment variable |
| `sleep(ms)` | Pause execution |

---

## The End (For Now)

You made it. You now know more about Duck than most people ever will (which is a low bar, but still).

Go build something. Make the goose proud. Or don't. The goose will judge you either way.

```duck
quack [print "Happy quacking!"]
```

*- The Goose*
