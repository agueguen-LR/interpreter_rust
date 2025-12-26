use crate::token::Token;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

pub static IDENTIFIERS: OnceLock<Mutex<HashMap<String, Token>>> = OnceLock::new();

macro_rules! get_identifier {
  ($key:expr) => {
    IDENTIFIERS
      .get()
      .expect("IDENTIFIERS not initialized")
      .lock()
      .unwrap()
      .get($key)
  };
}

macro_rules! set_identifier {
  ($key:expr, $value:expr) => {
    IDENTIFIERS
      .get()
      .expect("IDENTIFIERS not initialized")
      .lock()
      .unwrap()
      .insert($key.to_string(), $value)
  };
}
