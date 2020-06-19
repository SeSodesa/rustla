/// This module contains specifications
/// of state machines used by the parser.

pub mod states;
mod transitions;
mod tests;

use std::cmp;

use super::*;
use crate::utils;
use states::*;
use transitions::{TRANSITION_MAP, *};

/// ### TransitionMethod (TODO)
/// A function pointer type alias for a State transition method.
type TransitionMethod = fn(&mut Parser) -> ();

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
  /// Retrives the list of transitions from a `StateMachine` variant
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
  src_lines: Option<Vec<String>>,
  current_line: usize,
  state: S,
  doctree: Option<DocTree>
}

impl MachineWithState<Body> {

  /// ### new
  /// A state machine constructor. This is only implemented for
  /// the `Body` state, as it is the starting state when it
  /// comes to rST parsing. Transitions to and creation of
  /// other states is handled by implementing the `From`
  /// trait (the `from` function) for those states.
  pub fn new(src_lines: Option<Vec<String>>, current_line: usize, doctree: Option<DocTree>) -> Result<Self, &'static str> {

    if src_lines.is_some() && doctree.is_some()  {

      if current_line < src_lines.as_ref().unwrap().len() {
        Ok(
          Self {
            src_lines: src_lines,
            current_line: current_line,
            state: Body::new(),
            doctree: doctree,
          }
        )

      } else {
        Err("The given starting line number is too large.\nState machine says no...\n")
      }

    } else {
      Err("Either the source or doctree was not provided.\nState machine says no...\n")
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


  /// ### match_line
  /// Attempts to match the current line to each pattern
  /// in the list of transitions in the current `State`.
  /// If no match is found, the current line number
  /// is returned in and `Err`. If a line is matched,
  /// attempts to run the transition|parsing method
  /// related to the matched pattern.
  fn match_line(&mut self) -> Result<(), String>{

    unimplemented!();

  }


  /// ### get_source_from_line
  /// Attempts to retrieve the source from a given line number.
  /// Returns an `Ok` clone of it if successful, else
  /// returns and `Err` with a message.
  fn get_source_from_line (&self, line_num: Option<usize>) -> Result <String, String> {

    let line = line_num.unwrap_or(self.current_line);

    let src = match self.src_lines.as_ref().unwrap().get(line) {
      Some(line) => line.clone(),
      None => return Err(format!("No such line number ({} out of bounds).\nComputer says no...\n", line))
    };

    Ok(src)

  }


  /// ### jump_to_line
  /// Attempts to move `self.current_line` to the given index.
  /// Return an `Err` if not successful.
  fn jump_to_line(&mut self, line: usize) -> Result<(), &'static str> {

    if line < self.src_lines.as_ref().unwrap().len() {
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

    if self.current_line > self.src_lines.as_ref().unwrap().len() {
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

    if self.current_line > self.src_lines.as_ref().unwrap().len() {
      return Err("No such line number.\nComputer says no...\n")
    }

    Ok(())

  }


  /// ### DEFAULT_LINE_STEP
  /// The default step used by the functions
  /// `nth_{previous|next}_line`.
  const DEFAULT_LINE_STEP: usize = 1;

}

/// =================================
/// StateMachine associated functions
/// =================================
impl StateMachine {


  /// ### read_text_block
  /// Reads in an contiguous set of lines of text.
  /// A text block in rST terms is a set of lines
  /// separated from other elements by empty lines above and below.
  /// Checks for indentation:
  /// if indentation is not allowed but indentation is found,
  /// returns an error message in an `Err`.
  fn read_text_block(src_lines: &Vec<String>, start_line: usize, indent_allowed: Option<bool>) -> Result<Vec<String>, String> {

    // Default parameter for allowed indentation
    let indent_allowed = indent_allowed.unwrap_or(false);

    let mut line_num = start_line;
    let last_line = src_lines.len();

    let mut lines: Vec<String> = Vec::with_capacity(last_line - start_line);

    while line_num < last_line - 1 {

      let line: String = match src_lines.get(line_num) {
        Some(line) => line.clone(),
        None => return Err(format!("Text block could not be read because of line {}.\n", line_num))
      };

      if line.is_empty() {
        break
      }

      let has_indent: bool = match line.get(0..1) {
        Some(line) => {
          line.chars().next().unwrap().is_whitespace()
        },
        None => return Err(format!("The first character of line {} could not be read.", line_num))
      };

      if !indent_allowed && has_indent {
        return Err(format!("No indent allowed but indent found on line {}!\nComputer says no...\n", line_num))
      }

      lines.push(line.clone());

      line_num += 1;

    }

    lines.shrink_to_fit();

    Ok(lines)

  }


  /// ### read_indented_block
  /// Reads in a block of indented lines text.
  /// Determines the minimum level of indentation
  /// and uses it as a reference for ending the block.
  ///
  /// Returns a tuple
  /// ```rust
  /// {block: Vec<String>, min_indent<u32>, finished_with_blank: bool}
  /// ```
  /// if successful.
  fn read_indented_block (src_lines: &Vec<String>, start_line: Option<usize>, until_blank: Option<bool>,
    strip_indent: Option<bool>, block_indent: Option<usize>, first_indent: Option<usize>)
  -> Result<(Vec<String>, usize, bool), String> {

    // Default function parameters
    let start_line = start_line.unwrap_or(0);
    let until_blank = until_blank.unwrap_or(false);
    let strip_indent = strip_indent.unwrap_or(true);

    let mut line_num = start_line;

    // Setting the initial level of minimal indentation
    let mut minimal_indent = match block_indent {
      Some(indent) => Some(indent),
      None => None
    };

    eprintln!("Indent after block assignment: {:?}", minimal_indent);

    // If there is block indentation but no predetermined indentation for the first line,
    // set the indentation of the first line equal to block indentation.
    let first_indent = if let (Some(block_indent), None) = (block_indent, first_indent) {
      Some(block_indent)
    } else {
      None
    };

    // First line is ignored if `first_indent` was set
    // if !first_indent.is_none() {
    //   line_num += 1;
    // }

    let last_line_num = src_lines.len();

    let mut blank_finish: bool = false;

    let mut loop_broken = false; // Used to detect whether the below while loop was broken out of

    let mut block_lines: Vec<String> = Vec::with_capacity(last_line_num - start_line);

    while line_num < last_line_num {

      let line: String = match src_lines.get(line_num) {
        Some(line) => line.clone(),
        None => return Err(format!("Line {} could not be read\nComputer says no...\n", line_num))
      };

      // Check for sufficient indentation
      for (i, c) in line.chars().enumerate() {

        // No need to keep looping if we have reached a
        // sufficient level of indentation
        if !block_indent.is_none() && i == block_indent.unwrap() {
          break
        }

        if !c.is_whitespace() && i == 0 // No indentation at all
          || (!block_indent.is_none() && i < block_indent.unwrap() && !c.is_whitespace()) // Not enough indentation
        {

          eprintln!("Not enough indentation!\n");

          // Block is valid, iff the last indented line is blank
          blank_finish = (line_num > start_line) &&
            src_lines
              .get(line_num - 1)
              .unwrap()
              .trim()
              .is_empty();

          eprintln!("Blank finish: {:?}", blank_finish);

          // end while iteration
          line_num = last_line_num;
          break

        }

      }

      if line_num >= last_line_num {
        eprintln!("Breaking out of while loop\n");
        loop_broken = true;
        break
      }

      // Trim beginning whitespace from line under observation
      // for blank line check
      let no_indent_line = line.trim_start();

      let line_indent: usize;

      // Updating the minimal level of indentation, if line isn't blank
      // and there isn't predetermined block indentation
      if no_indent_line.is_empty() && until_blank {

        blank_finish = true;
        break

      } else if block_indent.is_none() {

        line_indent = line.chars().count() - no_indent_line.chars().count();

        eprintln!("Line indent: {:?} on line {:?}", line_indent, line_num);

        if minimal_indent.is_none() {
          minimal_indent = Some(line_indent);
        } else if line_indent > 0 {
          minimal_indent = Some(cmp::min(minimal_indent.unwrap(), line_indent));
        } 

      }

      eprintln!("Minimal indent {:?} on line {:?}", minimal_indent, line_num);

      if !line.trim().is_empty() {
        block_lines.push(line);
      }

      line_num += 1;

    }

    eprintln!("Loop broken: {:?}", loop_broken);

    if !loop_broken {
      blank_finish = true;
    }

    // If indentation was expected on the first line, remove it
    // if !first_indent.is_none() && !block_lines.is_empty() {
    //   if let Some(first_line) = block_lines.first_mut() {
        
    //     let mut cs = first_line.chars();

    //     for _i in 0..first_indent.unwrap() {
    //       cs.next();
    //     }

    //     let trunc_line = cs.as_str().to_string();

    //       *first_line = trunc_line;

    //   }
    // }

    // Strip all minimal indentation from each line
    if let Some(indent) = minimal_indent {
      if strip_indent {
        for (index, line) in block_lines.iter_mut().enumerate() {

          eprintln!("Draining line {:?} of minimal indent, {:?}...", line, indent);

          let trunc_line = match utils::strip_indent(line.clone(), indent) {
            Ok(line) => line,
            Err(e) => {
              eprintln!("{}", e);
              return Err(format!("Indentation removal error on line {} of block under inspection\n", index));
            }
          };

          *line = trunc_line;

          eprintln!("Line after drain: {:?}\n", line);
        }
      }
    }

    block_lines.shrink_to_fit(); // Free unnecessary used memory

    Ok((block_lines, minimal_indent.unwrap(), blank_finish))

  }

}
