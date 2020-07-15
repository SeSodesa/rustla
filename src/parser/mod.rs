/// This is the `parser` module of ruSTLa

mod state_machine;

use state_machine::{StateMachine, PushOrPop, LineAdvance};
use state_machine::transitions::COMPILED_INLINE_TRANSITIONS;

#[cfg(test)]
mod tests;

use std::cmp;
use std::io::{BufReader, Lines};
use std::fs::File;

use std::str;
use std::collections;

use regex;

use crate::doctree::{DocTree, TreeNode, TreeNodeType};
use crate::utils::{self, EnumDelims, EnumKind};

/// ### Parser
/// The parser type. Contains an optional
/// source line vector and a document tree.
/// These are optional to facilitate their passing
/// to any transitions functions via
/// `std::option::Option::take`
/// without invalidating the fields.
pub struct Parser {
  src_lines: Vec<String>,
  current_line: usize,
  doctree: Option<DocTree>,
  machine_stack: Vec<Option<StateMachine>>,
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
  fn new(src: String, doctree: DocTree, opt_initial_state: Option<StateMachine>) -> Self {

    let initial_state = opt_initial_state.or(Some(StateMachine::Body));

    Self {
      src_lines: src.lines().map(|s| s.to_string()).collect::<Vec<String>>(),
      current_line: 0,
      doctree: Some(doctree),
      machine_stack: vec!(initial_state)
    }

  }

  /// ### parse
  /// Starts the parsing process for a single file.
  /// Returns the `DocTree` generated by the `StateMachine`s.
  fn parse (&mut self) -> Result<Option<DocTree>, &'static str> {

    eprintln!("Initiating parse...\n");

    let init_machine = StateMachine::Body;

    self.machine_stack.push(Some(init_machine));

    let mut line_changed: bool = false;
    let mut line_not_changed_count: u32 = 0;

