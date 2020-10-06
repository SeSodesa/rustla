/// ## definition_list
/// A submodule with transition functions related to parsing definition list items.
/// Every other transition function is redefined to focus on tree parent and pop
/// from the parser state stack.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

/// ### text
/// A transition function for parsing definition list items.
pub fn text (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut doctree = doctree.unwrap();
  let detected_indent = captures.get(1).unwrap().as_str().chars().count();

  let next_line = src_lines.get(line_cursor.relative_offset() + 1);

  if let Some(n_line) = next_line {

    let next_line_indent = n_line.chars().take_while(|c| c.is_whitespace()).count();
    if next_line_indent <= detected_indent {

      doctree = doctree.focus_on_parent();

      return TransitionResult::Success {
        doctree: doctree,
        next_states: None,
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::None
      }
    }

    // Read in definition term, classifiers and parse first node block.
    let (term, classifiers): (String, Vec<String>) = if let Some(line) = src_lines.get(line_cursor.relative_offset()) {

      let mut term_and_classifiers= line.split(" : ");
      let term: String = if let Some(term) = term_and_classifiers.next() { term.to_string() } else {
        return TransitionResult::Failure {
          message: format!("A definition without a term to be defined on line {}?\nComputer says no...\n", line_cursor.sum_total()),
          doctree: doctree
        }
      };
      let classifiers: Vec<String> = term_and_classifiers.filter(|s| !s.trim().is_empty()).map(|s| s.to_string()).collect();
      (term, classifiers)
    } else {
      return TransitionResult::Failure {
        message: format!("Could not extract term (and classifiers)? from a definition list item.\nComputer says no...\n"),
        doctree: doctree
      }
    };

    let list_item_node = TreeNodeType::DefinitionListItem {
      term: term,
      classifiers: classifiers,
      body_indent: next_line_indent
    };

    doctree = doctree.push_data_and_focus(list_item_node);

    return TransitionResult::Success {
      doctree: doctree,
      next_states: Some(vec![StateMachine::ListItem]),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::Some(1)
    }
  } else {

    doctree = doctree.focus_on_parent();

    return TransitionResult::Success {
      doctree: doctree,
      next_states: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None
    }
  }
}