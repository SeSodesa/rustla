/// This module contains the regexes used
/// by the Lexer.

mod body;

use std::collections::HashMap;
use crate::lexer::state::State;
use crate::lexer::token::TokenType;

/// ### delta
/// The state transition function of the lexer.
/// Returns a list of Token--action--next_state
/// tuples to the lexer, the the lexer uses to
/// process its source. 
pub fn delta() {

}

/// ### val_from_key
/// Searches through a list of TokenType--regex pairs
/// for a mathing tokentype
pub fn val_from_key(search_key: &TokenType, map: &'static [(TokenType, &'static str, State)]) -> Option<String> {
  for (_, val, _) in map.iter().filter(|&(map_key, _, _)| map_key == search_key) { 
    return Some(val.to_string());
  }
  None
}
