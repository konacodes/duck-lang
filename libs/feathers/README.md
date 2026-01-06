# Feathers

Pretty terminal output for Duck. Like Python's Rich, but with more quacking.

## Installation

```bash
goose install konacodes/feathers v0.1.0
```

## Usage

```duck
quack [migrate "git+konacodes/feathers" as f]

-- Colors
quack [print f.red("Error!")]
quack [print f.green("Success!")]
quack [print f.yellow("Warning")]
quack [print f.cyan("Info")]

-- Styles
quack [print f.bold("Important")]
quack [print f.italic("Emphasized")]
quack [print f.underline("Underlined")]
quack [print f.dim("Muted")]

-- Semantic helpers
quack [f.print-success("Task completed")]
quack [f.print-error("Something went wrong")]
quack [f.print-warning("Proceed with caution")]
quack [f.print-info("Did you know?")]

-- Box drawing
quack [f.box("Hello, World!")]
quack [f.box-colored("Error!", "red")]

-- Headers
quack [f.header("My Section")]
quack [f.section("Subsection")]

-- Horizontal rules
quack [f.hr()]
quack [f.hr-double()]
quack [f.hr-dotted()]

-- Progress bar
quack [print f.progress-bar(7, 10, 20)]  -- [██████████████░░░░░░] 70%

-- Rainbow text (for fun)
quack [print f.rainbow("Party time!")]

-- Utility functions
quack [print f.pad-right("hello", 10)]   -- "hello     "
quack [print f.pad-left("hello", 10)]    -- "     hello"
quack [print f.center("hello", 10)]      -- "  hello   "
quack [print f.repeat-string("=", 20)]   -- "===================="
```

## Functions

### Colors
- `red(text)`, `green(text)`, `yellow(text)`, `blue(text)`
- `magenta(text)`, `cyan(text)`, `white(text)`, `gray(text)`

### Styles
- `bold(text)`, `italic(text)`, `underline(text)`, `dim(text)`

### Semantic
- `success(text)`, `error(text)`, `warning(text)`, `info(text)`, `muted(text)`

### Print Helpers
- `print-success(text)` - Prints with green ✓
- `print-error(text)` - Prints with red ✗
- `print-warning(text)` - Prints with yellow ⚠
- `print-info(text)` - Prints with cyan ℹ

### Box Drawing
- `box(text)` - Draw a box around text
- `box-colored(text, color)` - Colored box

### Headers
- `header(text)` - Bold text with underline
- `section(text)` - Cyan arrow with text

### Lines
- `hr()` - Horizontal rule (─)
- `hr-double()` - Double line (═)
- `hr-dotted()` - Dotted line (·)

### Progress
- `progress-bar(current, total, width)` - ASCII progress bar

### Utilities
- `repeat-string(s, n)` - Repeat string n times
- `pad-right(text, width)` - Right-pad with spaces
- `pad-left(text, width)` - Left-pad with spaces
- `center(text, width)` - Center text

### Fun
- `rainbow(text)` - Rainbow-colored text

## License

MIT - Do whatever you want with it. Just keep quacking.
