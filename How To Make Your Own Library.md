# How To Make Your Own Duck Library

*A guide for those brave enough to share their quacks with the world*

---

## So You Want to Build a Library?

Congratulations! You've decided to contribute to the Duck ecosystem. The goose is... *cautiously optimistic*. Let's make sure your library is something we can all be proud of.

**What you'll need:**
- A GitHub account (or any git host, really)
- Some Duck code worth sharing
- A `metadata.dm` file (the goose demands it)
- A dream (optional but recommended)

---

## Step 1: The Repository Structure

Every Duck library needs at least two files:

```
your-awesome-lib/
├── metadata.dm      <- The goose reads this first
├── lib.duck         <- Your actual code (can be named anything)
└── README.md        <- Be a good citizen
```

That's it. No node_modules. No cargo.toml. No 47 config files. Just... simplicity.

---

## Step 2: The Magical metadata.dm File

This file tells the goose what's going on with your library. It uses a simple format that even a goose can understand:

```dm
[about]
author: 'Your Name Here'
repo-url: 'https://github.com/yourusername/your-library'
description: 'A brief description of what your library does'
version: 'v0.1.0'

[point to]
./lib.duck
```

### The `[about]` Section

| Field | Required | What It Does |
|-------|----------|--------------|
| `author` | Yes | Who made this thing? |
| `repo-url` | Yes | Where does it live on the internet? |
| `description` | Yes | What does it do? Keep it short. |
| `version` | Yes | Semantic versioning, please! (v1.2.3) |

### The `[point to]` Section

This tells the goose which file to load when someone imports your library. Just put the path to your main Duck file:

```dm
[point to]
./lib.duck
```

Or if you're feeling fancy:

```dm
[point to]
./src/main-module.duck
```

The goose will find it. The goose always finds it.

---

## Step 3: Writing Your Library Code

Your library code is just regular Duck code! Here's a template to get you started:

```duck
-- My Awesome Library
-- Does awesome things, awesomely.

-- =============================================================================
-- Configuration
-- =============================================================================

quack [let VERSION be "1.0.0"]

-- =============================================================================
-- Public Functions
-- =============================================================================

-- Add two numbers together. Revolutionary stuff.
quack [define my-add taking [a, b] as
  quack [return a + b]
]

-- Greet someone by name
quack [define greet taking [name] as
  quack [print f"Hello, {name}! Quack!"]
]

-- =============================================================================
-- Internal Helpers (users can still call these, we're not savages)
-- =============================================================================

quack [define internal-helper taking [x] as
  quack [return x * 2]
]

-- Let users know the library loaded
quack [print "My Awesome Library loaded successfully!"]
```

### Best Practices

1. **Comment your code** - The goose can read your code, but future-you will thank past-you for the comments.

2. **Use descriptive function names** - `calculate-total-price` beats `calc` every time.

3. **Print a load message** - Let users know their `migrate` worked!

4. **Handle errors gracefully** - Use `attempt/rescue` for anything that might fail.

---

## Step 4: Publishing to GitHub

1. Create a new repository on GitHub

2. Push your code:
```bash
git init
git add .
git commit -m "initial release - quack"
git branch -M main
git remote add origin https://github.com/yourusername/your-library.git
git push -u origin main
```

3. (Optional but recommended) Create a version tag:
```bash
git tag v0.1.0
git push origin v0.1.0
```

---

## Step 5: Users Can Now Install Your Library!

Once your library is on GitHub, anyone can install it:

```bash
# Install from main branch
goose install yourusername/your-library main

# Install a specific version
goose install yourusername/your-library v0.1.0
```

And use it in their Duck code:

```duck
-- With a namespace (recommended)
quack [migrate "git+yourusername/your-library" as mylib]
quack [mylib.greet "World"]

-- Or directly into global scope
quack [migrate "git+yourusername/your-library"]
quack [greet "World"]
```

---

## Versioning: A Quick Note

Use **semantic versioning**:
- `v1.0.0` -> `v1.0.1` for bug fixes
- `v1.0.0` -> `v1.1.0` for new features (backwards compatible)
- `v1.0.0` -> `v2.0.0` for breaking changes

