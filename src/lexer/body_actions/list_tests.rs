/// Tests related to list mappings

use super::*;
use super::super::*;
use crate::lexer::val_from_key;
use regex::Regex;

#[cfg(test)]


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

  println!("{:#?}",toks);

  assert_eq!(TokenType::UnnumberedList, toks[0].t_type);

}


#[test]
fn match_unnumbered_list_() {
  let list
    = "* Tässä on lista-alkio\n* Jos toinenkin.\n";

  let pattern: &'static str = val_from_key(
    &TokenType::UnnumberedList, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}

#[test]
fn match_numbered_lrpar_list() {
  let list
    = "(1) Tässä on lista-alkio\n  (iv) Jos toinenkin.\n";

  let pattern = val_from_key(
    &TokenType::NumberedLRparList, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}


#[test]
fn match_numbered_rpar_list() {
  let list
    = "1) Tässä on lista-alkio\n  iv) Jos toinenkin.\n";

  let pattern = val_from_key(
    &TokenType::NumberedRparList, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}

#[test]
fn match_alpha_dot_list() {
  let list
    = "A. Tässä on lista-alkio\n  Z. Jos toinenkin.\n";

  let pattern = val_from_key(
    &TokenType::NoBolAlphaDotList, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}


#[test]
fn match_alpha_lrpar_list() {
  let list
    = "(A) Tässä on lista-alkio\n  (Z) Jos toinenkin.\n";

  let pattern = val_from_key(
    &TokenType::AlphaLRparList, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}


#[test]
fn match_alpha_rpar_list() {
  let list
    = "A) Tässä on lista-alkio\n  Z) Jos toinenkin.\n";

  let pattern = val_from_key(
    &TokenType::AlphaRparList, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
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

  let pattern = val_from_key(
    &TokenType::DefinitionList, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
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

  let pattern = val_from_key(
    &TokenType::DefinitionList, 
    BODY_TRANSITIONS).unwrap();
  let re = Regex::new(pattern).unwrap();
  if !re.is_match(list) {
    panic!();
  }
}
