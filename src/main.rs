mod ast;
mod parser;

use std::env;
use std::fs;

use crate::ast::ASTree;
use crate::ast::Token;
use crate::ast::TokenType;

fn main() {
  let argv: Vec<String> = env::args().collect();
  let argc: usize = argv.len();

  if argc != 2 {
    panic!("Expected two arguments, found {argc}");
  }

  let file_content: String =
    fs::read_to_string(argv[1].clone()).expect("Failed to read file: {argv[1]}");

  print!("{file_content}");
  let tokens = parser::tokenize(file_content);
  dbg!(tokens);

  let mut tree: ASTree = ASTree::new(Token::new(TokenType::BINARYOP, String::from("/")));
  tree.append(ASTree::new(Token::new(
    TokenType::NUMERIC,
    String::from("-45"),
  )));
  tree.append(ASTree::new(Token::new(
    TokenType::NUMERIC,
    String::from("15"),
  )));

  tree.eval();
}
