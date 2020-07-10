/// ## body
/// This module contains the transition functions related to `StateMachine::Body`.

use super::*;


/// ### bullet
/// The transition method for matching bullets in `Body` state.
/// Causes the parser to push a new machine in the state
/// `BulletList` on top of its machine stack. Leaves the reponsibility
/// of the actual parsing to that state.
pub fn bullet (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {

  let mut tree_wrapper = doctree.unwrap();

  let bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
  let bullet_indent = captures.get(1).unwrap().as_str().chars().count();
  let text_indent = captures.get(0).unwrap().as_str().chars().count();

  let bullet_list_data = TreeNodeType::BulletList{bullet: bullet, bullet_indent:bullet_indent, text_indent: text_indent};

  let list_node = TreeNode::new(bullet_list_data);

  tree_wrapper.tree.node.push_child(list_node);

  tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
    Ok(child_zipper) => child_zipper,
    Err(node_itself) => {
      return Err("An error occurred when adding a child to the current node.\n");
    }
  };

  let next_state = StateMachine::BulletList;

  Ok( ( Some(tree_wrapper), Some(next_state), PushOrPop::Push, LineAdvance::None))

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
pub fn enumerator (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {

  let mut tree_wrapper = doctree.unwrap();

  let detected_enumerator_indent = captures.name("indent").unwrap().as_str().chars().count();
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();

  let enumerator_type = if let Some(enumerator) = StateMachine::check_enumerator_type(&captures) {
    enumerator
  } else {
    return Err("Enumerator detected but no known enumerator type!\n")
  };

  let node_data = TreeNodeType::EnumeratedList {
    enum_type: enumerator_type,
    enumerator_indent: detected_enumerator_indent,
    text_indent: detected_text_indent,
  };

  let list_node = TreeNode::new(node_data);

  tree_wrapper.tree.push_child(list_node);

  tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
    Ok(tree)  => tree,
    Err(tree) => return Err("Couldn't focus on enumerated list at body level...\n")
  };

  let next_state = StateMachine::EnumeratedList;

  Ok( ( Some(tree_wrapper), Some(next_state), PushOrPop::Push, LineAdvance::None ) )

}


pub fn paragraph (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {

  let mut tree_wrapper = doctree.unwrap();
  let indent = captures.get(1).unwrap().as_str().chars().count();

  let block = match Parser::read_indented_block(src_lines, Some(*current_line), Some(true), None, Some(indent), None) {
    Ok((lines, min_indent, line_offset, blank_finish)) => {
      lines.join("\n")
    }
    Err(e) => {
      eprintln!("{}", e);
      return Err("Error when reading paragraph block in Body.\n")
    }
  };

  // Pass text to inline parser as a string
  let mut inline_nodes = if let Some(children) = StateMachine::inline_parse(block, current_line) {
    children
  } else {
    return Err("Couldn't parse paragraph for inline nodes\n")
  };

  let data = TreeNodeType::Paragraph;

  let paragraph_node = TreeNode::new(data);

  tree_wrapper.tree.push_child(paragraph_node);

  tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
    Ok(child) => child,
    Err(node_itself) => return Err("Couldn't focus on child paragraph\n")
  };

  tree_wrapper.tree.append_children(&mut inline_nodes);

  tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
    Ok(parent) => parent,
    Err(node_self) => return Err("Couldn't move focus to paragraph parent...\n")
  };

  return Ok((Some(tree_wrapper), None, PushOrPop::Neither, LineAdvance::Some(1)))

}
