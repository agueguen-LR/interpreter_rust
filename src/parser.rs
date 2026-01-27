//! Parser module for converting tokens into an Abstract Syntax Tree (AST).
//!
//! This module provides a `Parser` struct that takes a sequence of tokens
//! and converts them into an AST representation of the code.

use crate::ast::ASTree;
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

  /// Peeks at the next token without advancing the position.
  ///
  /// # Returns
  ///
  /// * Option::None if there is no next token
  /// * Option::Some(&Token) if there is a next token
  fn peek_next(&self) -> Option<&Token> {
    self.tokens.get(self.pos + 1)
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

  /// Consumes a token of the expected type.
  /// Use instead of advance when you expect a specific token type.
  ///
  /// # Arguments
  ///
  /// * `token_type` - The expected type of the token to consume.
  ///
  /// # Returns
  ///
  /// * `Result<Token, String>` - A result containing the consumed token or an error message.
  fn consume(&mut self, token_type: TokenType) -> Result<Token, String> {
    if *self.peek().get_type() != token_type {
      return Err(format!(
        "Expected {:?} before position {}, found {:?}",
        token_type,
        self.peek().get_position(),
        self.peek().get_type()
      ));
    }
    Ok(self.advance())
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
  /// Simultaneously builds the ASTree nodes for operands and operators.
  ///
  /// # Returns
  ///
  /// * `Result<Vec<ASTree>, String>` - A result containing the postfix ASTree vector or an error
  /// message.
  fn shunting_yard(&mut self) -> Result<Vec<ASTree>, String> {
    let mut output: Vec<ASTree> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();
    // Start prev as operator, binary operators cannot be start of an expression
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
            output.push(ASTree::new(operator_stack.pop().unwrap()))
          }

          operator_stack.push(self.advance());
          prev = ShuntingType::OPERATOR(val);
        }
        ShuntingType::OPERAND => {
          // If the previous token was also an operand, this is a different expression
          if matches!(prev, ShuntingType::OPERAND) {
            break;
          }

          // If the next token is a left parenthesis, this operand is a function call
          if self.peek_next().is_some()
            && matches!(self.peek_next().unwrap().get_type(), TokenType::LPAREN)
          {
            output.push(self.parse_fn_call()?);
          } else {
            output.push(ASTree::new(self.advance()));
          }
          prev = ShuntingType::OPERAND;
        }
        ShuntingType::END => break,
      }
    }

    while !operator_stack.is_empty() {
      output.push(ASTree::new(operator_stack.pop().unwrap()));
    }
    Ok(output)
  }

  /// Parses an expression using the Shunting Yard algorithm and constructs the AST.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the expression
  fn parse_expression(&mut self) -> Result<ASTree, String> {
    let postfix_expression: Vec<ASTree> = self.shunting_yard()?;
    let mut output: Vec<ASTree> = Vec::new();

    for tree in postfix_expression {
      if matches!(tree.get_type(), TokenType::BINARYOP) {
        let right: ASTree = output.pop().expect("Insufficient operands for operator");
        let left: ASTree = output.pop().expect("Insufficient operands for operator");

        let mut operator_node: ASTree = tree;
        operator_node.append(left);
        operator_node.append(right);
        output.push(operator_node);
      } else {
        output.push(tree);
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

  /// Parses an assignment statement.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the assignment
  fn parse_assign(&mut self) -> Result<ASTree, String> {
    let identifier: ASTree = ASTree::new(self.consume(TokenType::IDENTIFIER)?);
    let mut output: ASTree = ASTree::new(self.consume(TokenType::ASSIGN)?);
    let value: ASTree = self.parse_statement()?;

    output.append(identifier);
    output.append(value);
    Ok(output)
  }

  /// Parses a block of code enclosed in braces.
  ///
  /// # Arguments
  /// * `name` - The name of the block.
  /// * `scoped` - A boolean indicating whether the block should create a new scope
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the block
  fn parse_block(&mut self, name: String, scoped: bool) -> Result<ASTree, String> {
    let mut output: ASTree = ASTree::new(Token::new(
      TokenType::BLOCK(scoped),
      name,
      *self.peek().get_position(),
    ));
    self.consume(TokenType::LBRACE)?;

    while !matches!(self.peek().get_type(), TokenType::RBRACE) {
      output.append(self.parse_statement()?);
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
    let mut output: ASTree = ASTree::new(self.consume(TokenType::IF)?);

    self.consume(TokenType::LPAREN)?;
    output.append(self.parse_expression()?);
    self.consume(TokenType::RPAREN)?;

    output.append(self.parse_block(format!("if_block"), true)?);

    if matches!(self.peek().get_type(), TokenType::ELSE) {
      self.advance();
      output.append(self.parse_block(format!("else_block"), true)?);
    }

    Ok(output)
  }

  /// Parses a while loop.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the while loop
  fn parse_while(&mut self) -> Result<ASTree, String> {
    let mut output: ASTree = ASTree::new(self.consume(TokenType::WHILE)?);

    self.consume(TokenType::LPAREN)?;
    output.append(self.parse_expression()?);
    self.consume(TokenType::RPAREN)?;

    output.append(self.parse_block(format!("while_block"), true)?);

    Ok(output)
  }

  /// Parses a function definition.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the function definition
  fn parse_fn_def(&mut self) -> Result<ASTree, String> {
    let mut output: ASTree = ASTree::new(self.consume(TokenType::FN)?);
    let name: ASTree = ASTree::new(self.consume(TokenType::IDENTIFIER)?);
    output.append(name);

    self.consume(TokenType::LPAREN)?;
    if matches!(self.peek().get_type(), TokenType::IDENTIFIER) {
      output.append(ASTree::new(self.consume(TokenType::IDENTIFIER)?));
    }
    while matches!(self.peek().get_type(), TokenType::COMMA) {
      self.consume(TokenType::COMMA)?;
      output.append(ASTree::new(self.consume(TokenType::IDENTIFIER)?));
    }
    self.consume(TokenType::RPAREN)?;

    // Function body block shouldn't auto-create a new scope. During evaluation, parameters will be
    // need to be set within the function's scope, so ast::ASTree::eval_fn_call handles the scope
    // creation instead of ast::ASTree::eval_block
    output.append(self.parse_block("fn_body_block".to_string(), false)?);
    Ok(output)
  }

  /// Parses a function call.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the function call
  fn parse_fn_call(&mut self) -> Result<ASTree, String> {
    let mut output: ASTree = ASTree::new(self.consume(TokenType::IDENTIFIER)?);

    self.consume(TokenType::LPAREN)?;
    if !matches!(self.peek().get_type(), TokenType::RPAREN) {
      output.append(self.parse_expression()?);
    }
    while matches!(self.peek().get_type(), TokenType::COMMA) {
      self.consume(TokenType::COMMA)?;
      output.append(self.parse_expression()?);
    }
    self.consume(TokenType::RPAREN)?;

    Ok(output)
  }

  /// Parses a single statement based on the current token.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree, String>` - A result containing the ASTree for the statement or expression
  fn parse_statement(&mut self) -> Result<ASTree, String> {
    match self.peek().get_type() {
      TokenType::IF => self.parse_if(),
      TokenType::WHILE => self.parse_while(),
      TokenType::FN => self.parse_fn_def(),
      TokenType::EOF => return Err("Attempted to parse EOF token".to_string()),
      TokenType::LBRACE => self.parse_block(format!("sub_block"), true),
      TokenType::IDENTIFIER => {
        if self.peek_next().is_some()
          && matches!(self.peek_next().unwrap().get_type(), TokenType::ASSIGN)
        {
          self.parse_assign()
        } else {
          self.parse_expression()
        }
      }
      _ => self.parse_expression(),
    }
  }

  /// Parses the tokens an Abstract Syntax Trees (AST).
  /// This goes through all the tokens that have been set, and creates a single AST from them.
  /// The many ASTs that would result from normal parsing are all children of a single root BLOCK token,
  /// that can be considered the global scope of the program.
  ///
  /// # Returns
  ///
  /// * `Result<ASTree>, String>` - A result containing the ASTree or an error
  /// message.
  pub fn parse(&mut self) -> Result<ASTree, String> {
    let mut output: ASTree = ASTree::new(Token::new(
      TokenType::BLOCK(true),
      String::from("global_block"),
      0,
    ));
    while !matches!(self.peek().get_type(), TokenType::EOF) {
      output.append(self.parse_statement()?);
    }
    Ok(output)
  }
}
