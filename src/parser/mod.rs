/// This is the `parser` module of ruSTLa

mod state_machine;
use state_machine::{StateMachine, MachineWithState};

#[cfg(test)]
mod tests;

use std::io::{BufReader, Lines};
use std::fs::File;

use std::str;
use regex;

use crate::doctree::DocTree;

use std::collections;

/// ### Parser
/// The parser type. Contains an optional
/// source line vector and a document tree.
/// These are optional to facilitate their passing
/// to any state machine in `machine_stack` via
/// `std::option::Option::take`
/// without invalidating the fields.
pub struct Parser {
  src_lines: Option<Vec<String>>,
  doctree: Option<DocTree>,
  machine_stack: Vec<StateMachine>,
}


/// Parser type methods
impl Parser {

  /// ### new
  /// The `Parser` constructor. Transforms a given source string
  /// into a vector of lines and wraps this and a given `DocTree`
  /// in `Option`s. This wrapping allows the passing of these to owned
  /// state machnes via swapping the optional contents
  /// to `None` before granting ownership of the original contents.
  fn new(src: String, doctree: DocTree) -> Self {

    Self {
      src_lines: Some(src.lines().map(|s| s.to_string()).collect::<Vec<String>>()),
      doctree: Some(doctree),
      machine_stack: Vec::with_capacity(2)
    }

  }

  /// ### parse
  /// Starts the parsing process for a single file.
  /// Returns the `DocTree` generated by the `StateMachine`s.
  fn parse (&mut self) -> Result<Option<DocTree>, &'static str>{

    let init_state = match MachineWithState::new(self.src_lines.take(), 0, self.doctree.take()) {
      Ok(state) => state,
      Err(e) => {
        eprintln!("{}", e);
        return Err("\nInitial state could not be constructed.\n")
      }
    };

    let init_machine = StateMachine::Body(init_state);

    self.machine_stack.push(init_machine);

    // The parsing loop
    let dt = loop {
      todo!();
    };

    Ok(self.doctree.take())

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
