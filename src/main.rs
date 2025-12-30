//! An interpreter for a custom programming language.
//!
//! This module serves as the entry point for the interpreter, handling
//! reading input files, lexing, parsing, and evaluating the code.

mod ast;
mod identifiers;
mod lexer;
mod parser;
mod token;

use crate::lexer::Lexer;
use crate::parser::Parser;
use std::env;
use std::fs;

/// Interprets the given code string by lexing, parsing, and evaluating it.
///
/// # Arguments
///
/// * `code` - The code string to be interpreted.
fn interpret(code: String) {
  let mut lexer = Lexer::new();
  let mut parser = Parser::new();

  lexer.set_input(code);
  let tokens = match lexer.tokenize() {
    Err(error) => panic!("Error during lexing: {:?}", error),
    Ok(toks) => toks,
  };
  dbg!(&tokens);

  parser.set_tokens(tokens);
  let trees = match parser.parse() {
    Err(error) => panic!("Error during parsing: {error}"),
    Ok(tree) => tree,
  };
  dbg!(&trees);

  for mut tree in trees {
    match tree.eval() {
      Ok(_return_value) => {}
      Err(error) => panic!("Error during runtime: {error}"),
    };
  }
}

fn main() {
  let argv: Vec<String> = env::args().collect();
  let argc: usize = argv.len();

  if argc != 2 {
    panic!("Expected two arguments, found {argc}");
  }

  let file_content: String =
    fs::read_to_string(argv[1].clone()).expect("Failed to read file: {argv[1]}");
  print!("{file_content}");

  interpret(file_content);

  // testing area
  let a = identifiers::get_identifier(&String::from("a"));
  let b = identifiers::get_identifier(&String::from("b"));
  let c = identifiers::get_identifier(&String::from("c"));
  dbg!(a);
  dbg!(b);
  dbg!(c);
}
