# Duck 🦆

A programming language where you have to say "quack" or the goose won't run your code.

No, seriously.

## Quick Example

```duck
quack
[let greeting be "Hello, World!"]

quack
[print greeting]
```

Without the quacks? The goose ignores you.

```duck
[print "I will never run"]
-- Goose: "I see a block, but I didn't hear a quack. I'm not doing that."
```

## Installation

```bash
cargo build --release
```

The binary is called `goose`. Because obviously.

## Usage

```bash
# Run a file
goose run program.duck

# Run with arguments (accessible via quack-args)
goose run program.duck arg1 arg2 arg3

# Check for missing quacks
goose check program.duck

# Interactive mode
goose repl
```

## The Language

**Variables**
```duck
quack [let x be 42]
quack [x becomes x + 1]
```

**Strings** (with interpolation!)
```duck
quack [let name be "Duck"]
quack [print "Hello, {name}!"]
```

**Math Constants**
```duck
quack [print PI]   -- 3.141592653589793
quack [print E]    -- 2.718281828459045
quack [print TAU]  -- 6.283185307179586
```

**Loops**
```duck
quack [while x > 0 do
  quack [print x]
  quack [x becomes x - 1]
]
```

**Functions**
```duck
quack [define greet taking [name] as
  quack [print "Hello, {name}!"]
]

quack [greet("World")]
```

**Assertions** (honk!)
```duck
quack [let x be 5]
quack [honk x > 0]                    -- Passes silently
quack [honk x < 0 "x must be negative"]  -- HONK! x must be negative
```

**Multiple quacks** = multiple blocks authorized
```duck
quack quack quack
[let a be 1]
[let b be 2]
[let c be 3]
```

## The Goose

The goose has opinions about your code:

- Forgets to quack? *"The audacity of an unquacked block. Truly remarkable."*
- Division by zero? *"I'm not falling for that."*
- Failed assertion? *"HONK! The goose trusted you. The goose was betrayed."*
- Program works? *"I'm as surprised as you are."*

At the end, it rates your code from 1-10. Good luck getting a 10.

## Built-in Functions

**I/O**
- `print` - Print values
- `input` - Read from stdin

**Math**
- `floor`, `ceil`, `abs`, `sqrt`, `pow`, `min`, `max`, `random`

**Type Conversion**
- `string`, `number`, `type-of`

**Lists**
- `len`, `push`, `pop`, `range`, `reverse`, `sort`, `contains`

**Strings**
- `len`, `split`, `join`, `trim`, `uppercase`, `lowercase`, `contains`, `reverse`

**Files**
- `read-file`, `write-file`, `append-file`, `file-exists`

**Structs**
- `keys`, `values`

**Misc**
- `sleep` - Pause execution (milliseconds)

## Command-Line Arguments

Access arguments via `quack-args`:

```duck
-- greet.duck
quack [for each [name] in quack-args do
  quack [print "Hello, {name}!"]
]
```

```bash
$ goose run greet.duck Alice Bob
Hello, Alice!
Hello, Bob!
```

## File Extension

`.duck` obviously.

## Why?

Why not?

---

*Honk.* 🪿
