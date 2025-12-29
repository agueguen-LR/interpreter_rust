use crate::token::TypeValue;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

pub static IDENTIFIERS: OnceLock<Mutex<HashMap<String, TypeValue>>> = OnceLock::new();

pub fn init_identifiers() {
  match IDENTIFIERS.get() {
    None => IDENTIFIERS.set(Mutex::new(HashMap::new())).unwrap(),
    _ => {}
  }
}

pub fn get_identifier(key: &String) -> Option<TypeValue> {
  IDENTIFIERS
    .get()
    .expect("IDENTIFIERS not initialized")
    .lock()
    .unwrap()
    .get(key)
    .cloned()
}

pub fn set_identifier(key: String, value: TypeValue) {
  IDENTIFIERS
    .get()
    .expect("IDENTIFIERS not initialized")
    .lock()
    .unwrap()
    .insert(key.to_string(), value);
}
