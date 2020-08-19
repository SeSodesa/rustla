/// ## parser
/// This is the `parser` module of ruSTLa
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi


// =========
//  Imports
// =========

// Standard library
use std::cmp;
use std::io::{BufReader, Lines};
use std::fs::File;

use std::str;
use std::collections;


// External crates
// ---------------
use regex::{Regex, Captures};


// Own modules
// -----------

use super::*;

mod converters;
use converters::*;

mod types_and_aliases;
use types_and_aliases::*;

mod line_cursor;
use line_cursor::{LineCursor, Line};

mod state_machine;
use state_machine::{StateMachine, COMPILED_INLINE_TRANSITIONS};

mod directive_parsers;

use crate::doctree::DocTree;
use crate::doctree::tree_node::TreeNode;
use crate::doctree::tree_node_types::TreeNodeType;
use crate::common::{self, EnumDelims, EnumKind, FootnoteKind, HyperlinkTargetKind, InterpretedTextKind, NodeId, EnumAsInt, PatternName, SectionLineStyle};

// Unit test modules
// -----------------
mod test_admonitions;
mod test_converters;
mod test_block_reading;
mod test_bullet_lists;
mod test_definition_lists;
mod test_enumerated_lists;
mod test_field_lists;
mod test_hyperlink_targets;
mod test_mixed_structures;
mod test_inline_parsing;
mod test_literal_blocks;
mod test_sections_and_transitions;


// ==========================
//  The Parser specification
// ==========================

/// ### Parser
/// The parser type. Contains an optional
/// source line vector and a document tree.
/// These are optional to facilitate their passing
/// to any transitions functions via
/// `std::option::Option::take`
/// without invalidating the fields.
pub struct Parser {

  /// #### src_lines
  /// The source `String` converted to a vector of owned `String`s.
  src_lines: Vec<String>,

  /// #### line_cursor
  /// The absolute line index of src_lines.
  line_cursor: LineCursor,

  /// #### base_indent
  /// The level of basic indentation that the parser is working with.
  /// This is useful information during nested parsing sessions, where
  /// the level of indentation of the incoming block of text to be parsed
  /// needs to be passed to the nested parser for node comparison.
  base_indent: usize,

  /// #### section_level
  /// Keeps track of the section level the parser is currently focused on.
  /// Level 0 indicates document root.
  section_level: usize,

  /// #### doctree
  /// An `Option`al document tree. The optionality is necessary,
  /// as this needs to be given to transition functions for modification
  /// via `Option::take`.
  doctree: Option<DocTree>,

  /// #### machine_stack
  /// A stack of states that function as keys to vectors of state transitions.
  /// The set of transitios is chosen based on the current state on top of the stack.
  state_stack: Vec<StateMachine>,
}


/// ==============
/// Parser Methods
/// ==============
impl Parser {

  /// ### new
  /// The `Parser` constructor. Transforms a given source string
  /// into a vector of lines and wraps this and a given `DocTree`
  /// in `Option`s. This wrapping allows the passing of these to owned
  /// state machnes via swapping the optional contents
  /// to `None` before granting ownership of the original contents.
  fn new(src: String, doctree: DocTree, base_indent: Option<usize>, base_line: Line, initial_state: Option<StateMachine>, section_level: usize) -> Self {

    Self {
      src_lines: src.lines().map(|s| s.to_string()).collect::<Vec<String>>(),
      line_cursor: LineCursor::new(0, base_line),
      base_indent: base_indent.unwrap_or(0),
      section_level: section_level,
      doctree: Some(doctree),
      state_stack: vec!(initial_state.unwrap_or(StateMachine::Body))
    }
  }

