/// This submodule contains the states and related transitions of
/// the parser state machine.

pub mod body;

use super::*;

#[derive(Debug)]
pub struct Body;

#[derive(Debug)]
pub struct BulletList;

#[derive(Debug)]
pub struct Definition;

#[derive(Debug)]
pub struct DefinitionList;

#[derive(Debug)]
pub struct EnumeratedList;

#[derive(Debug)]
pub struct Explicit;

#[derive(Debug)]
pub struct ExtensionOptions;

#[derive(Debug)]
pub struct FieldList;

#[derive(Debug)]
pub struct Line;

#[derive(Debug)]
pub struct LineBlock;

#[derive(Debug)]
pub struct OptionList;

#[derive(Debug)]
pub struct RFC2822Body;

#[derive(Debug)]
pub struct RFC2822List;

#[derive(Debug)]
pub struct SubstitutionDef;

#[derive(Debug)]
pub struct Text;

/// MachineState
/// Constants and methods common to each `StateMachine` state.
pub trait MachineState <S> {

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
  const TRANSITIONS: [(&'static str, TransitionMethod, Option<S>)];

}


/// ### TransitionMethod (TODO)
/// A function pointer type alias for a State transition method.
type TransitionMethod = fn() -> ();
