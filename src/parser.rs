use crate::ast::ASTree;
use crate::identifiers;
use crate::token::Token;
use crate::token::TokenType;

use std::collections::HashMap;
use std::sync::Mutex;

pub fn parse(mut token_stack: Vec<Token>) -> ASTree {
  let mut identifiers: HashMap<String, Token> = HashMap::new();
  let mut nodes: Vec<ASTree> = Vec::new();

  while !token_stack.is_empty() {
    let token: Token = token_stack
      .pop()
      .expect("Failed to pop token from token stack during parsing");
    match token.get_type() {
      TokenType::IDENTIFIER => {
        identifiers.insert(
          token.get_value(),
          Token::new(TokenType::NULL, String::from("")),
        );
        nodes.push(ASTree::new(token));
      }
      TokenType::NUMERIC => {
        nodes.push(ASTree::new(token));
      }
      TokenType::BINARYOP => {
        let mut node: ASTree = ASTree::new(token);
        let right: ASTree = nodes
          .pop()
          .expect("Failed to pop right node from nodes stack during parsing for binary operation");
        let left: ASTree =
          ASTree::new(token_stack.pop().expect(
            "Failed to pop left token from token stack during parsing for binary operation",
          ));
        node.append(left);
        node.append(right);
        nodes.push(node);
      }
      TokenType::PRINT => {
        let mut node: ASTree = ASTree::new(token);
        node.append(nodes.pop().expect("No argument provided to print keyword"));
        nodes.push(node);
      }
      _ => {
        panic!(
          "Parser encountered unsupported token type during parsing: {:?}",
          token.get_type()
        );
      }
    }
  }

  match identifiers::IDENTIFIERS.set(Mutex::new(identifiers)) {
    Ok(_) => {}
    Err(err) => panic!(
      "Failed to set IDENTIFIERS static HashMap after parsing, IDENTIFIERS was erroneously already set to {err:?}"
    ),
  }

  let mut output: ASTree = ASTree::new(Token::new(TokenType::START, String::from("")));
  for node in nodes {
    output.append(node);
  }
  output
}
