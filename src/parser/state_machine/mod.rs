/// This module contains specifications
/// of state machines used by the parser.

// ===============================================
// Submodules for namespacing transition functions
// ===============================================
mod body;
mod bullet_list;
mod common;
mod enumerated_list;
mod field_list;
mod inline;
pub mod transitions;
mod tests;

use std::cmp;
use std::collections::HashMap;

use regex;
use lazy_static::lazy_static;

use super::*;
use crate::common::{EnumDelims, EnumKind, NodeId};
//use transitions::{TRANSITION_MAP, COMPILED_INLINE_TRANSITIONS, UncompiledTransition,  *};
use crate::doctree::{self, TreeNode};


/// ### StateMachine
/// An enum of `MachineWithState`s.
/// Enclosing machine variants with different states in an enum allows us
/// to give ownership of a generic machine to an arbitrary structure,
/// as enums are only as large as their largest variant.
/// Inspired heavily by [this](https://hoverbear.org/blog/rust-state-machine-pattern/)
/// article.
/// 
/// The variants are used as keys to the static `TRANSITION_MAP`, which stores vectors of
/// transitions as values.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum StateMachine {
  Body,
  BulletList,
  DefinitionList,
  EnumeratedList,
  ListItem,
  FieldList,
  Footnote,
  OptionList,
  LineBlock,
  ExtensionOptions,
  ExplicitMarkup,
  Text,
  Definition,
  Line,
  SubstitutionDef,
  Failure,
  EOF
}


// ====================
// Statemachine methods
// ====================
impl StateMachine {

  /// ### to_failure
  /// Transitions a `StateMachine` into a `Failure` state using the From trait,
  /// the implementation of which automatically implements the Into trait.
  pub fn to_failure (self) -> Self {
    match self {
      _ => StateMachine::Failure
    }
  }

  /// ### get_transitions
  /// Retrieves the list of transitions based on a given `StateMachine` variant
  /// using a `match` statement. First checks for end states that don't contain transitions,
  /// such as `EOF` or `Failure` and if these are not matched,
  /// retrieves a list of transitions from the `TRANSITION_MAP`.
  pub fn get_transitions (&self) -> Result<&Vec<Transition>, &'static str> {

    match self {
      StateMachine::EOF               => Err("Already moved past EOF. No transitions to perform.\n"),
      StateMachine::Failure           => Err("Failure state has no transitions\n"),
      StateMachine::ListItem          => {
        Ok(TRANSITION_MAP.get(&StateMachine::Body).unwrap())
      }
      _ => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      }
    }
  }
}


/// =================================
/// StateMachine associated functions
/// =================================
impl StateMachine {

  /// ### compile_state_transitions
  /// Takes in a reference/slice to an associated array of uncompiled transitions
  /// and compiles the regex patterns found. Returns a `Vec<Transition>` with compiled state machines
  /// in palce of the regex patterns.
  /// 
  /// Error handling needs to be added.
  fn compile_state_transitions (transitions: &[UncompiledTransition]) -> Vec<Transition> {

    let mut compiled_transitions = Vec::with_capacity(transitions.len());

    for (pat_name, expr, fun) in transitions.iter() {
      let r = regex::Regex::new(expr).unwrap();
      compiled_transitions.push((*pat_name, r, *fun));
    }

    compiled_transitions
  }

}

/// =================================
/// StateMachine associated constants
/// =================================
impl StateMachine {

}

lazy_static! {

  /// ### TRANSITION_MAP
  /// A static map of transititions for each state of
  /// the `Parser` `StateMachine`.
  /// 
  /// With this regexes are only compiled into automata once.
  pub static ref TRANSITION_MAP: HashMap<StateMachine, Vec<(PatternName, regex::Regex, TransitionMethod)>> = {

    let mut action_map = collections::HashMap::new();

    let body_actions = StateMachine::compile_state_transitions(&StateMachine::BODY_TRANSITIONS);
    action_map.insert(StateMachine::Body, body_actions);

    let bullet_actions = StateMachine::compile_state_transitions(&StateMachine::BULLET_LIST_TRANSITIONS);
    action_map.insert(StateMachine::BulletList, bullet_actions);

    let definition_actions = StateMachine::compile_state_transitions(&StateMachine::DEFINITION_LIST_TRANSITIONS);
    action_map.insert(StateMachine::DefinitionList, definition_actions);

    let enumerated_actions = StateMachine::compile_state_transitions(&StateMachine::ENUMERATED_LIST_TRANSITIONS);
    action_map.insert(StateMachine::EnumeratedList, enumerated_actions);

    let field_actions = StateMachine::compile_state_transitions(&StateMachine::FIELD_LIST_TRANSITIONS);
    action_map.insert(StateMachine::FieldList, field_actions);

    let option_actions = StateMachine::compile_state_transitions(&StateMachine::OPTION_LIST_TRANSITIONS);
    action_map.insert(StateMachine::OptionList, option_actions);

    let line_block_actions = StateMachine::compile_state_transitions(&StateMachine::LINE_BLOCK_TRANSITIONS);
    action_map.insert(StateMachine::LineBlock, line_block_actions);

    let extension_option_actions = StateMachine::compile_state_transitions(&StateMachine::EXTENSION_OPTION_TRANSITIONS);
    action_map.insert(StateMachine::ExtensionOptions, extension_option_actions);

    let explicit_markup_actions = StateMachine::compile_state_transitions(&StateMachine::EXPLICIT_MARKUP_TRANSITIONS);
    action_map.insert(StateMachine::ExplicitMarkup, explicit_markup_actions);

    let text_actions = StateMachine::compile_state_transitions(&StateMachine::TEXT_TRANSITIONS);
    action_map.insert(StateMachine::Text, text_actions);

    let definition_actions = StateMachine::compile_state_transitions(&StateMachine::DEFINITION_TRANSITIONS);
    action_map.insert(StateMachine::Definition, definition_actions);

    let line_actions = StateMachine::compile_state_transitions(&StateMachine::LINE_TRANSITIONS);
    action_map.insert(StateMachine::Line, line_actions);    

    let subst_def_actions = StateMachine::compile_state_transitions(&StateMachine::SUBSTITUTION_DEF_TRANSITIONS);
    action_map.insert(StateMachine::SubstitutionDef, subst_def_actions);    

    action_map

  };

  /// ### COMPILED_INLINE_TRANSITIONS
  /// A vector of transitions specific to MachineWithState<Inline>.
  /// Inline text has different parsing requirements than (nested)
  /// `Body` elements as they do not form blocks of text,
  /// making detecting by source line impractical.
  pub static ref COMPILED_INLINE_TRANSITIONS: Vec<(PatternName, regex::Regex, InlineParsingMethod)> = {

    let mut inline_transitions = Vec::with_capacity(StateMachine::INLINE_TRANSITIONS.len());

    for (pat_name, expr, fun) in StateMachine::INLINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      inline_transitions.push((*pat_name, r, *fun));
    }

    inline_transitions

  };

}
