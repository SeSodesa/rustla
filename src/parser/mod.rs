/// This is the `parser` module of ruSTLa

mod state_machine;
use state_machine::StateMachine;

#[cfg(test)]
mod tests;

use std::io::{BufReader, Lines};
use std::fs::File;

use std::str;
use regex;

use crate::doctree::DocTree;

use std::collections;


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
  fn parse (&mut self){

    unimplemented!();

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
