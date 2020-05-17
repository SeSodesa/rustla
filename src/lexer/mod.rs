/// This is the lexer module of ruSTLa

pub mod token;
mod tests;

use std::fmt;
use crate::lexer::token::Token;

#[derive(PartialEq)]
pub struct Lexer {
  source: String,
  tokens: Vec<Token>,
  lexeme_start: usize,
  lexeme_current: usize,
  row: usize,
  col: usize,
}

impl fmt::Debug for Lexer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Lexer")
        .field("source", &self.source)
        .field("id", &self.tokens)
        .finish()
  }
}

/// Lexer type methods
impl Lexer {

  /// Lexer constructor
  pub fn new(source: String) -> Lexer {
    Lexer {
      source: source,
      tokens: Vec::new(),
      lexeme_start: 0,
      lexeme_current: 0,
      row:0,
      col: 0,
    }
  }

}
