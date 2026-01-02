# Structs and Lists

Duck gives you two ways to collect data: lists (ordered sequences) and structs (named fields). Let's explore both.

---

# Lists

## How do I create a list?

Use the `list()` function:

```duck
quack [let empty be list()]
quack [let numbers be list(1, 2, 3, 4, 5)]
quack [let mixed be list("hello", 42, true, nil)]
```

Lists can hold any type, and you can mix types.

## How do I access an element?

Use `at`:

```duck
quack [let fruits be list("apple", "banana", "cherry")]
quack [print fruits at 0]  -- apple
quack [print fruits at 1]  -- banana
quack [print fruits at 2]  -- cherry
```

Indices start at 0, like a civilized language.

## How do I change an element?

Use `at` on the left side:

```duck
quack [let numbers be list(1, 2, 3)]
quack [numbers at 1 becomes 99]
quack [print numbers]  -- [1, 99, 3]
```

## How do I get the length?

Two options:

```duck
quack [let numbers be list(1, 2, 3)]

-- Option 1: len() function
quack [print len(numbers)]  -- 3

-- Option 2: length property
quack [print numbers length]  -- 3
```

## How do I add elements?

Use `push`:

```duck
quack [let numbers be list(1, 2, 3)]
quack [numbers push 4]
quack [numbers push 5]
quack [print numbers]  -- [1, 2, 3, 4, 5]
```

Or use the `push()` function:

```duck
quack [let unused be push(numbers, 6)]
```

## How do I remove elements?

Use `pop` to remove the last element:

```duck
quack [let numbers be list(1, 2, 3)]
quack [let last be pop(numbers)]
quack [print last]     -- 3
quack [print numbers]  -- [1, 2]
```

## How do I loop over a list?

Use `for each`:

```duck
quack [let names be list("Alice", "Bob", "Charlie")]

quack [for each [name] in names do
  quack [print f"Hello, {name}!"]
]
```

## What operations can I do on lists?

### reverse

```duck
quack [let nums be list(1, 2, 3)]
quack [let reversed be reverse(nums)]
quack [print reversed]  -- [3, 2, 1]
```

### sort

```duck
quack [let nums be list(3, 1, 4, 1, 5)]
quack [let sorted be sort(nums)]
quack [print sorted]  -- [1, 1, 3, 4, 5]
```

### join

Combine list elements into a string:

```duck
quack [let words be list("hello", "world")]
quack [let sentence be join(words, " ")]
quack [print sentence]  -- "hello world"
```

### contains

Check if a list contains a value:

```duck
quack [let nums be list(1, 2, 3)]
quack [print contains(nums, 2)]  -- true
quack [print contains(nums, 5)]  -- false
```

---

# Structs

## How do I define a struct?

Use `struct` and `with`:

```duck
quack [struct duck with [name, age, quackiness]]
```

This creates a new type called `duck` with three fields.

## How do I create an instance?

Call the struct like a function:

```duck
quack [struct duck with [name, age, quackiness]]
quack [let gerald be duck("Gerald", 5, 100)]
```

Arguments are matched to fields in order.

## How do I access fields?

Use dot notation:

```duck
quack [print gerald.name]       -- Gerald
quack [print gerald.age]        -- 5
quack [print gerald.quackiness] -- 100
```

## How do I modify fields?

Use `becomes` with dot notation:

```duck
quack [gerald.age becomes 6]
quack [gerald.quackiness becomes gerald.quackiness + 10]
quack [print gerald.age]  -- 6
```

## Can structs contain other structs?

Absolutely:

```duck
quack [struct duck with [name, age]]
quack [struct pond with [name, depth, favorite-duck]]

quack [let gerald be duck("Gerald", 5)]
quack [let my-pond be pond("Central Park", 10, gerald)]

quack [print my-pond.favorite-duck.name]  -- Gerald
```

## Can structs contain lists?

Yes:

```duck
quack [struct pond with [name, ducks]]

quack [let my-pond be pond("Central Park", list())]
quack [my-pond.ducks push duck("Gerald", 5)]
quack [my-pond.ducks push duck("Waddles", 3)]

quack [print len(my-pond.ducks)]  -- 2
```

## How do I get all fields?

Use `keys()` and `values()`:

```duck
quack [struct duck with [name, age]]
quack [let gerald be duck("Gerald", 5)]

quack [print keys(gerald)]    -- ["name", "age"]
quack [print values(gerald)]  -- ["Gerald", 5]
```

## Quick Reference

### Lists

| Syntax | Meaning |
|--------|---------|
| `list(a, b, c)` | Create list |
| `list at 0` | Access element |
| `list at 0 becomes x` | Modify element |
| `len(list)` or `list length` | Get length |
| `list push x` | Add to end |
| `pop(list)` | Remove from end |
| `reverse(list)` | Reverse list |
| `sort(list)` | Sort list |
| `join(list, sep)` | Join into string |
| `contains(list, x)` | Check membership |

### Structs

| Syntax | Meaning |
|--------|---------|
| `struct T with [a, b, c]` | Define struct |
| `T(v1, v2, v3)` | Create instance |
| `s.field` | Access field |
| `s.field becomes x` | Modify field |
| `keys(s)` | Get field names |
| `values(s)` | Get field values |
