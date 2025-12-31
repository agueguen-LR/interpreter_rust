//! This module defines the core token structures and types used by the lexer and parser.
//!
//! It provides the `TokenType` enum for classifying tokens,
//! and the `Token` struct for encapsulating token data, including its type, value, and position in the source code.

/// Represents the different types of tokens that can be identified by the lexer.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenType {
  /// Numeric literal.
  NUMERIC,
  /// Identifier (e.g., variable or function name).
  IDENTIFIER,
  /// String literal.
  STRING,
  /// Binary operator (e.g., +, -, *, /).
  BINARYOP,
  /// Assignment operator (e.g., =).
  ASSIGN,
  /// 'if' keyword.
  IF,
  /// 'while' keyword.
  WHILE,
  /// 'for' keyword.
  FOR,
  /// 'else' keyword.
  ELSE,
}

/// Represents a token with its type, value, and position in the source code.
#[derive(Clone, Debug)]
pub struct Token {
  /// The type of the token.
  token_type: TokenType,
  /// The string value of the token.
  value: String,
  /// The position of the token in the source code.
  pos: usize,
}

impl Token {
  /// Creates a new `Token` instance.
  ///
  /// # Arguments
  ///
  /// * `token_type` - The type of the token.
  /// * `value` - The string value of the token.
  /// * `position` - The position of the token in the source code.
  pub fn new(token_type: TokenType, value: String, position: usize) -> Token {
    Token {
      token_type: token_type,
      value: value,
      pos: position,
    }
  }

  /// Returns a reference to the value of the token.
  pub fn get_value(&self) -> &String {
    &self.value
  }

  /// Returns a reference to the type of the token.
  pub fn get_type(&self) -> &TokenType {
    &self.token_type
  }

  /// Returns a reference to the position of the token in the source code.
  pub fn get_position(&self) -> &usize {
    &self.pos
  }
}
