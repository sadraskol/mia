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

## Types

- `String`
- `Process`
- `Stream`
- `List<T>`
- `Num`
- `Completable<T>`

## Operators

`.` calls a method like `"nAmE".uppercase`.

`?` is a special characters that turns a completable of `T` into `T`.
This is very useful to manipulate strings instead of pointers.

```mia
var files = `ls`?.o.words() // or `ls`.o?.words() or `ls`.o.words()?
for f in files {
  `stat $f`
}
```

## Process and TerminatedProcess

| Method | Description | `Process` | `TerminatedProcess` |
| --- | --- | --- | --- |
| `o` | Pointer to the standard output | `Completable<String>` | `String` |
| `e` | Pointer to the error output | `Completable<String>` | `String` |
| `status_code` | status of the process | `Completable<Num>` | `Num` |
| `pid` | pid of the corresponding process | `Num` | `Num` |
| `ppid` | parent process id | `Num` | `Num ` |

## String methods

| Method | Description |
| --- | --- |
| `words` | Split the string in words of either list of chars or single quoted strings |
| `lines` | Split the string in lines |
