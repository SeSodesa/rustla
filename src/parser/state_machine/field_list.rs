/// ## field_list
/// A submodule that contains transition functions responsible for creating FieldListItem nodes.

use super::*;


/// ### field_marker
/// Creates FieldListItems, if parameters such as detected indentation and such match with the parent node ones.
pub fn field_marker (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;
  let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_marker_name = captures.get(2).unwrap().as_str();

  let detected_body_indent = if let Some(line) = src_lines.get(line_cursor.relative_offset() + 1) {
    if line.trim().is_empty() {
      detected_text_indent
    } else {
      let indent = line.chars().take_while(|c| c.is_whitespace()).count() + base_indent;
      if indent < detected_marker_indent + 1 { detected_text_indent } else { indent }
    }
  } else { detected_text_indent };

  // Make sure we are inside a FieldList and that indentations match
  match tree_wrapper.get_node_data() {

    TreeNodeType::FieldList { marker_indent} => {

      // Parse the marker for inline nodes
      if *marker_indent == detected_marker_indent {

        let marker_inline_nodes = if let Some(nodes) = Parser::inline_parse(detected_marker_name.to_string(), line_cursor, &mut tree_wrapper.node_count) {
          nodes
        } else {
          return TransitionResult::Failure { // Should not happen in the first place, if a field marker was detected...
            message: format!("Tried parsing a field marker on line {} for inline nodes but none found.\nMarker not valid...\n", line_cursor.sum_total())
          }
        };

        let item_node_data = TreeNodeType::FieldListItem {
          raw_marker_name: detected_marker_name.to_string(),
          marker_name_as_inline_nodes: marker_inline_nodes,
          marker_indent: detected_marker_indent,
          body_indent: detected_body_indent
        };
        tree_wrapper = tree_wrapper.push_and_focus(item_node_data);

        let (doctree, offset, state_stack) = match Parser::parse_first_node_block(tree_wrapper, src_lines, base_indent, line_cursor, detected_body_indent, Some(detected_text_indent), StateMachine::ListItem) {
          Some((doctree, nested_parse_offset, state_stack)) => (doctree, nested_parse_offset, state_stack),
          None => return TransitionResult::Failure {message: format!("Could not parse the first block of footnote on line {:#?}.\nComputer says no...\n", line_cursor.sum_total())}
        };

        tree_wrapper = doctree;

        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: None,
          push_or_pop: PushOrPop::Push,
          line_advance: LineAdvance::Some(offset),
          nested_state_stack: Some(state_stack)
        }

      } else {

        tree_wrapper = tree_wrapper.focus_on_parent();
        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: None,
          push_or_pop: PushOrPop::Pop,
          line_advance: LineAdvance::None,
          nested_state_stack: None
        }
      }
    }

    _ => return TransitionResult::Failure {
        message: format!("Attempted parsing a FieldListItem outside a FieldList on line {}.\nComputer says no...\n", line_cursor.sum_total())
    }
  }
}
