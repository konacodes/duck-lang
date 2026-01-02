# File I/O

Duck can read and write files. The goose prefers relative paths and will give you a hard time if you try anything sneaky.

## How do I read a file?

Use `read-file()`:

```duck
quack [let content be read-file("data.txt")]
quack [print content]
```

Returns the entire file as a string.

## What if the file doesn't exist?

You get an error with a sassy message:

```
The goose searched everywhere but couldn't find 'data.txt'
```

To handle this gracefully, check first:

```duck
quack [if file-exists("data.txt") then
  quack [let content be read-file("data.txt")]
  quack [print content]
otherwise
  quack [print "File not found!"]
]
```

## How do I write to a file?

Use `write-file()`:

```duck
quack [let unused be write-file("output.txt", "Hello, World!")]
```

This **overwrites** the file if it exists, or creates it if it doesn't.

## How do I append to a file?

Use `append-file()`:

```duck
quack [let unused be append-file("log.txt", "New entry\n")]
```

This adds to the end of the file without erasing what's already there.

## How do I check if a file exists?

Use `file-exists()`:

```duck
quack [if file-exists("config.txt") then
  quack [print "Config found!"]
otherwise
  quack [print "Using defaults..."]
]
```

## How do I read a file line by line?

Read the file, then split on newlines:

```duck
quack [let content be read-file("data.txt")]
quack [let lines be split(content, "\n")]

quack [for each [line] in lines do
  quack [print f"Line: {line}"]
]
```

## How do I write multiple lines?

Join your lines with newlines:

```duck
quack [let lines be list("Line 1", "Line 2", "Line 3")]
quack [let content be join(lines, "\n")]
quack [let unused be write-file("output.txt", content)]
```

## Can I use absolute paths?

No. The goose is paranoid about security:

```
Absolute paths not allowed - the goose prefers relative paths
```

All paths must be relative to your current working directory.

## Can I use `..` to go up directories?

Also no:

```
Path traversal (..) not allowed - the goose is suspicious
```

The goose doesn't trust you to wander around the filesystem.

## What about permissions?

If you don't have permission to read or write a file:

```
The goose is not allowed to look at 'secret.txt'
The goose is not allowed to write to '/etc/passwd'
```

## Example: Simple Config File

```duck
-- Read config if it exists, otherwise use defaults
quack [let config be ""]
quack [if file-exists("config.txt") then
  quack [config becomes read-file("config.txt")]
otherwise
  quack [config becomes "default_setting=true"]
  quack [let unused be write-file("config.txt", config)]
]

quack [print f"Config: {config}"]
```

## Example: Logging

```duck
quack [define log taking [message] as
  quack [let line be f"{message}\n"]
  quack [let unused be append-file("app.log", line)]
]

quack [let unused be log("Application started")]
quack [let unused be log("Processing...")]
quack [let unused be log("Done!")]
```

## Example: Reading CSV

```duck
quack [let content be read-file("data.csv")]
quack [let lines be split(content, "\n")]

quack [for each [line] in lines do
  quack [let cells be split(line, ",")]
  quack [print cells]
]
```

## Quick Reference

| Function | Description |
|----------|-------------|
| `read-file(path)` | Read entire file as string |
| `write-file(path, content)` | Write string to file (overwrite) |
| `append-file(path, content)` | Append string to file |
| `file-exists(path)` | Check if file exists (returns boolean) |

## Security Notes

- Paths must be relative (no leading `/`)
- No `..` in paths (no directory traversal)
- Sandboxed to current directory and subdirectories
- Permission errors are handled gracefully

The goose takes security seriously. Perhaps too seriously.
