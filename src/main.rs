mod ast;
mod identifiers;
mod lexer;
mod parser;
mod token;

use crate::parser::Parser;
use std::env;
use std::fs;

fn main() {
  let argv: Vec<String> = env::args().collect();
  let argc: usize = argv.len();

  if argc != 2 {
    panic!("Expected two arguments, found {argc}");
  }

  let file_content: String =
    fs::read_to_string(argv[1].clone()).expect("Failed to read file: {argv[1]}");
  print!("{file_content}");

  let tokens = match lexer::tokenize(file_content) {
    Err(error) => panic!("Error during lexing: {error}"),
    Ok(toks) => toks,
  };
  dbg!(&tokens);

  let mut parser = Parser::new();
  match parser.parse(tokens) {
    Err(error) => panic!("Error during parsing: {error}"),
    _ => {}
  }

  let astrees = parser.get_trees();
  dbg!(&astrees);
  for tree in astrees {
    match tree.eval() {
      Ok(return_code) => dbg!(return_code),
      Err(error) => panic!("Error during runtime: {error}"),
    };
  }
}
