/// This is the error module for the Lexer.
/// Implements errors for each lexing function
/// (where necessary).

use std::{fmt};

#[derive(Debug)]
pub struct TokenizeError <'a> {
  row: &'a usize,
  col: &'a usize,
}

impl<'a> TokenizeError<'a>  {
  pub fn new(row: &'a usize, col: &'a usize) -> TokenizeError<'a> {
    TokenizeError{
       // Borrows need to be dereferenced with *
      row: row,
      col: col
    }
  }
}

impl fmt::Display for TokenizeError <'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TokenizeError: \
    no matching lexeme for <{}, {}>", self.row, self.col)
  }
}
