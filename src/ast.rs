/* Abstract syntax tree (AST) */

use crate::token::Token;
use crate::token::TokenType;
use crate::token::TypeValue;

#[derive(Clone, Debug)]
pub struct ASTree {
  children: Vec<ASTree>,
  token: Token,
}

impl ASTree {
  pub fn new(token: Token) -> ASTree {
    ASTree {
      children: Vec::new(),
      token: token,
    }
  }

  pub fn get_token(&self) -> Token {
    self.token.clone()
  }

  pub fn append(&mut self, child: ASTree) {
    self.children.push(child);
  }

  pub fn eval(&mut self) -> Result<TypeValue, String> {
    match self.token.get_type() {
      TokenType::START => {
        let mut exit_code = 0;
        for child in &mut self.children {
          match child.eval() {
            Ok(_) => continue,
            Err(error) => {
              print!("Error during execution: {error}");
              exit_code = -1;
              break;
            }
          };
        }
        return Ok(TypeValue::INTEGER(exit_code));
      }

      TokenType::NUMERIC => match self.token.get_value().parse::<i32>() {
        Ok(result) => return Ok(TypeValue::INTEGER(result)),
        Err(error) => return Err(error.to_string()),
      },

      TokenType::BINARYOP => {
        if self.children.len() != 2 {
          return Err(String::from(
            "Invalid amount of params passed to Binary Operation Evaluation",
          ));
        }
        let param1: i32 = match self.children[0].eval() {
          Err(error) => return Err(error),
          Ok(val) => match val {
            TypeValue::INTEGER(n) => n,
            _ => return Err(String::from("Invalid type provided to Binary Operator")),
          },
        };
        let param2: i32 = match self.children[1].eval() {
          Err(error) => return Err(error),
          Ok(val) => match val {
            TypeValue::INTEGER(n) => n,
            _ => return Err(String::from("Invalid type provided to Binary Operator")),
          },
        };

        match self.token.get_value().as_str() {
          "+" => return Ok(TypeValue::INTEGER(param1 + param2)),
          "-" => return Ok(TypeValue::INTEGER(param1 - param2)),
          "*" => return Ok(TypeValue::INTEGER(param1 * param2)),
          "/" => return Ok(TypeValue::INTEGER(param1 / param2)),
          _ => return Err(String::from("Unexpected operator in BinOP evaluation")),
        }
      }

      TokenType::IDENTIFIER => return Err(String::from("Not yet implemented")),

      TokenType::PRINT => {
        if self.children.len() != 1 {
          return Err(String::from("Invalid amount of params passed to print"));
        }
        match self.children[0].eval() {
          Err(error) => return Err(error),
          Ok(val) => match val {
            TypeValue::INTEGER(n) => print!("{n}"),
            TypeValue::STRING(s) => print!("{s}"),
            _ => return Err(String::from("Unsupported Print TypeValue")),
          },
        };
        return Ok(TypeValue::INTEGER(0));
      }

      _ => return Err(String::from("Unexpected TokenType evaluated")),
    }
  }
}
