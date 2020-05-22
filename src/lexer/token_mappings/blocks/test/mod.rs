/// This is a test module for
/// the rST block mappings.

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

  println!("{}", list);

  let pattern:String = val_from_key(
    &TokenType::LiteralBlock, 
    BLOCK_RE_MAP).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
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

  println!("{}", list);

  let pattern:String = val_from_key(
    &TokenType::LineBlock, 
    BLOCK_RE_MAP).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}
