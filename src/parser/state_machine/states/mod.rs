/// This submodule contains the states and related transitions of
/// the parser state machine.

pub mod body;

use super::*;

#[derive(Debug)]
/// ### State
/// An enumeration of the possible states of `StateMachine`.
/// The indentations present here are an attempt to reflect
/// the class hierarchy found in the docutils rST parser
/// [States module](https://sourceforge.net/p/docutils/code/HEAD/tree/trunk/docutils/docutils/parsers/rst/states.py#l202).
/// 
/// Each type stored within the corresponding variant
/// has methods to handle the parsing and possible state transition
/// of the patterns it recognizes. Pointers to these methods are stored
/// in an associated constant, a list of tuples of the form
/// ```rust
/// (regex, match_method, Option<next_state>)
/// ```
/// 
pub enum State {
  Body(body::Body),
    SpecializedBody(body::SpecializedBody),
      BulletList(body::BulletList),
      DefinitionList(body::DefinitionList),
      EnumeratedList(body::EnumeratedList),
      FieldList(body::FieldList),
        ExtensionOptions,
      OptionList(body::OptionList),
      RFC2822List(body::RFC2822List),
      LineBlock(body::LineBlock),
      Explicit(body::Explicit),
    SubstitutionDefinition(body::SubstitutionDefinition),
  Text,
    SpecializedText,
      Definition,
      Line,
  QuotedLiteralBlock,

}

/// MachineState
/// Constants and methods common to each `StateMachine` state.
pub trait MachineState {

  /// ### TRANSITIONS
  /// A list of tuples of state transitions.
  /// The stucture of the tuples is as follows:
  /// ```rust
  /// (regex_pattern, parsing_method, optional_new_state)
  /// ```
  /// When a `State` object is initialized,
  /// every regex in this list is compiled into a state machine,
  /// and pushed into a vector of tuples
  /// ```rust
  /// (state_machine, parsing_method, next_state)
  /// ```
  /// This is the used by the state machine to perform the parsing.
  const TRANSITIONS: [(&'static str, TransitionMethod, Option<State>)];

}


/// ### TransitionMethod (TODO)
/// A function pointer type alias for a State transition method.
type TransitionMethod = fn() -> ();
