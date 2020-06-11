/// This module contains specifications
/// of state machines used by the parser.

mod states;
pub mod transitions;

use super::*;
use states::State;

pub struct StateMachine {
  src_lines: Vec<String>,
  current_line: usize,
  state: State,
}

impl StateMachine {

  /// ### new
  /// The `StateMachine` constructor.
  fn new (src_lines: Vec<String>, current_line: usize, initial_state: State) -> Self {

    StateMachine {
      src_lines: src_lines,
      current_line: current_line,
      state: initial_state,
    }

  }

}

/// ### Action
/// A function pointer type alias for a Lexer action
pub type Action = fn(&mut Parser, TokenType, &regex::Captures) -> ();

/// ### Actionvector
/// Contains tuples `(TokenType, Regex, Action)`
pub type ActionVector = Vec<(TokenType, regex::Regex, Action)>;

/// ### ActionMap
/// Maps Lexer states to suitable `ActionVector`s.
pub type ActionMap = collections::HashMap<states::State, ActionVector>;
