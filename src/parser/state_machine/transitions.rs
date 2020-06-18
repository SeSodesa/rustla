/// Module contains a list of transition tuples

use std::collections::HashMap;

use regex;
use lazy_static::lazy_static;

use super::*;

#[derive(Copy, Clone)]
pub enum PatternName {
  Bullet,


}


pub const BODY_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[
  (PatternName::Bullet, r"^[+-*\u{2022}]( +|$)", Body::bullet),
];


pub const BULLET_LIST_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];

pub const DEFINITION_LIST_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];

pub const ENUMERATED_LIST_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];

pub const FIELD_LIST_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];

pub const OPTION_LIST_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];


pub const LINE_BLOCK_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];


pub const EXTENSION_OPTION_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];


pub const EXPLICIT_MARKUP_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];

pub const TEXT_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];


pub const DEFINITION_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];


pub const LINE_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];

pub const SUBSTITUTION_DEF_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];


pub const INLINE_TRANSITIONS: &[(PatternName, &'static str, TransitionMethod)] = &[

];


lazy_static! {

  /// ### ACTION_MAP
  /// A static map of actions specified for the `Lexer` type.
  /// This allows for the easy creation of sublexers,
  /// as with both the parent and child, the type of actions
  /// can simply be a reference to this map.
  /// 
  /// Plus, with this regexes are only compiled into automata once.
  static ref ACTION_MAP: HashMap<&'static str, Vec<(PatternName, regex::Regex, TransitionMethod)>> = {

    let mut action_map = collections::HashMap::new();

    let mut body_actions = Vec::with_capacity(BODY_TRANSITIONS.len());
    let mut inline_actions = Vec::with_capacity(INLINE_TRANSITIONS.len());

    for (pat_name, expr, fun) in BODY_TRANSITIONS.iter() {
    let r = regex::Regex::new(expr).unwrap();
    body_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("Body", body_actions);

    for (pat_name, expr, fun) in INLINE_TRANSITIONS.iter() {
    let r = regex::Regex::new(expr).unwrap();
    inline_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("Inline", inline_actions);

    action_map

  };

}