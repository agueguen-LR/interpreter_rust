/* Abstract syntax tree (AST) */

use crate::token::Token;
use crate::token::TokenType;
use crate::token::TypeValue;

#[derive(Debug)]
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

  pub fn append(&mut self, child: ASTree) {
    self.children.push(child);
  }

  pub fn eval(&mut self) -> Result<TypeValue, String> {
    match self.token.get_type() {
      TokenType::NUMERIC => match self.token.get_value().parse::<i32>() {
        Ok(result) => return Ok(TypeValue::INTEGER(result)),
        Err(error) => return Err(error.to_string()),
      },

      TokenType::BINARYOP => {
        if self.children.len() != 2 {
          return Err(format!(
            "{} {}",
            "Invalid amount of params passed to Binary Operation Evaluation, at position:",
            self.token.get_position()
          ));
        }
        let param1: i32 = match self.children[0].eval() {
          Err(error) => return Err(error),
          Ok(val) => match val {
            TypeValue::INTEGER(n) => n,
            _ => {
              return Err(format!(
                "{} {}",
                "Invalid type provided to Binary Operator, at position:",
                self.children[0].token.get_position()
              ));
            }
          },
        };
        let param2: i32 = match self.children[1].eval() {
          Err(error) => return Err(error),
          Ok(val) => match val {
            TypeValue::INTEGER(n) => n,
            _ => {
              return Err(format!(
                "{} {}",
                "Invalid type provided to Binary Operator, at position:",
                self.children[1].token.get_position()
              ));
            }
          },
        };

        match self.token.get_value().as_str() {
          "+" => return Ok(TypeValue::INTEGER(param1 + param2)),
          "-" => return Ok(TypeValue::INTEGER(param1 - param2)),
          "*" => return Ok(TypeValue::INTEGER(param1 * param2)),
          "/" => return Ok(TypeValue::INTEGER(param1 / param2)),
          _ => {
            return Err(format!(
              "{} {}",
              "Unexpected operator in BinOP evaluation, at position:",
              self.token.get_position(),
            ));
          }
        }
      }

      TokenType::IDENTIFIER => return Err(String::from("Not yet implemented")),

      TokenType::PRINT => {
        if self.children.len() != 1 {
          return Err(format!(
            "{} {}",
            "Invalid amount of params passed to print, at position:",
            self.token.get_position()
          ));
        }
        match self.children[0].eval() {
          Err(error) => return Err(error),
          Ok(val) => match val {
            TypeValue::INTEGER(n) => print!("{n}"),
            TypeValue::STRING(s) => print!("{s}"),
            _ => {
              return Err(format!(
                "{} {}",
                "Unsupported Print TypeValue, at position:",
                self.children[0].token.get_position()
              ));
            }
          },
        };
        return Ok(TypeValue::INTEGER(0));
      }

      _ => {
        return Err(format!(
          "{} {:?}",
          "Unexpected TokenType evaluated: ",
          self.token.get_type()
        ));
      }
    }
  }
}
