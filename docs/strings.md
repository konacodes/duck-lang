# Strings

Text manipulation in Duck. Less exciting than ducks, but occasionally necessary.

## How do I create a string?

Double quotes:

```duck
quack [let message be "Hello, World!"]
```

That's it. No single quotes, no backticks, no template literals. Just double quotes.

## How do I include special characters?

Use escape sequences:

| Escape | Character |
|--------|-----------|
| `\n` | Newline |
| `\t` | Tab |
| `\"` | Double quote |
| `\\` | Backslash |
| `\{` | Literal { (in f-strings) |
| `\}` | Literal } (in f-strings) |

```duck
quack [print "Line 1\nLine 2"]
quack [print "She said \"hello\""]
```

## How do I do string interpolation?

Use f-strings with curly braces:

```duck
quack [let name be "Gerald"]
quack [let age be 5]

quack [print f"Hello, {name}! You are {age} years old."]
-- Hello, Gerald! You are 5 years old.
```

The `f` before the opening quote is mandatory. Any expression works inside the braces:

```duck
quack [print f"2 + 2 = {2 + 2}"]
quack [print f"Uppercase: {uppercase(name)}"]
quack [print f"List length: {len(my-list)}"]
```

## How do I concatenate strings?

Use `+`:

```duck
quack [let greeting be "Hello, " + "World!"]
quack [print greeting]  -- Hello, World!
```

You can chain them:

```duck
quack [let full be "Hello" + " " + name + "!"]
```

But honestly, f-strings are usually cleaner.

## How do I get the length?

Use `len()`:

```duck
quack [let message be "Hello"]
quack [print len(message)]  -- 5
```

## How do I convert to uppercase/lowercase?

```duck
quack [let text be "Hello World"]

quack [print uppercase(text)]  -- HELLO WORLD
quack [print lowercase(text)]  -- hello world
```

## How do I trim whitespace?

```duck
quack [let messy be "  hello  "]
quack [let clean be trim(messy)]
quack [print clean]  -- "hello"
```

## How do I split a string?

Use `split()`:

```duck
quack [let csv be "apple,banana,cherry"]
quack [let fruits be split(csv, ",")]
quack [print fruits]  -- ["apple", "banana", "cherry"]
```

## How do I join a list into a string?

Use `join()`:

```duck
quack [let words be list("hello", "world")]
quack [let sentence be join(words, " ")]
quack [print sentence]  -- "hello world"
```

## How do I check if a string contains something?

Use `contains()`:

```duck
quack [let text be "Hello, World!"]

quack [print contains(text, "World")]   -- true
quack [print contains(text, "Quack")]   -- false
```

## How do I access individual characters?

Use `at` (same as lists):

```duck
quack [let text be "Hello"]
quack [print text at 0]  -- H
quack [print text at 4]  -- o
```

## How do I reverse a string?

Use `reverse()`:

```duck
quack [let text be "Hello"]
quack [let backwards be reverse(text)]
quack [print backwards]  -- olleH
```

## How do I convert other types to strings?

Use `string()`:

```duck
quack [print string(42)]     -- "42"
quack [print string(3.14)]   -- "3.14"
quack [print string(true)]   -- "true"
quack [print string(nil)]    -- "null"
```

## How do I convert strings to numbers?

Use `number()`:

```duck
quack [let n be number("42")]
quack [print n + 8]  -- 50
```

Fails if the string isn't a valid number:

```duck
quack [let n be number("not a number")]  -- Error!
```

## Quick Reference

| Function | Description |
|----------|-------------|
| `len(s)` | Length of string |
| `uppercase(s)` | Convert to uppercase |
| `lowercase(s)` | Convert to lowercase |
| `trim(s)` | Remove leading/trailing whitespace |
| `split(s, sep)` | Split into list |
| `join(list, sep)` | Join list into string |
| `contains(s, sub)` | Check for substring |
| `reverse(s)` | Reverse string |
| `string(x)` | Convert to string |
| `number(s)` | Convert to number |
| `s at i` | Get character at index |
| `f"..."` | String interpolation |
