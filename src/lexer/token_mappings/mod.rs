/// This module contains the regexes used
/// by the Lexer.

pub mod body;
pub mod inline;

use crate::lexer::Lexer;
use crate::lexer::token::TokenType;

use regex;

/// ### type Action
/// A type alias for a tokenizer function
type Action = fn(&mut Lexer, TokenType, regex::Captures);


/// ### val_from_key
/// Searches through a list of TokenType--regex pairs
/// for a mathing tokentype
pub fn val_from_key(search_key: &TokenType, map: &[(TokenType, &'static str, Action)]) -> Option<String> {
  for (_, val, _) in map.iter().filter(|&(map_key, _, _)| map_key == search_key) { 
    return Some(val.to_string());
  }
  None
}
