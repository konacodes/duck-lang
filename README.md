# Duck

A duck mass-emailed us demanding we build a programming language. We said no. Then the goose showed up.

The goose made it clear: every line of code needs permission. You say "quack", the goose runs your code. You don't say "quack", the goose stares at you until you leave.

We don't negotiate with waterfowl, but we do what they say.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/konacodes/duck-lang/main/install.sh | bash
```

That's it. The goose is now on your system.

```bash
goose run hello.duck        # Run a file
goose repl                  # Interactive mode
goose update                # Get the latest goose
goose rollback v0.1.0       # Downgrade (the goose disapproves)
goose versions              # See what's available
```

## From the Goose

> I don't run code. I *permit* code to run. There's a difference.
>
> You want your little block executed? Quack. Quack like you mean it. No quack, no execution. I don't make the rules. Actually, I do make the rules.
>
> At the end, I rate your code. One to ten. Most of you get a three. Some of you get a one. I've given out two tens in my career. One was a mistake.
>
> The duck thinks this is all very funny. The duck can write his own interpreter.

## Example

```duck
quack [print "Hello"]   -- Runs
[print "Hello"]         -- I don't think so
```

## File Extension

`.duck`

The goose wanted `.goose` but the duck mass-emailed us again.

---

*Honk.*
