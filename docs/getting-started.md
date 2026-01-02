# Getting Started with Duck

So you want to learn Duck? Bold choice. Most people just learn Python and call it a day. But you? You chose the language where you have to *ask permission* to run code. Respect.

## How do I install Duck?

First, you need the interpreter. It's called **Goose** because nothing in this language makes sense and we embrace that.

```bash
# On macOS/Linux
curl -fsSL https://raw.githubusercontent.com/konacodes/duck-lang/master/install.sh | bash

# Or build from source (for the brave)
git clone https://github.com/konacodes/duck-lang
cd duck-lang
cargo build --release
```

After installation, `goose` should be in your PATH. Test it:

```bash
goose --version
```

If that works, congratulations. If not, check that `~/.duck/bin` is in your PATH and try again.

## How do I run my first program?

Create a file called `hello.duck`:

```duck
quack [print "Hello, World!"]
```

Run it:

```bash
goose run hello.duck
```

You should see "Hello, World!" plus some snarky commentary from the goose.

## Wait, what's "quack"?

Ah yes, the core innovation of Duck: **every code block must be preceded by `quack`**.

```duck
quack [print "This runs"]
[print "This does not"]  -- Goose refuses to execute unquacked code
```

Think of it as saying "please" before asking the computer to do things. Except the computer is a passive-aggressive waterfowl.

## How do I write comments?

Use `--` (double dash):

```duck
-- This is a comment
quack [print "Hello"]  -- This is also a comment
```

## How do I use the REPL?

For quick experiments:

```bash
goose repl
```

Now you can type code directly:

```
duck> quack [print "Hello"]
Hello
   *happy honk*
duck> quack [let x be 42]
   The goose nods approvingly.
duck> exit
Goodbye! *waddles away*
```

## How do I check my code without running it?

```bash
goose check myfile.duck
```

This will tell you if you forgot to quack anywhere. It's like a linter, but more judgmental.

## What's next?

- [The Quack System](./syntax.md) - Understanding Duck's core concept
- [Variables and Types](./variables-and-types.md) - How to store things
- [Control Flow](./control-flow.md) - Making decisions
- [Functions](./functions.md) - Reusable code
- [Common Mistakes](./common-mistakes.md) - Learn from others' suffering

## The Goose Rating

At the end of every program, the goose rates your code from 1-10. Higher is better. Factors include:
- Did you quack properly?
- Did you define functions?
- Did you use structs?
- Are you a worthy programmer?

The goose is harsh but fair. Mostly harsh.
