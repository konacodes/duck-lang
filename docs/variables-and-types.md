# Variables and Types

Duck has variables. Revolutionary, I know. But we do them *slightly* differently.

## How do I create a variable?

Use `let` and `be`:

```duck
quack [let x be 42]
quack [let name be "Gerald"]
quack [let is-duck be true]
```

Not `=`. Not `:=`. Just `be`. The goose appreciates natural language.

## How do I change a variable?

Use `becomes`:

```duck
quack [let x be 10]
quack [x becomes x + 1]
quack [print x]  -- 11
```

Again, not `=`. The word `becomes` makes it clear that something is changing.

```duck
-- Declaration: "let x BE 42" (x is 42)
-- Assignment: "x BECOMES 43" (x changes to 43)
```

## Can I use hyphens in variable names?

Yes! This is actually one of Duck's nicer features:

```duck
quack [let my-awesome-variable be 100]
quack [let user-name be "Gerald"]
quack [let is-logged-in be false]
```

No camelCase or snake_case needed. Just write naturally.

## What types exist?

### Numbers

All numbers are floating-point (like JavaScript, but we don't talk about that):

```duck
quack [let age be 25]
quack [let pi be 3.14159]
quack [let negative be -42]
```

### Strings

Double-quoted text:

```duck
quack [let greeting be "Hello, World!"]
```

### Booleans

`true` or `false`:

```duck
quack [let is-duck be true]
quack [let is-goose be false]
```

### Null

The absence of a value:

```duck
quack [let nothing be nil]
```

### Lists

Ordered collections:

```duck
quack [let numbers be list(1, 2, 3)]
quack [let mixed be list("a", 42, true)]
```

See [Lists and Structs](./structs-and-lists.md) for more.

### Structs

Custom data types:

```duck
quack [struct duck with [name, age]]
quack [let gerald be duck("Gerald", 5)]
```

See [Lists and Structs](./structs-and-lists.md) for more.

## How do I check a variable's type?

Use `type-of()`:

```duck
quack [print type-of(42)]        -- "number"
quack [print type-of("hello")]   -- "string"
quack [print type-of(true)]      -- "boolean"
quack [print type-of(list())]    -- "list"
```

## How do I convert between types?

### To String

```duck
quack [let s be string(42)]      -- "42"
quack [let s be string(true)]    -- "true"
```

### To Number

```duck
quack [let n be number("42")]    -- 42
quack [let n be number(true)]    -- 1
quack [let n be number(false)]   -- 0
```

## String Interpolation

Put variables inside strings with `{curly braces}`:

```duck
quack [let name be "Gerald"]
quack [let age be 5]
quack [print f"Hello, {name}! You are {age} years old."]
-- Hello, Gerald! You are 5 years old.
```

Note the `f` before the string. That's what enables interpolation.

You can put any expression inside the braces:

```duck
quack [print f"2 + 2 = {2 + 2}"]
quack [print f"List length: {len(my-list)}"]
```

To print a literal `{`, escape it:

```duck
quack [print f"Use \{braces\} like this"]
```

## Scope

Variables exist within their block and any nested blocks:

```duck
quack [let x be 10]
quack [if true then
  quack [let y be 20]    -- y exists here
  quack [print x]        -- x is accessible
]
-- y doesn't exist out here, x does
```

Function parameters are scoped to the function:

```duck
quack [define greet taking [name] as
  quack [print f"Hello, {name}"]  -- name exists here
]
-- name doesn't exist out here
```

## Quick Reference

| Syntax | Meaning |
|--------|---------|
| `let x be 42` | Create variable |
| `x becomes 43` | Change variable |
| `type-of(x)` | Get type name |
| `string(x)` | Convert to string |
| `number(x)` | Convert to number |
| `f"Hello {x}"` | String interpolation |
