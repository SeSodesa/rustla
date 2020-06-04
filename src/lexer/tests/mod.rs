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


#[test]
fn lex_section_titles() {

  let mut src_iter = "
  
=====
  tekstiä1
========


tekstiä2
^^^^
  
  ".chars();

  let pos = &mut Pos::new();

  let mut lexer = Lexer::new(&mut src_iter, pos, State::Body);

  lexer.lex();

  let tokens = lexer.tokens;

  println!("{:#?}",tokens);

  assert_eq!(tokens[0].t_type, TokenType::BlankLines);
  assert_eq!(tokens[1].t_type, TokenType::EqualsOverlinedHeading);
  assert_eq!(tokens[2].t_type, TokenType::BlankLines);
  assert_eq!(tokens[3].t_type, TokenType::CaretHeading);

}
