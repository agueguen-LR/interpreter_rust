//! Abstract Syntax Tree implementation and evaluation logic.
//!
//! This module defines the structure of the Abstract Syntax Tree (AST) used to represent
//! parsed code and provides evaluation functionality for the AST nodes.

use crate::identifiers;
use crate::token::Token;
use crate::token::TokenType;

/// Represents a value during runtime evaluation.
#[derive(Clone, Debug)]
pub enum RuntimeValue {
  /// An integer value.
  INTEGER(i32),
  /// A string value.
  STRING(String),
  /// A null value.
  NULL,
  /// A boolean value.
  BOOL(bool),
}

/// Represents a node in the Abstract Syntax Tree (AST).
#[derive(Debug)]
pub struct ASTree {
  children: Vec<ASTree>,
  token: Token,
}

impl ASTree {
  /// Creates a new ASTree node with the given token.
  ///
  /// # Arguments
  ///
  /// * `token` - The token associated with this AST node.
  ///
  /// # Returns
  ///
  /// * A new ASTree instance.
  pub fn new(token: Token) -> ASTree {
    ASTree {
      children: Vec::new(),
      token: token,
    }
  }

  /// Appends a child ASTree node to this node.
  ///
  /// # Arguments
  ///
  /// * `child` - The child ASTree node to append.
  pub fn append(&mut self, child: ASTree) {
    self.children.push(child);
  }

  /// Evaluates the ASTree node and returns the resulting RuntimeValue.
  ///
  /// # Returns
  ///
  /// * `Ok(RuntimeValue)` if evaluation is successful.
  /// * `Err(String)` if an error occurs during evaluation.
  pub fn eval(&mut self) -> Result<RuntimeValue, String> {
    match self.token.get_type() {
      TokenType::NUMERIC => match self.token.get_value().parse::<i32>() {
        Ok(result) => return Ok(RuntimeValue::INTEGER(result)),
        Err(error) => return Err(error.to_string()),
      },

      TokenType::BINARYOP => {
        if self.children.len() != 2 {
          return Err(format!(
            "Invalid amount of params passed to Binary Operation Evaluation, at position: {}",
            self.token.get_position()
          ));
        }
        let param1: i32 = match self.children[0].eval() {
          Err(error) => return Err(error),
          Ok(val) => match val {
            RuntimeValue::INTEGER(n) => n,
            _ => {
              return Err(format!(
                "Invalid type provided to Binary Operator, at position: {}",
                self.children[0].token.get_position()
              ));
            }
          },
        };
        let param2: i32 = match self.children[1].eval() {
          Err(error) => return Err(error),
          Ok(val) => match val {
            RuntimeValue::INTEGER(n) => n,
            _ => {
              return Err(format!(
                "Invalid type provided to Binary Operator, at position: {}",
                self.children[1].token.get_position()
              ));
            }
          },
        };

        match self.token.get_value().as_str() {
          "+" => return Ok(RuntimeValue::INTEGER(param1 + param2)),
          "-" => return Ok(RuntimeValue::INTEGER(param1 - param2)),
          "*" => return Ok(RuntimeValue::INTEGER(param1 * param2)),
          "/" => return Ok(RuntimeValue::INTEGER(param1 / param2)),
          _ => {
            return Err(format!(
              "Unexpected operator in BinOP evaluation, at position: {}",
              self.token.get_position(),
            ));
          }
        }
      }

      TokenType::IDENTIFIER => match identifiers::get_identifier(self.token.get_value()) {
        Option::Some(val) => Ok(val),
        Option::None => Err(format!(
          "Attempted to access unset identifier: '{}', at position: {}",
          self.token.get_value(),
          self.token.get_position()
        )),
      },

      TokenType::ASSIGN => {
        identifiers::set_identifier(
          self.children[0].token.get_value().clone(),
          self.children[1].eval()?,
        );
        Ok(RuntimeValue::BOOL(true))
      }

      _ => {
        return Err(format!(
          "Unexpected TokenType evaluated: {:?}",
          self.token.get_type()
        ));
      }
    }
  }
}
