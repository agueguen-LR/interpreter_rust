//! Abstract Syntax Tree implementation and evaluation logic.
//!
//! This module defines the structure of the Abstract Syntax Tree (AST) used to represent
//! parsed code and provides evaluation functionality for the AST nodes.

use crate::context::Context;
use crate::token::Token;
use crate::token::TokenType;

use std::rc::Rc;

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
  // Usage of a reference count pointer to allow sharing ownership of AST nodes with Context,
  // notably for functions.
  children: Vec<Rc<ASTree>>,
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
    self.children.push(Rc::new(child));
  }

  /// Evaluates a binary operation between two integer values.
  ///
  /// # Arguments
  ///
  /// * `val1` - The first integer value.
  /// * `val2` - The second integer value.
  ///
  /// # Returns
  ///
  /// * `Ok(RuntimeValue)` if evaluation is successful.
  /// * `Err(String)` if an error occurs during evaluation.
  fn eval_binop_ints(&self, val1: i32, val2: i32) -> Result<RuntimeValue, String> {
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

  /// Evaluates a binary operation between two boolean values.
  ///
  /// # Arguments
  ///
  /// * `val1` - The first boolean value.
  /// * `val2` - The second boolean value.
  ///
  /// # Returns
  ///
  /// * `Ok(RuntimeValue)` if evaluation is successful.
  /// * `Err(String)` if an error occurs during evaluation.
  fn eval_binop_bools(&self, val1: bool, val2: bool) -> Result<RuntimeValue, String> {
    match self.token.get_value().as_str() {
      "&&" => Ok(RuntimeValue::BOOL(val1 && val2)),
      "||" => Ok(RuntimeValue::BOOL(val1 || val2)),
      _ => Err(format!(
        "Unsupported binary operator: '{}' between booleans, at position: {}",
        self.token.get_value(),
        self.token.get_position()
      )),
    }
  }

  /// Evaluates a binary operation between two string values.
  ///
  /// # Arguments
  ///
  /// * `val1` - The first string value.
  /// * `val2` - The second string value.
  ///
  /// # Returns
  ///
  /// * `Ok(RuntimeValue)` if evaluation is successful.
  /// * `Err(String)` if an error occurs during evaluation.
  fn eval_binop_strings(&self, val1: &String, val2: &String) -> Result<RuntimeValue, String> {
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

  /// Evaluates a binary operation ASTree node.
  ///
  /// # Returns
  ///
  /// * `Ok(RuntimeValue)` if evaluation is successful.
  /// * `Err(String)` if an error occurs during evaluation.
  fn eval_binary_op(&self, ctx: &mut Context) -> Result<RuntimeValue, String> {
    // Expecting two children from parser: left and right operands
    let param1: RuntimeValue = self.children[0].eval(ctx)?;
    let param2: RuntimeValue = self.children[1].eval(ctx)?;

    match (&param1, &param2) {
      (RuntimeValue::INTEGER(val1), RuntimeValue::INTEGER(val2)) => {
        self.eval_binop_ints(*val1, *val2)
      }

      (RuntimeValue::BOOL(val1), RuntimeValue::BOOL(val2)) => self.eval_binop_bools(*val1, *val2),

      (RuntimeValue::STRING(val1), RuntimeValue::STRING(val2)) => {
        self.eval_binop_strings(val1, val2)
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

  /// Evaluates an if-statement ASTree node.
  ///
  /// # Arguments
  ///
  /// * `ctx` - The context for variable bindings during evaluation.
  ///
  /// # Returns
  ///
  /// * `Ok(RuntimeValue)` if evaluation is successful.
  /// * `Err(String)` if an error occurs during evaluation.
  fn eval_if(&self, ctx: &mut Context) -> Result<RuntimeValue, String> {
    // Expecting two or three children from parser: condition, then-branch, else-branch (optional)
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
      // Then-branch
      self.children[1].eval(ctx)
    } else if self.children.len() == 3 {
      // Else-branch
      self.children[2].eval(ctx)
    } else {
      Ok(RuntimeValue::NULL)
    }
  }

  /// Evaluates a while-loop ASTree node.
  ///
  /// # Arguments
  ///
  /// * `ctx` - The context for variable bindings during evaluation.
  ///
  /// # Returns
  ///
  /// * `Ok(RuntimeValue::NULL)` if evaluation is successful.
  /// * `Err(String)` if an error occurs during evaluation.
  fn eval_while(&self, ctx: &mut Context) -> Result<RuntimeValue, String> {
    // Expecting two children from parser: condition and body
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

  /// Evaluates an assignment ASTree node.
  ///
  /// # Arguments
  ///
  /// * `ctx` - The context for variable bindings during evaluation.
  ///
  /// # Returns
  ///
  /// * `Ok(RuntimeValue::NULL)` if evaluation is successful.
  /// * `Err(String)` if an error occurs during evaluation.
  fn eval_assign(&self, ctx: &mut Context) -> Result<RuntimeValue, String> {
    // Expecting two children from parser: identifier and value
    let name = self.children[0].token.get_value();
    let value = self.children[1].eval(ctx)?;
    ctx.set_variable(name.clone(), value);
    Ok(RuntimeValue::NULL)
  }

  /// Evaluates a function definition ASTree node.
  ///
  /// # Arguments
  ///
  /// * `ctx` - The context for variable bindings during evaluation.
  ///
  /// # Returns
  ///
  /// * `Ok(RuntimeValue::NULL)` if evaluation is successful.
  /// * `Err(String)` if an error occurs during evaluation.
  fn eval_fn_def(&self, ctx: &mut Context) -> Result<RuntimeValue, String> {
    // Expecting 2 or more children from parser: function name, parameters..., body
    let name = self.children[0].token.get_value();
    let body = self.children.last().unwrap();
    let mut params: Vec<String> = Vec::new();
    for i in 1..(self.children.len() - 1) {
      params.push(self.children[i].token.get_value().clone());
    }
    // body.clone() is cheap due to Rc
    ctx.set_function(name.clone(), params, body.clone());
    Ok(RuntimeValue::NULL)
  }

  fn eval_fn_call(&self, ctx: &mut Context) -> Result<RuntimeValue, String> {
    // Expecting 0 or more children from parser: arguments...
    let func_body: Rc<ASTree> = match ctx.get_function_body(self.token.get_value()) {
      Option::Some(func) => func,
      Option::None => {
        return Err(format!(
          "Attempted to call unset function: '{}', at position: {}",
          self.token.get_value(),
          self.token.get_position()
        ));
      }
    };
    // Since a function body exists, parameters must also exist
    // Rc to take ownership to avoid mutability issues
    let func_params: Rc<Vec<String>> = ctx.get_function_params(self.token.get_value()).unwrap();
    ctx.push_scope();
    for (i, param_name) in func_params.iter().enumerate() {
      let arg_value = self.children[i].eval(ctx)?;
      ctx.set_variable(param_name.clone(), arg_value);
    }
    let result = func_body.eval(ctx);
    dbg!(&ctx);
    ctx.pop_scope();
    result
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
  pub fn eval(&self, ctx: &mut Context) -> Result<RuntimeValue, String> {
    match self.token.get_type() {
      TokenType::NUMERIC => match self.token.get_value().parse::<i32>() {
        Ok(result) => return Ok(RuntimeValue::INTEGER(result)),
        Err(error) => return Err(error.to_string()),
      },

      TokenType::STRING => Ok(RuntimeValue::STRING(self.token.get_value().clone())),

      TokenType::BINARYOP => self.eval_binary_op(ctx),

      TokenType::IDENTIFIER => {
        if self.children.len() == 0 {
          // Variable access
          return match ctx.get_variable(self.token.get_value()) {
            Option::Some(val) => Ok(val.clone()),
            Option::None => Err(format!(
              "Attempted to access unset identifier: '{}', at position: {}",
              self.token.get_value(),
              self.token.get_position()
            )),
          };
        } else {
          // Function call
          self.eval_fn_call(ctx)
        }
      }

      TokenType::IF => self.eval_if(ctx),

      TokenType::WHILE => self.eval_while(ctx),

      TokenType::ASSIGN => self.eval_assign(ctx),

      TokenType::FN => self.eval_fn_def(ctx),

      TokenType::BLOCK(make_scope) => {
        let mut last_value: RuntimeValue = RuntimeValue::NULL;
        if *make_scope {
          ctx.push_scope();
        }
        for child in &self.children {
          last_value = child.eval(ctx)?;
        }
        if *make_scope {
          dbg!(&ctx);
          ctx.pop_scope();
        }
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
