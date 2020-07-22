/// ## field_list
/// A submodule that contains transition functions responsible for creating FieldList nodes.

use super::*;


/// ### field_marker
/// Creates FieldListItems, if parameters such as detected indentation and such match with the parent node paraneters.
pub fn field_marker (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {
  todo!()
}