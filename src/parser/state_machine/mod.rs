/// This module contains specifications
/// of state machines used by the parser.

mod states;
pub mod transitions;

use super::*;
use states::State;

pub struct StateMachine {
  source_lines: Vec<String>,
  state: State,
  transitions: &'static ActionMap,
  row: usize,

}

impl StateMachine {

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
