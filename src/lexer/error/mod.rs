/// This is the error module for the Lexer.
/// Implements errors for each lexing function
/// (where necessary).

mod tests;

use std::{fmt};

#[derive(Debug)]
pub struct TokenizeError {
  pub row: usize,
  pub col: usize,
}

pub struct LexError {
  row: usize,
  col: usize,
}

impl<'t> TokenizeError  {
  pub fn new(row: &usize, col: &usize) -> TokenizeError {
    TokenizeError{
      row: row.clone(),
      col: col.clone(),
    }
  }
}

impl<'a> fmt::Display for TokenizeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TokenizeError: \
    no matching lexeme for <row, col> = <{}, {}>", self.row, self.col)
  }
}


impl<'a> LexError  {
  pub fn new(row: &'a usize, col: &'a usize) -> LexError {
    LexError{
      row: row.clone(),
      col: col.clone(),
    }
  }
}


impl<'a> fmt::Display for LexError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "LexError: \
    something went wrong because of <row, col> = <{}, {}>", self.row, self.col)
  }
}
