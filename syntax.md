# Duck Language Syntax Reference

Quick reference for Duck language syntax. Every code block must be preceded by `quack` to execute.

## Core Syntax

| Concept | Syntax | Example |
|---------|--------|---------|
| Execute block | `quack [...]` | `quack [print "Hello"]` |
| Variable declaration | `let name be value` | `quack [let x be 42]` |
| Variable assignment | `name becomes value` | `quack [x becomes x + 1]` |
| Comments | `-- comment` | `-- This is a comment` |

## Data Types

| Type | Syntax | Example |
|------|--------|---------|
| Number | `42`, `3.14` | `quack [let n be 42]` |
| String | `"text"` | `quack [let s be "hello"]` |
| Boolean | `true`, `false` | `quack [let b be true]` |
| Null | `nil` | `quack [let n be nil]` |
| List | `list(...)` or `[a, b, c]` | `quack [let l be list(1, 2, 3)]` |

## Strings

### Regular Strings
Regular strings treat `{` and `}` as literal characters (great for JSON!):
```duck
quack [let json be "{\"key\": \"value\"}"]
quack [print json]  -- {"key": "value"}
```

### F-Strings (Interpolated)
F-strings use `f"..."` prefix to enable `{expr}` interpolation:
```duck
quack [let name be "World"]
quack [print f"Hello, {name}!"]      -- Hello, World!
quack [print f"Sum: {1 + 2}"]        -- Sum: 3
```

To include literal braces in f-strings, escape them: `\{` and `\}`

## Operators

| Category | Operators |
|----------|-----------|
| Arithmetic | `+`, `-`, `*`, `/`, `%` |
| Comparison | `==`, `!=`, `<`, `<=`, `>`, `>=` |
| Logical | `and`, `or`, `not` |
| String | `+` (concat) |

## Control Flow

### If/Otherwise
```duck
quack [if condition then
  quack [print "true"]
otherwise
  quack [print "false"]
]
```

### While Loop
```duck
quack [while x > 0 do
  quack [print x]
  quack [x becomes x - 1]
]
```

### Repeat Loop
```duck
quack [repeat 5 times
  quack [print "quack!"]
]
```

### For Each Loop
```duck
quack [for each [item] in my-list do
  quack [print item]
]
```

### Match Statement
```duck
quack [match value with
  [when 1 then quack [print "one"]]
  [when 2 then quack [print "two"]]
  [when _ then quack [print "other"]]
]
```

## Functions

### Definition
```duck
quack [define greet taking [name] as
  quack [print "Hello, {name}!"]
]

quack [define add taking [a, b] as
  quack [return a + b]
]
```

### Lambdas
```duck
quack [let double be [x] -> x * 2]
quack [let add be [x, y] -> x + y]
```

## Structs

```duck
quack [struct person with [name, age]]
quack [let p be person("Alice", 30)]
quack [print p.name]              -- Field access
quack [p.age becomes 31]          -- Field assignment
```

## Lists

| Operation | Syntax | Example |
|-----------|--------|---------|
| Create | `list(...)` | `quack [let l be list(1, 2, 3)]` |
| Access | `list at index` | `quack [print l at 0]` |
| Push | `list push value` | `quack [l push 4]` |
| Pop | `pop(list)` | `quack [let x be pop(l)]` |
| Length | `len(list)` | `quack [print len(l)]` |

## Error Handling

```duck
quack [attempt
  quack [let data be read-file("file.txt")]
rescue err
  quack [print "Error: {err}"]
]
```

## Module Import

```duck
quack [migrate "path/to/file.duck"]           -- Import into current scope
quack [migrate "utils.duck" as utils]         -- Import with namespace
quack [let result be utils.some-function()]   -- Use namespaced function
```

## Assertions

```duck
quack [honk x > 0]                    -- Assert condition
quack [honk x > 0 "x must be positive"] -- With message
```

## Built-in Functions

### I/O
- `print(value)` - Print to stdout
- `input([prompt])` - Read from stdin

### Math
- `floor(n)`, `ceil(n)`, `abs(n)`, `sqrt(n)`
- `pow(base, exp)`, `min(...)`, `max(...)`
- `random()` - Random float 0.0-1.0

### String
- `len(str)` - Length
- `split(str, sep)` - Split into list
- `join(list, sep)` - Join list to string
- `trim(str)`, `uppercase(str)`, `lowercase(str)`
- `contains(str, substr)` - Check substring

### List
- `len(list)`, `push(list, val)`, `pop(list)`
- `reverse(list)`, `sort(list)`
- `contains(list, val)`, `range(start, end)`

### Higher-Order Functions
- `map(list, fn)` - Transform each element
- `filter(list, fn)` - Keep matching elements
- `fold(list, init, fn)` - Reduce to single value
- `find(list, fn)` - First matching element
- `any(list, fn)` - Check if any match
- `all(list, fn)` - Check if all match

### Type Conversion
- `type-of(val)` - Get type name
- `string(val)` - Convert to string
- `number(val)` - Convert to number

### File I/O
- `read-file(path)` - Read file contents
- `write-file(path, content)` - Write to file
- `append-file(path, content)` - Append to file
- `file-exists(path)` - Check if file exists

### JSON
- `json-parse(str)` - Parse JSON string
- `json-stringify(val)` - Convert to JSON

### HTTP
- `http-get(url, [headers])` - GET request
- `http-post(url, body, [headers])` - POST request

### Other
- `env(name)` - Get environment variable
- `sleep(ms)` - Sleep for milliseconds
- `base64-encode(str)`, `base64-decode(str)`
- `keys(struct)`, `values(struct)`

## Pre-defined Constants

- `PI` - 3.14159...
- `E` - 2.71828...
- `TAU` - 6.28318...
- `quack-args` - Command-line arguments

## Important Notes

1. **Every block needs `quack`** - Even inside functions and loops
2. **Use `be` for declaration** - Not `=`
3. **Use `becomes` for assignment** - Not `=`
4. **Hyphens in identifiers** - `my-variable` is valid
5. **Use `at` for indexing** - `list at 0` not `list[0]`
6. **Comments use `--`** - Not `//` or `#`
