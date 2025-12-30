mod ast;
mod identifiers;
mod lexer;
mod parser;
mod token;

use crate::lexer::Lexer;
use crate::parser::Parser;
use std::env;
use std::fs;

fn interpret(code: String) {
  let mut lexer = Lexer::new();
  let mut parser = Parser::new();

  lexer.set_input(code);
  let tokens = match lexer.tokenize() {
    Err(error) => panic!("Error during lexing: {error}"),
    Ok(toks) => toks,
  };
  dbg!(&tokens);

  parser.set_tokens(tokens);
  let mut tree = match parser.parse() {
    Err(error) => panic!("Error during parsing: {error}"),
    Ok(tree) => tree,
  };
  dbg!(&tree);

  match tree.eval() {
    Ok(return_value) => dbg!(return_value),
    Err(error) => panic!("Error during runtime: {error}"),
  };
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
}
