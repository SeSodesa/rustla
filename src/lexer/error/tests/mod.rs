/// Test module for Lexer errors

use super::*;

#[cfg(test)]

#[test]
/// A test for
/// crate::lexer::error::TokenizeError::new
fn new() {
  let row: usize = 1;
  let col: usize = 2;
  let tok_err
    = TokenizeError::new(&row, &col);
  assert_eq!(tok_err.row, &row);
}