  /// ### parse
  /// Starts the parsing process for a single file.
  /// Returns the `DocTree` generated by the `StateMachine`s.
  fn parse (&mut self) -> ParsingResult {

    println!("=====================\n Initiating parse...\n=====================\n");

    eprintln!("... with base indent {:#?} on line {:#?}\n", self.base_indent, self.line_cursor.sum_total());

    let mut line_changed: bool = false;
    let mut line_not_changed_count: u32 = 0;

    // The parsing loop
    loop {

      eprintln!("Section level: {:#?}", self.section_level);
      eprintln!("Line {:#?} state stack: {:#?}\n", self.line_cursor.sum_total(), self.state_stack);
      eprintln!("Focused on {:#?}\n", self.doctree.as_ref().unwrap().shared_node_data());

      if !line_changed && line_not_changed_count >= 10 {
        eprintln!("Line not advanced even after {} iterations of the parsing loop on line {}.\nClearly something is amiss...\n", line_not_changed_count, self.line_cursor.sum_total());
        break
      }

      line_changed = false;

      let mut match_found = false;

      // Retrieving a clone of the transitions stored in the latest state
      // A clone is needed because the below for loop takes
      // ownership of a reference given to it, which would prevent us from
      // modifying the machine stack.
      let latest_state_transitions = if let Some(machine) = self.state_stack.last() {

        match machine {

          StateMachine::EOF => {
            eprintln!("Moved past EOF...\n");
            match self.doctree.take() {
              Some(doctree) => {
                return ParsingResult::EOF { doctree: doctree, state_stack: self.state_stack.drain(..self.state_stack.len() - 1).collect() }
              }
              None => {
                return ParsingResult::Failure { message: String::from("Tree should not be in the possession of a transition method after moving past EOF...\n") }
              }
            };
          }

          StateMachine::Failure{ .. } => {
            return ParsingResult::Failure { message: String::from("Parsing ended in Failure state...\n") }
          }

          _ => {
            if let Ok(transitions_ref) = machine.get_transitions() {
              transitions_ref
            } else {
              return ParsingResult::Failure { message: String::from("No transitions for this state...\n") }
            }
          }
        }
      } else { break };

      // Iterating over a clone of the transitions
      for (pattern_name, regex, method) in latest_state_transitions.iter() {

        // Fetching a reference to current line
        let src_line: &str = match Parser::get_source_from_line(&self.src_lines, self.line_cursor.relative_offset()) {
          Some(line) => line,
          None => {
            return ParsingResult::Failure { message: String::from("Parsing ended prematurely because of an unqualified move past EOF...\n") }
          }
        };

        // Running the current line of text through a DFA compiled from a regex
        if regex.is_match(src_line) {

          eprintln!("Found match for {:?}...\n", pattern_name);

          match_found = true;

          let captures = regex.captures(src_line).unwrap();

          eprintln!("Match: {:#?}", captures.get(0).unwrap().as_str());
          eprintln!("Executing transition method...\n");

          let line_before_transition = self.line_cursor.sum_total();

          self.doctree = match method(&self.src_lines, &self.base_indent, &mut self.section_level, &mut self.line_cursor, self.doctree.take(), captures, pattern_name) {

            TransitionResult::Success{doctree, next_states, push_or_pop, line_advance} => {

              match (push_or_pop, next_states) {

                (PushOrPop::Push, next_states) if next_states.is_some() => {
                  let mut next_states = next_states.unwrap();
                  eprintln!("Appending {:#?} to stack...\n", next_states);
                  self.state_stack.append(&mut next_states);
                },

                (PushOrPop::Pop, _) => {
                  eprintln!("Received POP instruction...\n");
                  match self.state_stack.pop() {
                    Some(machine) => (),
                    None => {
                      return ParsingResult::Failure { message: String::from("Can't pop from empty stack...\n") }
                    }
                  };
                }

                (PushOrPop::Neither, next_states) if next_states.is_some() => {

                  if let Some(state) = self.state_stack.pop() {
                    self.state_stack.append(&mut next_states.unwrap());
                  } else {
                    return ParsingResult::Failure {
                      message: format!("Attempted to POP from an empty stack on line {}...\n", self.line_cursor.sum_total())
                    }
                  }
                }

                (PushOrPop::Neither, None) => {} // No need to do anything to the stack...

                (push_or_pop, next_states) => {
                  eprintln!("No action for received (PushOrPop, Vec<Statemachine>) = ({:#?}, {:#?}) pair...\n", push_or_pop, next_states);
                  return ParsingResult::Failure {
                    message: format!("Transition performed, but conflicting result on line {:#?}\nAborting...\n", self.line_cursor.sum_total())
                  }
                }
              };

              if let LineAdvance::Some(offset) = line_advance {
                *self.line_cursor.relative_offset_mut_ref() += offset;
              }

              // Incrementing the line_not_changed counter, if match was found but no incrementing occurred
              if self.line_cursor.sum_total() == line_before_transition {
                line_not_changed_count += 1;
              } else {
                line_changed = true;
                line_not_changed_count = 0;
              }

              Some(doctree)
            }

            TransitionResult::Failure {message} => {
              eprintln!("{}", message);
              return ParsingResult::Failure { message: String::from("An error was encountered while executing a transition method.\n") }
            }
          };

          break // Match found so stop looking for matches
        }
      }

      // No matches in current state so pop from state stack and attempt
      // parsing in the previous state down stack
      if !match_found {

        eprintln!("No match found.\nPopping from machine stack...\n");

        if let None = self.state_stack.pop() {
          eprintln!("Cannot pop from an empty stack.\n");
          return ParsingResult::EmptyStateStack { doctree: self.doctree.take().unwrap(), state_stack: self.state_stack.drain(..self.state_stack.len()).collect() }
        };

        if let Some(doctree) = self.doctree.take() {
          self.doctree = Some(doctree.focus_on_parent());
        } else {
          return ParsingResult::Failure {
            message: format!("Doctree in possession of transition method after transition on line {}.\nComputer says no...\n", self.line_cursor.sum_total())
          }
        }
      }

      if self.line_cursor.relative_offset() >= self.src_lines.len() {
        self.state_stack.push(StateMachine::EOF);
      }
    };

    ParsingResult::EOF { doctree: self.doctree.take().unwrap(), state_stack: self.state_stack.drain(..self.state_stack.len()).collect() }
  }


