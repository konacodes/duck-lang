# Libraries

Duck has a package system. You can use libraries other people made, and you can make your own. The goose approves of code reuse.

## How do I install a library?

Use `goose install`:

```bash
goose install konacodes/discord v0.1.0
```

This clones the library from GitHub to `~/.duck/libs/konacodes/discord/v0.1.0/`.

The format is `goose install user/repo version`.

## How do I see what's installed?

```bash
goose libs
```

This lists all installed libraries with their versions and descriptions.

## How do I use an installed library?

Use `migrate` with the `git+` prefix:

```duck
quack [migrate "git+konacodes/discord" as dc]

-- Now use the library
quack [let unused be dc.discord-send(token, channel, "Hello!")]
```

The `as dc` part creates a namespace. All the library's functions and variables are accessed through `dc.`.

## Can I import without a namespace?

Yes, just leave out `as`:

```duck
quack [migrate "git+konacodes/discord"]

-- Functions are now global
quack [let unused be discord-send(token, channel, "Hello!")]
```

This imports everything directly into your scope. Convenient, but can cause name collisions.

## How do I specify a version?

Add `@version` to the path:

```duck
quack [migrate "git+konacodes/discord@v0.1.0" as dc]
```

If you don't specify, it uses whatever's installed as `main`.

## How do I import a local file?

Just use a regular path:

```duck
quack [migrate "./helpers.duck" as h]
quack [print h.some-function()]
```

Or without namespace:

```duck
quack [migrate "./helpers.duck"]
```

---

# Creating Your Own Library

So you want to share your code with the world. The goose respects that.

## What files do I need?

At minimum:
- `lib.duck` - Your actual code
- `metadata.dm` - Library metadata

Optionally:
- `README.md` - Documentation

## What goes in metadata.dm?

```dm
[about]
author: 'your-github-username'
repo-url: 'https://github.com/yourname/your-library'
description: 'A brief description of what this does'
version: 'v0.1.0'

[point to]
./lib.duck
```

The `[point to]` section tells the goose which file to load when someone imports your library.

## Example Library Structure

```
my-awesome-lib/
├── lib.duck          # Main code
├── metadata.dm       # Metadata
├── README.md         # Documentation
└── examples/         # Optional examples
    └── demo.duck
```

## Example lib.duck

```duck
-- My Awesome Library
-- Provides utilities for doing awesome things

-- A constant
quack [let AWESOME-VERSION be "1.0.0"]

-- A function
quack [define do-awesome-thing taking [x] as
  quack [return x * 2 + 42]
]

-- Another function
quack [define make-awesome taking [name] as
  quack [return f"{name} is awesome!"]
]

-- A struct
quack [struct awesome-thing with [name, power-level]]

-- Print a message when loaded
quack [print "Awesome library loaded!"]
```

## How do I publish?

1. Create a GitHub repository
2. Add your files
3. Create a release/tag (e.g., `v0.1.0`)
4. Users can now install with:

```bash
goose install yourname/your-library v0.1.0
```

## Best Practices

### Use descriptive function names

```duck
-- Good
quack [define calculate-tax taking [amount, rate] as ...]

-- Bad
quack [define calc taking [a, r] as ...]
```

### Document your functions

```duck
-- Calculate the tax on a given amount
-- amount: The base amount (number)
-- rate: Tax rate as decimal (0.1 = 10%)
-- Returns: The tax amount
quack [define calculate-tax taking [amount, rate] as
  quack [return amount * rate]
]
```

### Use a consistent prefix

If you're worried about name collisions, prefix your functions:

```duck
-- All functions start with "awsm-"
quack [define awsm-do-thing taking [] as ...]
quack [define awsm-other-thing taking [] as ...]
```

### Test your library

Create an `examples/` directory with working examples that users can run.

### Version semantically

- `v1.0.0` - Major release, breaking changes
- `v1.1.0` - Minor release, new features
- `v1.1.1` - Patch release, bug fixes

## Quick Reference

### Installing

```bash
goose install user/repo version
goose libs  # List installed
```

### Importing

```duck
-- With namespace
quack [migrate "git+user/repo" as prefix]
quack [prefix.function-name()]

-- Without namespace
quack [migrate "git+user/repo"]
quack [function-name()]

-- With specific version
quack [migrate "git+user/repo@v1.0.0" as prefix]

-- Local file
quack [migrate "./file.duck" as local]
```

### metadata.dm Format

```dm
[about]
author: 'username'
repo-url: 'https://github.com/user/repo'
description: 'What it does'
version: 'vX.Y.Z'

[point to]
./lib.duck
```
