/// This is the error module for the Lexer.
/// Implements errors for each lexing function
/// (where necessary).

use std::{fmt};

#[derive(Debug)]
pub struct TokenizeError {
  row: usize,
  col: usize,
}

impl TokenizeError {
  fn new(row: usize, col: usize) -> TokenizeError {
    TokenizeError{
      row: row,
      col: col
    }
  }
}

impl fmt::Display for TokenizeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TokenizeError: \
    no matching lexeme for <{}, {}>", self.row, self.col)
  }
}
