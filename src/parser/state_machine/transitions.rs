/// Module contains a list of transition tuples

use std::collections::HashMap;

use regex;
use lazy_static::lazy_static;

use super::*;


pub const BODY_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];


pub const BULLET_LIST_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];

pub const DEFINITION_LIST_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];

pub const ENUMERATED_LIST_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];

pub const FIELD_LIST_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];

pub const OPTION_LIST_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];


pub const LINE_BLOCK_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];


pub const EXTENSION_OPTION_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];


pub const EXPLICIT_MARKUP_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];

pub const TEXT_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];


pub const DEFINITION_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];


pub const LINE_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];

pub const SUBSTITUTION_DEF_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];


pub const INLINE_TRANSITIONS: &[(&'static str, TransitionMethod)] = &[

];


lazy_static! {

  /// ### ACTION_MAP
  /// A static map of actions specified for the `Lexer` type.
  /// This allows for the easy creation of sublexers,
  /// as with both the parent and child, the type of actions
  /// can simply be a reference to this map.
  /// 
  /// Plus, with this regexes are only compiled into automata once.
  static ref ACTION_MAP: HashMap<&'static str, Vec<(regex::Regex, TransitionMethod)>> = {
  let mut action_map = collections::HashMap::new();

  let mut body_actions = Vec::with_capacity(BODY_TRANSITIONS.len());
  let mut inline_actions = Vec::with_capacity(INLINE_TRANSITIONS.len());

  for (re, fun) in BODY_TRANSITIONS.iter() {
  let r = regex::Regex::new(re).unwrap();
  body_actions.push((r, *fun));
  }

  action_map.insert("BODY", body_actions);

  for (re, fun) in INLINE_TRANSITIONS.iter() {
  let r = regex::Regex::new(re).unwrap();
  inline_actions.push((r, *fun));
  }

  action_map.insert("Inline", inline_actions);

  action_map

  };

}