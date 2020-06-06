/// This is the error module for the parser.
/// Implements errors for each lexing function
/// (where necessary).

#[cfg(test)]
mod tests;

use std::{fmt};
use crate::parser::position::Pos;

#[derive(Debug)]
pub struct TokenizeError {
  pub row: usize,
  pub col: usize,
}

pub struct ParseError {
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


impl ParseError  {
  pub fn new(pos: &Pos) -> Self {
    ParseError{
      row: pos.row,
      col: pos.col,
    }
  }
}


impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "parserror: \
    something went wrong because of <row, col> = <{}, {}>", self.row, self.col)
  }
}
