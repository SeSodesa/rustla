/// This is the token module used by
/// the ruSTLa Lexer and Parser

mod tests;

use std::fmt;

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
  // For testing
  Test,

  // Closing symbols
  LParenth, RSquareBrack, RWavyBrack, RAngleBrack,
  RSingleQuot, RDoubleQuot, RDoubleAngleBrack,
  Exclamation, Question,

  // Unicode delimiters
  Hyphen, NonBreakingHyphen, FigureDash, EnDash,
  EmDash, NonBreakingSpace,

  //
}
