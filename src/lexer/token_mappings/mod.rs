/// This module contains the regexes used
/// by the Lexer.

pub mod titles;
pub mod lists;
pub mod blocks;

use crate::lexer::token::TokenType;

/// ### val_from_key
/// Searches through a 
pub fn val_from_key<'t>(search_key: &TokenType, map: &[(TokenType, &'static str)]) -> Option<String> {
  for (_, val) in map.iter().filter(|&(map_key, _)| map_key == search_key) { 
    return Some(val.to_string());
  }
  None
}
