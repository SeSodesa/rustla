/// Tests for regexes related to reference targets

use super::*;
use super::super::val_from_key;
use regex::Regex;

#[cfg(test)]

#[test]
fn match_ref_target() {
  let list
    = "adasdasds
    adasdasdsad
    aasdasd
    
    .. _asdasd: asdasdsadasd
    asdasdasd
    adasdasdasd
    ";

  let pattern = val_from_key(
    &TokenType::ReferenceTarget, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}


#[test]
fn match_footnote_or_citation_target() {
  let list
    = "
   dadasdasdasd
   adasdssaddasd
    .. [asdasd]:
    
    adadasadadad";

  let pattern = val_from_key(
    &TokenType::FootnoteOrCitationTarget, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}

#[test]
fn match_substitution_definition() {
  let list
    = "
   dadasdasdasd
   adasdssaddasd
    .. |asdasd|dasdasdasda::dasdasdasdad
    
    adadasadadad";

  let pattern = val_from_key(
    &TokenType::SubstitutionDefinition, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}
