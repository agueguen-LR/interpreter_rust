/* struct declaration and methods for an abstract syntax tree (AST) */

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
  START,
}

#[derive(Clone, Debug)]
pub struct Token {
  token_type: TokenType,
  value: String,
}

#[derive(Clone, Debug)]
pub struct ASTree {
  children: Vec<ASTree>,
  token: Token,
}

impl Token {
  pub fn new(token_type: TokenType, value: String) -> Token {
    Token {
      token_type: token_type,
      value: value,
    }
  }

  pub fn eval(&mut self, tree_pos: ASTree) -> Result<i32, String> {
    match self.token_type {
      TokenType::NUMERIC => match self.value.parse::<i32>() {
        Ok(result) => return Ok(result),
        Err(error) => return Err(error.to_string()),
      },

      TokenType::BINARYOP => {
        if tree_pos.children.len() != 2 {
          return Err(String::from(
            "Invalid amount of params passed to Binary Operation Evaluation",
          ));
        }
        let param1: i32 = tree_pos.children[0]
          .clone()
          .token
          .eval(tree_pos.children[0].clone())
          .expect("Child 1 didn't evaluate to i32 in BinOP evaluation");
        let param2: i32 = tree_pos.children[1]
          .clone()
          .token
          .eval(tree_pos.children[1].clone())
          .expect("Child 2 didn't evaluate to i32 in BinOP evaluation");

        match self.value.as_str() {
          "+" => return Ok(param1 + param2),
          "-" => return Ok(param1 - param2),
          "*" => return Ok(param1 * param2),
          "/" => return Ok(param1 / param2),
          _ => return Err(String::from("Unexpected operator in BinOP evaluation")),
        }
      }

      TokenType::IDENTIFIER => return Err(String::from("Not yet implemented")),

      _ => return Err(String::from("Unexpected TokenType evaluated")),
    }
  }
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

  pub fn eval(&mut self) {
    match self.token.eval(self.clone()) {
      Ok(exit_code) => {
        print!("{exit_code}")
      }
      Err(error) => {
        panic!("{error}")
      }
    }
  }
}
