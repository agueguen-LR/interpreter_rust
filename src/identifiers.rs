//! Module for managing global identifiers in the AST.
//!
//! This module provides functionality to initialize, get, and set global identifiers

use crate::ast::RuntimeValue;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

/// A global, thread-safe map of identifiers to their corresponding values during runtime.
pub static IDENTIFIERS: OnceLock<Mutex<HashMap<String, RuntimeValue>>> = OnceLock::new();

/// Initializes the global IDENTIFIERS map if it hasn't been initialized yet.
pub fn init_identifiers() {
  match IDENTIFIERS.get() {
    Option::None => IDENTIFIERS.set(Mutex::new(HashMap::new())).unwrap(),
    _ => {}
  }
}

/// Gets the `RuntimeValue` associated with the given identifier key.
pub fn get_identifier(key: &String) -> Option<RuntimeValue> {
  IDENTIFIERS
    .get()
    .expect("IDENTIFIERS not initialized")
    .lock()
    .expect("Failed to lock IDENTIFIERS mutex whilst getting identifier")
    .get(key)
    .cloned()
}

/// Sets the `RuntimeValue` for the given identifier key.
pub fn set_identifier(key: String, value: RuntimeValue) {
  IDENTIFIERS
    .get()
    .expect("IDENTIFIERS not initialized")
    .lock()
    .expect("Failed to lock IDENTIFIERS mutex whilst setting identifier")
    .insert(key.to_string(), value);
}
