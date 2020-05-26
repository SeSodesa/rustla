/// This module contains the regexes used
/// by the Lexer.

mod body;
mod inline;

use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::lexer::state::State;
use crate::lexer::token::{TokenType};

use regex;

/// ### type Action
/// A type alias for a tokenizer function
type Action = fn(&mut Lexer, TokenType, regex::Captures);

impl PartialEq for Action {
  fn eq(&self, other: &Self) -> bool {

    let self_ptr = *self as *const fn() as *const usize;
    let other_ptr = *other as *const fn() as *const usize;

    if self_ptr == other_ptr {
      true
    } else {
      false
    }
  }
}

impl Eq for Action  {

}

/// ### type Action
/// A type alias for Lexer action container
pub type LexerActions = HashMap<State, Vec<(TokenType, &'static str, Action)>>;



/// ### lexer_actions
/// The state transition function of the lexer.
/// Returns a list of Token--action--next_state
/// tuples to the lexer, the the lexer uses to
/// process its source. 
pub fn lexer_actions() -> HashMap<State, Vec<(TokenType, &'static str, Action)>> {
  let mut hm:HashMap<State, Vec<(TokenType, &'static str, Action)>> = HashMap::new();
  
  for (tt, re, fun) in body::BODY_TRANSITIONS.iter() {
    if let Some(v) = hm.get_mut(&State::Body) {
      v.push((tt.clone(), re, fun.clone()));
    } else {
      hm.insert(State::Body, Vec::new());
    };
    
  }

  for (tt, re, fun) in inline::INLINE_TRANSITIONS.iter() {
    
    if let Some(v) = hm.get_mut(&State::Body) {
      v.push((tt.clone(), re, fun.clone()));
    } else {
      hm.insert(State::Body, Vec::new());
    };
    
  }

  hm
}

/// ### val_from_key
/// Searches through a list of TokenType--regex pairs
/// for a mathing tokentype
pub fn val_from_key(search_key: &TokenType, map: &[(TokenType, &'static str, Action)]) -> Option<String> {
  for (_, val, _) in map.iter().filter(|&(map_key, _, _)| map_key == search_key) { 
    return Some(val.to_string());
  }
  None
}
