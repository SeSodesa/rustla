/// This module contains specifications
/// of state machines used by the parser.

pub mod states;
pub mod transitions;

use super::*;
use states::State;

pub struct StateMachine {
  src_lines: Vec<String>,
  current_line: usize,
  state: State,
  doctree: DocTree
}

impl StateMachine {

  /// ### new
  /// The `StateMachine` constructor.
  /// A state machine holds a mutable reference to the
  /// doctree owned by the parent `Parser`. If any new machines are
  /// pushed onto the `Parser` machine stack, ownership of this
  /// reference is passed to the
  /// new machine, which upon termination returns it back
  /// to the preceding machine, if there is one.
  /// Also, an immutable reference is held to the source files,
  /// to allow reading and creation of nodes out of it.
  pub fn new (src_lines: Vec<String>, current_line: usize, initial_state: State, doctree: DocTree) -> Self {

    StateMachine {
      src_lines: src_lines,
      current_line: current_line,
      state: initial_state,
      doctree: doctree,
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