  /// ### jump_to_line
  /// Attempts to move `self.current_line` to the given index.
  /// Return an `Err` if not successful.
  fn jump_to_line(&mut self, line: usize) -> Result<(), &'static str> {

    if line < self.src_lines.len() {
      *self.line_cursor.relative_offset_mut_ref() = line;
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

    *self.line_cursor.relative_offset_mut_ref() = match self.line_cursor.relative_offset().checked_add(n) {
      Some(value) => value,
      None =>
        return Err("Attempted indexing with integer overflow.\nComputer says no...\n")
    };

    if self.line_cursor.relative_offset() > self.src_lines.len() {
      return Err("No such line number.\nComputer says no...\n")
    }

    Ok(())
  }


  /// ### nth_previous_line
  /// Attempts to decrement `self.current_line` by `n`.
  /// Returns nothing if successful, otherwise returns `Err(&str)`.
  /// The called must handle the `Err` case.
  fn nth_previous_line(&mut self, n: usize) -> Result<(), &'static str> {

    *self.line_cursor.relative_offset_mut_ref() = match self.line_cursor.relative_offset().checked_sub(n) {
      Some(value) => value,
      None =>
        return Err("Attempted indexing with integer overflow.\nComputer says no...\n")
    };

    if self.line_cursor.relative_offset() > self.src_lines.len() {
      return Err("No such line number.\nComputer says no...\n")
    }

    Ok(())
  }


  /// ### DEFAULT_LINE_STEP
  /// The default step used by the functions
  /// `nth_{previous|next}_line`.
  const DEFAULT_LINE_STEP: usize = 1;
}


/// ===========================
/// Parser associated functions
/// ===========================
impl Parser {

  /// ### get_source_from_line
  /// Attempts to retrieve the source from a given line number.
  /// Returns an `Ok` clone of it if successful, else
  /// returns and `Err` with a message.
  fn get_source_from_line <'src_lines> (src_lines: &Vec<String>, line_num: usize) -> Option<&str> {

