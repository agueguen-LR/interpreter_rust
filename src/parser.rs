use crate::ast::ASTree;
use crate::identifiers;
use crate::token::Token;
use crate::token::TokenType;

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Parser {
  trees: Vec<ASTree>,
}

impl Parser {
  pub fn new() -> Parser {
    Parser { trees: Vec::new() }
  }

  fn initialize_identifiers(&mut self, identifiers: HashMap<String, Token>) {
    match identifiers::IDENTIFIERS.set(Mutex::new(identifiers)) {
      Ok(_) => {}
      Err(err) => panic!(
        "Failed to set IDENTIFIERS static HashMap after parsing, IDENTIFIERS was erroneously already set to {err:?}"
      ),
    }
  }

  pub fn get_trees(&mut self) -> &mut Vec<ASTree> {
    &mut self.trees
  }

  pub fn parse(&mut self, mut token_stack: Vec<Token>) -> Result<String, String> {
    let identifiers: HashMap<String, Token> = HashMap::new();

    while !token_stack.is_empty() {
      let mut token: Token = token_stack
        .pop()
        .expect("Failed to pop token from token stack during parsing");
      match token.get_type() {
        // TokenType::IDENTIFIER => {
        //   identifiers.insert(
        //     token.get_value(),
        //     Token::new(TokenType::NULL, String::from("")),
        //   );
        //   self.trees.push(ASTree::new(token));
        // }
        TokenType::NUMERIC => {
          self.trees.push(ASTree::new(token));
        }
        TokenType::BINARYOP => {
          let mut node: ASTree = ASTree::new(token);
          let right: ASTree = self.trees.pop().expect(
            "Failed to pop right node from self.trees stack during parsing for binary operation",
          );
          let left: ASTree = ASTree::new(token_stack.pop().expect(
            "Failed to pop left token from token stack during parsing for binary operation",
          ));
          node.append(left);
          node.append(right);
          self.trees.push(node);
        }
        TokenType::PRINT => {
          let mut node: ASTree = ASTree::new(token);
          node.append(
            self
              .trees
              .pop()
              .expect("No argument provided to print keyword"),
          );
          self.trees.push(node);
        }
        _ => {
          panic!(
            "Parser encountered unsupported token type during parsing: {:?}",
            token.get_type()
          );
        }
      }
    }

    self.initialize_identifiers(identifiers);
    Ok(String::from("Parsing successful, "))
  }
}
