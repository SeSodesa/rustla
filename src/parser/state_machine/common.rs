/// ## common
/// A submodule for transition functions common to all states, such as blank line detection.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


/// ### empty_line
/// Simply adds an empty line to the children of the curren node.
pub fn empty_line (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, current_line: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult  {

  let mut tree_wrapper = doctree.unwrap();

  let node = TreeNode::new(TreeNodeType::EmptyLine, tree_wrapper.node_count, None);

  tree_wrapper.push_child(node);

  TransitionResult::Success {
    doctree: tree_wrapper,
    next_states: None,
    push_or_pop: PushOrPop::Neither,
    line_advance: LineAdvance::Some(1),
  }

}
