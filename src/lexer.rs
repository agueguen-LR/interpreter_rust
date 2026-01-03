//! A lexer module for tokenizing input strings.
//!
//! This module provides a `Lexer` struct that can tokenize input strings into a sequence of
//! tokens.

use crate::token::Token;
use crate::token::TokenType;

/// Represents the current state of the lexer.
enum LexerState {
  /// Parsing a number.
  NUMBER,
  /// Parsing an identifier.
  IDENTIFIER,
  /// Parsing a string literal
  STRING,
  /// Parsing a symbol
  SYMBOL,
  /// No current state.
  NONE,
}

/// A lexer for tokenizing input strings.
pub struct Lexer {
  input: String,
  index: usize,
  state: LexerState,
  current_token_string: String,
  current_token_position: usize,
}

impl Lexer {
  /// Creates a new `Lexer` instance.
  pub fn new() -> Lexer {
    Lexer {
      input: String::new(),
      index: 0,
      state: LexerState::NONE,
      current_token_string: String::new(),
      current_token_position: 0,
    }
  }

  /// Sets the input string for the lexer.
  ///
  /// # Arguments
  ///
  /// * `input` - The input string to be tokenized.
  pub fn set_input(&mut self, input: String) {
    self.input = input;
    self.index = 0;
    self.state = LexerState::NONE;
    self.current_token_string.clear();
    self.current_token_position = 0;
  }

  /// Checks if a character is a valid symbol.
  fn is_valid_symbol(character: char) -> bool {
    match character {
      '+' | '-' | '*' | '/' | '=' | '!' | '&' | '|' => true,
      _ => false,
    }
  }

  /// Emits a number token based on the current token string.
  ///
  /// # Arguments
  ///
  /// * `tokens` - A mutable reference to the vector of tokens.
  fn emit_number_token(&mut self, tokens: &mut Vec<Token>) {
    tokens.push(Token::new(
      TokenType::NUMERIC,
      self.current_token_string.clone(),
      self.current_token_position,
    ));
    self.current_token_string.clear();
    self.state = LexerState::NONE;
  }

  /// Emits an identifier token based on the current token string.
  ///
  /// # Arguments
  ///
  /// * `tokens` - A mutable reference to the vector of tokens.
  fn emit_identifier_token(&mut self, tokens: &mut Vec<Token>) {
    let token_type = match self.current_token_string.as_str() {
      "if" => TokenType::IF,
      "while" => TokenType::WHILE,
      "for" => TokenType::FOR,
      "else" => TokenType::ELSE,
      "fn" => TokenType::FN,
      _ => TokenType::IDENTIFIER,
    };
    tokens.push(Token::new(
      token_type,
      self.current_token_string.clone(),
      self.current_token_position,
    ));
    self.current_token_string.clear();
    self.state = LexerState::NONE;
  }

  /// Emits a string token based on the current token string.
  ///
  /// # Arguments
  ///
  /// * `tokens` - A mutable reference to the vector of tokens.
  fn emit_string_token(&mut self, tokens: &mut Vec<Token>) {
    tokens.push(Token::new(
      TokenType::STRING,
      self.current_token_string.clone(),
      self.current_token_position,
    ));
    self.current_token_string.clear();
    self.state = LexerState::NONE;
  }

  /// Emits a symbol token based on the current token string.
  ///
  /// # Arguments
  ///
  /// * `tokens` - A mutable reference to the vector of tokens.
  ///
  /// # Returns
  ///
  /// * `Result<(), String>` - A result indicating success or an error message.
  fn emit_symbol_token(&mut self, tokens: &mut Vec<Token>) -> Result<(), String> {
    let token_type = match self.current_token_string.as_str() {
      "+" | "-" | "*" | "/" | "==" | "!=" | "&&" | "||" => TokenType::BINARYOP,
      "=" => TokenType::ASSIGN,
      _ => {
        return Err(format!(
          "Invalid symbol '{}' at position {}",
          self.current_token_string, self.current_token_position
        ));
      }
    };
    tokens.push(Token::new(
      token_type,
      self.current_token_string.clone(),
      self.current_token_position,
    ));
    self.current_token_string.clear();
    self.state = LexerState::NONE;
    Ok(())
  }

