use crate::ast::ASTree;
use crate::identifiers;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TypeValue;

use std::collections::HashMap;
use std::sync::Mutex;

enum ShuntingType {
  OPERATOR(u8),
  NUMBER,
  END,
}

fn match_operator_to_priority(operator: &str) -> u8 {
  match operator {
    "+" => 1,
    "-" => 1,
    "/" => 2,
    "*" => 2,
    _ => panic!("Unsupported Operator in match_operator_to_priority"),
  }
}

fn convert_to_shunting_type(token: &Token) -> ShuntingType {
  match token.get_type() {
    TokenType::NUMERIC => ShuntingType::NUMBER,
    TokenType::FOR => ShuntingType::END,
    TokenType::PRINT => ShuntingType::END,
    TokenType::WHILE => ShuntingType::END,
    TokenType::ELSE => ShuntingType::END,
    TokenType::IF => ShuntingType::END,
    TokenType::IDENTIFIER => ShuntingType::NUMBER,
    TokenType::BINARYOP => {
      ShuntingType::OPERATOR(match_operator_to_priority(token.get_value().as_str()))
    }
    TokenType::INVALID => {
      panic!("Invalid TokenType passed to Shunting Yard algorithm during expression parsing")
    }
  }
}

pub struct Parser {
  tokens: Vec<Token>,
  pos: usize,
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      tokens: Vec::new(),
      pos: 0,
    }
  }

  fn peek(&self) -> &Token {
    &self.tokens[self.pos]
  }

  fn advance(&mut self) -> Token {
    let token = self.tokens[self.pos].clone();
    self.pos += 1;
    token
  }

  pub fn set_tokens(&mut self, tokens: Vec<Token>) {
    self.tokens = tokens;
  }

  fn initialize_identifiers(&mut self, identifiers: HashMap<String, TypeValue>) {
    match identifiers::IDENTIFIERS.set(Mutex::new(identifiers)) {
      Ok(_) => {}
      Err(err) => panic!(
        "Failed to set IDENTIFIERS static HashMap after parsing, IDENTIFIERS was erroneously already set: {:?}",
        err
      ),
    }
  }

  fn shunting_yard(&mut self) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();
    // let mut state: TokenType = *self.peek();
    while self.pos < self.tokens.len() {
      match convert_to_shunting_type(self.peek()) {
        ShuntingType::OPERATOR(val) => {
          while operator_stack.len() > 0
            && val
              <= match_operator_to_priority(operator_stack.last().expect("").get_value().as_str())
          {
            output.push(
              operator_stack
                .pop()
                .expect("Error emptying operator stack in Shunting yard algorithm"),
            )
          }
          operator_stack.push(self.advance());
        }
        ShuntingType::NUMBER => output.push(self.advance()),
        ShuntingType::END => break,
      }
    }
    while !operator_stack.is_empty() {
      output.push(
        operator_stack
          .pop()
          .expect("Error emptying operator stack in Shunting yard algorithm"),
      );
    }
    dbg!(&output);
    Ok(output)
  }

  fn parse_if(&mut self) -> Result<ASTree, String> {
    Err(format!("Not implemented"))
  }

  fn parse_expression(&mut self) -> Result<ASTree, String> {
    let tokens: Vec<Token> = self.shunting_yard().expect("Error during Shunting yard");
    let mut output: Vec<ASTree> = Vec::new();

    for token in tokens {
      match token.get_type() {
        // TokenType::IDENTIFIER => {
        //   identifiers.insert(token.get_value().to_string(), TypeValue::NULL);
        //   self.trees.push(ASTree::new(token));
        // }
        TokenType::NUMERIC => {
          output.push(ASTree::new(token));
        }
        TokenType::BINARYOP => {
          let mut node: ASTree = ASTree::new(token);
          let right: ASTree = output.pop().expect(
            "Failed to pop right node from self.trees stack during parsing for binary operation",
          );
          let left: ASTree = output.pop().expect(
            "Failed to pop left token from token stack during parsing for binary operation",
          );
          node.append(left);
          node.append(right);
          output.push(node);
        }
        TokenType::PRINT => {
          let mut node: ASTree = ASTree::new(token);
          node.append(output.pop().expect("No argument provided to print keyword"));
          output.push(node);
        }
        _ => {
          return Err(format!(
            "{}{:?}",
            String::from("Parser encountered unsupported token type during parsing: "),
            token.get_type()
          ));
        }
      }
    }

    if output.len() == 1 {
      return Ok(
        output
          .pop()
          .expect("Unexpectedly empty Vec popped at end of expression parsing"),
      );
    }
    Err(format!(
      "Expression parsing failed to resolve to singular ASTree"
    ))
  }

  pub fn parse(&mut self) -> Result<ASTree, String> {
    match self.peek().get_type() {
      TokenType::IF => self.parse_if(),
      _ => self.parse_expression(),
    }
  }
}
