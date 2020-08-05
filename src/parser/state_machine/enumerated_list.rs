/// ## enumerated_list
/// A submodule for `Statemachine::EnumeratedList` related transition functions.

use super::*;

pub fn enumerator (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let (list_delims, list_kind, list_start_index, n_of_items,list_enumerator_indent) = match tree_wrapper.get_mut_node_data() {
    TreeNodeType::EnumeratedList { delims, kind, start_index, n_of_items, enumerator_indent } => (delims, kind, start_index, n_of_items, enumerator_indent),
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

  let (detected_enum_as_usize, detected_kind) = match Parser::enum_str_to_int_and_kind(detected_enum_str, &detected_kind, &list_kind, true, Some(*n_of_items), Some(*list_start_index)) {
    Some((int, kind)) => (int, kind),
    None => return TransitionResult::Failure {
      message: String::from("Unknown enumerator type detected...?\n")
    }
  };

  eprintln!("Detected enumerator type pair ({:#?}, {:#?}) as {:#?}...\n", detected_delims, detected_kind, detected_enum_as_usize);

  if *list_delims == detected_delims && detected_kind == *list_kind && *list_enumerator_indent == detected_enumerator_indent && detected_enum_as_usize == *n_of_items + *list_start_index {

    // Modify relevant list parameters
    *n_of_items += 1;

    let item_node_data = TreeNodeType::EnumeratedListItem {
      delims: *list_delims,
      kind: detected_kind,
      index_in_list: detected_enum_as_usize,
      enumerator_indent: detected_enumerator_indent,
      text_indent: detected_text_indent
    };

    tree_wrapper = tree_wrapper.push_data_and_focus(item_node_data);

    let (doctree, offset, state_stack) = match Parser::parse_first_node_block(tree_wrapper, src_lines, base_indent, line_cursor, detected_text_indent, None, StateMachine::ListItem) {
      Some((doctree, nested_parse_offset, state_stack)) => (doctree, nested_parse_offset, state_stack),
      None => return TransitionResult::Failure {message: format!("Could not parse the first block of list item on line {:#?}", line_cursor.sum_total())}
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
