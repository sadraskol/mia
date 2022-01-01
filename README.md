# Mia configuration language

## Goal

Mia, a configuration language.
The idea is to provide a lightweight [Dhall language](https://dhall-lang.org).

The plan is to support yaml and json configuration output.
The current version only support json.

## Design

Source -> Scanner -> Parser -> Type Checker -> Virtual Machine -> Formatter

For now, the scanner is a single pass compiler to an AST.
The virtual machine executes this AST into an abstract object representation.
The final step is the formatter that turns the object into JSON.

## TODO

A lot of features remain:

- Correct stack, variable, scopes, etc.
- module import/export
- enums and adt

## Tests

There are no unit tests, we only test the output of the program.
See `tests/integration_test.rs` for more info.