    // The parsing loop
    loop {

      if !line_changed && line_not_changed_count >= 10 {
        eprintln!("Line not advanced even after {} iterations of the parsing loop on line {}.\nClearly something is amiss...\n", line_not_changed_count, self.current_line);
        break
      }

      line_changed = false;

      let mut match_found = false;

      // Retrieving a clone of the transitions stored in the latest state
      // A clone is needed because the below for loop takes
      // ownership of a reference given to it, which would prevent us from
      // modifying the machine stack.
      let latest_state_transitions = if let Some(machine) = self.machine_stack.last() {

        // We need as_ref(), as unwrap() returns a reference to an Option and not the StateMachine itself
        if let Some(machine) = machine.as_ref() {

          match machine {
            StateMachine::EOF => {
              eprintln!("Moved past EOF...\n");

              // Walk to doctree root before returning it
              match self.doctree.take() {
                Some(mut tree_wrapper) => {
                  tree_wrapper.tree = tree_wrapper.tree.walk_to_root();
                  self.doctree.replace(tree_wrapper);
                }
                None => {
                  return Err("Tree should not be in the possession of a transition method after moving past EOF...\n")
                }
              };
              break
            }
            StateMachine::Failure{ .. } => {
              return Err("Parsing ended in Failure state...\n");
            }
            _ => {
              if let Ok(transitions_ref) = machine.get_transitions() {
                transitions_ref.clone()
              } else {
                return Err("No transitions for this state...\n")
              }
            }
          }
        } else {
          return Err("Parsing ended in Failure state.\n")
        }
      } else {
        break
      };

      // Iterating over a clone of the transitions
      for (pattern_name, regex, method) in latest_state_transitions.iter() {

        // Fetching a reference to current line
        let src_line: &str = match Parser::get_source_from_line(&self.src_lines, self.current_line) {
          Ok(line) => line,
          Err(e) => {
            eprintln!("{}", e);
            return Err("Parsing ended prematurely because of an unqualified move past EOF.\n")
          }
        };

        // Running the current line of text through a DFA compiled from a regex
        if regex.is_match(src_line) {

          eprintln!("Found match for {:?}...\n", pattern_name);

          match_found = true;

          let captures = regex.captures(src_line).unwrap();

          eprintln!("Match: {:#?}", captures.get(0).unwrap().as_str());

          eprintln!("Executing transition method...\n");

          let line_before_transition = self.current_line;

          self.doctree = match method(&self.src_lines, &mut self.current_line, self.doctree.take(), captures, pattern_name) {

            Ok((opt_doctree, opt_next_state, push_or_pop, opt_line_advance)) => {

              match push_or_pop {
                PushOrPop::Push => {
                  // If a transition method returns a state, check whether we should transition to it or
                  // push it on top of the stack...
                  if let Some(next_state) = opt_next_state {
                    eprintln!("Pushing {:#?} on top of stack...\n", next_state);
                    self.machine_stack.push(Some(next_state))
                  }
                },
                PushOrPop::Pop => {
                  eprintln!("Received POP instruction...\n");
                  match self.machine_stack.pop() {
                    Some(machine) => (),
                    None => {
                      return Err("Can't pop from empty stack.\n")
                    }
                  };
                }
                PushOrPop::Neither => {
                  if let Some(next_state) = opt_next_state {
                    let machine = match self.machine_stack.last_mut() {
                      Some(opt_machine) => opt_machine.replace(next_state),
                      None => return Err("No machine on top of stack.\nCan't perform transition after executing transition method.\n")
                    };
                  }
                }
              };

              match opt_line_advance {
                LineAdvance::Some(offset) => {
                  self.current_line += offset;
                }
                _ => ()
              }

              if self.current_line == line_before_transition {
                line_not_changed_count += 1;
              } else {
                line_changed = true;
                line_not_changed_count = 0;
              }

              opt_doctree

            }

            Err(e) => {
              eprintln!("{} on line {}", e, self.current_line);
              return Err("An error was encountered while executing a transition method.\n");
            }
          };
        }

        if match_found {
          break
        }

      }

      // Transtition the latest machine to a general failure state if no match found...
      if !match_found {

        eprintln!("No match found.\nPopping from machine stack...\n");

        let mut doctree = self.doctree.take().unwrap();

        doctree.tree = match doctree.tree.focus_on_parent() {
          Ok(parent) => parent,
          Err(root_node) => return Err("No node parent when reducing nesting level...\n")
        };

        self.doctree.replace(doctree);

        let _ = match self.machine_stack.pop() {
          Some(machine) => machine,
          None => return Err("Cannot pop from an empty stack.\n")
        };
      }



      if self.current_line >= self.src_lines.len() {

        let opt_machine = if let Some(machine)  = self.machine_stack.last_mut() {
          let eof_state = StateMachine::EOF;
          machine.replace(eof_state)
        } else {
          return Err("Cannot transition missing machine to EOF state\n")
        };
      }

      eprintln!("Machine stack state: {:#?}", self.machine_stack);
      eprintln!("On line {:#?}\n", self.current_line);

    };

