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
  assert_eq!(lex.lookahead, 0);
  assert_eq!(lex.row, 0);
  assert_eq!(lex.col, 0);
}


#[test]
/// Tests the scanning of the entire source
fn scan_tokens() {
  panic!();
}


#[test]
/// Tests the scanning of a single token
fn scan_token() {
  panic!();
}


#[test]
/// Test the advancement of the
/// "lexing buffer"
fn advance_lookahead() {
  let mut lex = Lexer::new(String::from(""));
  lex.advance();
  assert_eq!(lex.lookahead, 1);
}


#[test]
/// Test the advancement of the
/// "lexing buffer"
fn advance_char() {
  let mut lex = Lexer::new(String::from("abc"));
  let c:char = lex.advance().unwrap();
  assert_eq!(c,'a');
}


#[test]
/// Tests the addition of a token to
/// the Lexer field `tokens`
fn add_token() {
  panic!();
}

#[test]
/// A test for finding the EOF
fn is_at_eof() {
  panic!();
}
