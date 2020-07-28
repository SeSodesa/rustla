/// This is the `parser` module of ruSTLa

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
use line_cursor::LineCursor;

mod state_machine;
use state_machine::{StateMachine, COMPILED_INLINE_TRANSITIONS};

use crate::doctree::{DocTree, TreeNode, TreeNodeType};
use crate::common::{self, EnumDelims, EnumKind, FootnoteKind, InterpretedTextKind, NodeId, EnumAsInt};

#[cfg(test)]
mod tests;


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
  line_cursor: usize,

  /// #### base_indent
  /// The level of basic indentation that the parser is working with.
  /// This is useful information during nested parsing sessions, where
  /// the level of indentation of the incoming block of text to be parsed
  /// needs to be passed to the nested parser for node comparison.
  base_indent: usize,

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
  fn new(src: String, doctree: DocTree, base_indent: Option<usize>, initial_state: Option<StateMachine>) -> Self {

    Self {
      src_lines: src.lines().map(|s| s.to_string()).collect::<Vec<String>>(),
      line_cursor: 0,
      base_indent: base_indent.unwrap_or(0),
      doctree: Some(doctree),
      state_stack: vec!(initial_state.unwrap_or(StateMachine::Body))
    }
  }

  /// ### parse
  /// Starts the parsing process for a single file.
  /// Returns the `DocTree` generated by the `StateMachine`s.
  fn parse (&mut self) -> ParsingResult {

    println!("=====================\n Initiating parse...\n=====================\n");

    eprintln!("... with base indent: {:#?}\n", self.base_indent);

    let mut line_changed: bool = false;
    let mut line_not_changed_count: u32 = 0;

    // The parsing loop
    loop {

      eprintln!("Line {:#?} state stack: {:#?}\n", self.line_cursor, self.state_stack);
      eprintln!("Focused on {:#?}\n", self.doctree.as_ref().unwrap().tree.node.data);

      if !line_changed && line_not_changed_count >= 10 {
        eprintln!("Line not advanced even after {} iterations of the parsing loop on line {}.\nClearly something is amiss...\n", line_not_changed_count, self.line_cursor);
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

            // Walk to doctree root before returning it
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
              transitions_ref.clone()
            } else {
              return ParsingResult::Failure { message: String::from("No transitions for this state...\n") }
            }
          }
        }

      } else {
        break
      };

      // Iterating over a clone of the transitions
      for (pattern_name, regex, method) in latest_state_transitions.iter() {

        // Fetching a reference to current line
        let src_line: &str = match Parser::get_source_from_line(&self.src_lines, self.line_cursor) {
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

          let line_before_transition = self.line_cursor;

          self.doctree = match method(&self.src_lines, &self.base_indent, &mut self.line_cursor, self.doctree.take(), captures, pattern_name) {

            TransitionResult::Success{doctree, next_state, push_or_pop, line_advance, nested_state_stack} => {

              match (push_or_pop, next_state, nested_state_stack) {

                (_, next_state, nested_state_stack) if next_state.is_some() && nested_state_stack.is_some() => {
                  return ParsingResult::Failure {
                    message: String::from("Transition returned both, a single next state and the state stack of a nested parser.\nComputer says no...\n")
                  }
                }

                (PushOrPop::Push, next_state, nested_state_stack)  if next_state.is_some() && nested_state_stack.is_none() => {
                  let next_state = next_state.unwrap();
                  eprintln!("Pushing {:#?} on top of stack...\n", next_state);
                  self.state_stack.push(next_state);
                },

                (PushOrPop::Push, next_state, nested_state_stack)  if next_state.is_none() && nested_state_stack.is_some() => {
                  let mut next_states = nested_state_stack.unwrap();
                  eprintln!("Appending {:#?} to stack...\n", next_states);
                  self.state_stack.append(&mut next_states);
                },

                (PushOrPop::Pop, _, _) => {
                  eprintln!("Received POP instruction...\n");
                  match self.state_stack.pop() {
                    Some(machine) => (),
                    None => {
                      return ParsingResult::Failure { message: String::from("Can't pop from empty stack...\n") }
                    }
                  };
                }

                (PushOrPop::Neither, next_state, nested_state_stack) if next_state.is_some() && nested_state_stack.is_none() => {
                  let machine = match self.state_stack.last_mut() {
                    Some(opt_machine) => *opt_machine = next_state.unwrap(),
                    None => {
                      eprintln!("No machine on top of stack.\nCan't perform transition after executing transition method...\n");
                      return ParsingResult::EmptyStateStack { doctree: self.doctree.take().unwrap(), state_stack: self.state_stack.drain(..).collect() }
                    }
                  };
                }

                (PushOrPop::Neither, next_state, nested_state_stack) if next_state.is_none() && nested_state_stack.is_some() => {

                  if let Some(state) = self.state_stack.pop() {
                    self.state_stack.append(&mut nested_state_stack.unwrap());
                  } else {
                    return ParsingResult::Failure {
                      message: format!("Attempted to POP from an empty stack on line {}...\n", self.line_cursor)
                    }
                  }
                }

                (PushOrPop::Neither, None, None) => {
                  // No need to do anything to the stack...
                }

                (push_or_pop, next_state, nested_state_stack) => {
                  eprintln!("No action for received (PushOrPop, StateMachine, Vec<Statemachine>) = ({:#?}, {:#?}, {:#?}) triplet...\n", push_or_pop, next_state, nested_state_stack);
                  return ParsingResult::Failure {
                    message: format!("Transition performed, but conflicting result on line {:#?}\nAborting...\n", self.line_cursor)
                  }
                }
              };

              if let LineAdvance::Some(offset) = line_advance {
                self.line_cursor += offset;
              }

              // Incrementing the line_not_changed counter, if match was found but no incrementing occurred
              if self.line_cursor == line_before_transition {
                line_not_changed_count += 1;
              } else {
                line_changed = true;
                line_not_changed_count = 0;
              }

              Some(doctree)

            }

            TransitionResult::Failure {message} => {
              eprintln!("{} on line {}", message, self.line_cursor);
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

        let mut doctree = self.doctree.take().unwrap();
        doctree.tree = doctree.tree.focus_on_parent().unwrap();
        self.doctree.replace(doctree);

      }

      if self.line_cursor >= self.src_lines.len() {
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
      self.line_cursor = line;
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

    self.line_cursor = match self.line_cursor.checked_add(n) {
      Some(value) => value,
      None =>
        return Err("Attempted indexing with integer overflow.\nComputer says no...\n")
    };

    if self.line_cursor > self.src_lines.len() {
      return Err("No such line number.\nComputer says no...\n")
    }

    Ok(())
  }


  /// ### nth_previous_line
  /// Attempts to decrement `self.current_line` by `n`.
  /// Returns nothing if successful, otherwise returns `Err(&str)`.
  /// The called must handle the `Err` case.
  fn nth_previous_line(&mut self, n: usize) -> Result<(), &'static str> {

    self.line_cursor = match self.line_cursor.checked_sub(n) {
      Some(value) => value,
      None =>
        return Err("Attempted indexing with integer overflow.\nComputer says no...\n")
    };

    if self.line_cursor > self.src_lines.len() {
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
  fn inline_parse (inline_src_block: String, current_line: &mut usize, node_counter: &mut NodeId) -> Option<Vec<TreeNode>> {

    let mut nodes: Vec<TreeNode> = Vec::new();

    let mut col: usize = 0;

    // Remove backslashes
    let src_without_escapes = inline_src_block.replace("\\", "");

    let src_chars = &mut src_without_escapes.chars();

    match Parser::match_inline_str(&src_chars, node_counter) {
      Some((node, offset)) => {

        nodes.push(node);

        // Move iterator to start of next possible match
        for _ in 0..offset - 1 {
          let c = src_chars.next().unwrap();
          // eprintln!("Consuming {:#?}...", c);

          col += 1;

          if c == '\n' {
            // eprintln!("Detected newline...\n");
            *current_line += 1;
            col = 0;
          }
        }
      },

      None => {
        // eprintln!("No match on line {}, col {}.\nProceeding to consume next character...\n", current_line, col);
      }
    }

    while let Some(c) = src_chars.next() {

      // eprintln!("Consuming {:#?}...\n", c);

      col += 1;

      if c == '\n' {
        // eprintln!("Detected newline...\n");
        *current_line += 1;
        col = 0;
      }

      match Parser::match_inline_str(&src_chars, node_counter) {
        Some((node, offset)) => {

          nodes.push(node);

          // Move iterator to start of next possible match
          for _ in 0..offset - 1 {
            let c = src_chars.next().unwrap();
            // eprintln!("Consuming {:#?}", c);

            col += 1;

            if c == '\n' {
              // eprintln!("Detected newline...\n");
              *current_line += 1;
              col = 0;
            }
          }
        },

        None => {
          // eprintln!("No match on line {}, col {}.\n", current_line, col);
        }
      }
    }

    if nodes.is_empty() {
      return None
    }

    Some(nodes)
  }

  /// ### match_inline_str
  /// A function for checking the string representation of
  /// a given `Chars` iterator for a regex match and executing
  /// the corresponding parsing method. Returns the `Option`al
  /// generated node if successful, otherwise returns with `None`.
  fn match_inline_str <'chars> (chars_iter: &'chars str::Chars, node_id: &mut NodeId) -> Option<(TreeNode, usize)> {

    let src_str = chars_iter.as_str();

    if src_str.is_empty() {
      // eprintln!("Source has been drained of characters.\n");
      return None
    }

    // eprintln!("Matching against {:#?}\n", src_str);

    for (pattern_name, regexp, parsing_function) in COMPILED_INLINE_TRANSITIONS.iter() {

      match regexp.captures(src_str) {

        Some(capts) => {

          // eprintln!("Match found for {:#?}\n", pattern_name);

          let (node, offset) = parsing_function(*pattern_name, &capts, node_id);

          //eprintln!("{:#?}", node);

          return Some((node, offset));

        },

        None => {
          //eprintln!("No match for {:#?}", pattern_name);
          continue // no match, do nothing
        }
      };
    }

    None
  }


  /// ### first_list_item_block
  /// Parses the first block of a list item, in case it contains body level nodes
  /// right after the enumerator, on the same line.
  fn first_list_item_block (doctree: DocTree, src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, text_indent: usize, first_indent: Option<usize>) -> Option<(DocTree, usize, Vec<StateMachine>)> {

    eprintln!("Line before nested parse: {:?}...\n", current_line);


    let relative_first_indent = first_indent.or(None);
    let relative_block_indent = text_indent - base_indent;

    // Read indented block here. Notice we need to subtract base indent from assumed indent for this to work with nested parsers.
    let (block, line_offset) = match Parser::read_indented_block(src_lines, Some(*current_line), Some(true), None, Some(relative_block_indent), relative_first_indent) {
      Ok((lines, min_indent, line_offset, blank_finish)) => {
        (lines.join("\n"), line_offset)
      }
      Err(e) => {
        eprintln!("{}\n", e);
        eprintln!("Error when reading list item block.\n");
        return None
      }
    };

    // Run a nested `Parser` over the first indented block with base indent set to `text_indent`.
    let (doctree, state_stack) = match Parser::new(block.clone(), doctree, Some(text_indent), Some(StateMachine::ListItem)).parse() {
      ParsingResult::EOF {doctree, state_stack} | ParsingResult::EmptyStateStack { doctree, state_stack } => (doctree, state_stack),
      ParsingResult::Failure {message} => {
        eprintln!("{:?}", message);
        eprintln!("Nested parse ended in failure...\n");
        return None
      }
    };

    eprintln!("Line after nested parse: {:?}...\n", current_line);

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
        return Err(format!("No indent allowed but indent found on line {}!\nComputer says no...\n", line_num))
      }

      if let Some(alignment) = alignment {
        if alignment != line_indent {
          return Err(format!("Block alignment was set but line {} indent does not match...\nComputer says no...\n", line_num))
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
  ///
  /// Returns a tuple
  /// ```rust
  /// {block: Vec<String>, min_indent<usize>, finished_with_blank: bool}
  /// ```
  /// if successful.
  fn read_indented_block (src_lines: &Vec<String>, start_line: Option<usize>, until_blank: Option<bool>,
    strip_indent: Option<bool>, block_indent: Option<usize>, first_indent: Option<usize>)
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

    // eprintln!("Minimal indent after block assignment: {:?}", minimal_indent);

    // If there is block indentation but no predetermined indentation for the first line,
    // set the indentation of the first line equal to block indentation.
    let first_indent = if let (Some(block_indent), None) = (block_indent, first_indent) {
      // eprintln!("Setting first line indentation equal to block indentation: {}...\n", block_indent);
      Some(block_indent)
    } else {
      first_indent
    };

    // eprintln!("First indent set to {:?}", first_indent);

    // Push first line into `block_lines` and increment
    // line number to ignore indentation (for now) if first_indent was set
    if first_indent.is_some() {
      // eprintln!("Pushing line {} to block_lines", line_num);
      let line = src_lines.get(line_num).unwrap().to_owned();
      block_lines.push(line);
      line_num += 1;
    }

    let mut blank_finish: bool = false;

    let mut loop_broken = false; // Used to detect whether the below while loop was broken out of

    while line_num < last_line_num {

      let line: String = match src_lines.get(line_num) {
        Some(line) => line.clone(),
        None => return Err(format!("Line {} could not be read\nComputer says no...\n", line_num))
      };

      // Check for sufficient indentation if line isn't empty

      let line_indent = line.as_str().chars().take_while(|c| c.is_whitespace()).count();

      if !line.trim().is_empty() && ( line_indent < 1 || block_indent.is_some() && line_indent < block_indent.unwrap() ) {
        eprintln!("Line: {:#?}", line);
        eprintln!("Not enough indentation on line {:?}!\n", line_num);

        // Ended with a blank finish if the last line before unindent was blank
        blank_finish = (line_num > start_line) && src_lines.get(line_num - 1).unwrap().is_empty();
        loop_broken = true;
        break
      }

      // Updating the minimal level of indentation, if line isn't blank
      // and there isn't predetermined block indentation
      if line.trim().is_empty() && until_blank {

        blank_finish = true;
        break

      } else if block_indent.is_none() {

        eprintln!("Line indent: {:?} on line {:?}", line_indent, line_num);

        if minimal_indent.is_none() {
          minimal_indent = Some(line_indent);
        } else if line_indent > 0 {
          minimal_indent = Some(cmp::min(minimal_indent.unwrap(), line_indent));
        } 
      }

      eprintln!("Minimal indent {:?} on line {:?}", minimal_indent, line_num);
      block_lines.push(line);
      line_num += 1;
    }

    // eprintln!("Loop broken: {:?}\n", loop_broken);

    if !loop_broken {
      blank_finish = true;
    }

    // If indentation was expected on the first line, remove it
    if !first_indent.is_none() && !block_lines.is_empty() {

      // eprintln!("Removing first line indentation...\n");

      if let Some(first_line) = block_lines.first_mut() {
        let mut cs = first_line.chars();
        for _i in 0..first_indent.unwrap() {
          cs.next();
        }
        let trunc_line = cs.as_str().to_string();
          *first_line = trunc_line;
      }
    }

    // Strip all minimal indentation from each line
    if let Some(indent) = minimal_indent {
      if strip_indent {
        // eprintln!("Removing indentation from lines...\n");
        for (index, line) in block_lines.iter_mut().enumerate() {
          if first_indent.is_some() && index == 0 {
            // eprintln!("Cursor currently on the first line of block and \nfirst line had own indentation.\nContinuing...\n");
            continue
          }
          // eprintln!("Draining line {:?} of minimal indent, {:?}...", line, indent);
          let trunc_line = match common::strip_indent(line.clone(), indent) {
            Ok(line) => line,
            Err(e) => {
              eprintln!("{}", e);
              return Err(format!("Indentation removal error on line {} of block under inspection\n", index));              
            }
          };
          *line = trunc_line;
          // eprintln!("Line after drain: {:?}\n", line);
        }
      }
    }

    block_lines.shrink_to_fit(); // Free unnecessary used memory
    let line_diff = block_lines.len();

    eprintln!("Block lines: {:#?}\n", block_lines);

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
