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

  /// Evaluates a binary operation ASTree node.
  ///
  /// # Returns
  ///
  /// * `Ok(RuntimeValue)` if evaluation is successful.
  /// * `Err(String)` if an error occurs during evaluation.
  fn eval_binary_op(&mut self) -> Result<RuntimeValue, String> {
    if self.children.len() != 2 {
      return Err(format!(
        "Invalid amount of params passed to Binary Operation Evaluation, at position: {}",
        self.token.get_position()
      ));
    }
    let param1: RuntimeValue = self.children[0].eval()?;
    let param2: RuntimeValue = self.children[1].eval()?;

    match (param1, param2) {
      (RuntimeValue::INTEGER(val1), RuntimeValue::INTEGER(val2)) => {
        match self.token.get_value().as_str() {
          "+" => Ok(RuntimeValue::INTEGER(val1 + val2)),
          "-" => Ok(RuntimeValue::INTEGER(val1 - val2)),
          "*" => Ok(RuntimeValue::INTEGER(val1 * val2)),
          "/" => {
            if val2 == 0 {
              Err(format!(
                "Division by zero error at position: {}",
                self.token.get_position()
              ))
            } else {
              Ok(RuntimeValue::INTEGER(val1 / val2))
            }
          }
          "==" => Ok(RuntimeValue::BOOL(val1 == val2)),
          "!=" => Ok(RuntimeValue::BOOL(val1 != val2)),
          _ => Err(format!(
            "Unsupported binary operator: '{}' between integers, at position: {}",
            self.token.get_value(),
            self.token.get_position()
          )),
        }
      }

      (RuntimeValue::BOOL(val1), RuntimeValue::BOOL(val2)) => match self.token.get_value().as_str()
      {
        "&&" => Ok(RuntimeValue::BOOL(val1 && val2)),
        "||" => Ok(RuntimeValue::BOOL(val1 || val2)),
        _ => Err(format!(
          "Unsupported binary operator: '{}' between booleans, at position: {}",
          self.token.get_value(),
          self.token.get_position()
        )),
      },
      _ => Err(format!(
        "Type mismatch for binary operation {} at position: {}",
        self.token.get_value(),
        self.token.get_position()
      )),
    }
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

      TokenType::BINARYOP => self.eval_binary_op(),

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
