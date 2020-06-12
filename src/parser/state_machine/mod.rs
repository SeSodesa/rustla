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

  /// ### run
  /// Starts the processing of the given source.
  /// Returns a modified `DocTree`.
  /// This function is called
  pub fn run (&mut self) -> Option<DocTree>{

    unimplemented!();

  }


  /// ### jump_to_line
  /// Attempts to move `self.current_line` to the given index.
  /// Return an `Err` if not successful.
  fn jump_to_line(&mut self, line: usize) -> Result<(), &'static str> {

    if line < self.src_lines.len() {
      self.current_line = line;
    } else {
      return Err("Attempted a move to a non-existent line.\nComputer says  no...\n")
    }

    Ok(())

  }


  /// ### nth_next_line
  /// Attempts to increment `self.current_line` by `n`.
  /// Returns nothing if successful, otherwise returns `Err(&str)`.
  /// The called must handle the `Err` case.
  fn nth_next_line(&mut self, n: usize) -> Result<(), &'static str> {
    self.current_line = match self.current_line.checked_add(n) {
      Some(value) => value,
      None =>
        return Err("Attempted indexing with integer overflow.\nComputer says no...\n")
    };

    if self.current_line > self.src_lines.len() {
      return Err("No such line number.\nComputer says no...\n")
    }

    Ok(())

  }


  /// ### nth_previous_line
  /// Attempts to decrement `self.current_line` by `n`.
  /// Returns nothing if successful, otherwise returns `Err(&str)`.
  /// The called must handle the `Err` case.
  fn nth_previous_line(&mut self, n: usize) -> Result<(), &'static str> {
    self.current_line = match self.current_line.checked_sub(n) {
      Some(value) => value,
      None =>
        return Err("Attempted indexing with integer overflow.\nComputer says no...\n")
    };

    if self.current_line > self.src_lines.len() {
      return Err("No such line number.\nComputer says no...\n")
    }

    Ok(())

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
