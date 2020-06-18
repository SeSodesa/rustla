/// Module contains a list of transition tuples

use std::collections::HashMap;

use regex;
use lazy_static::lazy_static;

use super::*;


#[derive(Copy, Clone)]
pub enum PatternName {
  Bullet,


}


/// ### UncompiledTransition
/// A type alias for a transition `(PatternName, regex_pattern, TransitionMethod)`, whose regex pattern has not
/// been compiled into a DFA yet.
type UncompiledTransition = (PatternName, &'static str, TransitionMethod);



pub const BODY_TRANSITIONS: &[UncompiledTransition] = &[
  (PatternName::Bullet, r"^[+-*\u{2022}]( +|$)", Body::bullet),
];


pub const BULLET_LIST_TRANSITIONS: &[Transition] = &[

];

pub const DEFINITION_LIST_TRANSITIONS: &[Transition] = &[

];

pub const ENUMERATED_LIST_TRANSITIONS: &[UncompiledTransition] = &[

];

pub const FIELD_LIST_TRANSITIONS: &[UncompiledTransition] = &[

];

pub const OPTION_LIST_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const LINE_BLOCK_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const EXTENSION_OPTION_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const EXPLICIT_MARKUP_TRANSITIONS: &[UncompiledTransition] = &[

];

pub const TEXT_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const DEFINITION_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const LINE_TRANSITIONS: &[UncompiledTransition] = &[

];

pub const SUBSTITUTION_DEF_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const INLINE_TRANSITIONS: &[UncompiledTransition] = &[

];


lazy_static! {

  /// ### TRANSITION_MAP
  /// A static map of transititions for each state of
  /// the `Parser` `StateMachine`.
  /// 
  /// With this regexes are only compiled into automata once.
  pub static ref TRANSITION_MAP: HashMap<&'static str, Vec<(PatternName, regex::Regex, TransitionMethod)>> = {

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