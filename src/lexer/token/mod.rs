/// This is the token module used by
/// the ruSTLa Lexer and Parser
/// 
use std::fmt;
use std::cmp;

#[derive(PartialEq)]
/// Token is a token of type `TokenType`
pub struct Token {
  t_type: TokenType,
  lexeme: String,
  line: usize,
}


impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Token")
      .field("t_type", &self.t_type)
      .field("t_type", &self.lexeme)
      .field("t_type", &self.line)
      .finish()
  }
}

/// Methods for the `Token` type
impl Token {
  pub fn new(t_type: TokenType, lexeme: String, line: usize) -> Token {
    Token{
      t_type: t_type,
      lexeme: lexeme,
      line: line
    }
  }
}

#[derive(Debug, PartialEq)]
/// TokenType lists the possible `Token` types
pub enum TokenType{
  Test
}

#[cfg(test)]
/// Tests for `Token` methods
mod tests {

  use super::*;

  /// Tests the constructor
  fn new(){
    let t = Token::new(TokenType::Test, String::from("test"), 3);
    assert_eq!(t.t_type, TokenType::Test);
  }

}

