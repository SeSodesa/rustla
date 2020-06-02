/// Test module for Lexer errors

use super::*;
use super::super::*;

#[cfg(test)]

#[test]
/// A test for
/// crate::lexer::error::TokenizeError::new
fn new() {
  let row: usize = 1;
  let col: usize = 2;
  let tok_err
    = TokenizeError::new(&row, &col);
  assert_eq!(tok_err.row, row);
}

#[test]
/// A test for
/// crate::lexer::error::TokenizeError::fmt
fn fmt() {

  let pos = &mut Pos::new();

  let lex
    = Lexer::new("abc", pos, State::Body);
  let tok_err
    = TokenizeError::new(&lex.pos.row, &lex.pos.col);

  assert_eq!(
    format!(
      "TokenizeError: no matching lexeme for <row, col> = <{}, {}>",
      tok_err.row, tok_err.col
    ),
    "TokenizeError: no matching lexeme for <row, col> = <0, 0>"
  )
}

