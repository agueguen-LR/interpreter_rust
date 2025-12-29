#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenType {
  NUMERIC,
  IDENTIFIER,
  BINARYOP,
  IF,
  WHILE,
  FOR,
  ELSE,
  INVALID,
  PRINT,
}

#[derive(Debug)]
pub enum TypeValue {
  INTEGER(i32),
  STRING(String),
  NULL,
  BOOL(bool),
}

#[derive(Clone, Debug)]
pub struct Token {
  token_type: TokenType,
  value: String,
  pos: usize,
}

impl Token {
  pub fn new(token_type: TokenType, value: String, position: usize) -> Token {
    Token {
      token_type: token_type,
      value: value,
      pos: position,
    }
  }

  pub fn get_value(&self) -> &String {
    &self.value
  }

  pub fn get_type(&self) -> &TokenType {
    &self.token_type
  }

  pub fn get_position(&self) -> &usize {
    &self.pos
  }
}
