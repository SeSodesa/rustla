/// ## definition_list
/// A submodule with transition functions related to parsing definition list items

use super::*;


/// ### text
/// A transition function for parsing definition list items.
pub fn text (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut doctree = doctree.unwrap();
  let detected_indent = captures.get(1).unwrap().as_str().chars().count();

  let next_line = src_lines.get(line_cursor.relative_offset() + 1);

  if let Some(n_line) = next_line {

    let next_line_indent = n_line.chars().take_while(|c| c.is_whitespace()).count();
    if next_line_indent <= detected_indent {
      return TransitionResult::Failure {
        message: format!("Tried parsing a definition list item on line {}, but definition does not have enough indent.\nComputer says no...\n", line_cursor.sum_total())
      }
    }

    // Read in definition term, classifiers and parse firstnode block.
    let (term, classifiers): (String, Vec<String>) = if let Some(line) = src_lines.get(line_cursor.relative_offset()) {

      let mut term_and_classifiers= line.split(" : ");
      let term: String = if let Some(term) = term_and_classifiers.next() { term.to_string() } else {
        return TransitionResult::Failure {
          message: format!("A definition without a term to be defined on line {}?\nComputer says no...\n", line_cursor.sum_total())
        }
      };
      let classifiers: Vec<String> = term_and_classifiers.filter(|s| !s.trim().is_empty()).map(|s| s.to_string()).collect();
      (term, classifiers)
    } else {
      return TransitionResult::Failure {
        message: format!("Could not extract term (and classifiers)? from a definition list item.\nComputer says no...\n")
      }
    };


  } else {
    return TransitionResult::Failure {
      message: format!("Found a definition list item candidate without a definition on line {}.\nComputer says no...\n", line_cursor.sum_total())
    }
  }

  todo!()
}