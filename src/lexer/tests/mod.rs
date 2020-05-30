/// Tests for the lexer module

#[cfg(test)]

use super::*;

#[test]
/// A test for the Lexer constructor
 fn new() {
  let ls = "Aaa!";
  let lex = Lexer::new(ls, State::Body);
  assert_eq!(lex.tokens, Vec::new());
  assert_eq!(lex.lexeme_start, 0);
  assert_eq!(lex.lookahead, 0);
  assert_eq!(lex.row, 0);
  assert_eq!(lex.col, 0);
}


#[test]
/// Tests the scanning of the entire source
fn lex_section_titles() {
  let src = "
  
=====
  tekstiä1
========


tekstiä2
^^^^
  
  ";
  let lexer = Lexer::new(src, State::Body);

  println!("{}",src);

  let tokens = lexer.lex();

  println!("{:?}",tokens);

  assert_eq!(tokens[0].t_type, TokenType::BlankLine);
  assert_eq!(tokens[1].t_type, TokenType::EqualsOverlinedHeading);
  assert_eq!(tokens[2].t_type, TokenType::BlankLine);
  assert_eq!(tokens[3].t_type, TokenType::CaretHeading);

}

#[test]
fn lexer_from_another() {

  let src = "aaaabbbbcccc";

  let parent = &mut Lexer::new(src, State::Body);

  parent.lexeme_start += 4;
  parent.row += 4;
  parent.col += 5;

  // let child = Lexer::new_from_lexer(parent, src, State::Inline);

  // assert_eq!(4, child.lexeme_start);

  // assert_eq!(TokenType::Escape, child.inline_actions.first().unwrap().0);

  // assert_eq!(4, child.row);
  // assert_eq!(5, child.col);

}


#[test]
fn scan_un_list_items () {
  let src = "  
  
* aaaabbbbcccc
  ccccbbbbaaaa

* xxxxyyyy
  yyyyxxxx'

";

  let toks = Lexer::new(src, State::Body).lex();

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
  let src = "========\ntekstiä\n=========\n";
  let mut lexer = Lexer::new(src, State::Body);
  let mut chars = src.chars();
  lexer.scan_token(&mut chars);

  println!("{:?}", lexer.tokens);

  assert_eq!(lexer.tokens[0].lexeme, "tekstiä");

}
