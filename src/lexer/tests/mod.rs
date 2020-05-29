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
fn lex_01() {
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

  assert_eq!(tokens[0].lexeme, "tekstiä1");
  assert_eq!(tokens[1].lexeme, "tekstiä2");

}

#[test]
fn lexer_from_another() {

  let src = "aaaabbbbcccc";

  let parent = &mut Lexer::new(src, State::Body);

  parent.lexeme_start += 4;

  let child = Lexer::new_from_lexer(parent, src, State::Inline);

  assert_eq!(4, child.lexeme_start);

}


#[test]
fn scan_un_list_item () {
  let src = "a
* asdsadasdsadadas
  adasdasdaDADasd";

  let toks = Lexer::new(src, State::Body).lex();

  println!("{:?}",toks);

  panic!();

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
