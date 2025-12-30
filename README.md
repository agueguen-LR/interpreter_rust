# Interpreter Rust

A simple interpreter coded in Rust.


My goal with this project is to learn Rust, and more about how interpreters work. My starting point was: no knowledge of Rust at all, and while I knew the principles behind interpreters, I didn't know the algorithms and structures they rely upon.


This project is still in its early stages, I've improved but there's still much to be learned and implemented.


## Current Features

- Usage of a Lexer, Parser, Abstract Syntax Trees (AST), and the Shunting Yard algorithm
- Basic arithmetic operations
- Variable assignment and usage

## Examples

The example folder contains some sample programs that can be run with the interpreter.
To run an example, use the following command:

```bash
cargo run example/<example-file>
```

## Building and Running

To build and run the interpreter, make sure you have Rust installed. Then, clone the repository and use Cargo to build and run the project:

```bash
git clone <repository-url>
cd interpreter-rust
cargo build
cargo run <input-file>
```

## Docs

You can generate the documentation using Cargo:

```bash
cargo doc --open
```
