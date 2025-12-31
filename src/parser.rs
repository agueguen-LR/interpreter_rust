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
  /// Represents an operator with its priority.
  OPERATOR(u8),
  /// Represents an operand.
  OPERAND,
  /// Represents the end of an expression.
  END,
}

/// Parser struct for parsing tokens into an Abstract Syntax Tree (AST).
pub struct Parser {
  /// The list of tokens to be parsed.
  tokens: Vec<Token>,
  /// The current position in the token list.
  pos: usize,
}

impl Parser {
  /// Creates a new Parser instance.
  pub fn new() -> Parser {
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
      "==" => 1,
      "!=" => 1,
      "&&" => 1,
      "||" => 1,
      "+" => 2,
      "-" => 2,
      "/" => 3,
      "*" => 3,
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
      TokenType::NUMERIC => ShuntingType::OPERAND,
      TokenType::IDENTIFIER => ShuntingType::OPERAND,
      TokenType::STRING => ShuntingType::OPERAND,
      TokenType::BINARYOP => {
        ShuntingType::OPERATOR(Self::match_operator_to_priority(token.get_value().as_str()))
      }
      _ => ShuntingType::END,
    }
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
    // Start prev as operator, binary operators cannot start an expression
    let mut prev: ShuntingType = ShuntingType::OPERATOR(0);

    // Loop can't be infinite, worst case will break when encountering a TokenType::EOF (ShuntingType::END)
    loop {
      match Self::convert_to_shunting_type(self.peek()) {
        ShuntingType::OPERATOR(val) => {
          if matches!(prev, ShuntingType::OPERATOR(_)) {
            return Err(format!(
              "Invalid operator placement at position {}",
              self.peek().get_position()
            ));
          }

          // auto-formatting makes this hard to read
          // while there are operators on the stack with greater or equal precedence than the
          // current operator, pop them to the output
          while operator_stack.len() > 0
            && val
              <= Self::match_operator_to_priority(
                operator_stack.last().unwrap().get_value().as_str(),
              )
          {
            output.push(operator_stack.pop().unwrap())
          }

          operator_stack.push(self.advance());
          prev = ShuntingType::OPERATOR(val);
        }
        ShuntingType::OPERAND => {
          // If the previous token was also an operand, this is a different expression
          if matches!(prev, ShuntingType::OPERAND) {
            break;
          } else {
            output.push(self.advance());
            prev = ShuntingType::OPERAND;
          }
        }
        ShuntingType::END => break,
      }
    }

    while !operator_stack.is_empty() {
      output.push(operator_stack.pop().unwrap());
    }
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

  /// Parses a block of code enclosed in braces.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the block
  fn parse_block(&mut self, name: String) -> Result<ASTree, String> {
    let mut output: ASTree = ASTree::new(Token::new(
      TokenType::BLOCK,
      name,
      *self.peek().get_position(),
    ));
    if !matches!(self.advance().get_type(), TokenType::LBRACE) {
      return Err(format!(
        "Expected '{{' at position {}, found {:?}",
        self.peek().get_position(),
        self.peek().get_type()
      ));
    }
    while !matches!(self.peek().get_type(), TokenType::RBRACE) {
      output.append(self.parse_once()?);
    }
    self.advance();
    Ok(output)
  }

  /// Parses an if statement.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the if statement
  fn parse_if(&mut self) -> Result<ASTree, String> {
    let mut output: ASTree = ASTree::new(self.advance());
    if !matches!(self.advance().get_type(), TokenType::LPAREN) {
      return Err(format!(
        "Expected '(' after 'if' at position {}",
        self.peek().get_position()
      ));
    }

    output.append(self.parse_expression()?);

    if !matches!(self.advance().get_type(), TokenType::RPAREN) {
      return Err(format!(
        "Expected ')' after if condition at position {}",
        self.peek().get_position()
      ));
    }
    output.append(self.parse_block(format!("if_block"))?);

    if matches!(self.peek().get_type(), TokenType::ELSE) {
      self.advance(); // consume 'else'
      output.append(self.parse_block(format!("else_block"))?);
    }

    Ok(output)
  }

  /// Parses a while loop.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the while loop
  fn parse_while(&mut self) -> Result<ASTree, String> {
    let mut output: ASTree = ASTree::new(self.advance());
    if !matches!(self.advance().get_type(), TokenType::LPAREN) {
      return Err(format!(
        "Expected '(' after 'while' at position {}",
        self.peek().get_position()
      ));
    }

    output.append(self.parse_expression()?);

    if !matches!(self.advance().get_type(), TokenType::RPAREN) {
      return Err(format!(
        "Expected ')' after while condition at position {}",
        self.peek().get_position()
      ));
    }
    output.append(self.parse_block(format!("while_block"))?);

    Ok(output)
  }

  /// Parses an expression using the Shunting Yard algorithm and constructs the AST.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the expression
  fn parse_expression(&mut self) -> Result<ASTree, String> {
    let tokens: Vec<Token> = self.shunting_yard()?;
    let mut output: Vec<ASTree> = Vec::new();

    for token in tokens {
      match token.get_type() {
        TokenType::IDENTIFIER => {
          output.push(ASTree::new(token));
        }
        TokenType::NUMERIC => {
          output.push(ASTree::new(token));
        }
        TokenType::STRING => {
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
        _ => {
          return Err(format!(
            "Parser encountered unsupported token type during parsing: {:?}",
            token.get_type()
          ));
        }
      }
    }

    if output.len() == 0 {
      return Err(format!(
        "Expected expression, found none at position {}",
        self.peek().get_position()
      ));
    }
    if output.len() == 1 {
      return Ok(output.pop().unwrap());
    }
    Err(format!(
      "Expression parsing failed to resolve to singular ASTree"
    ))
  }

  /// Parses a single statement or expression based on the current token.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the statement or expression
  fn parse_once(&mut self) -> Result<ASTree, String> {
    match self.peek().get_type() {
      TokenType::IF => self.parse_if(),
      TokenType::WHILE => self.parse_while(),
      TokenType::LBRACE => self.parse_block(format!("gen_block")),
      TokenType::IDENTIFIER => {
        if self.pos + 1 < self.tokens.len()
          && matches!(*self.tokens[self.pos + 1].get_type(), TokenType::ASSIGN)
        {
          self.parse_assign()
        } else {
          self.parse_expression()
        }
      }
      _ => self.parse_expression(),
    }
  }

  /// Parses the tokens into a list of Abstract Syntax Trees (AST).
  ///
  /// # Returns
  ///
  /// * `Result<Vec<ASTree>, String>` - A result containing a vector of ASTrees or an error
  /// message.
  pub fn parse(&mut self) -> Result<Vec<ASTree>, String> {
    let mut output: Vec<ASTree> = Vec::new();
    while !matches!(self.peek().get_type(), TokenType::EOF) {
      output.push(self.parse_once()?);
    }
    Ok(output)
  }
}
