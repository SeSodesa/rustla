/// This module contains specifications
/// of state machines used by the parser.

pub mod states;
pub mod transitions;

mod tests;

use std::cmp;

use super::*;
use states::State;

pub struct StateMachine {
  src_lines: Vec<String>,
  current_line: usize,
  state: State,
  doctree: DocTree
}


/// ====================
/// StateMachine methods
/// ====================
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
        Some(line) => line.chars().filter(|c| !c.is_whitespace()).collect(),
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
    let mut indent = match block_indent {
      Some(indent) => Some(indent),
      None => None
    };

    // If there is block indentation but no predetermined indentation for the first line,
    // set the indentation of the first line equal to block indentation.
    let first_indent = if let (Some(block_indent), None) = (block_indent, first_indent) {
      Some(block_indent)
    } else {
      None
    };

    if !first_indent.is_none() {
      line_num += 1;
    }

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

          // Block is valid, iff the last indented line is blank
          blank_finish = (line_num > start_line) &&
            src_lines
              .get(line_num - 1)
              .unwrap()
              .trim()
              .is_empty();

          // end while iteration
          line_num = last_line_num;
          break

        }

      }

      if line_num >= last_line_num {
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

        if indent.is_none() {
          indent = Some(line_indent);
        } else {
          indent = Some(cmp::min(indent.unwrap(), line_indent));
        }

      }

      block_lines.push(line);

      line_num += 1;

    }

    if !loop_broken {
      blank_finish = true;
    }

    // If indentation was expected on the first line, remove it
    if !first_indent.is_none() && !block_lines.is_empty() {
      if let Some(first_line) = block_lines.first_mut() {
        first_line.drain(..first_indent.unwrap());
      }
    }

    // Strip all minimal indentation from each line
    if let Some(indent) = indent {
      if strip_indent {
        for line in block_lines.iter_mut() {
          let _ = line.drain(..indent);
        }
      }
    }

    block_lines.shrink_to_fit(); // Free unnecessary used memory

    Ok((block_lines, indent.unwrap(), blank_finish))

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
