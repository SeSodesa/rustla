/// This submodule contains tests related to
/// rST comments.

use super::*;
use super::super::val_from_key;
use regex::Regex;

#[cfg(test)]

#[test]
fn match_comment() {
  let text = "
  asdasdasfd<d<
    aDADADADAD

.. asdasdQDAd
  adaDADasdasdda
  DADDSADADAD

ASDADADADADas
  ";

  let pattern:String = val_from_key(
    &TokenType::Comment, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
  if !re.is_match(text) {
    panic!();
  }
}
