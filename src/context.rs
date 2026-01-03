//! Module for managing context during code interpretation.
//!
//! This module defines the `Context` struct, which is responsible for managing variable
//! bindings and scopes during the interpretation of code.

use crate::ast::ASTree;
use crate::ast::RuntimeValue;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Function {
  /// The parameter names of the function.
  // This is an Rc to avoid mutability issues during function calls.
  // (Cannot mutably set params within function's scope while iterating over immutable
  // get_function_params -> &Vec<String>)
  params: Rc<Vec<String>>,
  body: Rc<ASTree>,
}

/// Represents the context for variable bindings during code interpretation.
#[derive(Debug)]
pub struct Context {
  /// A stack of variable scopes, where each scope is a mapping from variable names to their
  /// values.
  variables: Vec<HashMap<String, RuntimeValue>>,
  /// A stack of function scopes.
  functions: Vec<HashMap<String, Function>>,
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
      functions: Vec::new(),
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
    for i in (0..self.variables.len()).rev() {
      if let Some(value) = self.variables[i].get(name) {
        return Some(value);
      }
    }
    Option::None
  }

  /// Sets a function in the current scope.
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the function to set.
  /// * `func_ast` - The AST representation of the function.
  pub fn set_function(&mut self, name: String, params: Vec<String>, body: Rc<ASTree>) {
    self.functions.last_mut().unwrap().insert(
      name,
      Function {
        params: Rc::new(params),
        body,
      },
    );
  }

  /// Retrieves the AST representation of a function body from the current scope or any enclosing
  /// scopes.
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the function to retrieve.
  ///
  /// # Returns
  ///
  /// * `Some(&ASTree)` if the function is found, or `None` if it is not found.
  pub fn get_function_body(&self, name: &String) -> Option<Rc<ASTree>> {
    dbg!(&self.functions);
    for i in (0..self.functions.len()).rev() {
      if let Some(func_ast) = self.functions[i].get(name) {
        return Some(func_ast.body.clone());
      }
    }
    Option::None
  }

  /// Retrieves the parameter names of a function from the current scope or any enclosing scopes.
  ///
  /// # Arguments
  ///
  /// * `name` - The name of the function to retrieve parameters for.
  ///
  /// # Returns
  ///
  /// * `Some(&Vec<String>)` if the function is found, or `None` if it is not found.
  pub fn get_function_params(&self, name: &String) -> Option<Rc<Vec<String>>> {
    for i in (0..self.functions.len()).rev() {
      if let Some(func) = self.functions[i].get(name) {
        return Some(func.params.clone());
      }
    }
    Option::None
  }

  /// Pushes a new scope onto the stack.
  pub fn push_scope(&mut self) {
    self.variables.push(HashMap::new());
    self.functions.push(HashMap::new());
  }

  /// Pops the current scope from the stack.
  pub fn pop_scope(&mut self) {
    self.variables.pop();
    self.functions.pop();
  }
}
