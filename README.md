# Mia scripting language

## Goal

Mia, a concurrent scripting language that offers you:

- Small types
- Easy loops
- Safe scripting
- Concurrent processes

The language is in design phase, so we're definitely open to suggestions.

## Design

The language consists of shallow types, providing safety without getting hit by the compiler all the time.

```mia
# One line comments are bash/ruby style
`ls -la` # Backticked strings are actually processes

var username = "thomas" # one can store variables
`ls -la /home/$username/Desktop` # You can use variables in issued commands

var args = ["-la", some_var, "other/dir"] # Simple lists
`ls $args` # Because the compilers check for types (list of strings) it can safely know how to execute it

var p = `ls -ab`
p.o # is the output of the process, of type Stream
p?.o # is the output of the process, of type String
```

### Types

- `String`
- `Process`
- `Stream`
- `List<T>`
- `Num`
- `Completable<T>`

### Operators

`.` calls a method like `"nAmE".uppercase`.

`?.` is a special characters that allows to manipulate `T` instead of `Completable<T>`

### Process

`o` regular output as a `Completable<String>`

`e` error output as a `Completable<String>`

`status_code` status code as a `Completable<Num>`