    let src = match src_lines.get(line_num) {
      Some(line) => line.as_str(),
      None => {
        eprintln!("No such line number ({} out of bounds).\nComputer says no...\n", line_num);
        return None
      }
    };

    Some(src)
  }


  /// ### inline_parse
  /// A function that parses inline text. Returns the nodes generated,
  /// if there are any.
  fn inline_parse (inline_src_block: String, mut doctree: Option<DocTree>, line_cursor: &mut LineCursor) -> InlineParsingResult {

    let mut nodes_data: Vec<TreeNodeType> = Vec::new();

    let mut col: usize = 0;

    // Remove backslashes
    let src_without_escapes = inline_src_block.replace("\\", "");

    let src_chars = &mut src_without_escapes.chars();

    match Parser::match_inline_str(doctree.as_mut(), &src_chars) {
      Some((node_data, offset)) => {

        nodes_data.push(node_data);

        // Move iterator to start of next possible match
        for _ in 0..offset - 1 {
          let c = src_chars.next().unwrap();
          // eprintln!("Consuming {:#?}...", c);

          col += 1;

          if c == '\n' {
            // eprintln!("Detected newline...\n");
            *line_cursor.relative_offset_mut_ref() += 1;
            col = 0;
          }
        }
      },

      None => {} // No match, do nothing
    }

    while let Some(c) = src_chars.next() {

      col += 1;
      if c == '\n' {
        *line_cursor.relative_offset_mut_ref() += 1;
        col = 0;
      }

      match Parser::match_inline_str(doctree.as_mut(), &src_chars) {
        Some((node, offset)) => {

          nodes_data.push(node);

          // Move iterator to start of next possible match
          for _ in 0..offset - 1 {
            let c = src_chars.next().unwrap();
            col += 1;
            if c == '\n' {
              *line_cursor.relative_offset_mut_ref() += 1;
              col = 0;
            }
          }
        },

        None => {}
      }
    }

    if doctree.is_some() {
      let doctree = doctree.unwrap();
      return InlineParsingResult::DoctreeAndNodes(doctree, nodes_data)
    } else {
      if nodes_data.is_empty() {
        return InlineParsingResult::NoNodes
      } else {
        return InlineParsingResult::Nodes(nodes_data)
      }
    }
  }

  /// ### match_inline_str
  /// A function for checking the string representation of
  /// a given `Chars` iterator for a regex match and executing
  /// the corresponding parsing method. Returns the `Option`al
  /// generated node if successful, otherwise returns with `None`.
  fn match_inline_str <'chars> (opt_doctree_ref: Option<&mut DocTree>, chars_iter: &'chars str::Chars) -> Option<(TreeNodeType, usize)> {

    let src_str = chars_iter.as_str();

    if src_str.is_empty() { return None }

    for (pattern_name, regexp, parsing_function) in COMPILED_INLINE_TRANSITIONS.iter() {

      match regexp.captures(src_str) {

        Some(capts) => {
          let (node_type, offset) = parsing_function(opt_doctree_ref, *pattern_name, &capts);
          return Some((node_type, offset));
        },

        None => { continue } // no match, do nothing
      };
    }

    None
  }


  /// ### parse_first_node_block
  /// Parses the first block of a node, in case it contains body level nodes
  /// right after a marker such as an enumerator, on the same line.
  fn parse_first_node_block (doctree: DocTree, src_lines: &Vec<String>, base_indent: &usize, current_line: &mut LineCursor, text_indent: usize, first_indent: Option<usize>, start_state: StateMachine, section_level: &mut usize) -> Option<(DocTree, usize, Vec<StateMachine>)> {

    let relative_first_indent = first_indent.unwrap_or(text_indent) - base_indent;
    let relative_block_indent = text_indent - base_indent;

    eprintln!("First: {}", relative_first_indent);
    eprintln!("Block: {}\n", relative_block_indent);

    // Read indented block here. Notice we need to subtract base indent from assumed indent for this to work with nested parsers.
    let (block, line_offset) = match Parser::read_indented_block(src_lines, Some(current_line.relative_offset()), Some(true), None, Some(relative_block_indent), Some(relative_first_indent), true) {
      Ok((lines, min_indent, line_offset, blank_finish)) => {
        eprintln!("Block lines: {:#?}, line_offset: {:#?}\n", lines, line_offset);
        (lines.join("\n"), line_offset)
      }
      Err(e) => {
        eprintln!("{}\n", e);
        eprintln!("Error when reading list item block.\n");
        return None
      }
    };

    // Run a nested `Parser` over the first indented block with base indent set to `text_indent`.
    let (doctree, state_stack) = match Parser::new(block.clone(), doctree, Some(text_indent), current_line.sum_total(), Some(start_state), *section_level).parse() {
      ParsingResult::EOF {doctree, state_stack} | ParsingResult::EmptyStateStack { doctree, state_stack } => (doctree, state_stack),
      ParsingResult::Failure {message} => {
        eprintln!("{:?}", message);
        eprintln!("Nested parse ended in failure...\n");
        return None
      }
    };

    Some((doctree, line_offset, state_stack))
  }


  /// ### read_text_block
  /// Reads in an contiguous set of lines of text.
  /// A text block in rST terms is a set of lines
  /// separated from other elements by empty lines above and below.
  /// Checks for indentation:
  /// if indentation is not allowed but indentation is found,
  /// returns an error message in an `Err`.
  fn read_text_block(src_lines: &Vec<String>, start_line: usize, indent_allowed: bool, remove_indent: bool, alignment: Option<usize>) -> Result<(Vec<String>, usize), String> {

    let mut line_num = start_line;
    let last_line = src_lines.len();

    let mut lines: Vec<String> = Vec::with_capacity(last_line - start_line);

    while line_num < last_line {

      let mut line: String = match src_lines.get(line_num) {
        Some(line) => line.clone(),
        None => return Err(format!("Text block could not be read because of line {}.\n", line_num))
      };

      if line.trim().is_empty() {
        break
      }

      let line_indent = line.as_str().chars().take_while(|c| c.is_whitespace()).count();

      if !indent_allowed && line_indent > 0 {
        break
      }

      if let Some(alignment) = alignment {
        if alignment != line_indent {
          break
        }
      }

      if remove_indent {
        line = line.as_str().trim_start().to_string();
      }

      lines.push(line);
      line_num += 1;
    }

    lines.shrink_to_fit();
    let offset = lines.len();
    eprintln!("Lines: {:#?}\n", lines);
    Ok((lines, offset))
  }


  /// ### read_indented_block
  /// Reads in a block of indented lines text.
  /// Determines the minimum level of indentation
  /// and uses it as a reference for ending the block.
  fn read_indented_block (src_lines: &Vec<String>, start_line: Option<usize>, until_blank: Option<bool>,
    strip_indent: Option<bool>, block_indent: Option<usize>, first_indent: Option<usize>, force_alignment: bool)
  -> Result<(Vec<String>, usize, usize, bool), String> {

    if src_lines.is_empty() {
      return Err(String::from("An empty block of text was handed for parsing.\nComputer says no...\n"))
    }

    // Default function parameters
    let start_line = start_line.unwrap_or(0);
    let until_blank = until_blank.unwrap_or(false);
    let strip_indent = strip_indent.unwrap_or(true);

    let mut line_num = start_line;
    let last_line_num = src_lines.len();

    let mut block_lines: Vec<String> = Vec::with_capacity(last_line_num - start_line);

    // Setting the initial level of minimal indentation
    let mut minimal_indent = match block_indent {
      Some(indent) => Some(indent),
      None => None
    };

    // If there is block indentation but no predetermined indentation for the first line,
    // set the indentation of the first line equal to block indentation.
    let first_indent = if let (Some(block_indent), None) = (block_indent, first_indent) {
      Some(block_indent)
    } else {
      first_indent
    };

    // Push first line into `block_lines` and increment
    // line number to ignore indentation (for now) if first_indent was set
    if first_indent.is_some() {
      let line = src_lines.get(line_num).unwrap().to_owned();
      block_lines.push(line);
      line_num += 1;
    }

    let mut blank_finish: bool = false;
    let mut loop_broken: bool = false; // Used to detect whether the below while loop was broken out of

    while line_num < last_line_num {

      let line: String = match src_lines.get(line_num) {
        Some(line) => line.clone(),
        None => return Err(format!("Line {} could not be read\nComputer says no...\n", line_num))
      };

      // Check for sufficient (or correct if block alignment was forced) indentation if line isn't empty
      let line_indent = line.as_str().chars().take_while(|c| c.is_whitespace()).count();

      let break_when_not_aligned: bool = if block_indent.is_some() && force_alignment {
        line_indent != block_indent.unwrap()
      } else if block_indent.is_some() {
        line_indent < block_indent.unwrap()
      } else {
        false
      };

      if !line.trim().is_empty() && ( line_indent < 1 || break_when_not_aligned ) {

        // Ended with a blank finish if the last line before unindent was blank
        blank_finish = (line_num > start_line) && src_lines.get(line_num - 1).unwrap().trim().is_empty();
        loop_broken = true;
        break
      }

      // Updating the minimal level of indentation, if line isn't blank
      // and there isn't predetermined block indentation
      if line.trim().is_empty() && until_blank {
        blank_finish = true;
        break
      } else if block_indent.is_none() {
        if minimal_indent.is_none() {
          minimal_indent = Some(line_indent);
        } else if line_indent > 0 {
          minimal_indent = Some(cmp::min(minimal_indent.unwrap(), line_indent));
        } 
      }

      block_lines.push(line);
      line_num += 1;
    }

    if !loop_broken { blank_finish = true; } // Made it to the end of input

    // Strip all minimal indentation from each line
    if let Some(min_indent) = minimal_indent {
      if strip_indent {
        for (index, line) in block_lines.iter_mut().enumerate() {
          let indent = if first_indent.is_some() && index == 0 { first_indent.unwrap() } else { min_indent };
          let mut cs = line.chars();
          for _ in 0..indent { cs.next(); } // Remove indentation in characters
          *line = cs.as_str().to_string();
        }
      }
    }

    block_lines.shrink_to_fit(); // Free unnecessary used memory
    let line_diff = block_lines.len();

    Ok((block_lines, minimal_indent.unwrap(), line_diff, blank_finish))
  }
}


