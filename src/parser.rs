//! Parser module for converting tokens into an Abstract Syntax Tree (AST).
//!
//! This module provides a `Parser` struct that takes a sequence of tokens
//! and converts them into an AST representation of the code.

use crate::ast::ASTree;
use crate::identifiers;
use crate::token::Token;
use crate::token::TokenType;

/// Enum representing the type of token in the Shunting Yard algorithm.
enum ShuntingType {
  ///  Represents the start state.
  START,
  /// Represents an operator with its priority.
  OPERATOR(u8),
  /// Represents a number or identifier.
  NUMBER,
  /// Represents the end of an expression.
  END,
}

/// Matches an operator string to its corresponding priority.
///
/// # Arguments
///
/// * `operator` - A string slice representing the operator.
///
/// # Returns
///
/// * `u8` - The priority of the operator.
fn match_operator_to_priority(operator: &str) -> u8 {
  match operator {
    "+" => 1,
    "-" => 1,
    "/" => 2,
    "*" => 2,
    _ => panic!("Unsupported Operator in match_operator_to_priority"),
  }
}

/// Converts a token to its corresponding ShuntingType.
///
/// # Arguments
///
/// * `token` - A reference to the token to be converted.
///
/// # Returns
///
/// * `ShuntingType` - The corresponding ShuntingType.
fn convert_to_shunting_type(token: &Token) -> ShuntingType {
  match token.get_type() {
    TokenType::NUMERIC => ShuntingType::NUMBER,
    TokenType::FOR => ShuntingType::END,
    TokenType::WHILE => ShuntingType::END,
    TokenType::ELSE => ShuntingType::END,
    TokenType::IF => ShuntingType::END,
    TokenType::IDENTIFIER => ShuntingType::NUMBER,
    TokenType::BINARYOP => {
      ShuntingType::OPERATOR(match_operator_to_priority(token.get_value().as_str()))
    }
    _ => {
      panic!("Invalid TokenType passed to Shunting Yard algorithm during expression parsing")
    }
  }
}

/// Parser struct for parsing tokens into an Abstract Syntax Tree (AST).
pub struct Parser {
  tokens: Vec<Token>,
  pos: usize,
}

impl Parser {
  /// Creates a new Parser instance.
  pub fn new() -> Parser {
    identifiers::init_identifiers();
    Parser {
      tokens: Vec::new(),
      pos: 0,
    }
  }

  /// Peeks at the current token without advancing the position.
  ///
  /// # Returns
  ///
  /// * `&Token` - A reference to the current token.
  fn peek(&self) -> &Token {
    &self.tokens[self.pos]
  }

  /// Advances the position and returns the current token.
  ///
  /// # Returns
  ///
  /// * `Token` - The current token.
  fn advance(&mut self) -> Token {
    let token = self.tokens[self.pos].clone();
    self.pos += 1;
    token
  }

  /// Sets the tokens to be parsed.
  ///
  /// # Arguments
  ///
  /// * `tokens` - A vector of tokens to be parsed.
  pub fn set_tokens(&mut self, tokens: Vec<Token>) {
    self.tokens = tokens;
  }

  /// Implements the Shunting Yard algorithm to convert infix expressions to postfix.
  ///
  /// # Returns
  ///
  /// * `Result<Vec<Token>, String>` - A result containing the postfix token vector or an error
  /// message.
  fn shunting_yard(&mut self) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();
    let mut prev: ShuntingType = ShuntingType::START;
    while self.pos < self.tokens.len() {
      match convert_to_shunting_type(self.peek()) {
        ShuntingType::OPERATOR(val) => {
          if match prev {
            ShuntingType::OPERATOR(_) => true,
            _ => false,
          } {
            return Err(format!(
              "Syntax Error: Two operators in a row at position {}",
              self.peek().get_position()
            ));
          }

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
          prev = ShuntingType::OPERATOR(val);
        }
        ShuntingType::NUMBER => {
          // If the previous token was also a number, we've arrived at a new expression
          if match prev {
            ShuntingType::NUMBER => true,
            _ => false,
          } {
            break;
          }
          output.push(self.advance());
          prev = ShuntingType::NUMBER;
        }
        ShuntingType::END => break,
        ShuntingType::START => {}
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

  /// Parses an assignment statement.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the assignment
  fn parse_assign(&mut self) -> Result<ASTree, String> {
    let identifier: Token = self.advance();
    identifiers::set_identifier(
      identifier.get_value().clone(),
      crate::ast::RuntimeValue::NULL,
    );
    let mut output: ASTree = ASTree::new(self.advance());
    output.append(ASTree::new(identifier));
    let value: ASTree = self.parse_expression()?;
    output.append(value);
    Ok(output)
  }

  /// Parses an if statement.
  fn parse_if(&mut self) -> Result<ASTree, String> {
    Err(format!("Not implemented"))
  }

  /// Parses an expression using the Shunting Yard algorithm and constructs the AST.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the expression
  fn parse_expression(&mut self) -> Result<ASTree, String> {
    let tokens: Vec<Token> = self.shunting_yard().expect("Error during Shunting yard");
    let mut output: Vec<ASTree> = Vec::new();

    for token in tokens {
      match token.get_type() {
        TokenType::IDENTIFIER => {
          output.push(ASTree::new(token));
        }
        TokenType::NUMERIC => {
          output.push(ASTree::new(token));
        }
        TokenType::BINARYOP => {
          let mut node: ASTree = ASTree::new(token);
          let right: ASTree = output.pop().expect(
            "Failed to pop right node from token stack during parsing for binary operation",
          );
          let left: ASTree = output.pop().expect(
            "Failed to pop left token from token stack during parsing for binary operation",
          );
          node.append(left);
          node.append(right);
          output.push(node);
        }
        // TokenType::PRINT => {
        //   let mut node: ASTree = ASTree::new(token);
        //   node.append(output.pop().expect("No argument provided to print keyword"));
        //   output.push(node);
        // }
        _ => {
          return Err(format!(
            "Parser encountered unsupported token type during parsing: {:?}",
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

  /// Parses the tokens into a list of Abstract Syntax Trees (AST).
  ///
  /// # Returns
  ///
  /// * `Result<Vec<ASTree>, String>` - A result containing a vector of ASTrees or an error
  /// message.
  pub fn parse(&mut self) -> Result<Vec<ASTree>, String> {
    let mut output: Vec<ASTree> = Vec::new();
    while self.pos < self.tokens.len() {
      output.push(match self.peek().get_type() {
        TokenType::IF => self.parse_if()?,
        TokenType::IDENTIFIER => {
          if self.pos + 1 < self.tokens.len()
            && *self.tokens[self.pos + 1].get_type() == TokenType::ASSIGN
          {
            self.parse_assign()?
          } else {
            self.parse_expression()?
          }
        }
        _ => self.parse_expression()?,
      });
    }
    Ok(output)
  }
}
