/// ## body
/// This module contains the transition functions related to `StateMachine::Body`.

use super::*;


/// ### bullet
/// The transition method for matching bullets in `Body` state.
/// Causes the parser to push a new machine in the state
/// `BulletList` on top of its machine stack. Leaves the reponsibility
/// of the actual parsing to that state.
pub fn bullet (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
  let bullet_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;

  let bullet_list_data = TreeNodeType::BulletList{bullet: bullet, bullet_indent:bullet_indent, text_indent: text_indent};

  tree_wrapper.tree = match tree_wrapper.tree.push_and_focus(bullet_list_data) {
    Ok(tree) => tree,
    Err(..) => return TransitionResult::Failure {
      message: String::from("Couldn't focus on bullet list...\n")
    }
  };

  let next_state = StateMachine::BulletList;

  TransitionResult::Success {
    doctree: tree_wrapper,
    next_state: Some(StateMachine::BulletList),
    push_or_pop: PushOrPop::Push,
    line_advance: LineAdvance::None
  }

}


/// ### enumerator
/// Transition method for matching enumerators in the `Body` state.
/// Attempts to create a new enumerated list node and focus on it,
/// while at the same time pushing a new `EnumeratedList` state on
/// top of the parser machine stack.
/// 
/// This does not yet parse the first detected list item.
/// That responsibility is on the corresponding enumerator method
/// of the `EnumeratedList` state.
pub fn enumerator (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

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

  eprintln!("Start index: {}\n", detected_enum_as_usize);

  let list_node_data = TreeNodeType::EnumeratedList {
    delims: detected_delims,
    kind: detected_kind,
    start_index: detected_enum_as_usize,
    n_of_items: 0,
    enumerator_indent: detected_enumerator_indent,
    latest_text_indent: detected_text_indent,
  };

  eprintln!("List data: {:#?}\n", list_node_data);

  tree_wrapper.tree = tree_wrapper.tree.push_and_focus(list_node_data).unwrap();

  let next_state = StateMachine::EnumeratedList;

  TransitionResult::Success {
    doctree: tree_wrapper,
    next_state: Some(StateMachine::EnumeratedList),
    push_or_pop: PushOrPop::Push,
    line_advance: LineAdvance::None
  }

}


pub fn paragraph (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();
  let indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

  let block = match Parser::read_indented_block(src_lines, Some(*current_line), Some(true), None, Some(indent), None) {
    Ok((lines, min_indent, line_offset, blank_finish)) => {
      lines.join("\n")
    }
    Err(e) => {
      eprintln!("{}", e);
      return TransitionResult::Failure {
        message: String::from("Error when reading paragraph block in Body.\n")
      }
    }
  };

  // Pass text to inline parser as a string
  let mut inline_nodes = if let Some(children) = Parser::inline_parse(block, current_line) {
    children
  } else {
    return TransitionResult::Failure {
      message: String::from("Couldn't parse paragraph for inline nodes\n")
    }
  };

  let data = TreeNodeType::Paragraph;

  let paragraph_node = TreeNode::new(data);

  tree_wrapper.tree.push_child(paragraph_node);

  tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
    Ok(child) => child,
    Err(node_itself) => return TransitionResult::Failure {
      message: String::from("Couldn't focus on child paragraph\n")
    }
  };

  tree_wrapper.tree.append_children(&mut inline_nodes);

  tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
    Ok(parent) => parent,
    Err(node_self) => return TransitionResult::Failure {
      message: String::from("Couldn't move focus to paragraph parent...\n")
    }
  };

  TransitionResult::Success {
    doctree: tree_wrapper,
    next_state: None,
    push_or_pop: PushOrPop::Neither,
    line_advance: LineAdvance::Some(1)
  }

}
