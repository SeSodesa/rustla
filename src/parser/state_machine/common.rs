/// ## common
/// A submodule for transition functions common to all states, such as blank line detection.

use super::*;


/// ### empty_line
/// Simply adds an empty line to the children of the curren node.
pub fn empty_line (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult  {

  let mut tree_wrapper = doctree.unwrap();

  let node = TreeNode::new(TreeNodeType::EmptyLine);

  tree_wrapper.tree.push_child(node);

  TransitionResult::Success {
    doctree: tree_wrapper,
    next_state: None,
    push_or_pop: PushOrPop::Neither,
    line_advance: LineAdvance::Some(1),
  }
  

}