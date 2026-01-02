# HTTP and JSON

Duck can talk to the internet. This is how you build APIs, bots, and other networked things. The goose is surprisingly good at web requests.

## How do I make a GET request?

Use `http-get()`:

```duck
quack [let response be http-get("https://api.example.com/data")]
quack [print response.status]   -- 200
quack [print response.body]     -- Response content as string
quack [print response.headers]  -- List of header key-value pairs
```

## How do I make a POST request?

Use `http-post()`:

```duck
quack [let body be "{\"message\": \"hello\"}"]
quack [let response be http-post("https://api.example.com/data", body)]
quack [print response.status]
```

## How do I add headers?

Pass a list of key-value pairs:

```duck
quack [let headers be list(
  "Authorization", "Bearer my-token",
  "Content-Type", "application/json",
  "User-Agent", "DuckBot/1.0"
)]

quack [let response be http-get("https://api.example.com", headers)]
```

Headers are paired: key1, value1, key2, value2, etc.

For POST:

```duck
quack [let headers be list("Content-Type", "application/json")]
quack [let body be "{\"name\": \"Gerald\"}"]
quack [let response be http-post("https://api.example.com", body, headers)]
```

## What does the response look like?

The response is a struct with three fields:

| Field | Type | Description |
|-------|------|-------------|
| `status` | number | HTTP status code (200, 404, etc.) |
| `body` | string | Response body as text |
| `headers` | list | Response headers as key-value pairs |

```duck
quack [let r be http-get("https://example.com")]

quack [if r.status == 200 then
  quack [print "Success!"]
  quack [print r.body]
otherwise
  quack [print f"Error: {r.status}"]
]
```

## How do I parse JSON?

Use `json-parse()`:

```duck
quack [let response be http-get("https://api.example.com/user")]
quack [let data be json-parse(response.body)]

-- Now you can access fields
quack [print data.name]
quack [print data.email]
```

JSON arrays become Duck lists. JSON objects become Duck structs.

## How do I create JSON?

Use `json-stringify()`:

```duck
quack [struct user with [name, email, age]]
quack [let u be user("Gerald", "gerald@duck.pond", 5)]
quack [let json be json-stringify(u)]
quack [print json]  -- {"name":"Gerald","email":"gerald@duck.pond","age":5}
```

Works with lists too:

```duck
quack [let items be list(1, 2, 3)]
quack [let json be json-stringify(items)]
quack [print json]  -- [1,2,3]
```

## Example: Fetch and Display API Data

```duck
quack [let response be http-get("https://api.github.com/users/konacodes")]

quack [if response.status == 200 then
  quack [let user be json-parse(response.body)]
  quack [print f"Username: {user.login}"]
  quack [print f"Name: {user.name}"]
  quack [print f"Repos: {user.public_repos}"]
otherwise
  quack [print f"Failed to fetch: {response.status}"]
]
```

## Example: POST JSON Data

```duck
quack [struct message with [content, author]]
quack [let msg be message("Hello from Duck!", "Gerald")]

quack [let body be json-stringify(msg)]
quack [let headers be list(
  "Content-Type", "application/json",
  "Authorization", "Bearer my-secret-token"
)]

quack [let response be http-post("https://api.example.com/messages", body, headers)]

quack [if response.status >= 200 and response.status < 300 then
  quack [print "Message sent!"]
otherwise
  quack [print f"Error: {response.status}"]
]
```

## Example: Simple API Client

```duck
quack [define api-get taking [endpoint, token] as
  quack [let url be "https://api.example.com" + endpoint]
  quack [let headers be list(
    "Authorization", "Bearer " + token,
    "Content-Type", "application/json"
  )]
  quack [let response be http-get(url, headers)]

  quack [if response.status == 200 then
    quack [return json-parse(response.body)]
  otherwise
    quack [print f"API Error: {response.status}"]
    quack [return nil]
  ]
]

-- Usage
quack [let token be env("API_TOKEN")]
quack [let users be api-get("/users", token)]
quack [if users != nil then
  quack [for each [user] in users do
    quack [print user.name]
  ]
]
```

## Error Handling

HTTP requests can fail. Wrap them in `attempt`:

```duck
quack [attempt
  quack [let response be http-get("https://api.example.com")]
  quack [print response.body]
rescue err
  quack [print f"Request failed: {err}"]
]
```

## Quick Reference

| Function | Description |
|----------|-------------|
| `http-get(url)` | GET request |
| `http-get(url, headers)` | GET with headers |
| `http-post(url, body)` | POST request |
| `http-post(url, body, headers)` | POST with headers |
| `json-parse(string)` | Parse JSON to Duck value |
| `json-stringify(value)` | Convert Duck value to JSON |

## Response Structure

```duck
response.status   -- number (HTTP status code)
response.body     -- string (response content)
response.headers  -- list (key-value pairs)
```

## Common Status Codes

| Code | Meaning |
|------|---------|
| 200 | OK - Success |
| 201 | Created - Resource created |
| 400 | Bad Request - Invalid input |
| 401 | Unauthorized - Need auth |
| 403 | Forbidden - No permission |
| 404 | Not Found - Resource missing |
| 500 | Server Error - Their problem |
