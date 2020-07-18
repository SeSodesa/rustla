/// ## enumerated_list
/// A submodule for `Statemachine::EnumeratedList` related transition functions.

use super::*;

pub fn enumerator (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let (list_delims, list_kind, list_start_index, list_item_number,list_enumerator_indent, list_text_indent) = match &mut tree_wrapper.tree.node.data {
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

  let (detected_enum_as_usize, detected_kind) = match Parser::enum_str_to_int_and_kind(detected_enum_str, &detected_kind, None) {
    Some((int, kind)) => (int, kind),
    None => return TransitionResult::Failure {
      message: String::from("Unknown enumerator type detected...?\n")
    }
  };

  eprintln!("Detected enumerator type pair ({:#?}, {:#?}) as {:#?}...\n", detected_delims, detected_kind, detected_enum_as_usize);

  // Matching detected parameters against corresponding list ones and proceeding accordingly
  match (detected_delims, detected_kind, detected_enumerator_indent, detected_text_indent) {

    (delims, kind, enum_indent, text_indent) if delims == *list_delims && kind == *list_kind && enum_indent == *list_enumerator_indent && detected_enum_as_usize == *list_item_number + 1 => {

      // All parameters are the same, so this ListItem is a direct child of the current EnumeratedList.
      // Create a new ListItem node, focus on it and push a ListItem state on top of the parser stack.

      match &mut tree_wrapper.tree.node.data {
        TreeNodeType::EnumeratedList {n_of_items, latest_text_indent, ..} => {
          *n_of_items += 1;
          *latest_text_indent = text_indent;
        },
        _ => return TransitionResult::Failure {
          message: String::from("Only enumerated lists keep track of the number of item nodes in them...\n")
        }
      }

      let item_node_data = TreeNodeType::EnumeratedListItem {
        delims: delims,
        kind: kind,
        index_in_list: detected_enum_as_usize,
        enumerator_indent: enum_indent,
        text_indent: text_indent
      };

      tree_wrapper.tree = tree_wrapper.tree.push_and_focus(item_node_data).unwrap();

      tree_wrapper = match first_block_of_enum_list_item(tree_wrapper, src_lines, base_indent, current_line, text_indent) {
        Some(doctree) => doctree,
        None => return TransitionResult::Failure {message: format!("Could not parse the first block of list item on line {:#?}", current_line)}
      };

      let next_state = StateMachine::ListItem;

      TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: Some(StateMachine::ListItem),
        push_or_pop: PushOrPop::Push,
        line_advance: LineAdvance::Some(1)
      }
    }

    _ => {
      eprintln!("No specific instruction for found detected enumerator parameters.\nSimply POPping from stack in hopes of the previous state knowing better...\n");

      tree_wrapper.tree = tree_wrapper.tree.focus_on_parent().unwrap();

      TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: None,
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::None
      }
    }
  }
}

/// ### first_block_of_enum_list_item
/// Parses the first block of an enumerated list item, in case it contains body level nodes
/// right after the enumerator.
fn first_block_of_enum_list_item (mut doctree: DocTree, src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, text_indent: usize) -> Option<DocTree>{

  // Read indented block here
  let block = match Parser::read_indented_block(src_lines, Some(*current_line), Some(true), None, Some(text_indent), Some(text_indent)) {
    Ok((lines, min_indent, line_offset, blank_finish)) => {
      lines.join("\n")
    }
    Err(e) => {
      eprintln!("{}\n", e);
      eprintln!("Error when reading list item block.\n");
      return None
    }
  };

  // Run a nested `Parser` over the first indented block with base indent set to `text_indent`.
  let doctree = match Parser::new(block.clone(), doctree, Some(text_indent), Some(StateMachine::Body)).parse() {
    ParsingResult::EOF {doctree} => return Some(doctree), // All of block as parsed successfully
    ParsingResult::EmptyStateStack { doctree } => doctree,
    ParsingResult::Failure {message} => {
      eprintln!("{:#?}", message);
      eprintln!("Nested parse ended in failure...\n");
      return None
    }
  };

  Some(doctree)
}
