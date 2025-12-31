//! Module for managing context during code interpretation.
//!
//!! This module defines the `Context` struct, which is responsible for managing variable
//! bindings and scopes during the interpretation of code.

use crate::ast::RuntimeValue;

use std::collections::HashMap;

/// Represents the context for variable bindings during code interpretation.
#[derive(Debug)]
pub struct Context {
  /// A stack of variable scopes, where each scope is a mapping from variable names to their
  /// values.
  variables: Vec<HashMap<String, RuntimeValue>>,
}

impl Context {
  /// Creates a new empty `Context` instance.
  ///
  /// # Returns
  ///
  /// * A new `Context` instance, starting with an empty global scope.
  pub fn new() -> Context {
    Context {
      variables: Vec::new(),
    }
  }

  /// Sets a variable in the current scope.
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the variable to set.
  /// * `value` - The value to assign to the variable.
  pub fn set_variable(&mut self, name: String, value: RuntimeValue) {
    self.variables.last_mut().unwrap().insert(name, value);
  }

  /// Retrieves the value of a variable from the current scope or any enclosing scopes.
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the variable to retrieve.
  ///
  /// # Returns
  ///
  /// * `Some(&RuntimeValue)` if the variable is found, or `None` if it is not found.
  pub fn get_variable(&self, name: &String) -> Option<&RuntimeValue> {
    for i in (0..self.variables.len() - 1).rev() {
      if let Some(value) = self.variables[i].get(name) {
        return Some(value);
      }
    }
    Option::None
  }

  /// Pushes a new variable scope onto the stack.
  pub fn push_scope(&mut self) {
    self.variables.push(HashMap::new());
  }

  /// Pops the current variable scope from the stack.
  pub fn pop_scope(&mut self) {
    self.variables.pop();
  }
}