    Ok(self.doctree.take())

  }

  /// ### enum_str_to_int_and_kind
  /// Converts an enumerator &str to an integer using one of the
  /// coverters, if possible.
  fn enum_str_to_int_and_kind (detected_enum_str: &str, detected_kind: &EnumKind, list_item_number: Option<usize>) -> Option<(usize, EnumKind)> {

    let list_item_number = list_item_number.unwrap_or(0);

    if detected_enum_str == "i" && list_item_number == 0 {
      // LowerRoman list at our hands
      return Some((1, EnumKind::LowerRoman))
    } else if detected_enum_str == "I" && list_item_number == 0 {
      // UpperRoman list at our hands
      return Some((1, EnumKind::UpperRoman))
    }
    
    let detected_enum_as_usize = match detected_kind {

      EnumKind::Arabic => {
        detected_enum_str.parse::<usize>().unwrap() // Standard library has implemented conversions from str to integers
      }
  
      EnumKind::LowerAlpha | EnumKind::UpperAlpha => {
        if let Some(num) = Parser::alpha_to_usize(detected_enum_str) {
          num
        } else {
          eprintln!("Couldn't convert given alphabet to an integer...\n");
          return None
        }
      }
  
      EnumKind::LowerRoman => {
        if let Some(num) = Parser::lower_roman_to_usize(detected_enum_str) {
          num
        } else {
          eprintln!("Couldn't convert lower-case Roman numeral to an integer...\n");
          return None
        }
      }
  
      EnumKind::UpperRoman => {
        if let Some(num) = Parser::lower_roman_to_usize(detected_enum_str) {
          num
        } else {
          eprintln!("Couldn't convert upper-case Roman numeral to an integer...\n");
          return None
        }
      }
    };

    Some(
      (detected_enum_as_usize, *detected_kind)
    )

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



/// ===========================
/// Parser associated functions
/// ===========================
impl Parser {

  /// ### get_source_from_line
  /// Attempts to retrieve the source from a given line number.
  /// Returns an `Ok` clone of it if successful, else
  /// returns and `Err` with a message.
  fn get_source_from_line (src_lines: &Vec<String>, line_num: usize) -> Result <&str, String> {

    let src = match src_lines.get(line_num) {
      Some(line) => line.as_str(),
      None => return Err(format!("No such line number ({} out of bounds).\nComputer says no...\n", line_num))
    };

    Ok(src)

  }


    /// ### inline_parse
  /// A function that parses inline text. Returns the nodes generated,
  /// if there are any.
  fn inline_parse (inline_src_block: String, current_line: &mut usize) -> Option<Vec<TreeNode>> {

    let mut nodes: Vec<TreeNode> = Vec::new();

    let mut col: usize = 0;

    // Remove backslashes
    let src_without_escapes = inline_src_block.replace("\\", "");

    let src_chars = &mut src_without_escapes.chars();

    match Parser::match_iter(&src_chars) {
      Some((node, offset)) => {

        nodes.push(node);

        // Move iterator to start of next possible match
        for _ in 0..offset - 1 {
          let c = src_chars.next().unwrap();
          eprintln!("Consuming {:#?}...", c);

          col += 1;

          if c == '\n' {
            eprintln!("Detected newline...\n");
            *current_line += 1;
            col = 0;
          }
        }
      },

      None => {
        eprintln!("No match on line {}, col {}.\nProceeding to consume next character...\n", current_line, col);
      }
    }

    while let Some(c) = src_chars.next() {

      eprintln!("Consuming {:#?}...\n", c);

      col += 1;

      if c == '\n' {
        eprintln!("Detected newline...\n");
        *current_line += 1;
        col = 0;
      }

      match Parser::match_iter(&src_chars) {
        Some((node, offset)) => {

          nodes.push(node);

          // Move iterator to start of next possible match
          for _ in 0..offset - 1 {
            let c = src_chars.next().unwrap();
            eprintln!("Consuming {:#?}", c);

            col += 1;

            if c == '\n' {
              eprintln!("Detected newline...\n");
              *current_line += 1;
              col = 0;
            }
          }
        },

        None => {
          eprintln!("No match on line {}, col {}.\n", current_line, col);
        }
      }
    }

    if nodes.is_empty() {
      return None
    }

    Some(nodes)

  }

  /// ### match_iter
  /// A function for checking the string representation of
  /// a given `Chars` iterator for a regex match and executing
  /// the corresponding parsing method. Returns the `Option`al
  /// generated node if successful, otherwise returns with `None`.
  fn match_iter <'chars> (chars_iter: &'chars str::Chars) -> Option<(TreeNode, usize)> {

    let src_str = chars_iter.as_str();

    if src_str.is_empty() {
      eprintln!("Source has been drained of characters.\n");
      return None
    }

    eprintln!("Matching against {:#?}\n", src_str);

    for (pattern_name, regexp, parsing_function) in COMPILED_INLINE_TRANSITIONS.iter() {

      match regexp.captures(src_str) {

        Some(capts) => {

          eprintln!("Match found for {:#?}\n", pattern_name);

          let (node, offset) = parsing_function(*pattern_name, &capts);

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

    eprintln!("Loop broken: {:?}\n", loop_broken);

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
          let trunc_line = match utils::strip_indent(line.clone(), indent) {
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

    Ok((block_lines, minimal_indent.unwrap(), line_diff, blank_finish))

  }


  /// ### lower_alpha_to_u32
  fn alpha_to_usize (alpha_str: &str) -> Option<usize> {
    match alpha_str {
      "A" | "a" => Some(1),
      "B" | "b" => Some(2),
      "C" | "c" => Some(3),
      "D" | "d" => Some(4),
      "E" | "e" => Some(5),
      "F" | "f" => Some(6),
      "G" | "g" => Some(7),
      "H" | "h" => Some(8),
      "I" | "i" => Some(9),
      "J" | "j" => Some(10),
      "K" | "k" => Some(11),
      "L" | "l" => Some(12),
      "M" | "m" => Some(13),
      "N" | "n" => Some(14),
      "O" | "o" => Some(15),
      "P" | "p" => Some(16),
      "Q" | "q" => Some(17),
      "R" | "r" => Some(18),
      "S" | "s" => Some(19),
      "T" | "t" => Some(20),
      "U" | "u" => Some(21),
      "V" | "v" => Some(22),
      "W" | "w" => Some(23),
      "X" | "x" => Some(24),
      "Y" | "y" => Some(25),
      "Z" | "z" => Some(26),
      _         => {
        eprintln!("Error: Letter '{}' not recognized as integer by reStructuredText...\n", alpha_str);
        None
      }
    }
  }


  /// ### upper_roman_to_u32
  /// Converts a valid given upper-case Roman numeral to a `Some(u32)`.
  /// If the numeral isn't valid, `None` is returned instead
  fn upper_roman_to_usize (roman_str: &str) -> Option<usize> {

    let mut num_val: usize = 0;
    let mut buffer = String::with_capacity(2);
    let mut chars_iter = roman_str.chars().peekable();

    const ROMAN_MAX: usize = 4999;

    while let Some(c1) = chars_iter.next() {

      buffer.push(c1);

      match c1 {
        'C' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'M' || *c2 == 'D' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        'X' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'C' || *c2 == 'L' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        'I' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'X' || *c2 == 'V' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        _ => ()
      }


      // Convert the contents of the buffer to u32, if valid.
      let buf_str = buffer.as_str();

      match buf_str {
        "M"   => num_val += 1000,
        "CM"  => num_val += 900,
        "D"   => num_val += 500,
        "CD"  => num_val += 400,
        "C"   => num_val += 100,
        "XC"  => num_val += 90,
        "L"   => num_val += 50,
        "XL"  => num_val += 40,
        "X"   => num_val += 10,
        "IX"  => num_val += 9,
        "V"   => num_val += 5,
        "IV"  => num_val += 4,
        "I"   => num_val += 1,
        _     => {
          eprintln!("No match for supposed upper-case Roman numeral {}...\n", buf_str);
          return None
        }
      }

      if num_val > ROMAN_MAX {
        eprintln!("Roman numerals greater than {} not supported by reStructuredText\n", ROMAN_MAX);
        return None
      }

      buffer.clear();
    }

    Some(num_val)
  }


  /// ### lower_roman_to_u32
  /// Converts a valid given lower-case Roman numeral to a `Some(u32)`.
  /// If the numeral isn't valid, `None` is returned instead
  fn lower_roman_to_usize (roman_str: &str) -> Option<usize> {

    let mut num_val: usize = 0;
    let mut buffer = String::with_capacity(2);
    let mut chars_iter = roman_str.chars().peekable();

    const ROMAN_MAX: usize = 4999;

    while let Some(c1) = chars_iter.next() {

      buffer.push(c1);

      match c1 {
        'c' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'm' || *c2 == 'd' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        'x' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'c' || *c2 == 'l' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        'i' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'x' || *c2 == 'v' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        _ => ()
      }


      // Convert the contents of the buffer to u32, if valid.
      let buf_str = buffer.as_str();

      match buf_str {
        "m"   => num_val += 1000,
        "cm"  => num_val += 900,
        "d"   => num_val += 500,
        "cd"  => num_val += 400,
        "c"   => num_val += 100,
        "xc"  => num_val += 90,
        "l"   => num_val += 50,
        "xl"  => num_val += 40,
        "x"   => num_val += 10,
        "ix"  => num_val += 9,
        "v"   => num_val += 5,
        "iv"  => num_val += 4,
        "i"   => num_val += 1,
        _     => {
          eprintln!("No match for supposed lower-case Roman numeral {}...\n", buf_str);
          return None
        }
      }

      if num_val > ROMAN_MAX {
        eprintln!("Roman numerals greater than {} not supported by reStructuredText\n", ROMAN_MAX);
        return None
      }

      buffer.clear();
    }

    Some(num_val)
  }

}


// /// ### val_from_key
// /// Goes through a given list of tuples
// /// ```
// /// (TokenType, str_pattern, Action)
// /// ```
// /// and looks for a matching tokentype.
// /// If it finds one, returns and `Option<&'static str>`,
// /// otherwise returns `None`.
// fn val_from_key(search_key: &TokenType, map: &[(TokenType, &'static str, Action)]) -> Option<&'static str> {
//   for (_, val, _) in map.iter().filter(|&(map_key, _, _)| map_key == search_key) { 
//     return Some(val);
//   }
//   None
// }
