# The Quack System

The defining feature of Duck is that code only runs if you say "quack" first. This is either genius language design or a terrible joke that went too far. We're still not sure.

## How do blocks work?

Code in Duck lives inside `[brackets]`. These are called **blocks**:

```duck
[print "Hello"]    -- This is a block
```

But here's the thing: blocks don't run on their own. They need authorization.

## How do I authorize a block?

Say `quack` before it:

```duck
quack [print "Hello"]  -- This runs!
```

The `quack` keyword tells the goose "yes, I really want this to execute." Without it, the goose just... ignores you.

```duck
[print "Hello"]  -- Goose: "I don't think so."
```

Unquacked blocks are parsed (so syntax errors still get caught) but never executed.

## Can I quack multiple blocks at once?

Yes! Multiple quacks authorize multiple blocks:

```duck
quack quack quack [print "One"] [print "Two"] [print "Three"]
```

Or space them out:

```duck
quack [print "First"]
quack [print "Second"]
```

Both work. Use whatever reads better.

## Do I need to quack inside functions and loops?

**Yes.** Every block needs its own quack, even inside other structures:

```duck
quack [define greet taking [name] as
  quack [print f"Hello, {name}!"]  -- Still needs quack!
]

quack [while x > 0 do
  quack [print x]         -- Yep, quack here too
  quack [x becomes x - 1]
]
```

This might seem tedious, but it's actually useful for debugging. You can "comment out" code by removing its quack:

```duck
quack [while x > 0 do
  quack [print x]
  [print "Debug info"]    -- Temporarily disabled
  quack [x becomes x - 1]
]
```

## What happens if I forget to quack?

The goose will:
1. Notice your unquacked block
2. Skip it entirely
3. Make a passive-aggressive comment about it

```
   *Notices unquacked block on line 5*
   "How interesting. Did you perhaps mean to run this?"
```

At the end, your code rating will suffer. The goose remembers.

## How do I check for missing quacks?

```bash
goose check myfile.duck
```

This scans your code and reports any unquacked blocks without running anything.

## Why does this exist?

Honestly? It started as a joke. But it turns out that requiring explicit authorization for every code block has some interesting properties:

1. **Debugging**: Disable any line by removing its quack
2. **Intentionality**: You have to think about what runs and what doesn't
3. **Comedy**: Everything is a tiny bit funnier

Is it practical for production software? Probably not. Is it fun? Absolutely.

## Quick Reference

| Syntax | Meaning |
|--------|---------|
| `quack [...]` | Execute this block |
| `[...]` | Skip this block (with judgment) |
| `quack quack [...] [...]` | Execute both blocks |
| `--` | Comment (ignored) |
