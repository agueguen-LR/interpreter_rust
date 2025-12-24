use crate::ast::ASTree;
use crate::ast::Token;
use crate::ast::TokenType;

fn match_keyword(ident: &str) -> TokenType {
  match ident {
    "if" => TokenType::IF,
    "else" => TokenType::ELSE,
    "while" => TokenType::WHILE,
    "for" => TokenType::FOR,
    _ => TokenType::IDENTIFIER,
  }
}

fn match_operator(character: char) -> bool {
  match character {
    '+' => true,
    '-' => true,
    '*' => true,
    '/' => true,
    '=' => true,
    _ => false,
  }
}

pub fn tokenize(input: String) -> Result<Vec<Token>, String> {
  let mut output: Vec<Token> = Vec::new();
  let mut state: TokenType = TokenType::INVALID;
  let mut value: String = String::new();

  for (i, character) in input.chars().enumerate() {
    if character == ' ' || character == '\n' || match_operator(character) {
      if value.len() != 0 {
        if state == TokenType::IDENTIFIER {
          state = match_keyword(value.as_str());
        }
        output.push(Token::new(state.clone(), value.clone()));
        state = TokenType::INVALID;
        value = String::new();
      }
      if match_operator(character) {
        output.push(Token::new(TokenType::BINARYOP, character.to_string()));
      }
      continue;
    }

    match state {
      TokenType::NUMERIC => {
        if !character.is_ascii_digit() {
          return Result::Err(String::from(
            "Identifier cannot start with a number, invalid input at index: {i}",
          ));
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
      _ => {
        return Result::Err(String::from(
          "Unexpected state reached during tokenization.",
        ));
      }
    }
  }
  Result::Ok(output)
}

pub fn build_astree(tokens: Vec<Token>) -> ASTree {
  let output: ASTree = ASTree::new(Token::new(TokenType::START, "START".to_string()));
  let state: TokenType = TokenType::INVALID;
  for token in tokens {
    match state {
      TokenType::IDENTIFIER => {}
      _ => {}
    }
  }
  output
}
