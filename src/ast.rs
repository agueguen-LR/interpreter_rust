//! Abstract Syntax Tree implementation and evaluation logic.
//!
//! This module defines the structure of the Abstract Syntax Tree (AST) used to represent
//! parsed code and provides evaluation functionality for the AST nodes.

use crate::context::Context;
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
  fn eval_binary_op(&mut self, ctx: &mut Context) -> Result<RuntimeValue, String> {
    if self.children.len() != 2 {
      return Err(format!(
        "Invalid amount of params passed to Binary Operation Evaluation, at position: {}",
        self.token.get_position()
      ));
    }
    let param1: RuntimeValue = self.children[0].eval(ctx)?;
    let param2: RuntimeValue = self.children[1].eval(ctx)?;

    match (&param1, &param2) {
      (RuntimeValue::INTEGER(val1), RuntimeValue::INTEGER(val2)) => {
        match self.token.get_value().as_str() {
          "+" => Ok(RuntimeValue::INTEGER(val1 + val2)),
          "-" => Ok(RuntimeValue::INTEGER(val1 - val2)),
          "*" => Ok(RuntimeValue::INTEGER(val1 * val2)),
          "/" => {
            if *val2 == 0 {
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
        "&&" => Ok(RuntimeValue::BOOL(*val1 && *val2)),
        "||" => Ok(RuntimeValue::BOOL(*val1 || *val2)),
        _ => Err(format!(
          "Unsupported binary operator: '{}' between booleans, at position: {}",
          self.token.get_value(),
          self.token.get_position()
        )),
      },

      (RuntimeValue::STRING(val1), RuntimeValue::STRING(val2)) => {
        match self.token.get_value().as_str() {
          "+" => Ok(RuntimeValue::STRING(format!("{}{}", val1, val2))),
          "==" => Ok(RuntimeValue::BOOL(val1 == val2)),
          "!=" => Ok(RuntimeValue::BOOL(val1 != val2)),
          _ => Err(format!(
            "Unsupported binary operator: '{}' between strings, at position: {}",
            self.token.get_value(),
            self.token.get_position()
          )),
        }
      }

      _ => Err(format!(
        "Type mismatch for binary operation {} at position: {}\n Left operand type: {:?}\n Right operand type: {:?}",
        self.token.get_value(),
        self.token.get_position(),
        param1,
        param2
      )),
    }
  }

  /// Evaluates the ASTree node and returns the resulting RuntimeValue.
  ///
  /// # Arguments
  ///
  /// * `ctx` - The context for variable bindings during evaluation.
  ///
  /// # Returns
  ///
  /// * `Ok(RuntimeValue)` if evaluation is successful.
  /// * `Err(String)` if an error occurs during evaluation.
  pub fn eval(&mut self, ctx: &mut Context) -> Result<RuntimeValue, String> {
    match self.token.get_type() {
      TokenType::NUMERIC => match self.token.get_value().parse::<i32>() {
        Ok(result) => return Ok(RuntimeValue::INTEGER(result)),
        Err(error) => return Err(error.to_string()),
      },

      TokenType::STRING => Ok(RuntimeValue::STRING(self.token.get_value().clone())),

      TokenType::BINARYOP => self.eval_binary_op(ctx),

      TokenType::IDENTIFIER => match ctx.get_variable(self.token.get_value()) {
        Option::Some(val) => Ok(val.clone()),
        Option::None => Err(format!(
          "Attempted to access unset identifier: '{}', at position: {}",
          self.token.get_value(),
          self.token.get_position()
        )),
      },

      TokenType::IF => {
        if !(self.children.len() == 2 || self.children.len() == 3) {
          return Err(format!(
            "Invalid children count passed to If ASTree, position: {}",
            self.token.get_position()
          ));
        }
        let condition_result: bool = match self.children[0].eval(ctx)? {
          RuntimeValue::BOOL(val) => val,
          other => {
            return Err(format!(
              "If condition didn't evaluate to Boolean value, is: {:?}, at position {}",
              other,
              self.token.get_position()
            ));
          }
        };

        if condition_result {
          self.children[1].eval(ctx)
        } else if self.children.len() == 3 {
          self.children[2].eval(ctx)
        } else {
          Ok(RuntimeValue::NULL)
        }
      }

      TokenType::WHILE => {
        if self.children.len() != 2 {
          return Err(format!(
            "Invalid children count passed to While ASTree, position: {}",
            self.token.get_position()
          ));
        }
        while match self.children[0].eval(ctx)? {
          RuntimeValue::BOOL(val) => val,
          other => {
            return Err(format!(
              "While condition didn't evaluate to Boolean value, is: {:?}, at position {}",
              other,
              self.token.get_position()
            ));
          }
        } {
          self.children[1].eval(ctx)?;
        }
        Ok(RuntimeValue::NULL)
      }

      TokenType::ASSIGN => {
        if self.children.len() != 2 {
          return Err(format!(
            "Invalid children count passed to Assign ASTree, position: {}",
            self.token.get_position()
          ));
        }
        let name = self.children[0].token.get_value().clone();
        let value = self.children[1].eval(ctx)?;
        ctx.set_variable(name, value.clone());
        Ok(RuntimeValue::BOOL(true))
      }

      TokenType::BLOCK => {
        let mut last_value: RuntimeValue = RuntimeValue::NULL;
        ctx.push_scope();
        for child in &mut self.children {
          last_value = child.eval(ctx)?;
        }
        dbg!(&ctx);
        ctx.pop_scope();
        Ok(last_value)
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
