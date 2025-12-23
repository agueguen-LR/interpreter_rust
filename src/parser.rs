#[derive(Copy, Clone, PartialEq, Debug)]
enum TokenType {
  NUMERIC,
  IDENTIFIER,
  ADD,
  SUB,
  MULT,
  DIV,
  IF,
  WHILE,
  FOR,
  ELSE,
  INVALID,
  START,
}

#[derive(Clone, Debug)]
pub struct Token {
  token_type: TokenType,
  value: String,
}

#[derive(Clone, Debug)]
struct ASTree {
  children: Vec<ASTree>,
  token: Token,
}

impl ASTree {
  fn new() -> ASTree {
    ASTree {
      children: Vec::new(),
      token: Token {
        token_type: TokenType::START,
        value: String::new(),
      },
    }
  }
}

fn match_keyword(ident: &str) -> TokenType {
  match ident {
    "if" => TokenType::IF,
    "else" => TokenType::ELSE,
    "while" => TokenType::WHILE,
    "for" => TokenType::FOR,
    _ => TokenType::IDENTIFIER,
  }
}

fn match_operator(character: char) -> TokenType {
  match character {
    '+' => TokenType::ADD,
    '-' => TokenType::SUB,
    '*' => TokenType::MULT,
    '/' => TokenType::DIV,
    _ => TokenType::INVALID,
  }
}

pub fn tokenize(input: String) -> Result<Vec<Token>, String> {
  let mut output: Vec<Token> = Vec::new();
  let mut state: TokenType = TokenType::INVALID;
  let mut value: String = String::new();

  for (i, character) in input.chars().enumerate() {
    let operator_type = match_operator(character);
    if character == ' ' || character == '\n' || operator_type != TokenType::INVALID {
      if value.len() != 0 {
        if state == TokenType::IDENTIFIER {
          state = match_keyword(value.as_str());
        }
        output.push(Token {
          token_type: state.clone(),
          value: value.clone(),
        });
        state = TokenType::INVALID;
        value = String::new();
      }
      if operator_type != TokenType::INVALID {
        output.push(Token {
          token_type: operator_type,
          value: character.to_string(),
        });
      }
      continue;
    }

    match state {
      TokenType::NUMERIC => {
        if !character.is_ascii_digit() {
          return Result::Err(
            "Identifier cannot start with a number, invalid input at index: {i}".to_string(),
          );
        }
        value.push(character);
      }
      TokenType::IDENTIFIER => {
        value.push(character);
      }
      TokenType::INVALID => {
        if character.is_ascii_digit() {
          state = TokenType::NUMERIC;
          value.push(character);
        }
        if character.is_ascii_alphabetic() {
          state = TokenType::IDENTIFIER;
          value.push(character);
        }
      }
      _ => {}
    }
  }
  Result::Ok(output)
}

pub fn build_astree(tokens: Vec<Token>) -> ASTree {
  let output: ASTree = ASTree::new();
  for token in tokens {}
  output
}
