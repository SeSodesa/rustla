/// ## common
/// A submodule for transition functions common to all states, such as blank line detection.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


/// ### empty_line
/// Simply adds an empty line to the children of the curren node.
pub fn empty_line (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult  {

  let doctree = doctree.unwrap();

  // let node = TreeNode::new(TreeNodeType::EmptyLine, doctree.node_count(), None, None);
  
  // match doctree.push_child(node) {
  //   Ok(()) => {},
  //   Err(node) => panic!("Could not insert node into tree on line {}. Computer says no...", line_cursor.sum_total())
  // };

  TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Neither,
    line_advance: LineAdvance::Some(1),
  }
}
