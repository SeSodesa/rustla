/// Tests for block elements


use super::*;
use super::super::*;
use regex::Regex;

#[cfg(test)]

#[test]
fn match_literal_block() {
  let list
  = "asdasd::

  asdasdads

dadasdd  ";

  let pattern = val_from_key(
    &TokenType::LiteralBlock, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}

#[test]
fn match_per_line_literal_block() {
  let list
  = "asdasd::

> asdasdads
>   dadasdd

adasdasd
";

  let pattern = val_from_key(
    &TokenType::PerLineLiteralBlock, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}


#[test]
fn match_line_block() {
  let list
  = "asdasd::

| asdasdasfasf
| asdasfasfasdfa
|
|   asdafasf
    asdasdasda

dasdasd";

  let pattern = val_from_key(
    &TokenType::LineBlock, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}

#[test]
fn match_paragraph() {
  let list
  = "asdasd::

adasfasfsadf
asdfsadfsadgsgsggasgd
  asdfsafsdfasdafa 
   asdfsafsaf

dasdasd";

  let pattern = val_from_key(
    &TokenType::Paragraph, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}


#[test]
fn paragraph_01 () {

  let mut src_iter = "
  
  
asdasdasdasd
asdasdasdasdasdasd
asdasdasdasdads
asdasdasdd

".chars();

  let pos = &mut Pos::new();

  let mut lexer = Lexer::new(&mut src_iter, pos, State::Body);

  lexer.lex();

  let toks = lexer.tokens;

  println!("{:#?}", toks);

  assert_eq!(toks[1].t_type, TokenType::BlankLines);
  assert_eq!(toks[2].t_type, TokenType::Text);
  assert_eq!(toks[3].t_type, TokenType::Newline);
  assert_eq!(toks[4].t_type, TokenType::Text);
  assert_eq!(toks[5].t_type, TokenType::Newline);
  assert_eq!(toks[6].t_type, TokenType::Text);
  assert_eq!(toks[7].t_type, TokenType::Newline);
  assert_eq!(toks[8].t_type, TokenType::Text);
  assert_eq!(toks[9].t_type, TokenType::BlankLines);

}