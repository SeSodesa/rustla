/// This is the `parser` module of ruSTLa

mod state_machine;
use state_machine::StateMachine;


mod token;
mod position;
mod error;

#[cfg(test)]
mod tests;

use std::io::{self, BufReader, Lines};
use std::fs::File;

use std::fmt;
use std::str;
use regex;

use crate::doctree::DocTree;

use crate::parser::token::{Token, TokenType};
use crate::parser::position::Pos;
use std::collections;
use crate::parser::error::{TokenizeError, ParseError};
use state_machine::transitions::{BODY_TRANSITIONS, INLINE_TRANSITIONS};


pub struct Parser {
  line_iter: String,
  machine_stack: Vec<StateMachine>,
  has_error: bool
}


/// Parser type methods
impl Parser {

  /// ### new
  /// The `Parser` constructor. Transforms the given source into a vector
  /// of `String`s, generates a `DocTree`, and passes these
  /// to the `StateMachine` at the top of the `Parser` machine stack,
  /// which is initialized in the `Body` `State`.
  fn new(source_line_iter: Lines<BufReader<File>>) {

    let src_lines: Vec<String> = source_line_iter.collect::<Result<_, _>>().unwrap();

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
