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
/// Tests the scanning of the entire source
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

  println!("{:?}",tokens);

  assert_eq!(tokens[0].t_type, TokenType::BlankLine);
  assert_eq!(tokens[1].t_type, TokenType::EqualsOverlinedHeading);
  assert_eq!(tokens[2].t_type, TokenType::BlankLine);
  assert_eq!(tokens[3].t_type, TokenType::CaretHeading);

}

#[test]
fn lexer_from_another() {

  let mut src_iter = "aaaabbbbcccc".chars();

  let pos = &mut Pos::new();

  let parent = &mut Lexer::new(&mut src_iter, pos, State::Body);

  parent.pos.pos += 4;
  parent.pos.row += 4;
  parent.pos.col += 5;

  // let child = Lexer::new_from_lexer(parent, src, State::Inline);

  // assert_eq!(4, child.lexeme_start);

  // assert_eq!(TokenType::Escape, child.inline_actions.first().unwrap().0);

  // assert_eq!(4, child.row);
  // assert_eq!(5, child.col);

}


#[test]
fn scan_un_list_items () {
  let mut src_iter = "  
  
* aaaabbbbcccc
  ccccbbbbaaaa

* xxxxyyyy
  yyyyxxxx'

".chars();

  let pos = &mut Pos::new();

  let mut lexer = Lexer::new(&mut src_iter, pos, State::Body);

  lexer.lex();

  let toks = lexer.tokens;

  println!("{:?}",toks);

  assert_eq!(TokenType::Text, toks[2].t_type);
  assert_eq!(TokenType::Text, toks[3].t_type);
  assert_eq!(TokenType::BlankLine, toks[4].t_type);
  assert_eq!(TokenType::Bullet, toks[5].t_type);
  assert_eq!(TokenType::Text, toks[6].t_type);
  assert_eq!(TokenType::Text, toks[7].t_type);

}

#[test]
fn scan_token() {
  let mut src_iter = "========\ntekstiä\n=========\n".chars();

  let pos = &mut Pos::new();
  let mut lexer = Lexer::new(&mut src_iter, pos, State::Body);
  lexer.scan_token();

  println!("{:?}", lexer.tokens);

  assert_eq!(lexer.tokens[0].lexeme, "tekstiä");

}
