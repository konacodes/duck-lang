# Common Mistakes

Things that will make the goose honk angrily at you. Learn from others' suffering.

---

## "I forgot to quack"

**The Problem:**
```duck
[print "Hello"]  -- Nothing happens
```

**The Fix:**
```duck
quack [print "Hello"]  -- Works!
```

Every block needs a `quack`. Yes, every single one. Yes, even that one.

---

## "I used = instead of be/becomes"

**The Problem:**
```duck
quack [let x = 42]      -- Error!
quack [x = x + 1]       -- Error!
```

**The Fix:**
```duck
quack [let x be 42]        -- Declaration
quack [x becomes x + 1]    -- Assignment
```

Duck uses natural language keywords. `be` for initial values, `becomes` for changes.

---

## "I forgot to quack inside loops"

**The Problem:**
```duck
quack [while x > 0 do
  [print x]              -- Skipped!
  [x becomes x - 1]      -- Skipped!
]
```

**The Fix:**
```duck
quack [while x > 0 do
  quack [print x]
  quack [x becomes x - 1]
]
```

Blocks inside other blocks still need their own quacks.

---

## "I used [0] for list indexing"

**The Problem:**
```duck
quack [let nums be list(1, 2, 3)]
quack [print nums[0]]    -- Error!
```

**The Fix:**
```duck
quack [let nums be list(1, 2, 3)]
quack [print nums at 0]  -- Works!
```

Duck uses `at` for indexing, not brackets.

---

## "I called a function without storing the result"

**The Problem:**
```duck
quack [my-function("arg")]  -- Sometimes causes parse errors
```

**The Fix:**
```duck
quack [let result be my-function("arg")]
```

Function calls that return values often need to be assigned to a variable.

---

## "I used // for comments"

**The Problem:**
```duck
quack [print "Hello"]  // This breaks!
```

**The Fix:**
```duck
quack [print "Hello"]  -- This works!
```

Duck uses `--` for comments, not `//`.

---

## "I used single quotes for strings"

**The Problem:**
```duck
quack [let name be 'Gerald']  -- Error!
```

**The Fix:**
```duck
quack [let name be "Gerald"]  -- Works!
```

Duck only uses double quotes for strings.

---

## "I forgot the f before interpolated strings"

**The Problem:**
```duck
quack [let name be "Gerald"]
quack [print "Hello, {name}!"]  -- Prints literally: Hello, {name}!
```

**The Fix:**
```duck
quack [print f"Hello, {name}!"]  -- Prints: Hello, Gerald!
```

The `f` prefix is required for string interpolation.

---

## "I used elif/else if"

**The Problem:**
```duck
quack [if x > 10 then
  quack [print "big"]
elif x > 5 then          -- Error!
  quack [print "medium"]
]
```

**The Fix:**
```duck
quack [if x > 10 then
  quack [print "big"]
otherwise
  quack [if x > 5 then
    quack [print "medium"]
  otherwise
    quack [print "small"]
  ]
]
```

Duck uses `otherwise`, and you need to nest for chained conditions.

---

## "I used _ as a throwaway variable"

**The Problem:**
```duck
quack [let _ be some-function()]  -- Error!
```

**The Fix:**
```duck
quack [let unused be some-function()]
```

Underscore isn't a valid variable name. Use a descriptive name instead.

---

## "I tried to use break/continue without a loop"

**The Problem:**
```duck
quack [if something then
  quack [break]  -- Error: not in a loop
]
```

**The Fix:**
```duck
quack [while true do
  quack [if something then
    quack [break]  -- Works!
  ]
]
```

`break` and `continue` only work inside loops.

---

## "I forgot brackets around for-each variables"

**The Problem:**
```duck
quack [for each item in list do  -- Error!
  quack [print item]
]
```

**The Fix:**
```duck
quack [for each [item] in list do
  quack [print item]
]
```

Loop variables need `[brackets]`.

---

## "I used absolute paths for file operations"

**The Problem:**
```duck
quack [let content be read-file("/etc/passwd")]  -- Error!
```

**The Fix:**
```duck
quack [let content be read-file("local/data.txt")]  -- Works!
```

The goose only allows relative paths. No filesystem adventures.

---

## "I tried to access a field that doesn't exist"

**The Problem:**
```duck
quack [struct duck with [name, age]]
quack [let d be duck("Gerald", 5)]
quack [print d.species]  -- Error: no such field
```

**The Fix:**

Only access fields you defined:
```duck
quack [print d.name]  -- Works!
```

Or define the field:
```duck
quack [struct duck with [name, age, species]]
```

---

## "I forgot return in a function"

**The Problem:**
```duck
quack [define add taking [a, b] as
  quack [let result be a + b]
  -- Forgot to return!
]

quack [let sum be add(1, 2)]
quack [print sum]  -- Prints nil
```

**The Fix:**
```duck
quack [define add taking [a, b] as
  quack [let result be a + b]
  quack [return result]
]
```

Functions without explicit `return` give you `nil`.

---

## "I passed the wrong number of arguments"

**The Problem:**
```duck
quack [define greet taking [first, last] as
  quack [print f"Hello, {first} {last}"]
]

quack [let unused be greet("Gerald")]  -- Error: missing argument
```

**The Fix:**
```duck
quack [let unused be greet("Gerald", "Duck")]
```

Pass exactly the number of arguments the function expects.

---

## "I mixed up and/or precedence"

**The Problem:**
```duck
quack [if a or b and c then ...]  -- What does this mean?
```

**The Fix:**
```duck
quack [if a or (b and c) then ...]  -- Explicit
quack [if (a or b) and c then ...]  -- Also explicit
```

When in doubt, use parentheses.

---

## Quick Reference: Duck Syntax

| Wrong | Right |
|-------|-------|
| `x = 42` | `let x be 42` |
| `x = x + 1` | `x becomes x + 1` |
| `list[0]` | `list at 0` |
| `// comment` | `-- comment` |
| `'string'` | `"string"` |
| `"Hello {x}"` | `f"Hello {x}"` |
| `elif` | Nested `otherwise` + `if` |
| `_` | Named variable |

---

## The Golden Rule

When in doubt:
1. Did you quack?
2. Are you using `be`/`becomes`?
3. Are you using `at` for indexing?
4. Did you use `--` for comments?

If yes to all, check the error message. The goose is mean, but usually helpful.
