#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenType {
  NUMERIC,
  IDENTIFIER,
  BINARYOP,
  NULL,
  IF,
  WHILE,
  FOR,
  ELSE,
  INVALID,
  START,
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
}

impl Token {
  pub fn new(token_type: TokenType, value: String) -> Token {
    Token {
      token_type: token_type,
      value: value,
    }
  }

  pub fn get_value(&self) -> String {
    self.value.clone()
  }

  pub fn get_type(&self) -> TokenType {
    self.token_type
  }
}
