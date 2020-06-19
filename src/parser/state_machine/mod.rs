/// This module contains specifications
/// of state machines used by the parser.

pub mod states;
mod transitions;

use std::cmp;

use super::*;
use crate::utils;
use states::*;
use transitions::{TRANSITION_MAP, *};

/// ### TransitionMethod (TODO)
/// A function pointer type alias for a State transition method.
type TransitionMethod = fn(Option<DocTree>, regex::Captures) -> Result<Option<DocTree>, &'static str>;

/// ### Transition
/// A type alias for a tuple `(PatternName, Regex, TransitionMethod)`
type Transition = (PatternName, regex::Regex, TransitionMethod);

/// ### StateMachine
/// An enum of `MachineWithState`s.
/// Enclosing machine variants with different states in an enum allows us
/// to give ownership of a generic machine to an arbitrary structure,
/// as enums are only as large as their largest variant.
/// Inspired heavily by [this](https://hoverbear.org/blog/rust-state-machine-pattern/)
/// article.
pub enum StateMachine {
  Body(MachineWithState<Body>),
  BulletList(MachineWithState<BulletList>),
  DefinitionList(MachineWithState<DefinitionList>),
  EnumeratedList(MachineWithState<EnumeratedList>),
  FieldList(MachineWithState<FieldList>),
  OptionList(MachineWithState<OptionList>),
  LineBlock(MachineWithState<LineBlock>),
  ExtensionOptions(MachineWithState<ExtensionOptions>),
  ExplicitMarkup(MachineWithState<ExplicitMarkup>),
  Text(MachineWithState<Text>),
  Definition(MachineWithState<Definition>),
  Line(MachineWithState<Line>),
  SubstitutionDef(MachineWithState<SubstitutionDef>),
  RFC2822Body(MachineWithState<RFC2822Body>),
  RFC2822List(MachineWithState<RFC2822List>),
}

impl StateMachine {

  /// ### get_transitions
  /// Retrieves the list of transitions from a `StateMachine` variant
  /// using a `match` statement. This seems like a lot of repetition,
  /// but this is the only way of doing this when wrapping each
  /// different state machine type in an enum.
  pub fn get_transitions (&self) -> &Vec<Transition> {

    match self {
      StateMachine::Body(machine) => machine.state.transitions,
      StateMachine::BulletList(machine) => machine.state.transitions,
      StateMachine::DefinitionList(machine) => machine.state.transitions,
      StateMachine::EnumeratedList(machine) => machine.state.transitions,
      StateMachine::FieldList(machine) => machine.state.transitions,
      StateMachine::OptionList(machine) => machine.state.transitions,
      StateMachine::LineBlock(machine) => machine.state.transitions,
      StateMachine::ExtensionOptions(machine) => machine.state.transitions,
      StateMachine::ExplicitMarkup(machine) => machine.state.transitions,
      StateMachine::Text(machine) => machine.state.transitions,
      StateMachine::Definition(machine) => machine.state.transitions,
      StateMachine::Line(machine) => machine.state.transitions,
      StateMachine::SubstitutionDef(machine) => machine.state.transitions,
      StateMachine::RFC2822Body(machine) => machine.state.transitions,
      StateMachine::RFC2822List(machine) => machine.state.transitions,

    }

  }

}



/// ### MachineWithState
/// A state machine in a state `S`,
/// which is its own type. This allows different
/// state machines to hold common fields,
/// while the embedded state types can hold their
/// own specific fields like transition tables.
#[derive(Debug)]
pub struct MachineWithState <S> {
  state: S,
}


impl MachineWithState<Body> {

  /// ### new
  /// A state machine constructor. This is only implemented for
  /// the `Body` state, as it is the starting state when it
  /// comes to rST parsing. Transitions to and creation of
  /// other states is handled by implementing the `From`
  /// trait (the `from` function) for those states.
  pub fn new() -> Self {

    Self {
      state: Body::new(),
    }
  }

}



/// ====================
/// MachineWithState methods
/// ====================
impl <S> MachineWithState <S> {

  /// ### run
  /// Starts the processing of the given source.
  /// Returns a modified `DocTree`.
  /// This function is initially called by the parser,
  /// but subsequent calls can be made by the state
  /// machines on the top of the parser stack.
  pub fn run (&mut self) -> Option<DocTree>{

    unimplemented!();

  }

}

/// =================================
/// StateMachine associated functions
/// =================================
impl StateMachine {


}
