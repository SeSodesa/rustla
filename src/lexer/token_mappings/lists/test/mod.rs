/// Tests for list related regexes

use super::*;
use super::super::val_from_key;
use regex::Regex;

use crate::lexer::token_mappings;
#[cfg(test)]

#[test]
fn match_unnumbered_list_() {
  let list
    = "* Tässä on lista-alkio\n* Jos toinenkin.\n";

  let pattern:String = val_from_key(
    &TokenType::UnnumberedList, 
    LIST_RE_MAP).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}

#[test]
fn match_numbered_lrpar_list() {
  let list
    = "(1) Tässä on lista-alkio\n  (iv) Jos toinenkin.\n";

  let pattern:String = val_from_key(
    &TokenType::NumberedLRparList, 
    LIST_RE_MAP).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}


#[test]
fn match_numbered_rpar_list() {
  let list
    = "1) Tässä on lista-alkio\n  iv) Jos toinenkin.\n";

  let pattern:String = val_from_key(
    &TokenType::NumberedRparList, 
    LIST_RE_MAP).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}

#[test]
fn match_alpha_dot_list() {
  let list
    = "A. Tässä on lista-alkio\n  Z. Jos toinenkin.\n";

  let pattern:String = val_from_key(
    &TokenType::NoBolAlphaDotList, 
    LIST_RE_MAP).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}


#[test]
fn match_alpha_lrpar_list() {
  let list
    = "(A) Tässä on lista-alkio\n  (Z) Jos toinenkin.\n";

  let pattern:String = val_from_key(
    &TokenType::AlphaLRparList, 
    LIST_RE_MAP).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}


#[test]
fn match_alpha_rpar_list() {
  let list
    = "A) Tässä on lista-alkio\n  Z) Jos toinenkin.\n";

  let pattern:String = val_from_key(
    &TokenType::AlphaRparList, 
    LIST_RE_MAP).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}


#[test]
fn match_description_list() {
  let list
    = "what
    Definition lists associate a term with
    a definition.
  
  how
    The term is a one-line phrase, and the
    definition is one or more paragraphs or
    body elements, indented relative to the
    term. Blank lines are not allowed
    between term and definition.";

  let pattern:String = val_from_key(
    &TokenType::DefinitionList, 
    LIST_RE_MAP).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}

#[test]
fn match_field_list() {
  let list
    = ":Authors:
    Tony J. (Tibs) Ibbs,
    David Goodger

    (and sundry other good-natured folks)

:Version: 1.0 of 2001/08/08
:Dedication: To my father. ";

  let pattern:String = val_from_key(
    &TokenType::DefinitionList, 
    LIST_RE_MAP).unwrap();
  let re = Regex::new(pattern.as_str()).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}

