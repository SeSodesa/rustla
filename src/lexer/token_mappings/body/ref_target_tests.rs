/// Tests for regexes related to reference targets

use super::*;
use super::super::val_from_key;
use regex::Regex;

#[cfg(test)]

#[test]
#[test]
fn match_ref_target() {
  let list
    = "* Tässä on lista-alkio\n* Jos toinenkin.\n";

  let pattern:String = val_from_key(
    &TokenType::ReferenceTarget, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}