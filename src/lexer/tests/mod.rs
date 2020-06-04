/// Tests for the lexer module

#[cfg(test)]

use super::*;

#[test]
/// A test for the Lexer constructor
 fn new() {
  let mut src_iter = "Aaa!".chars();

  let pos = &mut Pos::new();

  let lex = Lexer::new(&mut src_iter, pos, State::Body);
  assert_eq!(lex.tokens, Vec::new());
  assert_eq!(lex.pos.pos, 0);
  assert_eq!(lex.pos.lookahead, 0);
  assert_eq!(lex.pos.row, 0);
  assert_eq!(lex.pos.col, 0);
}
