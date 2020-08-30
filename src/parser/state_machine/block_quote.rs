/// ## block_quote
/// Contains the transition function for parsing attributions.

use super::*;

///
pub fn attribution (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap();

  

  todo!()
}