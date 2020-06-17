/// This submodule contains the states and related transitions of
/// the parser state machine.

pub mod body;

use super::*;

#[derive(Debug)]
/// ### Body
/// A state for detecting and parsing the first lines
/// of different types of rST text blocks. Transitions to
/// other states for handling the following lines
/// of the block are handled by the `TransitionMethod`s
/// in this state.
pub struct Body;


impl From<MachineWithState<Body>> for MachineWithState<BulletList> {

  fn from(machine: MachineWithState<Body>) -> MachineWithState<BulletList> {

    // parsing before transition

    MachineWithState {
      src_lines: machine.src_lines,
      current_line: machine.current_line,
      state: BulletList,
      doctree: machine.doctree
    }

  }

}


impl From<MachineWithState<Body>> for MachineWithState<DefinitionList> {

  fn from(machine: MachineWithState<Body>) -> MachineWithState<DefinitionList> {

    // parsing before transition

    MachineWithState {
      src_lines: machine.src_lines,
      current_line: machine.current_line,
      state: DefinitionList,
      doctree: machine.doctree
    }

  }

}


impl From<MachineWithState<Body>> for MachineWithState<EnumeratedList> {

  fn from(machine: MachineWithState<Body>) -> MachineWithState<EnumeratedList> {

    // parsing before transition

    MachineWithState {
      src_lines: machine.src_lines,
      current_line: machine.current_line,
      state: EnumeratedList,
      doctree: machine.doctree
    }

  }

}


impl From<MachineWithState<Body>> for MachineWithState<FieldList> {

  fn from(machine: MachineWithState<Body>) -> MachineWithState<FieldList> {

    // parsing before transition

    MachineWithState {
      src_lines: machine.src_lines,
      current_line: machine.current_line,
      state: FieldList,
      doctree: machine.doctree
    }

  }

}


impl From<MachineWithState<Body>> for MachineWithState<OptionList> {

  fn from(machine: MachineWithState<Body>) -> MachineWithState<OptionList> {

    // parsing before transition

    MachineWithState {
      src_lines: machine.src_lines,
      current_line: machine.current_line,
      state: OptionList,
      doctree: machine.doctree
    }

  }

}


impl From<MachineWithState<Body>> for MachineWithState<LineBlock> {

  fn from(machine: MachineWithState<Body>) -> MachineWithState<LineBlock> {

    // parsing before transition

    MachineWithState {
      src_lines: machine.src_lines,
      current_line: machine.current_line,
      state: LineBlock,
      doctree: machine.doctree
    }

  }

}


impl From<MachineWithState<Body>> for MachineWithState<ExplicitMarkup> {

  fn from(machine: MachineWithState<Body>) -> MachineWithState<ExplicitMarkup> {

    // parsing before transition

    MachineWithState {
      src_lines: machine.src_lines,
      current_line: machine.current_line,
      state: ExplicitMarkup,
      doctree: machine.doctree
    }

  }

}


impl From<MachineWithState<Body>> for MachineWithState<Text> {

  fn from(machine: MachineWithState<Body>) -> MachineWithState<Text> {

    // parsing before transition

    MachineWithState {
      src_lines: machine.src_lines,
      current_line: machine.current_line,
      state: Text,
      doctree: machine.doctree
    }

  }

}


#[derive(Debug)]
/// ### BulletList
/// A transition to this state is made if a `BulletList`
/// is detected in state `Body`. Handles subsequent
/// `BulletList` items.
pub struct BulletList;

#[derive(Debug)]
/// ### Definition
/// A state for handling the second line of a possible
/// `DefinitionList` items.
pub struct Definition;

#[derive(Debug)]
/// ### DefinitionList
/// This state is transitioned to if a first line of `DefinitionList`
/// is detected. Handles the subsequent lines.
pub struct DefinitionList;

#[derive(Debug)]
/// ### EnumeratedList
/// A state that parses the lines followed by the detection of
/// the first line of a possibly detected `EnumeratedList`.
pub struct EnumeratedList;

#[derive(Debug)]
/// ### ExplicitMarkup
/// A state for parsing explicit markup followed by the detection
/// of a first such item.
pub struct ExplicitMarkup;

#[derive(Debug)]
/// ### ExtensionOptions
/// A state for parsing directive option fields.
pub struct ExtensionOptions;

#[derive(Debug)]
/// ### FieldList
/// A state for parsing subsequent fields in a field list.
pub struct FieldList;

#[derive(Debug)]
/// ### Line
/// A state for parsing a detected `Line` (section titles and such).
pub struct Line;


#[derive(Debug)]
/// ### LineBlock
/// A state for parsing subsequent lines of a line block.
pub struct LineBlock;

#[derive(Debug)]
/// ### A state for  parsing subsequent option list items.
pub struct OptionList;

#[derive(Debug)]
/// ### RFC2822Body
/// A state for parsing body items that follow the RFC2822 specification.
pub struct RFC2822Body;

#[derive(Debug)]
/// ### RFC2822List
/// A state for parsing list items that follow the RFC2822 specification.
pub struct RFC2822List;

#[derive(Debug)]
/// ### SubstitutionDef
/// A state for parsing substitution definitions
pub struct SubstitutionDef;

#[derive(Debug)]
/// ### Text
/// A state for parsing generic text.
pub struct Text;


/// MachineState
/// Constants and methods common to each `StateMachine` state.
pub trait Transitions <S> {

  /// ### TRANSITIONS
  /// A list of tuples of state transitions.
  /// Needs to be imple,ented for each state.
  /// A transition list consists of tuples
  /// ```
  /// (regex_pattern, transition_method)
  /// ```
  const TRANSITIONS: [(&'static str, TransitionMethod)];

}


/// ### TransitionMethod (TODO)
/// A function pointer type alias for a State transition method.
type TransitionMethod = fn() -> ();