Your users will thank you. The goose will judge you less harshly.

---

## The Full Example

Let's create a math library from scratch:

### `metadata.dm`
```dm
[about]
author: 'Math Duck'
repo-url: 'https://github.com/mathduck/duck-math'
description: 'Mathematical functions for Duck - because numbers matter'
version: 'v1.0.0'

[point to]
./lib.duck
```

### `lib.duck`
```duck
-- Duck Math Library
-- Mathematical functions that would make Euler quack with joy

-- Constants
quack [let MATH-PI be 3.14159265359]
quack [let MATH-E be 2.71828182846]
quack [let MATH-PHI be 1.61803398875]

-- Calculate the area of a circle
quack [define circle-area taking [radius] as
  quack [return MATH-PI * radius * radius]
]

-- Calculate the circumference of a circle
quack [define circle-circumference taking [radius] as
  quack [return 2 * MATH-PI * radius]
]

-- Factorial (recursive, naturally)
quack [define factorial taking [n] as
  quack [if n <= 1 then
    quack [return 1]
  otherwise
    quack [return n * factorial(n - 1)]
  ]
]

-- Fibonacci (because every math library needs one)
quack [define fibonacci taking [n] as
  quack [if n <= 0 then
    quack [return 0]
  ]
  quack [if n == 1 then
    quack [return 1]
  ]
  quack [let a be 0]
  quack [let b be 1]
  quack [let i be 2]
  quack [while i <= n do
    quack [let temp be a + b]
    quack [a becomes b]
    quack [b becomes temp]
    quack [i becomes i + 1]
  ]
  quack [return b]
]

-- Check if a number is prime
quack [define is-prime taking [n] as
  quack [if n < 2 then
    quack [return false]
  ]
  quack [let i be 2]
  quack [while i * i <= n do
    quack [if n % i == 0 then
      quack [return false]
    ]
    quack [i becomes i + 1]
  ]
  quack [return true]
]

quack [print "Duck Math Library loaded! Ready for calculations."]
```

### `README.md`
```markdown
# Duck Math Library

Mathematical functions for Duck-lang.

## Installation

\`\`\`bash
goose install mathduck/duck-math v1.0.0
\`\`\`

## Usage

\`\`\`duck
quack [migrate "git+mathduck/duck-math" as math]

quack [let area be math.circle-area(5)]
quack [print f"Area: {area}"]

quack [let fib be math.fibonacci(10)]
quack [print f"Fibonacci(10) = {fib}"]
\`\`\`

## Available Functions

- `circle-area(radius)` - Calculate circle area
- `circle-circumference(radius)` - Calculate circumference
- `factorial(n)` - Calculate n!
- `fibonacci(n)` - Get the nth Fibonacci number
- `is-prime(n)` - Check if n is prime

## Constants

- `MATH-PI` - 3.14159...
- `MATH-E` - 2.71828...
- `MATH-PHI` - 1.61803... (golden ratio)
```

---

## Troubleshooting

### "Library not found" error
Make sure you've:
1. Pushed to GitHub
2. Made the repository public
3. Used the correct username/repo format

### "Entry file not found" error
Check your `metadata.dm` - the `[point to]` path must match an actual file.

### "The goose is disappointed"
This is normal. The goose is always a little disappointed. Keep coding.

---

## Library Ideas

Need inspiration? Here are some libraries the Duck ecosystem could use:

- **duck-http-server** - A simple HTTP server
- **duck-database** - SQLite bindings
- **duck-testing** - A testing framework (the goose would appreciate this)
- **duck-cli** - Command-line argument parsing
- **duck-time** - Date and time utilities
- **duck-crypto** - Hashing and encryption

The world is your pond. Go make a splash.

---

## Final Words

Building a library is like teaching a duckling to swim - it takes patience, care, and occasionally you'll get splashed. But in the end, you'll have created something that helps other ducks paddle along.

Now go forth and quack responsibly!

*The Goose*

---

*P.S. - If your library is really good, the goose might rate your code higher than a 3/10. But don't get your hopes up.*
