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

**Loops**
```duck
quack [while x > 0 do
  quack [print x]
  quack [x becomes x - 1]
]
```

**Functions**
```duck
quack [define greet taking name as
  quack [print "Hello, {name}!"]
]

quack [greet("World")]
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
- Program works? *"I'm as surprised as you are."*

At the end, it rates your code from 1-10. Good luck getting a 10.

## Built-in Functions

`print`, `input`, `len`, `push`, `pop`, `range`, `random`, `floor`, `ceil`, `abs`, `sqrt`, `pow`, `min`, `max`, `type-of`, `string`, `number`

## File Extension

`.duck` obviously.

## Why?

Why not?

---

*Honk.* 🪿
