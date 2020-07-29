/// ## field_list
/// A submodule that contains transition functions responsible for creating FieldListItem nodes.

use super::*;


/// ### field_marker
/// Creates FieldListItems, if parameters such as detected indentation and such match with the parent node ones.
pub fn field_marker (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;
  let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_marker_name = captures.get(2).unwrap().as_str();

  let detected_body_indent = if let Some(next_line) = src_lines.get(*current_line + 1) {

    // Add check for an empty line?

    next_line.chars().take_while(|c| c.is_whitespace()).count() + base_indent
  } else {
    detected_text_indent
  };

  let one_line_body: bool = detected_marker_indent >= detected_body_indent;

  // Make sure we are inside a FieldList and that indentations match
  match tree_wrapper.tree.node.data {

    TreeNodeType::FieldList { marker_indent} => {

      if marker_indent == detected_marker_indent {
        let marker_inline_nodes = if let Some(nodes) = Parser::inline_parse(detected_marker_name.to_string(), current_line, &mut tree_wrapper.node_count) {
          nodes
        } else {
          return TransitionResult::Failure { // Should not happen in the first place, if a field marker was detected...
            message: format!("Tried parsing a field marker on line {} for inline nodes but none found.\nMarker not valid...\n", current_line)
          }
        };

        let (doctree, offset, state_stack) = if one_line_body {
          let item_node_data = TreeNodeType::FieldListItem {
            raw_marker_name: detected_marker_name.to_string(),
            marker_name_as_inline_nodes: marker_inline_nodes,
            marker_indent: detected_marker_indent,
            body_indent: detected_text_indent
          };
          tree_wrapper = tree_wrapper.push_and_focus(item_node_data);
          match Parser::first_list_item_block(tree_wrapper, src_lines, base_indent, current_line, detected_text_indent, None, StateMachine::ListItem) {
            Some((doctree, nested_parse_offset, state_stack)) => (doctree, nested_parse_offset, state_stack),
            None => return TransitionResult::Failure {message: format!("Could not parse the first block of list item on line {:#?}", current_line)}
          }
        } else {
          let item_node_data = TreeNodeType::FieldListItem {
            raw_marker_name: detected_marker_name.to_string(),
            marker_name_as_inline_nodes: marker_inline_nodes,
            marker_indent: detected_marker_indent,
            body_indent: detected_body_indent
          };

          tree_wrapper = tree_wrapper.push_and_focus(item_node_data);
          match Parser::first_list_item_block(tree_wrapper, src_lines, base_indent, current_line, detected_body_indent, Some(detected_text_indent), StateMachine::ListItem) {
            Some((doctree, nested_parse_offset, state_stack)) => (doctree, nested_parse_offset, state_stack),
            None => return TransitionResult::Failure {message: format!("Could not parse the first block of list item on line {:#?}", current_line)}
          }
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

    _ => {} // Not inside a field list...

  }

  return TransitionResult::Failure {
    message: format!("Attempted parsing a FieldListItem outside a FieldList on line {}.\nComputer says no...\n", current_line)
  }

}
