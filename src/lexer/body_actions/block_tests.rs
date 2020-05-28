/// Tests for block elements


use super::*;
use super::super::val_from_key;
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
