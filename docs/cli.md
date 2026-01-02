# The Goose CLI

The `goose` command is your gateway to running Duck programs. It's judgmental, but helpful.

## How do I run a program?

```bash
goose run myfile.duck
```

The goose will:
1. Parse your code
2. Execute it (if you quacked properly)
3. Rate your code at the end

## How do I pass arguments to my program?

```bash
goose run myfile.duck arg1 arg2 arg3
```

Arguments are available in your code as `quack-args`:

```duck
quack [print quack-args]        -- ["arg1", "arg2", "arg3"]
quack [print quack-args at 0]   -- "arg1"
```

## How do I check for quack issues without running?

```bash
goose check myfile.duck
```

This analyzes your code and reports any unquacked blocks:

```
QUACK ALERT! The following lines are missing quack:
   Line 5: No quack detected!
   Line 12: No quack detected!

Remember: Every block needs a quack to be valid.
   2 issue(s) found.
```

If everything is fine:

```
All blocks are properly quacked! Honk!
   Your code passes the vibe check.
```

## How do I start the REPL?

```bash
goose repl
```

This starts an interactive session:

```
Welcome to the Goose REPL. Type 'exit' to leave.
   Don't forget to quack!

duck> quack [let x be 42]
   The goose nods approvingly.
duck> quack [print x * 2]
84
   *happy honk*
duck> exit
Goodbye! *waddles away*
```

Great for quick experiments.

## How do I update goose?

```bash
goose update
```

This downloads the latest version from GitHub releases and replaces your current binary. Your old version is backed up just in case.

## How do I see available versions?

```bash
goose versions
```

This shows all released versions:

```
Available versions:

    1. v0.2.0 <-- current
    2. v0.1.0
    3. v0.0.1
```

## How do I rollback to a previous version?

```bash
goose rollback v0.1.0
```

This downloads and installs the specified version.

## How do I install a library?

```bash
goose install konacodes/discord v0.1.0
```

This clones the library from GitHub into `~/.duck/libs/`.

See [Libraries](./libraries.md) for more details.

## How do I see installed libraries?

```bash
goose libs
```

Output:

```
[*] Installed Duck Libraries
========================================

  konacodes/discord @ v0.1.0
    Discord API library for Duck-lang. Build bots that quack!
```

## How do I check the version?

```bash
goose --version
```

## Command Summary

| Command | Description |
|---------|-------------|
| `goose run file.duck` | Run a Duck program |
| `goose run file.duck args...` | Run with arguments |
| `goose check file.duck` | Check for quack issues |
| `goose repl` | Start interactive mode |
| `goose update` | Update to latest version |
| `goose versions` | List available versions |
| `goose rollback vX.Y.Z` | Install a specific version |
| `goose install user/repo version` | Install a library |
| `goose libs` | List installed libraries |
| `goose --version` | Show version |
| `goose --help` | Show help |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (parse, runtime, etc.) |

## Configuration

Duck looks for libraries in `~/.duck/libs/`.

The goose binary itself lives in `~/.duck/bin/goose` after installation.

You can override the install directory with the `DUCK_INSTALL_DIR` environment variable:

```bash
export DUCK_INSTALL_DIR=/custom/path
goose update  # Installs to /custom/path/bin/goose
```

## About That Rating

At the end of every program, the goose rates your code from 1-10:

```
═══════════════════════════════════════
  Goose rated your code: 7/10
  "Adequate. I've seen worse from geese."
═══════════════════════════════════════
```

Factors:
- **Quack ratio**: Did you quack all your blocks?
- **Functions**: Did you define any functions?
- **Structs**: Did you use structs?
- **Loops**: Did you iterate?
- **Penalties**: Unquacked blocks hurt your score

The goose's judgment is final. There is no appeals process.
