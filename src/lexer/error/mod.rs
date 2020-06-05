/// This is the error module for the Lexer.
/// Implements errors for each lexing function
/// (where necessary).

#[cfg(test)]
mod tests;

use std::{fmt};
use crate::lexer::position::Pos;

#[derive(Debug)]
pub struct TokenizeError {
  pub row: usize,
  pub col: usize,
}

pub struct LexError {
  row: usize,
  col: usize,
}

impl TokenizeError  {
  pub fn new(row: &usize, col: &usize) -> TokenizeError {
    TokenizeError{
      row: row.clone(),
      col: col.clone(),
    }
  }
}

impl fmt::Display for TokenizeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TokenizeError: \
    no matching lexeme for <row, col> = <{}, {}>", self.row, self.col)
  }
}


impl LexError  {
  pub fn new(pos: &Pos) -> LexError {
    LexError{
      row: pos.row,
      col: pos.col,
    }
  }
}


impl fmt::Display for LexError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "LexError: \
    something went wrong because of <row, col> = <{}, {}>", self.row, self.col)
  }
}
