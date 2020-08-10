/// ## literal_block
/// A submodule for parsing functiosn related to literal blocks of reStructuredText.

use super::*;


/// ### indented_literal_block
/// A function for parsing indented literal block nodes.
pub fn indented_literal_block (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {
  todo!()
}


/// ### indented_literal_block
/// A function for parsing "quoted" literal block nodes.
pub fn quoted_literal_block (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {
  todo!()
}
