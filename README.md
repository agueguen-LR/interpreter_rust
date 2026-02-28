[![CI](https://github.com/agueguen-LR/Interpreter/actions/workflows/rust.yml/badge.svg)](https://github.com/agueguen-LR/Interpreter/actions/workflows/rust.yml)
[![Docs](https://img.shields.io/badge/docs-GitHub%20Pages-success)](https://agueguen-lr.github.io/Interpreter/)
# Interpreter

A simple interpreter coded in Rust.


My goal with this project is to learn Rust, and more about how interpreters work. My starting point was: no knowledge of Rust at all, and while I knew the principles behind interpreters, I didn't know the algorithms and structures they rely upon.


This project is generally finished, more could be implemented, but I've done and learned enough to stop here, for now.


## Current Features

- Usage of a Lexer, Parser, Abstract Syntax Trees (AST), and the Shunting Yard algorithm
- Basic arithmetic operations
- Variable assignment and usage
- If-Else conditionals
- While loops
- Functions (working recursivity and local variables)

## Notable missing features

- Print
- Assignment to exterior scopes
- Comments
- Imports

## Examples

The example folder contains some sample programs that can be run with the interpreter.
To run an example, use the following command:

```bash
cargo run example/<example-file>
```

There is no way to print anything yet, so to see if the code is working, the interpreter automatically prints it's current context before exiting each scope.

This means, after running the code, you'll see the final state of all global variables and global functions. You can also see all the created scopes and local variables/functions if you scroll up enough.

Functions are stored as Abstract Syntax Trees, these can become quite illegible very quickly, so I recommend smart usage of scopes to see only what you want to see.

## Building and Running

To build and run the interpreter, make sure you have Rust installed. Then, clone the repository and use Cargo to build and run the project:

```bash
git clone <repository-url>
cd Interpreter
cargo build
cargo run <input-file>
```

## Docs

You can generate the documentation using Cargo:

```bash
cargo doc --open
```

## Grammar

An Extended Backus–Naur form (EBNF) representation of the language grammar is available [here](./grammar.txt)

All types of whitespace are ignored (spaces, tabs, newlines).
