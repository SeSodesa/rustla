/// ## field_list
/// A submodule that contains transition functions responsible for creating FieldListItem nodes.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


/// ### field_marker
/// Creates FieldListItems, if parameters such as detected indentation and such match with the parent node ones.
pub fn field_marker (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

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
  match tree_wrapper.shared_node_data() {

    TreeNodeType::FieldList { marker_indent } => {

      // Parse the marker for inline nodes
      if *marker_indent == detected_marker_indent {

        let marker_inline_nodes = if let InlineParsingResult::Nodes(nodes_data) = Parser::inline_parse(detected_marker_name.to_string(), None, line_cursor) {
          nodes_data
        } else {
          return TransitionResult::Failure { // Should not happen in the first place, if a field marker was detected...
            message: format!("Tried parsing a field marker on line {} for inline nodes but none found.\nMarker not valid...\n", line_cursor.sum_total()),
            doctree: tree_wrapper
          }
        };

        let item_node_data = TreeNodeType::FieldListItem {
          raw_marker_name: detected_marker_name.to_string(),
          marker_name_as_inline_nodes: marker_inline_nodes,
          marker_indent: detected_marker_indent,
          body_indent: detected_body_indent
        };
        tree_wrapper = tree_wrapper.push_data_and_focus(item_node_data);

        let (doctree, offset, state_stack) = match Parser::parse_first_node_block(tree_wrapper, src_lines, base_indent, line_cursor, detected_body_indent, Some(detected_text_indent), StateMachine::ListItem, section_level, false) {
          Ok((parsing_result, offset)) => if let ParsingResult::EOF { doctree, state_stack } | ParsingResult::EmptyStateStack { doctree, state_stack } = parsing_result {
            (doctree, offset, state_stack)
          } else {
            unreachable!("Returned from a nested parsing session on line {} without necessary information. Computer says no...", line_cursor.sum_total())
          },
          Err(ParsingResult::Failure { message, doctree }) => return TransitionResult::Failure {
            message: format!("Looks like field list item on line {} has no content.\nComputer says no...\n", line_cursor.sum_total()),
            doctree: doctree
          },
          _ => unreachable!("Parsing first node block on line {} resulted in unknown combination of return values. Computer says no...", line_cursor.sum_total())
        };

        tree_wrapper = doctree;

        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_states: Some(state_stack),
          push_or_pop: PushOrPop::Push,
          line_advance: LineAdvance::Some(offset),
        }

      } else {

        tree_wrapper = tree_wrapper.focus_on_parent();
        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_states: None,
          push_or_pop: PushOrPop::Pop,
          line_advance: LineAdvance::None,
        }
      }
    }

    _ => return TransitionResult::Failure {
        message: format!("Attempted parsing a FieldListItem outside a FieldList on line {}.\nComputer says no...\n", line_cursor.sum_total()),
        doctree: tree_wrapper
    }
  }
}
