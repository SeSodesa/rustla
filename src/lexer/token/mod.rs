/// This is the token module used by
/// the ruSTLa Lexer and Parser
/// 
use std::fmt;

pub struct Token {

}

impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Token")
        .finish()
  }
}