# Functions and Lambdas

Functions let you reuse code. Lambdas let you pass code around. Both involve more quacking than you'd expect.

## How do I define a function?

Use `define`, `taking`, and `as`:

```duck
quack [define greet taking [name] as
  quack [print f"Hello, {name}!"]
]
```

Breaking it down:
- `define greet` - We're creating a function called "greet"
- `taking [name]` - It accepts one parameter called "name"
- `as` - Here's what it does
- The rest is the function body

## How do I call a function?

Two options:

```duck
-- Option 1: Bracket syntax
quack [let result be greet("World")]

-- Option 2: Parentheses syntax
quack [let result be greet("World")]
```

Both work. Use whichever reads better.

## How do I return a value?

Use `return`:

```duck
quack [define add taking [a, b] as
  quack [return a + b]
]

quack [let sum be add(5, 3)]
quack [print sum]  -- 8
```

Functions without an explicit return give you `nil`.

## Can I have multiple parameters?

Yes, separate them with commas:

```duck
quack [define greet-person taking [first-name, last-name, age] as
  quack [print f"Hello, {first-name} {last-name}!"]
  quack [print f"You are {age} years old."]
]

quack [let unused be greet-person("Gerald", "Duck", 5)]
```

## Can I have no parameters?

Yes, just use empty brackets:

```duck
quack [define say-hello taking [] as
  quack [print "Hello!"]
]

quack [let unused be say-hello()]
```

## How do I return early?

Just use `return` wherever:

```duck
quack [define check-age taking [age] as
  quack [if age < 0 then
    quack [print "Invalid age!"]
    quack [return nil]
  ]
  quack [print f"Age is {age}"]
]
```

## What are lambdas?

Lambdas are anonymous functions. Quick, inline, no `define` needed:

```duck
quack [let double be [x] -> x * 2]

quack [print double(5)]   -- 10
quack [print double(21)]  -- 42
```

The syntax is `[params] -> expression`.

## Can lambdas have multiple parameters?

Yes:

```duck
quack [let add be [a, b] -> a + b]
quack [print add(3, 4)]  -- 7
```

## Can lambdas have multiple lines?

Not directly. Lambdas are single-expression functions. For multiple lines, use a regular function.

```duck
-- This is a lambda: single expression
quack [let square be [x] -> x * x]

-- This needs a regular function
quack [define complex-thing taking [x] as
  quack [let a be x * 2]
  quack [let b be a + 10]
  quack [return b * b]
]
```

## How do I pass functions as arguments?

Just pass them by name:

```duck
quack [define apply-twice taking [f, x] as
  quack [return f(f(x))]
]

quack [let double be [x] -> x * 2]
quack [let result be apply-twice(double, 5)]
quack [print result]  -- 20
```

## What higher-order functions exist?

Duck has several built-in:

### map

Transform every element in a list:

```duck
quack [let numbers be list(1, 2, 3, 4, 5)]
quack [let doubled be map(numbers, [x] -> x * 2)]
quack [print doubled]  -- [2, 4, 6, 8, 10]
```

### filter

Keep only elements that pass a test:

```duck
quack [let numbers be list(1, 2, 3, 4, 5, 6)]
quack [let evens be filter(numbers, [x] -> x % 2 == 0)]
quack [print evens]  -- [2, 4, 6]
```

### fold

Reduce a list to a single value:

```duck
quack [let numbers be list(1, 2, 3, 4, 5)]
quack [let sum be fold(numbers, 0, [acc, x] -> acc + x)]
quack [print sum]  -- 15
```

### find

Find the first matching element:

```duck
quack [let numbers be list(1, 2, 3, 4, 5)]
quack [let first-even be find(numbers, [x] -> x % 2 == 0)]
quack [print first-even]  -- 2
```

### any

Check if any element passes a test:

```duck
quack [let numbers be list(1, 3, 5, 7)]
quack [let has-even be any(numbers, [x] -> x % 2 == 0)]
quack [print has-even]  -- false
```

### all

Check if all elements pass a test:

```duck
quack [let numbers be list(2, 4, 6, 8)]
quack [let all-even be all(numbers, [x] -> x % 2 == 0)]
quack [print all-even]  -- true
```

## Do closures work?

Yes! Lambdas capture variables from their enclosing scope:

```duck
quack [let multiplier be 10]
quack [let multiply be [x] -> x * multiplier]

quack [print multiply(5)]  -- 50
```

## Quick Reference

| Syntax | Meaning |
|--------|---------|
| `define f taking [x] as ...` | Define function |
| `return value` | Return from function |
| `[x] -> expr` | Lambda (anonymous function) |
| `map(list, fn)` | Transform all elements |
| `filter(list, fn)` | Keep matching elements |
| `fold(list, init, fn)` | Reduce to single value |
| `find(list, fn)` | Find first match |
| `any(list, fn)` | Check if any match |
| `all(list, fn)` | Check if all match |