/// ### ParsingResult
/// An enumeration of the different ways a (nested) parsing session might terminate.
/// The return type of the `Parser::parse` method. Generally, finishing conditions
/// that are not outright failures will enclose the document tree fed to the parser
/// when it was initialized.
pub enum ParsingResult {

  /// #### EOF
  /// This will be returned, if the parser finished by passing over the last line of the source.
  /// This generally indicates that the source file was parsed successfully.
  EOF {
    doctree: DocTree,
    state_stack: Vec<StateMachine>
  },

  /// #### EmptyStateStack
  /// This will be returned if the parser was unable to parse any elements on some line of the source,
  /// as patterns not matching will drain the parser state stack of states. This might be useful during
  /// nested parsing sessions, when an empty stack right at the start of the parsing process indicates
  /// that there were no expected nested structures on the same line.
  EmptyStateStack {
    doctree: DocTree,
    state_stack: Vec<StateMachine>
  },

  /// #### Failure
  /// A simple failure type. This will be returned when there was clearly no way to recover.
  Failure {
    message: String
  }
}

impl ParsingResult {

  /// ### unwrap_tree
  /// Unwraps the contained doctree in one of the non-failure variants.
  /// Simply panics if this is attempted for the `Failure` variant.
  fn unwrap_tree(self) -> DocTree {

    match self {
      Self::EOF {doctree, state_stack} => doctree,
      Self::EmptyStateStack {doctree, state_stack} => doctree,
      _ => panic!("ParsingResult::Failure does not contain a DocTree...\n")
    }
  }
}
