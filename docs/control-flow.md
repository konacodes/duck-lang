# Control Flow

At some point, your code needs to make decisions. This is where Duck's control flow comes in. Prepare for more quacking.

## How do I write if/else?

Use `if`, `then`, and `otherwise`:

```duck
quack [let age be 18]

quack [if age >= 18 then
  quack [print "You can vote!"]
otherwise
  quack [print "Too young to vote."]
]
```

Note: both branches still need their own `quack`.

## Can I skip the else branch?

Yes, just leave out `otherwise`:

```duck
quack [if is-raining then
  quack [print "Bring an umbrella!"]
]
```

## How do I chain conditions?

Nest your ifs (there's no `elsif` or `else if`):

```duck
quack [if score >= 90 then
  quack [print "A"]
otherwise
  quack [if score >= 80 then
    quack [print "B"]
  otherwise
    quack [if score >= 70 then
      quack [print "C"]
    otherwise
      quack [print "F"]
    ]
  ]
]
```

Yeah, it's a bit verbose. We're considering adding `elsif` but honestly the nested version has a certain charm.

## What comparison operators exist?

| Operator | Meaning |
|----------|---------|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less than or equal |
| `>=` | Greater than or equal |

```duck
quack [if x == 5 then quack [print "five"]]
quack [if x != 0 then quack [print "not zero"]]
quack [if x > 10 then quack [print "big"]]
```

## What logical operators exist?

| Operator | Meaning |
|----------|---------|
| `and` | Both must be true |
| `or` | Either can be true |
| `not` | Negation |

```duck
quack [if age >= 18 and has-id then
  quack [print "Access granted"]
]

quack [if is-admin or is-moderator then
  quack [print "Welcome, staff member"]
]

quack [if not is-banned then
  quack [print "You may enter"]
]
```

## How do I write a while loop?

Use `while` and `do`:

```duck
quack [let countdown be 5]

quack [while countdown > 0 do
  quack [print countdown]
  quack [countdown becomes countdown - 1]
]

quack [print "Blast off!"]
```

Output:
```
5
4
3
2
1
Blast off!
```

## How do I repeat something N times?

Use `repeat` and `times`:

```duck
quack [repeat 3 times
  quack [print "Quack!"]
]
```

Output:
```
Quack!
Quack!
Quack!
```

This is cleaner than a while loop when you just need to repeat something.

## How do I loop over a list?

Use `for each`:

```duck
quack [let ducks be list("Gerald", "Waddles", "Quackers")]

quack [for each [name] in ducks do
  quack [print f"Hello, {name}!"]
]
```

Output:
```
Hello, Gerald!
Hello, Waddles!
Hello, Quackers!
```

Note the brackets around the loop variable: `[name]`.

## Can I loop over a range of numbers?

Use the `range()` function:

```duck
quack [for each [i] in range(1, 6) do
  quack [print i]
]
```

Output:
```
1
2
3
4
5
```

`range(a, b)` gives you numbers from `a` up to (but not including) `b`.

## How do I break out of a loop?

Use `break`:

```duck
quack [let i be 0]
quack [while true do
  quack [print i]
  quack [i becomes i + 1]
  quack [if i >= 5 then
    quack [break]
  ]
]
```

## How do I skip to the next iteration?

Use `continue`:

```duck
quack [for each [n] in range(1, 10) do
  quack [if n == 5 then
    quack [continue]  -- Skip 5
  ]
  quack [print n]
]
```

## Quick Reference

| Syntax | Meaning |
|--------|---------|
| `if ... then ... otherwise ...` | Conditional |
| `while ... do ...` | Loop while condition is true |
| `repeat N times ...` | Loop N times |
| `for each [x] in list do ...` | Loop over items |
| `break` | Exit loop |
| `continue` | Skip to next iteration |
| `and`, `or`, `not` | Logical operators |
