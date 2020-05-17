/// This is the token module used by
/// the ruSTLa Lexer and Parser
/// 
use std::fmt;

/// Token is a token of type TokenType
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

impl Token {
  pub fn new(t_type: TokenType, lexeme: String, line: usize) -> Token {
    Token{
      t_type: t_type,
      lexeme: lexeme,
      line: line
    }
  }
}

#[derive(Debug)]
/// TokenType lists the possible token types
pub enum TokenType{

}

#[cfg(test)]
/// Tests for Token
mod tests {

  use super::*;

}

