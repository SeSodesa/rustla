/// This module contains specifications
/// of state machines used by the parser.

pub mod states;
mod transitions;

use std::cmp;

use super::*;
use crate::utils;
use states::*;
use transitions::{TRANSITION_MAP, COMPILED_INLINE_TRANSITIONS, *};
use crate::doctree::{self, TreeNode};


/// ### TransitionMethod
/// A function pointer type alias for a State transition method.
/// `TransitionMethod`s take in the document tree and regex captures
/// for doctree modifications. Unless errors occur,
/// they return an `Ok`-wrapped tuple of optional doctree and a possible next state for the parser.
/// If the optional next state is *not* `None`, a new state machine
/// in the new state is pushed on top of the machine stack of the parser and parsing proceeds
/// in that state from the current line.
type TransitionMethod = fn(src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, next_state: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>), &'static str>;

/// ### Transition
/// A type alias for a tuple `(PatternName, Regex, TransitionMethod)`
type Transition = (PatternName, regex::Regex, TransitionMethod);


/// ### InlineTransitionMethod
/// A type alias for a function describing an inline transition.
type InlineTransitionMethod = fn ();


/// ### InlineTransition
/// A type alias for a tuple `(PatternName, regex pattern, InlineTransitionMethod)`.
type InlineTransition = (PatternName, &'static str, InlineTransitionMethod);


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
  Failure(MachineWithState<Failure>)
}

impl StateMachine {

  /// ### new
  /// A `StateMachine` enum constructor
  fn new (state_type_name: &PatternName) -> Self {
    match state_type_name {
      //"Body" => StateMachine::Body( MachineWithState::<Body> { state: Body::new() } ),
      PatternName::Bullet => StateMachine::BulletList( MachineWithState::<BulletList> { state: BulletList::new() } ),
      PatternName::Enumerator => StateMachine::EnumeratedList( MachineWithState::<EnumeratedList> { state: EnumeratedList::new() } ),
      PatternName::FieldMarker => StateMachine::FieldList( MachineWithState::<FieldList> { state: FieldList::new() } ),
      PatternName::OptionMarker => StateMachine::OptionList( MachineWithState::<OptionList> { state: OptionList::new() } ),
      PatternName::LineBlock => StateMachine::LineBlock( MachineWithState::<LineBlock> { state: LineBlock::new() } ),
      PatternName::ExplicitMarkup => StateMachine::ExplicitMarkup( MachineWithState::<ExplicitMarkup> { state: ExplicitMarkup::new() } ),
      //"DefinitionList" => StateMachine::DefinitionList( MachineWithState::<DefinitionList> { state: DefinitionList::new() } ),
      //"ExtensionOptions" => StateMachine::ExtensionOptions( MachineWithState::<ExtensionOptions> { state: ExtensionOptions::new() } ),
      //"Definition" => StateMachine::Definition( MachineWithState::<Definition> { state: Definition::new() } ),
      PatternName::Line => StateMachine::Line( MachineWithState::<Line> { state: Line::new() } ),
      //"SubstitutionDef" => StateMachine::SubstitutionDef( MachineWithState::<SubstitutionDef> { state: SubstitutionDef::new() } ),
      PatternName::Text => StateMachine::Text( MachineWithState::<Text> { state: Text::new() } ),
      _ => unreachable!()
    }
  }


  /// ### to_failure
  /// Transitions a `StateMachine` into a `Failure` state using the From trait,
  /// the implementation of which automatically implements the Into trait.
  pub fn to_failure (self) -> Self {
    match self {
      StateMachine::Body(machine) => StateMachine::Failure(machine.into()),
      StateMachine::BulletList(machine) => StateMachine::Failure(machine.into()),
      StateMachine::DefinitionList(machine) => StateMachine::Failure(machine.into()),
      StateMachine::EnumeratedList(machine) => StateMachine::Failure(machine.into()),
      StateMachine::FieldList(machine) => StateMachine::Failure(machine.into()),
      StateMachine::OptionList(machine) => StateMachine::Failure(machine.into()),
      StateMachine::LineBlock(machine) => StateMachine::Failure(machine.into()),
      StateMachine::ExtensionOptions(machine) => StateMachine::Failure(machine.into()),
      StateMachine::ExplicitMarkup(machine) => StateMachine::Failure(machine.into()),
      StateMachine::Text(machine) => StateMachine::Failure(machine.into()),
      StateMachine::Definition(machine) => StateMachine::Failure(machine.into()),
      StateMachine::Line(machine) => StateMachine::Failure(machine.into()),
      StateMachine::SubstitutionDef(machine) => StateMachine::Failure(machine.into()),
      _ => unreachable!()
    }
  }

  /// ### get_transitions
  /// Retrieves the list of transitions from a `StateMachine` variant
  /// using a `match` statement. This seems like a lot of repetition,
  /// but this is the only way of doing this when wrapping each
  /// different state machine type in an enum.
  pub fn get_transitions (&self) -> Result<&Vec<Transition>, &'static str> {

    match self {
      StateMachine::Body(machine) => Ok(machine.state.transitions),
      StateMachine::BulletList(machine) => Ok(machine.state.transitions),
      StateMachine::DefinitionList(machine) => Ok(machine.state.transitions),
      StateMachine::EnumeratedList(machine) => Ok(machine.state.transitions),
      StateMachine::FieldList(machine) => Ok(machine.state.transitions),
      StateMachine::OptionList(machine) => Ok(machine.state.transitions),
      StateMachine::LineBlock(machine) => Ok(machine.state.transitions),
      StateMachine::ExtensionOptions(machine) => Ok(machine.state.transitions),
      StateMachine::ExplicitMarkup(machine) => Ok(machine.state.transitions),
      StateMachine::Text(machine) => Ok(machine.state.transitions),
      StateMachine::Definition(machine) => Ok(machine.state.transitions),
      StateMachine::Line(machine) => Ok(machine.state.transitions),
      StateMachine::SubstitutionDef(machine) => Ok(machine.state.transitions),
      StateMachine::Failure( .. ) => Err("Failure state has no transitions\n")
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

impl MachineWithState<Inline> {

  /// ### new
  /// MachineWithState<Inline> constructor.
  fn new() -> Self {
    Self {
      state: Inline::new(),
    }
  }


  /// ### parse
  /// A function that parses inline text. Returns the tokens generated.
  fn parse (&self, inline_src_block: String) -> Vec<TreeNode> {

    let nodes: Vec<TreeNode> = Vec::new();

    let mut src_chars = inline_src_block.chars();

    while let Some(c) = src_chars.next() {
      let remaining = src_chars.as_str();

      for (pattern_name, regexp, parsing_function) in self.state.transitions.iter() {

        let captures = match regexp.captures(remaining) {
          Some(capts) => {

            let full_match = capts.get(0).unwrap();

            let node = parsing_function();

            // match found => advance chars iterator to end of match, as
            // inline patterns match at the start of the give source
            let capt_len = full_match.end() - full_match.start();

            for _ in 0..capt_len - 1 {
              src_chars.next();
            }

            capts

          },
          None => continue // no match, do nothing
        };

      }

    }

    todo!();


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
