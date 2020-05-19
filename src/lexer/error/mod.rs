/// This is the error module for the Lexer.
/// Implements errors for each lexing function
/// (where necessary).

mod tests;

use std::{fmt};

#[derive(Debug)]
pub struct TokenizeError <'a> {
  row: &'a usize,
  col: &'a usize,
}

pub struct LexError <'a> {
  row: &'a usize,
  col: &'a usize,
}

impl<'a> TokenizeError<'a>  {
  pub fn new(row: &'a usize, col: &'a usize) -> TokenizeError<'a> {
    TokenizeError{
      row: row,
      col: col
    }
  }
}

impl<'a> fmt::Display for TokenizeError <'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TokenizeError: \
    no matching lexeme for <row, col> = <{}, {}>", self.row, self.col)
  }
}


impl<'a> LexError<'a>  {
  pub fn new(row: &'a usize, col: &'a usize) -> LexError<'a> {
    LexError{
      row: row,
      col: col
    }
  }
}


impl<'a> fmt::Display for LexError <'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "LexError: \
    something went wrong because of <row, col> = <{}, {}>", self.row, self.col)
  }
}
