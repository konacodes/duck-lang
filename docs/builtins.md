# Built-in Functions Reference

Every function that comes with Duck, ready to use. No imports needed.

---

## I/O

### print

Print values to the console:

```duck
quack [print "Hello"]
quack [print "Multiple" "values" "work"]
quack [print f"Interpolation: {2 + 2}"]
```

### input

Read a line from the user:

```duck
quack [let name be input("What's your name? ")]
quack [print f"Hello, {name}!"]
```

The prompt is optional.

---

## Type Conversion

### string

Convert anything to a string:

```duck
quack [print string(42)]     -- "42"
quack [print string(true)]   -- "true"
```

### number

Convert strings or booleans to numbers:

```duck
quack [print number("42")]   -- 42
quack [print number(true)]   -- 1
quack [print number(false)]  -- 0
```

### type-of

Get the type of a value:

```duck
quack [print type-of(42)]         -- "number"
quack [print type-of("hello")]    -- "string"
quack [print type-of(list())]     -- "list"
quack [print type-of(true)]       -- "boolean"
```

---

## Math

### abs

Absolute value:

```duck
quack [print abs(-5)]   -- 5
quack [print abs(5)]    -- 5
```

### floor / ceil

Round down or up:

```duck
quack [print floor(3.7)]  -- 3
quack [print ceil(3.2)]   -- 4
```

### sqrt

Square root:

```duck
quack [print sqrt(16)]  -- 4
quack [print sqrt(2)]   -- 1.41421...
```

### pow

Raise to a power:

```duck
quack [print pow(2, 8)]   -- 256
quack [print pow(3, 3)]   -- 27
```

### min / max

Find smallest or largest:

```duck
quack [print min(1, 5, 3)]  -- 1
quack [print max(1, 5, 3)]  -- 5
```

### random

Random number between 0 and 1:

```duck
quack [let r be random()]
quack [print r]  -- 0.something
```

### range

Create a list of numbers:

```duck
quack [let nums be range(0, 5)]
quack [print nums]  -- [0, 1, 2, 3, 4]
```

Note: End is exclusive.

---

## Lists

### len

Get length:

```duck
quack [print len(list(1, 2, 3))]  -- 3
```

### push

Add to end (mutates the list):

```duck
quack [let nums be list(1, 2)]
quack [nums push 3]
quack [print nums]  -- [1, 2, 3]
```

### pop

Remove from end (mutates the list, returns removed item):

```duck
quack [let nums be list(1, 2, 3)]
quack [let last be pop(nums)]
quack [print last]  -- 3
quack [print nums]  -- [1, 2]
```

### reverse

Create a reversed copy:

```duck
quack [let reversed be reverse(list(1, 2, 3))]
quack [print reversed]  -- [3, 2, 1]
```

### sort

Create a sorted copy:

```duck
quack [let sorted be sort(list(3, 1, 2))]
quack [print sorted]  -- [1, 2, 3]
```

### contains

Check if list contains a value:

```duck
quack [print contains(list(1, 2, 3), 2)]  -- true
```

---

## Strings

### len

Get length:

```duck
quack [print len("hello")]  -- 5
```

### uppercase / lowercase

Change case:

```duck
quack [print uppercase("hello")]  -- HELLO
quack [print lowercase("HELLO")]  -- hello
```

### trim

Remove leading/trailing whitespace:

```duck
quack [print trim("  hello  ")]  -- "hello"
```

### split

Split into list:

```duck
quack [print split("a,b,c", ",")]  -- ["a", "b", "c"]
```

### join

Join list into string:

```duck
quack [print join(list("a", "b"), "-")]  -- "a-b"
```

### contains

Check for substring:

```duck
quack [print contains("hello world", "world")]  -- true
```

### reverse

Reverse a string:

```duck
quack [print reverse("hello")]  -- "olleh"
```

---

## Higher-Order Functions

### map

Transform each element:

```duck
quack [let doubled be map(list(1, 2, 3), [x] -> x * 2)]
quack [print doubled]  -- [2, 4, 6]
```

### filter

Keep matching elements:

```duck
quack [let evens be filter(list(1, 2, 3, 4), [x] -> x % 2 == 0)]
quack [print evens]  -- [2, 4]
```

### fold

Reduce to a single value:

```duck
quack [let sum be fold(list(1, 2, 3), 0, [acc, x] -> acc + x)]
quack [print sum]  -- 6
```

### find

Find first matching element:

```duck
quack [let first be find(list(1, 2, 3, 4), [x] -> x > 2)]
quack [print first]  -- 3
```

### any

Check if any element matches:

```duck
quack [print any(list(1, 2, 3), [x] -> x > 2)]  -- true
```

### all

Check if all elements match:

```duck
quack [print all(list(2, 4, 6), [x] -> x % 2 == 0)]  -- true
```

---

## Structs

### keys

Get field names:

```duck
quack [struct duck with [name, age]]
quack [let d be duck("Gerald", 5)]
quack [print keys(d)]  -- ["name", "age"]
```

### values

Get field values:

```duck
quack [print values(d)]  -- ["Gerald", 5]
```

---

## File I/O

### read-file

Read file contents:

```duck
quack [let content be read-file("data.txt")]
```

### write-file

Write to a file (overwrites):

```duck
quack [let unused be write-file("out.txt", "Hello!")]
```

### append-file

Append to a file:

```duck
quack [let unused be append-file("log.txt", "New line\n")]
```

### file-exists

Check if file exists:

```duck
quack [if file-exists("config.txt") then
  quack [print "Found it!"]
]
```

---

## Environment

### env

Get environment variable:

```duck
quack [let home be env("HOME")]
quack [let missing be env("NONEXISTENT")]  -- nil
```

---

## JSON

### json-parse

Parse JSON string into Duck value:

```duck
quack [let data be json-parse("{\"name\": \"Gerald\", \"age\": 5}")]
quack [print data.name]  -- Gerald
```

### json-stringify

Convert Duck value to JSON string:

```duck
quack [let json be json-stringify(my-struct)]
quack [print json]
```

---

## HTTP

### http-get

Make a GET request:

```duck
quack [let response be http-get("https://api.example.com/data")]
quack [print response.status]  -- 200
quack [print response.body]    -- Response content
```

With headers:

```duck
quack [let headers be list("Authorization", "Bearer token123")]
quack [let response be http-get("https://api.example.com", headers)]
```

### http-post

Make a POST request:

```duck
quack [let body be "{\"message\": \"hello\"}"]
quack [let headers be list("Content-Type", "application/json")]
quack [let response be http-post("https://api.example.com", body, headers)]
```

---

## Base64

### base64-encode

Encode string to base64:

```duck
quack [print base64-encode("Hello!")]  -- SGVsbG8h
```

### base64-decode

Decode base64 to string:

```duck
quack [print base64-decode("SGVsbG8h")]  -- Hello!
```

---

## System

### sleep

Pause execution (milliseconds):

```duck
quack [let unused be sleep(1000)]  -- Wait 1 second
```
