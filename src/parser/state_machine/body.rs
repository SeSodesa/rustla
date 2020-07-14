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

  let detected_enumerator_indent = captures.get(1).unwrap().as_str().chars().count();
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();
  let detected_enum_str = captures.get(2).unwrap().as_str();

  let (detected_delims, mut detected_kind) = if let PatternName::Enumerator { delims, kind} = pattern_name {
    (*delims, *kind)
  } else {
    return Err("No enumerator inside enumerator transition method.\nWhy...?\n")
  };

  let mut detected_enum_as_usize = match detected_kind {

    EnumKind::Arabic => {
      detected_enum_str.parse::<usize>().unwrap() // Standard library has implemented conversions from str to integers
    }

    EnumKind::LowerAlpha | EnumKind::UpperAlpha => {
      if let Some(num) = Parser::alpha_to_usize(detected_enum_str) {
        num
      } else {
        return Err("Couldn't convert alphabet to an integer...\n")
      }
    }

    EnumKind::LowerRoman => {
      if let Some(num) = Parser::lower_roman_to_usize(detected_enum_str) {
        num
      } else {
        return Err("Couldn't convert lower-case Roman numeral to an integer...\n")
      }
    }

    EnumKind::UpperRoman => {
      if let Some(num) = Parser::upper_roman_to_usize(detected_enum_str) {
        num
      } else {
        return Err("Couldn't convert upper-case Roman numeral to an integer...\n")
      }
    }
  };

  if detected_enum_str == "i" {
    // LowerRoman list at our hands
    detected_kind = EnumKind::LowerRoman;
    detected_enum_as_usize = 1;
  } else if detected_enum_str == "I"{
    // UpperRoman list at our hands
    detected_kind = EnumKind::LowerRoman;
    detected_enum_as_usize = 1;
  }

  eprintln!("Start index: {}\n", detected_enum_as_usize);

  let node_data = TreeNodeType::EnumeratedList {
    delims: detected_delims,
    kind: detected_kind,
    start_index: detected_enum_as_usize,
    n_of_items: 0,
    enumerator_indent: detected_enumerator_indent,
    text_indent: detected_text_indent,
  };

  eprintln!("List data: {:#?}\n", node_data);

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
  let mut inline_nodes = if let Some(children) = Parser::inline_parse(block, current_line) {
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
