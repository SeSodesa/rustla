/// Tests for the lexer module

#[cfg(test)]
use super::*;

#[test]
/// A test for the Lexer constructor
fn new() {
  let ls = String::from("Aaa!");
  let lex = Lexer::new(ls);
  assert_eq!(lex.tokens, Vec::new());
  assert_eq!(lex.lexeme_start, 0);
  assert_eq!(lex.lexeme_current, 0);
  assert_eq!(lex.row, 0);
  assert_eq!(lex.col, 0);
}
