# Duck Language Documentation

Welcome to the official Duck documentation. May your quacks be plentiful.

## Start Here

- **[Getting Started](./getting-started.md)** - Install Duck and run your first program
- **[The Quack System](./syntax.md)** - Understanding Duck's core concept

## Language Basics

- **[Variables and Types](./variables-and-types.md)** - Creating and using variables
- **[Control Flow](./control-flow.md)** - If/else, loops, and branching
- **[Functions](./functions.md)** - Defining functions and using lambdas
- **[Structs and Lists](./structs-and-lists.md)** - Data structures
- **[Strings](./strings.md)** - Text manipulation

## Built-in Features

- **[Built-in Functions](./builtins.md)** - Complete reference of all built-ins
- **[File I/O](./file-io.md)** - Reading and writing files
- **[HTTP and JSON](./http-and-json.md)** - Web requests and data parsing

## Ecosystem

- **[Libraries](./libraries.md)** - Using and creating libraries
- **[The Goose CLI](./cli.md)** - Command-line tools

## Troubleshooting

- **[Common Mistakes](./common-mistakes.md)** - Learn from others' suffering

---

## Quick Reference

### The Essentials

```duck
quack [let x be 42]              -- Create variable
quack [x becomes x + 1]          -- Change variable
quack [print f"Value: {x}"]      -- Print with interpolation
```

### Functions

```duck
quack [define add taking [a, b] as
  quack [return a + b]
]
```

### Lambdas

```duck
quack [let double be [x] -> x * 2]
```

### Control Flow

```duck
quack [if condition then
  quack [do-thing()]
otherwise
  quack [do-other-thing()]
]

quack [while x > 0 do
  quack [x becomes x - 1]
]

quack [for each [item] in list do
  quack [print item]
]
```

### Data Structures

```duck
quack [struct duck with [name, age]]
quack [let d be duck("Gerald", 5)]
quack [print d.name]

quack [let nums be list(1, 2, 3)]
quack [print nums at 0]
```

---

## Getting Help

- **Check the code**: Run `goose check file.duck` to find quack issues
- **Read the errors**: The goose is sassy but informative
- **Check common mistakes**: Most errors are covered in our troubleshooting guide

Happy quacking!