  /// Tokenizes the input string into a vector of tokens.
  ///
  /// # Returns
  ///
  /// * `Result<Vec<Token>, String>` - A result containing a vector of tokens or an error message.
  pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();

    while self.index < self.input.len() {
      let character: char = self.input.chars().nth(self.index).unwrap();
      match self.state {
        LexerState::NONE => {
          if character.is_ascii_digit() {
            self.state = LexerState::NUMBER;
            self.current_token_position = self.index;
          } else if character.is_ascii_alphabetic() || character == '_' {
            self.state = LexerState::IDENTIFIER;
            self.current_token_position = self.index;
          } else if Self::is_valid_symbol(character) {
            self.state = LexerState::SYMBOL;
            self.current_token_position = self.index;
          } else if character.is_whitespace() {
            self.index += 1;
          } else {
            match character {
              '"' => {
                self.index += 1;
                self.state = LexerState::STRING;
                self.current_token_position = self.index;
              }
              '{' => {
                tokens.push(Token::new(TokenType::LBRACE, "{".to_string(), self.index));
                self.index += 1;
              }
              '}' => {
                tokens.push(Token::new(TokenType::RBRACE, "}".to_string(), self.index));
                self.index += 1;
              }
              '(' => {
                tokens.push(Token::new(TokenType::LPAREN, "(".to_string(), self.index));
                self.index += 1;
              }
              ')' => {
                tokens.push(Token::new(TokenType::RPAREN, ")".to_string(), self.index));
                self.index += 1;
              }
              ',' => {
                tokens.push(Token::new(TokenType::COMMA, ",".to_string(), self.index));
                self.index += 1;
              }
              _ => {
                return Err(format!(
                  "Invalid character '{}' at position {}",
                  character, self.index
                ));
              }
            }
          }
        }

        LexerState::NUMBER => {
          if !character.is_ascii_digit() {
            self.emit_number_token(&mut tokens);
          } else {
            self.current_token_string.push(character);
            self.index += 1;
          }
        }

        LexerState::IDENTIFIER => {
          if !(character.is_ascii_alphanumeric() || character == '_') {
            self.emit_identifier_token(&mut tokens);
          } else {
            self.current_token_string.push(character);
            self.index += 1;
          }
        }

        LexerState::STRING => {
          if character == '"' {
            self.emit_string_token(&mut tokens);
            self.index += 1;
          } else {
            self.current_token_string.push(character);
            self.index += 1;
          }
        }

        LexerState::SYMBOL => {
          if !Self::is_valid_symbol(character) {
            self.emit_symbol_token(&mut tokens)?;
          } else {
            self.current_token_string.push(character);
            self.index += 1;
          }
        }
      }
    }

    if !self.current_token_string.is_empty() {
      match self.state {
        LexerState::NUMBER => self.emit_number_token(&mut tokens),
        LexerState::IDENTIFIER => self.emit_identifier_token(&mut tokens),
        LexerState::STRING => {
          return Err(format!(
            "Unterminated string literal starting at position {}",
            self.current_token_position
          ));
        }
        LexerState::SYMBOL => self.emit_symbol_token(&mut tokens)?,
        LexerState::NONE => {}
      }
    }

    // DO NOT REMOVE THIS EOF TOKEN - PARSER EXPECTS IT TO BE PRESENT
    // AT THE END OF THE TOKEN STREAM, INFINITY LOOPS WILL OCCUR OTHERWISE
    tokens.push(Token::new(TokenType::EOF, String::new(), self.index));
    Ok(tokens)
  }
}
