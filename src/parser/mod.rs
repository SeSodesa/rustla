/// This is the `parser` module of ruSTLa

mod state_machine;


mod token;
mod state;
mod position;
mod error;

#[cfg(test)]
mod tests;

use std::io::{self, BufReader, Lines};
use std::fs::File;

use std::fmt;
use std::str;
use regex;
use lazy_static::lazy_static;

use crate::parser::token::{Token, TokenType};
use crate::parser::state::{State};
use crate::parser::position::Pos;
use std::collections;
use crate::parser::error::{TokenizeError, ParseError};
use state_machine::transitions::{BODY_TRANSITIONS, INLINE_TRANSITIONS};


//#[derive(PartialEq)]
pub struct Parser {
  line_iter: Lines<BufReader<File>>,
  state: State,
  actions: &'static ActionMap,
  // body_actions: Vec<(TokenType, regex::Regex, Action)>,
  // inline_actions: Vec<(TokenType, regex::Regex, Action)>,
  tokens: Vec<Token>,
  pos: Pos,
  has_error: bool
}


/// Lexer type methods
impl Parser {

}


/// ### val_from_key
/// Goes through a given list of tuples
/// ```
/// (TokenType, str_pattern, Action)
/// ```
/// and looks for a matching tokentype.
/// If it finds one, returns and `Option<&'static str>`,
/// otherwise returns `None`.
fn val_from_key(search_key: &TokenType, map: &[(TokenType, &'static str, Action)]) -> Option<&'static str> {
  for (_, val, _) in map.iter().filter(|&(map_key, _, _)| map_key == search_key) { 
    return Some(val);
  }
  None
}



lazy_static! {

  /// ### ACTION_MAP
  /// A static map of actions specified for the `Lexer` type.
  /// This allows for the easy creation of sublexers,
  /// as with both the parent and child, the type of actions
  /// can simply be a reference to this map.
  /// 
  /// Plus, with this regexes are only compiled into automata once.
  static ref ACTION_MAP: ActionMap = {
    let mut action_map = collections::HashMap::new();

    let mut body_actions = Vec::with_capacity(BODY_TRANSITIONS.len());
    let mut inline_actions = Vec::with_capacity(INLINE_TRANSITIONS.len());

    for (tt, re, fun) in BODY_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      body_actions.push((tt.clone(), r, *fun));
    }

    action_map.insert(State::Body, body_actions);

    for (tt, re, fun) in INLINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      inline_actions.push((tt.clone(), r, *fun));
    }

    action_map.insert(State::Inline, inline_actions); 
    
    action_map

  };
}


/// ### Action
/// A function pointer type alias for a Lexer action
pub type Action = fn(&mut Parser, TokenType, &regex::Captures) -> ();

/// ### Actionvector
/// Contains tuples `(TokenType, Regex, Action)`
pub type ActionVector = Vec<(TokenType, regex::Regex, Action)>;

/// ### ActionMap
/// Maps Lexer states to suitable `ActionVector`s.
pub type ActionMap = collections::HashMap<state::State, ActionVector>;



impl fmt::Debug for Parser {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Lexer")
        .field("lexeme_start", &self.pos.pos)
        .field("lookahead", &self.pos.lookahead)
        .finish()
  }
}

impl fmt::Display for Parser {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Lexer location: row = {}, col = {}", self.pos.row, self.pos.col)
  }
}

