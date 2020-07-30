/// ## enumerated_list
/// A submodule for `Statemachine::EnumeratedList` related transition functions.

use super::*;

pub fn enumerator (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let (list_delims, list_kind, list_start_index, list_item_number,list_enumerator_indent, list_text_indent) = match tree_wrapper.tree.node.data {
    TreeNodeType::EnumeratedList { delims, kind, start_index, n_of_items, enumerator_indent, latest_text_indent } => (delims, kind, start_index, n_of_items, enumerator_indent, latest_text_indent),
    _ => return TransitionResult::Failure {
      message: String::from("Not focused on EnumeratedList...\n")
    }
  };

  let detected_enumerator_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;
  let detected_enum_str = captures.get(2).unwrap().as_str();

  let (detected_delims, detected_kind) = if let PatternName::Enumerator { delims, kind} = pattern_name {
    (*delims, *kind)
  } else {
    return TransitionResult::Failure {
      message: String::from("No enumerator inside enumerator transition method.\nWhy...?\n")
    }
  };

  let (detected_enum_as_usize, detected_kind) = match Parser::enum_str_to_int_and_kind(detected_enum_str, &detected_kind, &list_kind, true, Some(list_item_number), Some(list_start_index)) {
    Some((int, kind)) => (int, kind),
    None => return TransitionResult::Failure {
      message: String::from("Unknown enumerator type detected...?\n")
    }
  };

  eprintln!("Detected enumerator type pair ({:#?}, {:#?}) as {:#?}...\n", detected_delims, detected_kind, detected_enum_as_usize);

  match tree_wrapper.tree.node.data {

    TreeNodeType::EnumeratedList { delims, kind, enumerator_indent, latest_text_indent, n_of_items, start_index } => {

      if delims == detected_delims && detected_kind == list_kind && enumerator_indent == detected_enumerator_indent && detected_enum_as_usize == list_item_number + list_start_index {
        // Modify list parameters
        match &mut tree_wrapper.tree.node.data {
          TreeNodeType::EnumeratedList {n_of_items, latest_text_indent, ..} => {
            *n_of_items += 1;
            *latest_text_indent = detected_text_indent;
          },
          _ => return TransitionResult::Failure {
            message: String::from("Only enumerated lists keep track of the number of item nodes in them...\n")
          }
        }

        let item_node_data = TreeNodeType::EnumeratedListItem {
          delims: delims,
          kind: detected_kind,
          index_in_list: detected_enum_as_usize,
          enumerator_indent: detected_enumerator_indent,
          text_indent: detected_text_indent
        };

        tree_wrapper = tree_wrapper.push_and_focus(item_node_data);

        let (doctree, offset, state_stack) = match Parser::parse_first_node_block(tree_wrapper, src_lines, base_indent, line_cursor, list_text_indent, None, StateMachine::ListItem) {
          Some((doctree, nested_parse_offset, state_stack)) => (doctree, nested_parse_offset, state_stack),
          None => return TransitionResult::Failure {message: format!("Could not parse the first block of list item on line {:#?}", line_cursor.sum_total())}
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

    _ => {} // Not inside enumerated list...

  }

  TransitionResult::Failure {
    message: format!("Only enumerated lists may contain enumerated list items.\nComputer says no...\n")
  }
}
